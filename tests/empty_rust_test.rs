use elrond_wasm::types::{Address, ManagedAddress, BigUint, ManagedBuffer};
use elrond_wasm_debug::{rust_biguint, testing_framework::*, DebugApi};
use ecity_test::*;

const WASM_PATH: &'static str = "output/ecity_test.wasm";

struct ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> ecity_test::ContractObj<DebugApi>,
{
    pub blockchain_wrapper: BlockchainStateWrapper,
    pub owner_address: Address,
    pub user_address: Address,
    pub contract_wrapper: ContractObjWrapper<ecity_test::ContractObj<DebugApi>, ContractObjBuilder>,
}

fn setup_contract<ContractObjBuilder>(
    cf_builder: ContractObjBuilder,
) -> ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> ecity_test::ContractObj<DebugApi>,
{
    let rust_zero = rust_biguint!(0u64);
    let mut blockchain_wrapper = BlockchainStateWrapper::new();
    let owner_address = blockchain_wrapper.create_user_account(&rust_biguint!(5000000000000000u64));
    let user_address = blockchain_wrapper.create_user_account(&rust_zero);
    let cf_wrapper = blockchain_wrapper.create_sc_account(
        &rust_zero,
        Some(&owner_address),
        cf_builder,
        WASM_PATH,
    );

    blockchain_wrapper
        .execute_tx(&owner_address, &cf_wrapper, &rust_zero, |sc| {
            sc.init();
        })
        .assert_ok();

    blockchain_wrapper.add_mandos_set_account(cf_wrapper.address_ref());

    ContractSetup {
        blockchain_wrapper,
        owner_address,
        user_address,
        contract_wrapper: cf_wrapper,
    }
}

#[test]
fn deploy_test() {
    let mut setup = setup_contract(ecity_test::contract_obj);

    // simulate deploy
    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.init();
            },
        )
        .assert_ok();
}

#[test]
fn set_router() {
    let rust_zero = &rust_biguint!(0u64);
    let mut sc_setup = setup_contract(ecity_test::contract_obj);
    let b_wrapper = &mut sc_setup.blockchain_wrapper;
    let user_address = &sc_setup.user_address;
    let owner_address = &sc_setup.owner_address;

    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.set_router(ManagedAddress::from(user_address.clone()));
    }).assert_ok();
}

#[test]
fn add_vesting_schedule() {
    let rust_zero = &rust_biguint!(0u64);
    let mut sc_setup = setup_contract(ecity_test::contract_obj);
    let b_wrapper = &mut sc_setup.blockchain_wrapper;
    let user_address = &sc_setup.user_address;
    let owner_address = &sc_setup.owner_address;

    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.set_router(ManagedAddress::from(user_address.clone()));
    }).assert_ok();

    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.episode_vesting_push(BigUint::from(500000000000000u64));
    }).assert_ok();

    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.episode_vesting_push(BigUint::from(300000000000000u64));
    }).assert_ok();

    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.episode_vesting_push(BigUint::from(100000000000000u64));
    }).assert_ok();

}

#[test]
fn issue_token() {
    let rust_zero = &rust_biguint!(0u64);
    let mut sc_setup = setup_contract(ecity_test::contract_obj);
    let b_wrapper = &mut sc_setup.blockchain_wrapper;
    let user_address = &sc_setup.user_address;
    let owner_address = &sc_setup.owner_address;

    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.set_router(ManagedAddress::from(user_address.clone()));
    }).assert_ok();

    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.episode_vesting_push(BigUint::from(100000000000000u64));
    }).assert_ok();

    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_biguint!(5000000000000000u64), |sc| {
        sc.issue_token(
            BigUint::from(5000000000000000u64),
            ManagedBuffer::from("ECITY"),
            ManagedBuffer::from("ECT"));
    }).assert_ok();
/*
    b_wrapper.execute_tx(&owner_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.premint(
            BigUint::from(1000000u64),
            ManagedAddress::from(user_address.clone()));
    }).assert_ok();

    b_wrapper.execute_tx(&user_address, &sc_setup.contract_wrapper, &rust_zero, |sc| {
        sc.mint();
    }).assert_ok();
*/
}
