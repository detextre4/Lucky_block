# Rust Smart Contract - Luky_Game

## Getting started

To deploy contract:

Se puede cambiar semigoso.testnet por la cuenta q se tenga.

1. Create a account to deploy the contract: 
    `near create-account luky.semigoso.testnet --masterAccount semigoso.testnet --initialBalance 50`
2. Construct the .wasm file: ./build.sh
    `./build.sh`
3. Deploy the contract in the account:
    `near deploy $ID --wasmFile res/luky_game.wasm`
4. Initialize the contract:
    `near call luky.semigoso.testnet init_lucky --acountId luky.semigoso.testnet`
5. Create 2 accounts to play using command 1.
6. Call the function check:
    `near view luky.semigoso.testnet check --accountId luky.semigoso.testnet`
7. Call the function first_player from another account:
    `near call luky.semigoso.testnet first_player '{"amount": 1, "vector": [3, 2, 1, 3, 4]}' --accountId bob.luky.semigoso.testnet`
8. Call the function check again to see the status of the game.
9. Call the function second_player from the second account:
    `near call luky.semigoso.testnet first_player '{"vector": [4, 6, 1, 4, 4]}' --accountId alice.luky.semigoso.testnet`
10. Call the function check again to see the status of the game.
11. Call the function check_ganador para revisasr quien es el ganador.

Cosas que faltan:
    - Los near del testnet no se estan enviando.
    - Aun falta la funcion que mande el dinero al ganador.
    - Faltan los unit tests.
# Lucky_block
>>>>>>> 735378f0ad13cc2c16574967b38e62a4f7eacd8e
