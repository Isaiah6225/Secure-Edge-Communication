use crate::{
    common::{
        structs::{Device, DeviceEnrl, DeviceStdComm},
        errors::ServerError,
        enums::ParsedStruct
    },
};

pub fn parse(data: &str) -> Result<ParsedStruct, ServerError> {
    let init_data = Device::new(data);
    let header_byte = init_data.unwrap().header_byte;

    if header_byte == 0 {
        let device_enrl = DeviceEnrl::new(data)?;
        return Ok(ParsedStruct::DeviceEnrlParsed(device_enrl))
    } else if header_byte == 1 {
        let device_stdcomm = DeviceStdComm::new(data)?;
        return Ok(ParsedStruct::DeviceStdCommParsed(device_stdcomm))
    } else {
        return Err(ServerError::MissingHeaderByteErr)
    }
}
