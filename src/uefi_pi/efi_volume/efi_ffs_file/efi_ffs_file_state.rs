#[derive(Debug)]
pub(crate) struct EfiFfsFileState (pub u8);

impl EfiFfsFileState {
    pub fn get_state(&self) -> u8 {
        self.0 ^ 0xff
    }
}