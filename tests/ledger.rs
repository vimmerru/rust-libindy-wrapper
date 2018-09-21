#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate rmp_serde;
extern crate byteorder;
extern crate rust_libindy_wrapper as indy;
#[macro_use]
mod utils;

use indy::did::Did;
use indy::ErrorCode;
use indy::ledger::Ledger;
use std::sync::mpsc::channel;
use std::time::Duration;
use utils::b58::{FromBase58, IntoBase58};
use utils::constants::{DID_1, INVALID_TIMEOUT, METADATA, PROTOCOL_VERSION, SEED_1, VALID_TIMEOUT, VERKEY_1, VERKEY_ABV_1};
use utils::setup::{Setup, SetupConfig};
use utils::wallet::Wallet;


#[cfg(test)]
mod test_sign_and_submit_request {

    use super::*;


    // see libsovtoken/tests/build_verify_req_test.rs

    const REQUEST_JSON: &str = r#"{
                                  "reqId":1496822211362017764,
                                  "identifier":"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL",
                                  "operation":{
                                       "type":"1",
                                       "dest":"VsKV7grR1BUE29mG2Fm2kX",
                                       "verkey":"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa"
                                       }
                              }"#;


    #[test]
    pub fn sign_and_submit_request_success() {
        indy::pool::Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = indy::pool::Pool::open_ledger(&setup.pool_name, None).unwrap();
        let (did, verkey) = Did::new(wallet.handle, "{}").unwrap();


        let result = indy::ledger::Ledger::sign_and_submit_request(pool_handle, wallet.handle, &did, REQUEST_JSON);

        let mut response : String = "".to_string();

        match result {
            Ok(return_response) => { response = return_response; },
            Err(ec) => { assert!(false, "sign_and_submit_request_success got error code {:?}", ec); },
        }

        indy::pool::Pool::close(pool_handle).unwrap();

        assert!(false, "response {:?}", response);
    }

    #[test]
    pub fn sign_and_submit_request_async_success() {
        indy::pool::Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = indy::pool::Pool::open_ledger(&setup.pool_name, None).unwrap();
        let (did, verkey) = Did::new(wallet.handle, "{}").unwrap();

        let (sender, receiver) = channel();
        let cb = move |ec, stuff| {
            sender.send((ec, stuff)).unwrap();
        };

        indy::ledger::Ledger::sign_and_submit_request_async(pool_handle, wallet.handle, &did, REQUEST_JSON, cb);

        let (ec, stuff) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();

        indy::pool::Pool::close(pool_handle).unwrap();

        assert!(false, "response {:?}", stuff);
    }

    #[test]
    pub fn sign_and_submit_request_timeout_success() {

        indy::pool::Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = indy::pool::Pool::open_ledger(&setup.pool_name, None).unwrap();
        let (did, verkey) = Did::new(wallet.handle, "{}").unwrap();

        let result = indy::ledger::Ledger::sign_and_submit_request_timeout(pool_handle, wallet.handle, &did, REQUEST_JSON, VALID_TIMEOUT);

        match result {
            Ok(str) => {},
            Err(ec) => { assert!(false, "sign_and_submit_request_timeout_success got error code {:?}", ec); },
        }

        indy::pool::Pool::close(pool_handle).unwrap();

    }

    #[test]
    pub fn sign_and_submit_request_timeout_times_out() {
        indy::pool::Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = indy::pool::Pool::open_ledger(&setup.pool_name, None).unwrap();;
        let (did, verkey) = Did::new(wallet.handle, "{}").unwrap();

        let result = indy::ledger::Ledger::sign_and_submit_request_timeout(pool_handle, wallet.handle, &did, REQUEST_JSON, INVALID_TIMEOUT);

        match result {
            Ok(str) => {},
            Err(ec) => { assert!(false, "sign_and_submit_request_timeout_times_out got error code {:?}", ec); },
        }

        indy::pool::Pool::close(pool_handle).unwrap();

    }

}