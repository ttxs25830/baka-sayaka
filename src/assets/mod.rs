#[cfg(feature = "embed-asset")]
mod embed;
#[cfg(feature = "embed-asset")]
impl<'a> DynData for embed::CowU8Reader<'a> {}

#[cfg(not(feature = "embed-asset"))]
mod unembed;
#[cfg(not(feature = "embed-asset"))]
use std::fs::File;
#[cfg(not(feature = "embed-asset"))]
impl DynData for File {}

use rodio::decoder::Decoder;
use std::io::{Read, Seek};
pub trait DynData: Read + Seek + Send + Sync {}

pub fn get_source() -> Decoder<Box<dyn DynData + 'static>> {
    #[cfg(feature = "embed-asset")]
    let decoder =
        Decoder::new_mp3(Box::new(embed::get_asset()) as Box<dyn DynData + 'static>).unwrap();
    #[cfg(not(feature = "embed-asset"))]
    let decoder =
        Decoder::new_mp3(Box::new(unembed::get_asset()) as Box<dyn DynData + 'static>).unwrap();
    decoder
}
