use snarkos_dpc::{
    base_dpc::instantiated::*,
    test_data::{generate_test_accounts, ledger_genesis_setup, setup_or_load_parameters, GenesisAttributes},
};
use snarkos_models::{algorithms::CRH, dpc::DPCScheme};
use snarkos_objects::Account;
use snarkos_storage::test_data::*;
use snarkos_utilities::{bytes::ToBytes, to_bytes};

use once_cell::sync::Lazy;
use rand::SeedableRng;
use rand_xorshift::XorShiftRng;

pub static FIXTURE: Lazy<Fixture> = Lazy::new(|| setup(false));
pub static FIXTURE_VK: Lazy<Fixture> = Lazy::new(|| setup(true));

// helper for setting up e2e tests
pub struct Fixture {
    pub parameters: <InstantiatedDPC as DPCScheme<MerkleTreeLedger>>::Parameters,
    pub test_accounts: [Account<Components>; 3],
    pub ledger_parameters: CommitmentMerkleParameters,
    pub genesis_attributes: GenesisAttributes,
    pub predicate: Predicate,
    pub rng: XorShiftRng,
}

impl Fixture {
    pub fn ledger(&self) -> MerkleTreeLedger {
        initialize_test_blockchain(
            self.ledger_parameters.clone(),
            self.genesis_attributes.genesis_cm,
            self.genesis_attributes.genesis_sn,
            self.genesis_attributes.genesis_memo,
        )
    }
}

fn setup(verify_only: bool) -> Fixture {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    // Generate or load parameters for the ledger, commitment schemes, and CRH
    let (ledger_parameters, parameters) = setup_or_load_parameters(verify_only, &mut rng);

    // Generate addresses
    let test_accounts = generate_test_accounts(&parameters, &mut rng);

    let genesis_attributes = ledger_genesis_setup(&parameters, &test_accounts[0], &mut rng);

    let predicate_vk_hash = to_bytes![
        PredicateVerificationKeyHash::hash(
            &parameters.circuit_parameters.predicate_verification_key_hash,
            &to_bytes![parameters.predicate_snark_parameters().verification_key].unwrap()
        )
        .unwrap()
    ]
    .unwrap();

    let predicate = Predicate::new(predicate_vk_hash);

    Fixture {
        parameters,
        test_accounts,
        ledger_parameters,
        genesis_attributes,
        predicate,
        rng,
    }
}
