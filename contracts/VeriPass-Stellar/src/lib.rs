#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol};


#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Pass(Symbol), // Stores pass details using the pass code as the key
}


#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct VisitorPass {
    pub resident: Address,
    pub expiration: u64,
    pub is_used: bool,
}


#[contract]
pub struct VeriPassContract;


#[contractimpl]
impl VeriPassContract {
    /// Issues a temporary visitor pass with a unique code and expiration timestamp.
    pub fn issue_pass(env: Env, resident: Address, pass_code: Symbol, duration_secs: u64) {
        // Ensure the resident authorizes the creation of this pass
        resident.require_auth();


        let expiration = env.ledger().timestamp().saturating_add(duration_secs);
        let pass = VisitorPass {
            resident,
            expiration,
            is_used: false,
        };


        // Write the pass details to smart contract storage
        env.storage().persistent().set(&DataKey::Pass(pass_code.clone()), &pass);
    }


    /// Verifies the visitor pass at the gate. Can only be invoked by an authorized security guard account.
    pub fn verify_pass(env: Env, guard: Address, pass_code: Symbol) {
        // Ensure only the authorized guard can execute verification
        guard.require_auth();


        let key = DataKey::Pass(pass_code.clone());
       
        // Retrieve the pass or panic if it doesn't exist
        let mut pass: VisitorPass = env.storage().persistent().get(&key).expect("Pass not found");


        // Core business logic validations
        if pass.is_used {
            panic!("Pass has already been used");
        }
        if env.ledger().timestamp() > pass.expiration {
            panic!("Pass has expired");
        }


        // Update state to prevent double-entry
        pass.is_used = true;
        env.storage().persistent().set(&key, &pass);
    }


    /// Read-only utility function to check pass status
    pub fn get_pass(env: Env, pass_code: Symbol) -> VisitorPass {
        env.storage().persistent().get(&DataKey::Pass(pass_code)).expect("Pass not found")
    }
}
