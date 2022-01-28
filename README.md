# Rust Smart Contract - Lucky_Game

## About the project

El proyecto trata acerca de un juego de apuestas. 2 jugadores se necesitan para jugar. 
El juego consiste en que cada jugador debe ingresar una arreglo de 5 números, del 0 al 9. 
El contracto trae un arreglo de 5 números random.
El que tenga su número más cercano al del contrato, gana un punto. Gana el que tenga más puntos.
El ganador se lleva todo el dinero.

## Getting started

To deploy contract:

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
    `near call luky.semigoso.testnet first_player '{"vector": [3, 2, 1, 3, 4]}' --accountId bob.luky.semigoso.testnet --amount 2`
8. Call the function check again to see the status of the game.
9. Call the function second_player from the second account:
    `near call luky.semigoso.testnet first_player '{"vector": [4, 6, 1, 4, 4]}' --accountId alice.luky.semigoso.testnet --amount 2`
10. Call the function check again to see the status of the game.
11. Call the function check_ganador para revisas quien es el ganador.
12. Revisa tu estado o llama a la funcion check_ganador.

Se puede cambiar semigoso.testnet por la cuenta q se tenga.
Se pueden cambiar el amount que se apuesta.
Es importante que el segundo jugador acepte el amount de apuesta.

Cosas que faltan:
    - Faltan los unit tests.
    
# Lucky_block
>>>>>>> 735378f0ad13cc2c16574967b38e62a4f7eacd8e

# link hacia el modelo de la app:
https://www.figma.com/proto/fLIPUHFqFMpNqtkdX7QLIW/LuckyBlock?node-id=31%3A3&scaling=scale-down&page-id=0%3A1&starting-point-node-id=31%3A3
