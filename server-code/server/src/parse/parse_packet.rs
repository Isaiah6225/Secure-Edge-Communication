use crate::{
    common::{
        structs::{HeaderByte, DeviceEnrl, DeviceStdComm, IsValid},
        errors::ServerError,
        enums::ParsedStruct
    },
};

pub fn parse(data: &str) -> Result<ParsedStruct, ServerError> {
    let init_data = HeaderByte::new(data);
    let header_byte = init_data.unwrap().header_byte;
        
    match header_byte {
        0 => { 
            let device_enrl = DeviceEnrl::new(data)?;
            return Ok(ParsedStruct::DeviceEnrlParsed(device_enrl))
        }, 
        100 => {
            let receive_initial_enrl = IsValid::new(data)?; 
            return Ok(ParsedStruct::DeviceReceiveInitialEnrlParsed(receive_initial_enrl))
        },
        1 => {
            let device_enrl = DeviceStdComm::new(data)?;
            return Ok(ParsedStruct::DeviceStdCommParsed(device_enrl))
        }, 
        _ => return Err(ServerError::MissingHeaderByteErr)
    }
}

