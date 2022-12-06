#![no_std]
use rand::{Error, RngCore, SeedableRng};

use rand::rngs::SmallRng as Rng;
use soroban_sdk::Env;

pub struct SorobanRng {
    rng: Rng,
}

impl SorobanRng {
    pub fn init(e: Env) -> SorobanRng {

        // Maybe find a better seed
        // let h: [u8; 8] = e.current_contract().to_array().split_at(8).0.try_into().expect("Contract ID too short");
        // let state: u64 = e.ledger().timestamp()  * u64::from_le_bytes(h);
        let state: u64 = e.ledger().timestamp().wrapping_mul(e.ledger().sequence() as u64);
        SorobanRng {
            rng: Rng::seed_from_u64(state as u64),
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
    extern crate std;
    use std::{println, time::UNIX_EPOCH};

    use rand::{RngCore, Rng, rngs::SmallRng};
    use soroban_sdk::{Env, testutils::{Ledger, self}};

    use crate::SorobanRng;

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

        let mut rng = SorobanRng::init(env);

        
        let len = 1000000;
        let mut dist: [i32; 10] = [0; 10];
        for _i in 1..len {
            dist[rng.gen_range(0..dist.len())] += 1;
        }

        for n in dist {
            println!("{n}");
            assert!(n > 99000);
        }        
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

        let mut rng = SorobanRng::init(env);

        let len = 1000000;
        let mut dist: [i32; 10] = [0; 10];
        for _i in 1..len {
            dist[rng.gen_range(0..dist.len())] += 1;
        }

        for n in dist {
            println!("{n}");
            assert!(n > 99000);
        }   
        
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

        let mut rng = SorobanRng::init(env);

        let len = 1000000;
        let mut dist: [i32; 10] = [0; 10];
        for _i in 1..len {
            dist[rng.gen_range(0..dist.len())] += 1;
        }

        for n in dist {
            println!("{n}");
            assert!(n > 99000);
        }   
        
    }
}
