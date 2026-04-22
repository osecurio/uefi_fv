

pub fn read_data_slice_n(data: &[u8], offset: usize, len: usize) -> Vec<u8> {
    data[offset..offset+len].to_vec()
}

pub fn read_data_slice_u8(data: &[u8], offset: usize) -> Option<u8> {
    let u8_bytes = *data[offset..offset + size_of::<u8>()].as_array()?;
    Some(u8::from_le_bytes(u8_bytes))
}

pub fn read_data_slice_u16(data: &[u8], offset: usize) -> Option<u16> {
    let u16_bytes = *data[offset..offset + size_of::<u16>()].as_array()?;
    Some(u16::from_le_bytes(u16_bytes))
}

pub fn read_data_slice_u32(data: &[u8], offset: usize) -> Option<u32> {
    let u32_bytes = *data[offset..offset + size_of::<u32>()].as_array()?;
    Some(u32::from_le_bytes(u32_bytes))
}

pub fn read_data_slice_u64(data: &[u8], offset: usize) -> Option<u64> {
    let u64_bytes = *data[offset..offset + size_of::<u64>()].as_array()?;
    Some(u64::from_le_bytes(u64_bytes))
}

pub fn find_magic_first(data: &[u8], pat: &[u8]) -> Option<usize> {
    data.windows(pat.len()).position(|w| w == pat)
}

pub fn find_aligned_magic(data: &[u8], pat: &[u8]) -> Option<usize> {
    let chunk_index = data.chunks(pat.len()).position(|c| c == pat)?;
    Some(chunk_index * pat.len())
}