//nuevo
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log ,env, Balance, Promise, AccountId, near_bindgen};
//use rand::Rng; // incompatible con wasm https://github.com/rust-random/rand/issues/715, traté pero no se logró

const NUMERO_JUGADORES: usize = 2;
const NUMERO_RESPUESTAS: usize = 5;
//const YOCTONEAR: u128 = 1_000_000_000_000_000_000_000_000;

fn generate_random_array() -> Vec<usize> {
    //let mut rng = rand::thread_rng();
    //let vals: Vec<usize> = (0..NUMERO_RESPUESTAS).map(|_| rng.gen_range(0,10)).collect();
    //vals
    vec![5,5,5,5,5]
}

fn ganador(vector_1: Vec<usize> , vector_2: Vec<Vec<usize>>) -> usize {
    let reference = &vector_1;
    let player_1 = &vector_2[0];
    let player_2 = &vector_2[1];
    let mut acumulador = 0;
    //let player_1_puntuacion = 0;
    //let player_2_puntuacion = 0;
    for i in 0..NUMERO_RESPUESTAS {
        //print!("Referencia: {}\n", reference[i]);
        //print!("Player 1: {}\n", player_1[i]);
        //print!("Player 2: {}\n", player_2[i]);
        let comparacion = closer(reference[i], player_1[i], player_2[i]);
        //print!("Acumulador += {}\n", comparacion);
        acumulador += comparacion;
    }
    ganador_numero(acumulador)
}

fn closer(ref_ : usize, pl1_: usize, pl2_: usize) -> isize {
    let rest_1 = (ref_ as isize - pl1_ as isize).abs();
    let rest_2 = (ref_ as isize - pl2_ as isize).abs();
    if rest_1 < rest_2 {
        return -1
    } else if rest_1 > rest_2 {
        return 1
    } else {
        return 0
    }
}

fn ganador_numero(number: isize) -> usize {
    if number < 0 {
        return 0
    } else if number > 0 {
        return 1
    } else {
        return 2
    }
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    // SETUP CONTRACT STATE
    respuesta_referencia: Vec<usize>,
    amount_to_debt: Balance,
    jugadores: Vec<AccountId>,
    ready_to_play: bool,
    ready_to_withdraw: bool,
    respuestas: Vec<Vec<usize>>,
    ganador: usize,
}

#[near_bindgen]
impl Contract {
    // ADD CONTRACT METHODS HERE
    #[init]
    pub fn init_lucky() -> Self{
        Self {
            respuesta_referencia: generate_random_array(),
            amount_to_debt: 0,
            jugadores: Vec::new(),
            ready_to_play: false,
            ready_to_withdraw: false,
            respuestas: Vec::new(),
            ganador: 2,
        }
    }

    #[private]
    pub fn check_answers(&self) {
        log!("Referencia {:?}", self.respuesta_referencia);
        log!("Jugador 1 {}: {:?}", self.jugadores[0], self.respuestas[0]);
        log!("Jugador 2 {}: {:?}", self.jugadores[1], self.respuestas[1]);
    }

    pub fn check(&self) {
        let jugadores_ = self.jugadores.len();
        if jugadores_ == NUMERO_JUGADORES {
            env::log_str("The game is finished. Write 'check_ganador' command");
        } else if jugadores_ == 1 {
            env::log_str("Waiting for one more player, please wait.");            
        } else {
            env::log_str("There are no players, you can start the game using 'first_player' command.");
        }
    }

    pub fn check_amount(&self) -> Balance {
        self.amount_to_debt.clone()
        //amount_to_show = self.amount_to_debt.clone();
        //log!("Amount: {} nears", amount_to_show);
    }

    #[payable]
    pub fn first_player(&mut self, vector: Vec<usize>) {
        let jugadores_ = self.jugadores.len();
        assert!(jugadores_ != NUMERO_JUGADORES, "The game finished!");
        let jugador = env::signer_account_id();

        //log!("\nCurrent: {}\nPredecessor: {}\nSigner: {}", env::current_account_id(), env::predecessor_account_id(), env::signer_account_id());

        if jugadores_ == 0 {
            let amount = env::attached_deposit();
            log!("\nAmount: {}", amount);
            env::log_str("You are the first player, you stable the bet amount.");
            self.jugadores.push(jugador);
            self.respuestas.push(vector);
            //Promise::new(env::current_account_id()).transfer(amount*YOCTONEAR); //no funciona, que raro....
            self.amount_to_debt = amount;
            env::log_str("Please, wait until another player wants to participate.");
            env::log_str("You can use the 'check' command.");
        } else if jugadores_ == 1 {
            env::log_str("You are the second player. Check the amount to bet using 'check_amount' command");
            env::log_str("Then, if you are ok with the amount to bet, use the 'second_player' command")
        }
    }

    #[payable]
    pub fn second_player(&mut self, vector: Vec<usize>) {
        //log!("\nCurrent: {}\nPredecessor: {}\nSigner: {}", env::current_account_id(), env::predecessor_account_id(), env::signer_account_id());
        let jugador = env::signer_account_id();
        self.jugadores.push(jugador);
        let jugadores_ = self.jugadores.len();
        assert!(jugadores_ == NUMERO_JUGADORES, "Someone must bet before you.");
        let amount_to_bet = self.amount_to_debt.clone();
        let amount_send = env::attached_deposit();
        assert!(amount_send == amount_to_bet);
        log!("\nYou have bet: {}", amount_send);
        self.respuestas.push(vector);
        //Promise::new(env::current_account_id()).transfer(amount*YOCTONEAR);
        let referencia_clone = self.respuesta_referencia.clone();
        let respuestas_clone = self.respuestas.clone();
        let ganador_numero_ = ganador(referencia_clone, respuestas_clone);
        self.ganador = ganador_numero_;
        let jugadores_aux = self.jugadores.clone();
        let user_ganador = jugadores_aux[ganador_numero_].clone();
        env::log_str("You have played. Find the winner using 'check_ganador' command or check your's account state.");
        Promise::new(user_ganador).transfer(amount_send * 2);
    }

    pub fn check_ganador(&self) {
        
        if self.ganador == 2 { 
            env::log_str("There is not winner.");
        } else {
            log!("Winner: {}", self.jugadores[self.ganador]);  
        }
    }

}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
}
