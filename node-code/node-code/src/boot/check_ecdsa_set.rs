//check if ecc key pair is provisioned. 
use crate::common::{
    error::NodeError, 
    enums::EccStatus
};
use log::info;

pub fn check_ecdsa_set(get_ecc: Result<([u8; 32], [u8; 33]), NodeError>) -> EccStatus {
    match get_ecc {
        Ok(keys) => {
            info!("[boot::check_ecc] got keys: {:?}", keys);
            EccStatus::Set
        },
        Err(e) => {
            info!("[boot::check_ecc] error: {:?}", e);
            EccStatus::NotSet
        }
    }
}

