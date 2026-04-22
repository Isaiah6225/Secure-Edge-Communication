use esp_nvs::Nvs;
use esp_storage::FlashStorage;
use crate::{
    common::error::NodeError,
};

pub fn set_nvs_handle(storage: FlashStorage<'_>) -> Result<Nvs<FlashStorage<'_>>, NodeError> {
    let partition_offset = 0x390000;
    let partition_size = 0x32000;
    
    let nvs_handle = Nvs::new(partition_offset, partition_size, storage)?;
    Ok(nvs_handle)
}

