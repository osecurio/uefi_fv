use crate::util::{find_aligned_magic, read_data_slice_u32};


#[derive(Debug)]
pub struct EfiFvBlockMapEntry {
    num_blocks: u32,
    length: u32
}

pub(crate) fn load_block_map_entries(data: &[u8], offset: &mut usize, header_len: u16) -> Option<Vec<EfiFvBlockMapEntry>> {
    let mut block_maps = vec![];

    let lbo_offset = &data[*offset..];
    let Some(last_block_offset) = find_aligned_magic(lbo_offset, "\x00\x00\x00\x00\x00\x00\x00\x00".as_bytes()) else {
        return None;
    };

    //println!("last block offset {last_block_offset}");

    if last_block_offset % 8 != 0 {
        //println!("last_block_offset % 8 = {}", last_block_offset % 8);
        return None;
    }

    let block_map_count = last_block_offset / 8;

    // This is an '=block_map_count' because it adds a single iteration for the NULL block mapping
    for _ in 0..=block_map_count {
        let Some(num_blocks) = read_data_slice_u32(data, *offset) else {
            return None;
        };

        *offset += size_of::<u32>();

        let Some(length) = read_data_slice_u32(data, *offset) else {
            return None;
        };

        *offset += size_of::<u32>();

        block_maps.push(
            EfiFvBlockMapEntry { num_blocks, length }
        );
    }

    // Check block_map size
    if block_maps.len() * 8 != header_len as usize - 0x38 {
        return None;
    }

    Some(block_maps)
}

#[cfg(test)]
mod tests {
    use crate::uefi_pi::efi_volume::efi_fv_block_map::load_block_map_entries;

    #[test]
    fn test_load_block_map_entries() {
        let data: [u8; 16] = [
	        0xe8, 0x07, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
        ];

        let mut offset = 0;

        let blocks = load_block_map_entries(&data, &mut offset, 0x48).unwrap();

        assert_eq!(offset, 0x10);

        let block_one = &blocks[0];
        let null_block = &blocks[1];

        assert_eq!(block_one.num_blocks, 0x7e8);
        assert_eq!(block_one.length, 0x200);
        assert_eq!(null_block.num_blocks, 0x0);
        assert_eq!(null_block.length, 0x0);

    }
}