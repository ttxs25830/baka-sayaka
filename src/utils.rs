use rodio::{OutputStream, OutputStreamHandle, StreamError};

use std::ops::{Deref, DerefMut};

pub struct PlayHandle(OutputStream, OutputStreamHandle);
impl PlayHandle {
    pub fn try_default() -> Result<Self, StreamError> {
        let (v1, v2) = OutputStream::try_default()?;
        Ok(Self(v1, v2))
    }
}
impl Deref for PlayHandle {
    type Target = OutputStreamHandle;
    fn deref(&self) -> &Self::Target {
        &self.1
    }
}
impl DerefMut for PlayHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.1
    }
}
