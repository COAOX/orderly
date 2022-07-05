near deploy $a_id --wasmFile="./res/token.wasm"
near deploy $b_id --wasmFile="./res/token.wasm"
near deploy $amm_id --wasmFile="./res/amm.wasm"
near call $a_id new '{"owner_id":"'$owner_id'", "name":"A Token Contract", "symbol":"A", "total_supply":1000000000000, "decimals": 18}' --accountId=$owner_id
near call $b_id new '{"owner_id":"'$owner_id'", "name":"B Token Contract", "symbol":"B", "total_supply":20000000000000, "decimals": 15}' --accountId=$owner_id
near call $amm_id new '{"gov":"'$owner_id'", "token_a":"'$a_id'", "token_b":"'$b_id'"}' --accountId=$owner_id --gas=55000000000000
