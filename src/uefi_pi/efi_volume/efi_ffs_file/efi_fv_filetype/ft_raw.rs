pub struct FtRaw {
    data: Vec<u8>,
}

impl FtRaw {
    pub fn load(data: &[u8], size: usize) -> Self {
        let data = data.to_vec();
        Self { data }
    }
}
