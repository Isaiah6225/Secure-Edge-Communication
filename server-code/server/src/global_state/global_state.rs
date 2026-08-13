use crate::{
    common::{
        structs::{Device, DBClient},
        errors::ServerError
    },
    enrollment_checks::{
        enrollment_time,
        check_device_id
    },
};
use tokio::{
    net::TcpStream,
};

pub async fn manage_enrollment(stream: TcpStream, data_parsed: Device, mut db_client: DBClient) -> Result<(), ServerError>{
    //complete enrollment checks
    enrollment_time::check_window()?;
    check_device_id::check_id(&data_parsed.device_id)?;
    db_client.check_dev_db(&data_parsed.device_id, &data_parsed.device_pub).await?;
    db_client.save_dev_db(&data_parsed.device_id, &data_parsed.device_pub, &data_parsed.nonce).await?;


    Ok(())
}
