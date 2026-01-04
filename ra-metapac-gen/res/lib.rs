#![no_std]

pub mod common;
pub mod _peripherals;

#[cfg(feature = "pac")]
pub mod pac {
    include!(env!("RA_METAPAC_PAC_PATH"));
}
#[cfg(feature = "pac")]
pub use pac::*;

#[cfg(feature = "metadata")]
pub mod metadata {
    include!("metadata.rs");
    include!(env!("RA_METAPAC_METADATA_PATH"));
}

pub trait Peripheral {
    #[cfg(feature = "metadata")]
    fn metadata() -> &'static metadata::Peripheral;
}
