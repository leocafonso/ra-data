#![no_std]

pub mod common;
pub mod _peripherals;

#[cfg(feature = "pac")]
include!(env!("RA_METAPAC_PAC_PATH"));

#[cfg(feature = "metadata")]
pub mod metadata {
    include!("metadata.rs");
    include!(env!("RA_METAPAC_METADATA_PATH"));
}
