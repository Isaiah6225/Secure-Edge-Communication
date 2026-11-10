use crate::{
    nonce::gen_nonce,
    boot::{
        check_provision, 
        check_ecdsa_set, 
        gen_ecc, 
        gen_ecdh, 
        read_id, 
    },
    formatter::{format_enrollment},
    common::{
        communicators::GSCManager, 
        structs::{
            StorageManager,
            CryptoClient,
            WifiManager
        },
        enums::{
            GlobalStates,
            ProvisionStatus,
            SendSteps,
            WifiResponse
       },
    },
};
use embassy_sync::{
    channel::Channel,
    blocking_mutex::raw::CriticalSectionRawMutex,
};
use esp_storage::FlashStorage;
use log::info;

#[embassy_executor::task]
pub async fn manage_global_state(
    mut manage_storage: StorageManager<FlashStorage<'static>>, 
    gsc_manager: GSCManager,
    gsc_channel: &'static Channel<CriticalSectionRawMutex, SendSteps, 8>,
    crypto_client: CryptoClient,
)
{
    let mut state = GlobalStates::IsProvisioned;

    loop {
        match state {
            GlobalStates::IsProvisioned => {
                let get_pro_flag = manage_storage.get_provision_flag();
                state = GlobalStates::Enrollment
            }

            GlobalStates::Enrollment => {
                //VERIFY KEYS
                info!("[Global State: Enrollment] checking ecc key pair");

                                
                //nvs call to get key pair
                let ecdsa_keys = match manage_storage.get_ecdsa() {
                    Ok((priv_key, pub_key)) => {
                        info!("[Global State: Enrollment] got keys {:?} and {:?}", priv_key, pub_key);
                        Ok((priv_key, pub_key))
                    },

                    Err(e) => {
                        info!("[Global State: Enrollment] got error when retrieving keys: {:?}", e);
                        info!("[Global State: Enrollment] setting keys");
                        state = GlobalStates::IsProvisioned;
                        Err(e)
                    }
                };
                
                //check if get_ecdsa returned an error 
                if let Err(ref e) = ecdsa_keys {
                    info!("[Global State: Enrollment] keys not set in NVS with error: {:?}", e);
                    state = GlobalStates::IsProvisioned;
                };

                let (_ecdsa_priv, ecdsa_pub) = ecdsa_keys.unwrap();

                //INITIAL ENROLLMENT
                info!("[Global State: Enrollment] sending initial enrollment request");

                let mac = read_id::read_mac();
                let nonce = gen_nonce::gen_nonce();
                let init_header_byte = 0;
                let init_packet = format_enrollment::format_enrollment_initial(init_header_byte, &mac, &nonce);
                
                //send and receive initial enrollment
                gsc_manager.send_initial_enrollment(SendSteps::InitialEnrollment(init_packet)).await;
                let initial_enrollment_recv = gsc_manager.receive_initial_enrollment().await;

                if initial_enrollment_recv == WifiResponse::Failure {
                    info!("[Global State: Enrollment] wifi task returned WifiResponse Failure.");
                    state = GlobalStates::IsProvisioned;
                }; 

                info!("{:?}", initial_enrollment_recv);
            }

            GlobalStates::StandardComm => {
                info!("something to go here");
            }
        } 
    }
}
        /*
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
                        let value: u8 = 0; 
                        match manage_storage.set_provision_flag(value) {
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
                                    info!("[Global State: EccStatus::Set] Ecc key pair is set");
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
                            match manage_storage.get_ecc() {
                                Ok((priv_key, pub_key)) => {
                                    info!("[Global State: get_ecdsa_pub] received priv key and pub key with: {:?}  {:?} ", priv_key, pub_key);
                                    info!("[Global State: Enrollment] moving to enrollment steps");
                                    enrollment_steps = EnrollmentSteps::Enrollment(priv_key, pub_key)
                                }


                                Err(_) => {
                                    info!("[Global State: get_ecdsa_pub] error trying to get the ecdsa pub key moving to enrollment");
                                    enrollment_steps = EnrollmentSteps::VerifyKeys;
                                }
                            };
                        }
                        
                        //move to initial communication phase
                        EnrollmentSteps::Enrollment(priv_key, pub_key) => {
                            info!("[Global State: EnrollmentSteps::Enrollment] moving to EnrollmentSteps::Enrollment");
                            gsc_manager.send_enrollment(&SendSteps::Enroll(EnrollmentSteps::Enrollment(priv_key, pub_key))).await;

                            //TODO figure out a better way to create a method around this block
                            match gsc_manager.receive_enrollment().await {
                                WifiCommand::Success => {
                                    info!("[Global State: EnrollmentSteps::Enrollment] success received from wifi_task moving to StandardComm");
                                    state = GlobalStates::StandardComm;
                                }
                                WifiCommand::Failure => {
                                    info!("[Global State: EnrollmentSteps::Enrollment] failure received from wifi_task moving to VerifyKeys");
                                    enrollment_steps = EnrollmentSteps::VerifyKeys;
                                }
                            }
                        }, 
                    }
            }
            
            //standard communication state
            GlobalStates::StandardComm => { 
                info!("Standard Communication state");
                //clearing channel queue for visibility of future messages
                gsc_channel.clear(); 

                //send data to wifi_task

                let std_comm_steps = SendSteps::StdComm(StdCommSteps::StandardCommunication);
                gsc_manager.send_session(&std_comm_steps).await;
                match gsc_manager.receive_session().await {
                    WifiCommand::Success => {
                        info!("[Global State: StandardComm] success received from wifi_task");
                    }
                    WifiCommand::Failure => {
                        info!("[Global State: StandardComm] failure received from wifi_task retrying standard comm");
                        break;
                    }
                }
            }            
        }
        
    }
}
*/
