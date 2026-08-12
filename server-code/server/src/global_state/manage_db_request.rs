use tokio::{
    net::TcpStream,
    sync::{
        oneshot,
        mpsc::Sender,
    }
};
use crate::{
    common::{
        enums::{
            DBOps,
            ResultDBOps, 
            DBSave
        },
        structs::Device,
        errors::ServerError,
    },
};

pub async fn manage_check_dev(db_tx_clone: Sender<DBOps>, data_parsed: Device) -> Result<(), ServerError> {
    let (tx, rx) = oneshot::channel();
    db_tx_clone.send(DBOps::CheckDevice(tx, data_parsed));

    rx.await?
}   
