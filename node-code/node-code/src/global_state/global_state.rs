use crate::{
    GlobalState
};

pub async fn manage_global_state() {
    let state = GlobalState::IsProvisioned;
    loop {
        match state {
            GlobalState::IsProvisioned => {

            }
        }
    }
}
