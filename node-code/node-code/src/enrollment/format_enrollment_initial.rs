//Format mac address, verifying key, and device_nonce into a struct
//for initial request from node.
use crate::{
    common::{
        structs::SendPacketInitialEnrl,
        enums::WifiData,
    },
};
use log::info;


pub fn format_enrollment_initial(mac: [u8; 6], sv_key_bytes: [u8; 33], nonce: u32) -> SendPacketInitialEnrl {
    let spi = SendPacketInitialEnrl {dev_mac_add: mac, serialized_vkey:sv_key_bytes, device_nonce: nonce};
    info!("[format_enrollment] initial packet: {}", spi);
    return SendPacketInitialEnrl { dev_mac_add: mac, serialized_vkey: sv_key_bytes, device_nonce: nonce}
}
/*
pub fn format_enrollment_initial_write() {
    loop {
        info!("[wifi_task] EnrollmentSteps::Initial] write retry count before entering if statement: {}", write_retry_count);
        //create send buffer
        if write_retry_count < 3 {
            let mut init_send = String::<1024>::new();
            info!("[wifi_task] EnrollmentSteps::Initial] created buffer to send data to remote server");
            if let Ok(_) = write!(
                init_send,
                "device_id: {:?}, device_pub: {:?}, nonce: {}",
                init_packet.dev_mac_add, init_packet.serialized_vkey, init_packet.device_nonce
            ) {
                info!("[wifi_task] EnrollmentSteps::Initial] successfully wrote data to buffer");

                let init_request = tcp_socket.write(init_send.as_bytes()).await;

                if let Err(e) = init_request {
                    info!("[wifi_task] EnrollmentSteps::Initial] buffer failed to create with: {e:?}. Retrying the buffer creation to send");
                    write_retry_count += 1;
                    info!("[wifi_task] EnrollmentSteps::Initial] write retry count: {}", write_retry_count);
                } else {
                    break;
                }
                info!("[wifi_task] EnrollmentSteps::Initial] successfully sent written buffer");
            } else {
                info!("[wifi_task] EnrollmentSteps::Initial] write failed procing write retry count: {}", write_retry_count);
                write_retry_count += 1;
            }
            /*
            if write!(
                init_send,
                "device_id: {:?}, device_pub: {:?}, nonce: {}",
                init_packet.dev_mac_add, init_packet.serialized_vkey, init_packet.device_nonce
            ).is_ok() {
                let init_request = tcp_socket.write(init_send.as_bytes()).await;
                if let Err(e) = init_request {
                    info!("[wifi_task] EnrollmentSteps::Initial] buffer failed to create with: {e:?}. Retrying the buffer creation to send");
                    write_retry_count += 1;
                    info!("[wifi_task] EnrollmentSteps::Initial] write retry count: {}", write_retry_count);
                } else {
                    break;
                }
            } else {
                info!("[wifi_task] EnrollmentSteps::Initial] write failed procing write retry count: {}", write_retry_count);
                write_retry_count += 1;
            }
            */

        } else {
            info!("[wifi_task] EnrollmentSteps::Initial] write failed after 3 attempts. sending error response to GSC to retry EnrollmentSteps::Initial");
            write_retry_count = 0;
            wtc_sender_handle.send(WifiCommand::Initial).await;
            break;
        }
    }
}
*/
