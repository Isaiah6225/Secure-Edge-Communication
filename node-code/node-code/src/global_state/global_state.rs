use crate::{
    boot::{check_provision, check_ecdsa_set, gen_ecc},
    common::{
        structs::{
            StorageManager,
            GSCManager
        },
        enums::{
            GlobalStates,
            ProvisionStatus,
            EnrollmentSteps,
            EccStatus,
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
    let mut enrollment_steps = EnrollmentSteps::VerifyKeys; 

    loop {
        match state {
            //provisioing check
            GlobalStates::IsProvisioned => {
                info!("[Global State: IsProvisioned]");


                //get provisioning flag and move to the standard comm or enrollment
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
                    //Enrollment sub steps 
                    match enrollment_steps {

                        //ensure ecdsa key pair is set and get public key 
                        EnrollmentSteps::VerifyKeys => {
                            info!("[Global State: Enrollment] checking ecc key pair");
                            let get_ecc_keys = manage_storage.get_ecc();

                            //check to see if the key pair was set properly
                            match check_ecdsa_set(get_ecc_keys) {
                                EccStatus::Set => {
                                    info!("[Global State: EccStatus::Set] Ecc key pair is set whoo");
                                } 

                                EccStatus::NotSet => {
                                    info!("[Global State: EccStatus::NotSet] Ecc key pair is not set booo, setting it now");
                                    let (priv_key, pub_key) = gen_ecc::gen_key_pair();
                                    match manage_storage.set_ecc(&priv_key, &pub_key) {
                                        Ok(()) => {
                                            info!("[Global State: EccStatus::NotSet] set ecdsa key pair moving to getting pub key");
                                        }, 
                                        Err(e) => {
                                            enrollment_steps = EnrollmentSteps::VerifyKeys;
                                            info!("[Global State: EccStatus::NotSet] error trying to set ecc key pair moving to enrollment error: {:?}", e);
                                        }
                                    }
                                }
                            }

                            //get verifying key from nvs
                            match manage_storage.get_ecdsa_pub() {
                                Ok(pub_key) => {
                                    info!("[Global State: get_ecdsa_pub] received pub key from nvs: {:?}", pub_key);
                                    info!("[Global State: Enrollment] moving to enrollment steps");
                                    enrollment_steps = EnrollmentSteps::Initial(pub_key)
                                }


                                Err(_) => {
                                    info!("[Global State: get_ecdsa_pub] error trying to get the ecdsa pub key moving to enrollment");
                                    enrollment_steps = EnrollmentSteps::VerifyKeys;
                                }
                            };
                        }
                        
                        //move to initial communication phase
                        EnrollmentSteps::Initial(pub_key) => {
                            info!("[Global State: EnrollmentSteps::Initial] moving to EnrollmentSteps::Initial");
                            gsc_manager.send_enrollment(&EnrollmentSteps::Initial(pub_key)).await;
                            enrollment_steps = gsc_manager.receive_enrollment().await;
                        }, 
                        
                        //move to final verification communication phase 
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
