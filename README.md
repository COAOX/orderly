# Orderly-Test
## Create Accounts
In order to test the AMM contract, we first need three different accounts to deploy TokenA, TokenB and the AMM contract respectively. In Near we can create three sub-accounts using the same master account. Let's say the master account is `orderly-test.testnet`, you may change it to your account to test in your self. We can use the below command to create three subaccounts.
```bash
master_account=orderly-test.testnet
```

```bash
token_a=tokena.$master_account
token_b=tokenb.$master_account
amm_account=amm.$master_account

near login
near create-account $token_a --masterAccount $master_account --initialBalance 30
near create-account $token_b --masterAccount $master_account --initialBalance 30
near create-account $amm_account --masterAccount $master_account --initialBalance 30
```

## Build and Deploy the Contract
```bash
chmod 777 ./build.sh
./build.sh

near deploy $token_a --wasmFile="./res/token.wasm"
near deploy $token_b --wasmFile="./res/token.wasm"
near deploy $amm_account --wasmFile="./res/amm.wasm"
```

## Init the Contract(can only call once)
The AMM contract will automatically create wallets in tokenA and tokenB after initialization.
```bash
near call $token_a new '{"owner_id":"'$master_account'", "name":"A Token Contract", "symbol":"A", "total_supply":1000000000000, "decimals": 18}' --accountId=$master_account
near call $token_b new '{"owner_id":"'$master_account'", "name":"B Token Contract", "symbol":"B", "total_supply":20000000000000, "decimals": 15}' --accountId=$master_account
near call $amm_account new '{"gov":"'$master_account'", "token_a":"'$token_a'", "token_b":"'$token_b'"}' --accountId=$master_account --gas=55000000000000
```

## Pre-processing before testing
After we finish initializing the contract, we can use the following command to view the token contract and the metadata of the AMM contract.
```bash
# view the metadata of token contract
near view $token_a ft_balance_of '{"account_id": "'$master_account'"}'
near view $token_b ft_balance_of '{"account_id": "'$master_account'"}'
# view the metadata of amm contract
near view $amm_account amm_metadata
```

In order for AMM to start exchanging, the AMM owner needs to transfer the token to amm to initialize the ratio.
```bash
near call $token_a ft_transfer_call '{"receiver_id":"'$amm_account'", "amount":"10000000000", "msg":"CHANGE_K"}' --accountId=$master_account --deposit=0.000000000000000000000001 --gas=95000000000000
near call $token_b ft_transfer_call '{"receiver_id":"'$amm_account'", "amount":"50000000000", "msg":"CHANGE_K"}' --accountId=$master_account --deposit=0.000000000000000000000001 --gas=95000000000000
```

Check if the ratio has been changed.
```bash
near view $amm_account amm_metadata
```

Now the AMM Contract is ready for exchange.

## Test the Contract
Send some token A to the amm contract and the amm contract will send you some token b back.
```bash
# check the Balance
near view $token_a ft_balance_of '{"account_id": "'$master_account'"}'
near view $token_b ft_balance_of '{"account_id": "'$master_account'"}'

near call $token_a ft_transfer_call '{"receiver_id":"'$amm_account'", "amount":"50000000000", "msg":""}' --accountId=$master_account --deposit=0.000000000000000000000001 --gas=205000000000000

# check the Balance
near view $token_a ft_balance_of '{"account_id": "'$master_account'"}'
near view $token_b ft_balance_of '{"account_id": "'$master_account'"}'
```

Then use some tokenB to exchange tokenA
```bash
# check the Balance
near view $token_a ft_balance_of '{"account_id": "'$master_account'"}'
near view $token_b ft_balance_of '{"account_id": "'$master_account'"}'

near call $token_b ft_transfer_call '{"receiver_id":"'$amm_account'", "amount":"50000000000", "msg":""}' --accountId=$master_account --deposit=0.000000000000000000000001 --gas=205000000000000

# check the Balance
near view $token_a ft_balance_of '{"account_id": "'$master_account'"}'
near view $token_b ft_balance_of '{"account_id": "'$master_account'"}'
```

With the log and balance we think the amm contract is working properly.