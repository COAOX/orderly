# Orderly-Test
## Create Accounts
In order to test the AMM contract, we first need three different accounts to deploy TokenA, TokenB and the AMM contract respectively. In Near we can create three sub-accounts using the same master account. Let's say the master account is `orderly-test.testnet`. We can use the below command to create three subaccounts.
```bash
master_account=orderly-test.testnet
token_a=tokena.orderly-test.testnet
token_b=tokenb.orderly-test.testnet
amm_account=amm.orderly-test.testnet

near login
near create-account $token_a --masterAccount $master_account
near create-account $token_b --masterAccount $master_account
near create-account $amm_account --masterAccount $master_account
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
```bash
near call $a_id new '{"owner_id":"'$owner_id'", "name":"A Token Contract", "symbol":"A", "total_supply":1000000000000, "decimals": 18}' --accountId=$owner_id
near call $b_id new '{"owner_id":"'$owner_id'", "name":"B Token Contract", "symbol":"B", "total_supply":20000000000000, "decimals": 15}' --accountId=$owner_id
near call $amm_id new '{"gov":"'$owner_id'", "token_a":"'$a_id'", "token_b":"'$b_id'"}' --accountId=$owner_id --gas=55000000000000
```

## Test the Contract
```bash
near view $token_a ft_balance_of '{"account_id": "'$master_account'"}'
near view $token_b ft_balance_of '{"account_id": "'$master_account'"}'
near view $token_a ft_balance_of '{"account_id": "'$amm_account'"}'
near view $token_b ft_balance_of '{"account_id": "'$amm_account'"}'
```


