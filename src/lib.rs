#![no_std]
use rand::{Error, RngCore, SeedableRng};

use rand::rngs::SmallRng as Rng;
use soroban_sdk::{Env, symbol, Symbol};

const SOROBAN_RAND_KEY: Symbol = symbol!("SOROBANRND");

pub struct SorobanRng {
    rng: Rng,
}

impl SorobanRng {
    pub fn init(e: Env) -> SorobanRng {

        // Seed from the contract deployment context
        let contract_id = e.current_contract().to_array();
        let h: [u8; 8] = contract_id.split_at(8).0.try_into().expect("msg");
        let l: [u8; 8] = contract_id.split_at(24).1.try_into().expect("msg");
        let sum: u64 = u64::from_be_bytes(h).wrapping_add(u64::from_be_bytes(l));

        // Seed from the contract execution context
        let nonce: u32 = e.data().get(SOROBAN_RAND_KEY).unwrap_or(Ok(1)).unwrap();

        // Update the nonce
        e.data().set(SOROBAN_RAND_KEY, nonce+1);

        // Seed from the ledger context
        let time = e.ledger().timestamp().wrapping_mul(e.ledger().sequence() as u64);
        
        // Maybe a better formula can be found
        // ((timestamp * sequence) + (h+l)) * nonce
        let state: u64 = sum.wrapping_add(time).wrapping_mul(nonce.into());

        SorobanRng {
            rng: Rng::seed_from_u64(state)
        }
    }
}

impl RngCore for SorobanRng {
    #[inline(always)]
    fn next_u32(&mut self) -> u32 {
        self.rng.next_u32()
    }

    #[inline(always)]
    fn next_u64(&mut self) -> u64 {
        self.rng.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.rng.fill_bytes(dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.rng.try_fill_bytes(dest)
    }
}

#[cfg(test)]
mod tests {
    mod contract {
        use soroban_sdk::{contractimpl, Env};

        use crate::SorobanRng;

        use rand::Rng;

        pub struct RandomTest;

        #[contractimpl]
        impl RandomTest {
            pub fn random(env: Env, max: u32) -> u32 {

                let mut rng = SorobanRng::init(env);
                rng.gen_range(0..max)
            }
        }
    }
    extern crate std;
    use std::{println, time::UNIX_EPOCH};

    use soroban_sdk::{Env, testutils::{Ledger, self}};

    use crate::{tests::contract::RandomTestClient};

    #[test]
    fn test() {
        
            let env = Env::default();
    
            env.ledger().set(testutils::LedgerInfo {
                protocol_version: 0,
                sequence_number: 0,
                timestamp: std::time::SystemTime::now().duration_since(UNIX_EPOCH).expect("Time flies").as_secs(),
                network_passphrase: [0u8].to_vec(),
                base_reserve: 0,
            });
            let contract_id = env.register_contract(None, contract::RandomTest);
            let rolling_dice =  RandomTestClient::new(&env, contract_id);
            let r = rolling_dice.random(&10);

            println!("Random max 10: {r}");
    }
    
    #[test]
    fn test_limit_sequence() {
        let env = Env::default();


        env.ledger().set(testutils::LedgerInfo {
            protocol_version: 0,
            sequence_number: u32::MAX,
            timestamp: std::time::SystemTime::now().duration_since(UNIX_EPOCH).expect("Time flies").as_secs(),
            network_passphrase: [0u8].to_vec(),
            base_reserve: 0,
        });

        let contract_id = env.register_contract(None, contract::RandomTest);
            let rolling_dice =  RandomTestClient::new(&env, contract_id);
            let r = rolling_dice.random(&100);

            println!("Random max 100: {r}");
        
    }
    #[test]
    fn test_limit_all() {
        let env = Env::default();


        env.ledger().set(testutils::LedgerInfo {
            protocol_version: 0,
            sequence_number: u32::MAX,
            timestamp: u64::MAX - 1,
            network_passphrase: [0u8].to_vec(),
            base_reserve: 0,
        });

        let contract_id = env.register_contract(None, contract::RandomTest);
            let rolling_dice =  RandomTestClient::new(&env, contract_id);
            let r = rolling_dice.random(&1000);

            println!("Random max 1000: {r}");
        
    }
}
