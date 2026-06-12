use crate::{
    boot::check_provision,
    common::{
        structs::{
            StorageManager,
            GSCManager
        },
        enums::{
            GlobalStates,
            ProvisionStatus,
            EnrollmentSteps, 
        }
    },
    
};
use esp_storage::FlashStorage;
use log::info;


#[embassy_executor::task]
pub async fn manage_global_state(
    mut manage_storage: StorageManager<FlashStorage<'static>>, 
    gsc_manager: GSCManager,
)
{
    let mut state = GlobalStates::IsProvisioned;
    let mut enrollment_steps = EnrollmentSteps::Initial; 

    loop {
        match state {
            //provisioing check
            GlobalStates::IsProvisioned => {
                info!("[Global State: IsProvisioned]");
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
                info!("[Global State: Enrollment] moving to enrollment steps");
                match enrollment_steps {
                    EnrollmentSteps::Initial => {
                        info!("[Global State: EnrollmentSteps::Initial] moving to EnrollmentSteps::Initial");
                        gsc_manager.send_enrollment(&enrollment_steps).await;
                        enrollment_steps = gsc_manager.receive_enrollment().await;
                    }, 
                    
                    EnrollmentSteps::FinalVerification => {
                        info!("[Global State: EnrollmentSteps::FinalVerification] moving to EnrollmentSteps::FinalVerification");
                        state = GlobalStates::StandardComm;
                    }
                } 
            }
            
            //standard communication state
            GlobalStates::StandardComm => {
                info!("Standard Communication state");
            }            
        }
    }
}
