#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env, Symbol};


    // Test 1: Happy Path - Pass is issued and verified successfully
    #[test]
    fn test_happy_path_verify() {
        let env = Env::default();
        env.mock_all_auths();


        let contract_id = env.register_contract(None, VeriPassContract);
        let client = VeriPassContractClient::new(&env, &contract_id);


        let resident = Address::generate(&env);
        let guard = Address::generate(&env);
        let pass_code = Symbol::new(&env, "PASS123");


        // Issue pass valid for 3600 seconds (1 hour)
        client.issue_pass(&resident, &pass_code, &3600);


        // Verify pass at the gate
        client.verify_pass(&guard, &pass_code);


        // Check state update
        let pass = client.get_pass(&pass_code);
        assert!(pass.is_used);
    }


    // Test 2: Edge Case - Prevent verification if pass is already expired
    #[test]
    #[should_panic(expected = "Pass has expired")]
    fn test_expired_pass_fails() {
        let mut env = Env::default();
        env.mock_all_auths();


        let contract_id = env.register_contract(None, VeriPassContract);
        let client = VeriPassContractClient::new(&env, &contract_id);


        let resident = Address::generate(&env);
        let guard = Address::generate(&env);
        let pass_code = Symbol::new(&env, "PASS_EXP");


        // Issue pass valid for only 10 seconds
        client.issue_pass(&resident, &pass_code, &10);


        // Fast-forward ledger time by 20 seconds to force expiration
        env.ledger().with_mut(|ledger| {
            ledger.timestamp = 20;
        });


        // This must panic
        client.verify_pass(&guard, &pass_code);
    }


    // Test 3: Edge Case - Prevent reuse of an already flagged/used pass
    #[test]
    #[should_panic(expected = "Pass has already been used")]
    fn test_double_use_fails() {
        let env = Env::default();
        env.mock_all_auths();


        let contract_id = env.register_contract(None, VeriPassContract);
        let client = VeriPassContractClient::new(&env, &contract_id);


        let resident = Address::generate(&env);
        let guard = Address::generate(&env);
        let pass_code = Symbol::new(&env, "PASS_REUSE");


        client.issue_pass(&resident, &pass_code, &3600);
        client.verify_pass(&guard, &pass_code);


        // Second verification attempt must fail
        client.verify_pass(&guard, &pass_code);
    }


    // Test 4: State Verification - Confirm state field matching pre/post verification
    #[test]
    fn test_state_integrity() {
        let env = Env::default();
        env.mock_all_auths();


        let contract_id = env.register_contract(None, VeriPassContract);
        let client = VeriPassContractClient::new(&env, &contract_id);


        let resident = Address::generate(&env);
        let pass_code = Symbol::new(&env, "PASS_STATE");


        client.issue_pass(&resident, &pass_code, &1800);


        let pass_state = client.get_pass(&pass_code);
        assert_eq!(pass_state.resident, resident);
        assert_eq!(pass_state.is_used, false);
    }


    // Test 5: Authorization Check - Missing guard authorization fails structural checks
    #[test]
    #[should_panic]
    fn test_unauthorized_guard_fails() {
        let env = Env::default();
        // Explicitly NOT mocking auth to test cryptographic failure properties
       
        let contract_id = env.register_contract(None, VeriPassContract);
        let client = VeriPassContractClient::new(&env, &contract_id);


        let guard = Address::generate(&env);
        let pass_code = Symbol::new(&env, "PASS_AUTH");


        // Will fail because require_auth invoke credentials are empty
        client.verify_pass(&guard, &pass_code);
    }
}
