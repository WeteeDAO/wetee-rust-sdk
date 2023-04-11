#![allow(unused_imports)]
#![cfg(test)]

use once_cell::sync::OnceCell;
// use crate::hander::{balance::Balance, wetee_app::Wetee};
use sp_core::{sr25519::Pair, Pair as TraitPair};
use sp_runtime::print;
use std::sync::Mutex;
use std::{thread, time::Duration};

use crate::hander::balance::Balance;
use crate::hander::wetee_app::Wetee;
use crate::hander::wetee_asset::WeteeAsset;
use crate::hander::wetee_dao::WeteeDAO;

use super::*;
const SEED: &str = "gloom album notable jewel divorce never trouble lesson month neck sign harbor";
const URL: &str = "ws://127.0.0.1:9944";
pub static DAO_ID: OnceCell<u64> = OnceCell::new();

// #[test]
// pub fn test_seed() {
//     let seed_str = account::generate();
//     let seeds: Vec<&str> = seed_str.split(' ').collect();
//     println!("seed_str => {:?}", seed_str);
//     println!("seeds => {:?}", seeds);
// }

#[test]
fn test_add_seed_keyring() {
    let (address, _) = account::add_keyring_from_seed(SEED.into()).unwrap();
    let pair = account::get_from_address(address.clone()).unwrap();

    let address2 = account::format_public_key::<Pair>(pair.public().into());
    println!("address => {:?} ||| address2 => {:?}", address, address2);

    assert_eq!(address, address2)
}

// #[test]
// fn test_add_keyring() {
//     let key =
//         account::get_seed_phrase(SEED.into(), "test".to_owned(), "123456".to_owned()).unwrap();
//     let jstr = serde_json::to_string(&key).unwrap();
//     println!("jstr => {:?}", jstr);

//     assert!(account::add_keyring(key.clone(), "1234567".to_owned()).is_err());

//     let (address, _ss58address) = account::add_keyring(key, "123456".to_owned()).unwrap();
//     let pair = account::get_from_address(address.clone()).unwrap();

//     let address2 = account::format_public_key::<Pair>(pair.public().into());
//     println!("address => {:?} ||| address2 => {:?}", address, address2);

//     assert_eq!(address, address2)
// }

#[tokio::test]
async fn test_blance() {
    let (address, ss58address) = account::add_keyring_from_seed(SEED.into()).unwrap();
    // 创建连接
    let client: Client = Client::new(URL.to_string()).unwrap();
    println!("address {:?}", ss58address);

    let mut balance = Balance::new(client.clone());
    let (_, _, _, _) = balance.balance(address.clone()).unwrap();

    balance
        .transfer(
            address,
            "0x7cada500b9cc0f99ab1b73e96827fa4c08df23087452f81041361045f70ba254".to_string(),
            1000,
        )
        .unwrap();
}

#[tokio::test]
async fn test_dao() {
    let (address, ss58address) = account::add_keyring_from_seed(SEED.into()).unwrap();
    // 创建连接
    let client = Client::new(URL.to_string()).unwrap();
    println!("address {:?}", ss58address);

    let mut dao = WeteeDAO::new(client);
    let dao_id = dao.nex_dao_id().unwrap();
    println!("dao ===> {}", dao_id);
    DAO_ID.set(dao_id).unwrap();

    dao.create_dao(
        address.clone(),
        "test".to_string(),
        "为了成功".to_string(),
        "{}".to_string(),
    )
    .unwrap();
}

#[tokio::test]
async fn test_dao_asset() {
    let (address, ss58address) = account::add_keyring_from_seed(SEED.into()).unwrap();
    // 创建连接
    let client = Client::new(URL.to_string()).unwrap();
    println!("address {:?}", ss58address);

    let mut dao = WeteeAsset::new(client);
    let dao_id = DAO_ID.get().unwrap();
    dao.create_asset(
        address.clone(),
        dao_id.clone(),
        "TET".to_string(),
        "test".to_string(),
        10000,
        10000,
    )
    .unwrap();

    let asset_b = dao.balance(*dao_id, address).unwrap();
    print!("asset_b => {:?}", asset_b);
}

// #[tokio::test]
// async fn test_wetee() {
//     let (_, ss58address) = account::add_keyring_from_seed(SEED.into()).unwrap();
//     // 创建连接
//     let mut client: Client = Client::new(URL.to_string()).unwrap();

//     println!("address {:?}", ss58address);

//     let (block_number, _) = client.get_block_number().await.unwrap();
//     assert!(block_number > 0);

//     println!("block_number {:?}", block_number);

//     let mut wetee = Wetee::new(client);
//     let pool = wetee.get_wait_pool().await.unwrap();
//     println!("poolpool ===> {:?}", pool);
// }

// #[tokio::test]
// async fn test_wetee_get_app() {
//     let (_, ss58address) = account::add_keyring_from_seed(SEED.into()).unwrap();
//     // 创建连接
//     let mut client: Client = Client::new(URL.to_string()).unwrap();

//     println!("address {:?}", ss58address);

//     let (block_number, _) = client.get_block_number().await.unwrap();
//     assert!(block_number > 0);

//     println!("block_number {:?}", block_number);

//     let mut wetee = Wetee::new(client);
//     let pool = wetee.get_app(1).await.unwrap();
//     println!("poolpool ===> {:?}", pool);
// }

// #[tokio::test]
// async fn test_wetee_run_app() {
//     let (address, ss58address) = account::add_keyring_from_seed(SEED.into()).unwrap();
//     // 创建连接
//     let mut client: Client = Client::new(URL.to_string()).unwrap();

//     println!("address {:?}", ss58address);

//     let (block_number, _) = client.get_block_number().await.unwrap();
//     assert!(block_number > 0);

//     println!("block_number {:?}", block_number);

//     let mut wetee = Wetee::new(client);
//     // let pool = wetee.get_app(1).await.unwrap();

//     wetee.run_app(address, 1).await.unwrap();
//     // println!("poolpool ===> {:?}", pool);
// }

#[tokio::test]
async fn test_sign() {
    let key =
        account::get_seed_phrase(SEED.into(), "test".to_owned(), "123456".to_owned()).unwrap();

    let (address, _ss58address) = account::add_keyring(key, "123456".to_owned()).unwrap();
    let str = account::sign_from_address(address, String::from("test")).unwrap();

    println!("str {:?}", str);
}
