use crate::{
    GlobalStates,
    StorageManager,

};
use esp_storage::FlashStorage;
use log::info;

#[embassy_executor::task]
pub async fn manage_global_state(mut manage_storage: StorageManager<FlashStorage<'static>>) {
    let mut state = GlobalStates::IsProvisioned;
    loop {
        match state {
            GlobalStates::IsProvisioned => {
                info!("[Global State: IsProvisioned]");
                state = GlobalStates::Enrollment;
                manage_storage.get_provision_flag(); 
            }

            GlobalStates::Enrollment => {
                info!("Enrollment state");
                state = GlobalStates::StandardComm;
            }
            
            GlobalStates::StandardComm => {
                info!("Standard Communication state");
            }            
        }
    }
}
