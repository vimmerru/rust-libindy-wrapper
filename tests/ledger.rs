#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate rmp_serde;
extern crate byteorder;
extern crate rust_libindy_wrapper as indy;
#[macro_use]
mod utils;

use indy::did::Did;
use indy::ErrorCode;
use std::sync::mpsc::channel;
use std::time::Duration;
use utils::b58::{FromBase58, IntoBase58};
use utils::constants::{DID_1, SEED_1, VERKEY_1, METADATA, VERKEY_ABV_1};
use utils::setup::{Setup, SetupConfig};
use utils::wallet::Wallet;


#[cfg(test)]
mod test_sign_and_submit_request {

}