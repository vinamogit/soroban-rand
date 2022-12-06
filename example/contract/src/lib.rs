#![no_std]

use soroban_sdk::{contractimpl, Env};

use soroban_rand::SorobanRng;

use rand::Rng;

pub struct RollingDice;

#[contractimpl]
impl RollingDice {
    
    pub fn roll(env: Env, max: u32) -> u32 {

        let mut rng = SorobanRng::init(env);
        rng.gen_range(0..max)
    }
}

