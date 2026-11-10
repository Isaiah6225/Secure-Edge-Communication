use std::{io, env, fmt, fmt::Display};
use crate::common::enums::DBOps;
use sha2::digest;

//error enum
#[derive(Debug)]
pub enum ServerError {
    VarErr(env::VarError),
    IoErr(io::Error),
    SerdeStrErr(serde_json::Error),
    SqlErr(rusqlite::Error),
    OneshotRecvErr(tokio::sync::oneshot::error::RecvError),
    MpscSendErr(tokio::sync::mpsc::error::SendError<DBOps>),
    ReadSigningKeyErr(p256::pkcs8::Error),
    ReadVerifyingKeyErr(p256::pkcs8::spki::Error),
    RandSysErr(rand::rngs::SysError),
    ECDSAErr(p256::ecdsa::Error),
    FmtErr(std::fmt::Error),
    ParseStrErr(std::str::Utf8Error),
    DigestLengthErr(digest::InvalidLength), 
    HkdfErr(hkdf::InvalidLength),
    ECCErr(p256::elliptic_curve::Error),
    AesErr(aes_gcm::Error),
    InfallibleErr(std::convert::Infallible),
    EnrollmentClosedErr,
    CheckDeviceIDErr,
    MissingHeaderByteErr,
    DeviceExistErr,
    EmptyReceiveErr,
    InvalidReceiveEnrollment, 
    InvalidStruct,
    DeviceNotVerifiedErr
}

impl Display for ServerError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServerError::EnrollmentClosedErr => write!(f, "enrollment window closed"),
            ServerError::CheckDeviceIDErr => write!(f, "check device id failed"),
            ServerError::MissingHeaderByteErr => write!(f, "received packet is missing the header byte"),
            ServerError::DeviceExistErr => write!(f, "device exist in device_registry.db"),
            ServerError::EmptyReceiveErr => write!(f, "device sent empty message"),
            ServerError::InvalidReceiveEnrollment => write!(f, "device responded with invalid receive initial enrollment"),
            ServerError::InvalidStruct => write!(f, "unexpected struct received"),
            ServerError::DeviceNotVerifiedErr => write!(f, "device enrollment_status does not equal 'Verified'"),
            _=> Ok(())
        }
    }
}

impl From<env::VarError> for ServerError {
    fn from(error: env::VarError) -> Self {
        ServerError::VarErr(error)
    }
}

impl From<io::Error> for ServerError {
    fn from(error: io::Error) -> Self {
        ServerError::IoErr(error)
    }
}

impl From<serde_json::Error> for ServerError {
    fn from(error: serde_json::Error) -> Self {
        ServerError::SerdeStrErr(error)
    }
}

impl From<rusqlite::Error> for ServerError {
    fn from(error: rusqlite::Error) -> Self {
        ServerError::SqlErr(error)
    }
}

impl From<tokio::sync::oneshot::error::RecvError> for ServerError {
    fn from(error: tokio::sync::oneshot::error::RecvError) -> Self {
        ServerError::OneshotRecvErr(error)
    }
}

impl From<tokio::sync::mpsc::error::SendError<DBOps>> for ServerError {
    fn from(error: tokio::sync::mpsc::error::SendError<DBOps>) -> Self {
        ServerError::MpscSendErr(error)
    }
}

impl From<p256::ecdsa::Error> for ServerError {
    fn from(error: p256::ecdsa::Error) -> Self {
        ServerError::ECDSAErr(error)
    }
}

impl From<p256::pkcs8::Error> for ServerError {
    fn from(error: p256::pkcs8::Error) -> Self {
        ServerError::ReadSigningKeyErr(error)
    }
}

impl From<p256::pkcs8::spki::Error> for ServerError {
    fn from(error: p256::pkcs8::spki::Error) -> Self {
        ServerError::ReadVerifyingKeyErr(error)
    }
}

impl From<rand::rngs::SysError> for ServerError {
    fn from(error: rand::rngs::SysError) -> Self {
        ServerError::RandSysErr(error)
    }
}

impl From<std::fmt::Error> for ServerError {
    fn from(error: std::fmt::Error) -> Self {
        ServerError::FmtErr(error)
    }
}

impl From<std::str::Utf8Error> for ServerError {
    fn from(error: std::str::Utf8Error) -> Self {
        ServerError::ParseStrErr(error)
    }
}

impl From<hkdf::InvalidLength> for ServerError {
    fn from(error: hkdf::InvalidLength) -> Self {
        ServerError::HkdfErr(error)
    }
}

impl From<digest::InvalidLength> for ServerError {
    fn from(error: digest::InvalidLength) -> Self {
        ServerError::DigestLengthErr(error)
    }
}

impl From<p256::elliptic_curve::Error> for ServerError {
    fn from(error: p256::elliptic_curve::Error) -> Self {
        ServerError::ECCErr(error)
    }
}

impl From<aes_gcm::Error> for ServerError {
    fn from(error: aes_gcm::Error) -> Self {
        ServerError::AesErr(error)
    }
}

impl From<std::convert::Infallible> for ServerError {
    fn from(error: std::convert::Infallible) -> Self {
        ServerError::InfallibleErr(error)
    }
}
