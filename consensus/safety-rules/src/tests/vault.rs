// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::{tests::suite, PersistentSafetyStorage, SafetyRulesManager, TSafetyRules};
use libra_secure_storage::{BoxedStorage, KVStorage, VaultStorage};
use libra_types::{validator_signer::ValidatorSigner, waypoint::Waypoint};

/// A test for verifying VaultStorage properly supports the SafetyRule backend.  This test
/// depends on running Vault, which can be done by using the provided docker run script in
/// `docker/vault/run.sh`
#[ignore]
#[test]
fn test() {
    suite::run_test_suite(safety_rules);
}

fn safety_rules() -> (Box<dyn TSafetyRules>, ValidatorSigner) {
    let signer = ValidatorSigner::from_int(0);
    let host = "http://localhost:8200".to_string();
    let token = "root_token".to_string();
    let mut storage = BoxedStorage::from(VaultStorage::new(host, token, None, None));
    storage.reset_and_clear().unwrap();

    let waypoint = Waypoint::default();
    let storage =
        PersistentSafetyStorage::initialize(storage, signer.private_key().clone(), waypoint);
    let safety_rules_manager = SafetyRulesManager::new_local(signer.author(), storage);
    let safety_rules = safety_rules_manager.client();
    (safety_rules, signer)
}
