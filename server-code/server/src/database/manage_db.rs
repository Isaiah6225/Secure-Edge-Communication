use crate::{
    database::*,
    common::enums::DBOps,
};
use rusqlite::Connection; 
use tokio::{
    sync::{
        mpsc::Receiver
    }
};


pub async fn manage_db(db_conn: Connection, rx_mpsc: Receiver<DBOps>) {
    loop {
        
    }
}
