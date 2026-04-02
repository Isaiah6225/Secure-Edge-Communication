use esp_hal::{
    rng::{Trng, TrngSource, TrngError}
};
use log::info; 
use crate::{
    TrngWrapper,
    NodeError
}; 


pub fn gen_nonce(_trng_source: TrngSource<'static>) -> Result<u32, NodeError>{
    let trng = Trng::try_new();
    
    let wrapper = TrngWrapper(match trng {
        Ok(trng) => trng, 
        Err(e) => {
            info!("{:?}", e);
            return Err(NodeError::Rng(TrngError::TrngSourceNotEnabled));
        }
    });

    let nonce = wrapper.0.random();
    info!("Node nonce {:?}", nonce);
    Ok(nonce)
}
