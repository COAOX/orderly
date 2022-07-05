near call $amm_id amm_metadata --accountId=liangx.testnet               


near call $a_id ft_transfer_call '{"receiver_id":"amm1.liangx.testnet", "amount":"10000000000", "msg":"CHANGE_K"}' --accountId=liangx.testnet --deposit=0.000000000000000000000001 --gas=95000000000000
near call $b_id ft_transfer_call '{"receiver_id":"amm1.liangx.testnet", "amount":"50000000000", "msg":"CHANGE_K"}' --accountId=liangx.testnet --deposit=0.000000000000000000000001 --gas=95000000000000


near call $b_id ft_transfer_call '{"receiver_id":"amm1.liangx.testnet", "amount":"50000000000", "msg":""}' --accountId=liangx.testnet --deposit=0.000000000000000000000001 --gas=205000000000000


near call $b_id ft_balance_of '{"account_id":"liangx.testnet"}' --accountId=liangx.testnet
near call $a_id ft_balance_of '{"account_id":"liangx.testnet"}' --accountId=liangx.testnet