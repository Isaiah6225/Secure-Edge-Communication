//check if ecc key pair is provisioned. 
use crate::common::error::NodeError;
use log::info;

pub fn check_ecdsa_set(get_ecc: Result<([u8; 32], [u8; 33]), NodeError>) ->  Result<(), NodeError> {
    match get_ecc {
        Ok(keys) => {
            info!("[boot::check_ecc] got keys: {:?}", keys);
            Ok(())
        },
        Err(e) => {
            info!("[boot::check_ecc] error: {:?}", e);
            Err(e)
        }
    }
}

