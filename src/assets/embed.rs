use rust_embed::RustEmbed;
use std::borrow::Cow;

#[derive(RustEmbed)]
#[folder = "music"]
struct SongsAsset;

pub fn get_asset() -> CowU8Reader<'static> {
    CowU8Reader::from(SongsAsset::get("Decretum.mp3").unwrap().data)
}
pub struct CowU8Reader<'a> {
    data: Cow<'a, [u8]>,
    index: usize,
}
impl<'a> From<Cow<'a, [u8]>> for CowU8Reader<'a> {
    fn from(value: Cow<'a, [u8]>) -> Self {
        Self {
            data: value,
            index: 0,
        }
    }
}
impl<'a> std::io::Read for CowU8Reader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        use std::cmp::min;

        let buf_len = buf.len();
        let data_len = self.data.len() - self.index;
        let read_len = min(buf_len, data_len);
        buf[0..read_len].copy_from_slice(&self.data[self.index..(self.index + read_len)]);
        self.index += read_len;
        Ok(read_len)
    }
}
impl<'a> std::io::Seek for CowU8Reader<'a> {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        use std::io;
        let index = match pos {
            std::io::SeekFrom::Start(index) => index.try_into().map(|i| Some(i)),
            std::io::SeekFrom::End(index) => index
                .try_into()
                .map(|index: isize| index.checked_sub_unsigned(self.data.len())),
            std::io::SeekFrom::Current(index) => index
                .try_into()
                .map(|index: isize| index.checked_add_unsigned(self.index)),
        };
        if let Ok(Some(index)) = index {
            if !index.is_negative() {
                let index = index as usize;
                self.index = index;
                return Ok(index as u64);
            }
        }
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid argument",
        ))
    }
}
