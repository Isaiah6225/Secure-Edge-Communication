//check if provisioned to server
use crate::{
    NodeError, 
    ProvisionStatus
};
use log::info;

pub fn check_provision(get_pro: Result<u8, NodeError>) -> ProvisionStatus {
    match get_pro {
        Ok(provision_flag) => {
            if provision_flag == 1 {
                return ProvisionStatus::Provisioned
            } else {
                return ProvisionStatus::NotProvisioned
            }
        },
        Err(e) => {
            info!("{:?}", e);
            return ProvisionStatus::NotSet
        }
    }
}
