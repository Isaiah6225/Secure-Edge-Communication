use crate::{
    GlobalStates,
    StorageManager
};
use esp_storage::FlashStorage;
use log::info;

#[embassy_executor::task]
pub async fn manage_global_state(manage_storage: StorageManager<FlashStorage<'static>>) {
    let mut state = GlobalStates::IsProvisioned;
    loop {
        match state {
            GlobalStates::IsProvisioned => {
                info!("provisioned state");
                state = GlobalStates::Enrollment;
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
