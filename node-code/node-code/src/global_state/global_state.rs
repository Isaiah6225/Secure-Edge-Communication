use crate::{
    boot::check_provision,
    common::{
        structs::StorageManager,
        enums::{
            GlobalStates,
            ProvisionStatus,
        }
    },
    
};
use esp_storage::FlashStorage;
use log::info;

#[embassy_executor::task]
pub async fn manage_global_state(mut manage_storage: StorageManager<FlashStorage<'static>>) {
    let mut state = GlobalStates::IsProvisioned;
    loop {
        match state {
            //provisioing check
            GlobalStates::IsProvisioned => {
                info!("[Global State: IsProvisioned]");
                state = GlobalStates::Enrollment;
                let get_pro_flag = manage_storage.get_provision_flag();

                match check_provision(get_pro_flag) {
                    ProvisionStatus::Provisioned => {
                        info!("[Global State: IsProvisioned] provisioned moving to standard communication");
                        state = GlobalStates::StandardComm;
                    } 

                    ProvisionStatus::NotProvisioned => {
                        info!("[Global State: IsProvisioned] not provisioned moving to enrollment");
                        state = GlobalStates::Enrollment;
                    }

                    ProvisionStatus::NotSet => {
                        info!("[Global State: IsProvisioned] provision flag not set setting to 0 moving to enrollment");
                        //replaying 'IsProvisioned' state in case the flag cannot be set. 
                        match manage_storage.set_provision_flag() {
                            Ok(()) => {
                                state = GlobalStates::Enrollment;
                            }, 

                            Err(_) => {
                                state = GlobalStates::IsProvisioned; 
                            }
                        }
                    }
                }
            }
            
            //enrollment states
            GlobalStates::Enrollment => {
                info!("[Global State: Enrollment]");
                state = GlobalStates::StandardComm;
            }
            
            //standard communication state
            GlobalStates::StandardComm => {
                info!("Standard Communication state");
            }            
        }
    }
}
