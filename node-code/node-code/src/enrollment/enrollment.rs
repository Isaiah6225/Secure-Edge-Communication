use crate::{
    enrollment::format_data, 
};
use esp_hal::rng::TrngSource;


#[embassy_executor::task]
pub async fn enroll (trng_source: TrngSource<'static>) {
    format_data::format_packet(trng_source);
}
