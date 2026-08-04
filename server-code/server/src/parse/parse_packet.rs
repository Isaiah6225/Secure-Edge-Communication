use crate::{
    common::{
        structs::Device,
        errors::ServerError
    },
};

pub fn parse(data: &str) -> Result<Device, ServerError> {
    let data_parsed = Device::new(data);
    data_parsed
}
