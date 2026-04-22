use core::fmt;

use crate::{uefi_pi::{UefiFvError, UefiFvlResult, efi_volume::{efi_fv_block_map::{EfiFvBlockMapEntry, load_block_map_entries}, efi_fvb_attributes_2::EfiFvbAttributes2, efi_guid::{EFI_VOL_GUID_SZ, EfiGuid}}}, util::{read_data_slice_n, read_data_slice_u8, read_data_slice_u16, read_data_slice_u32, read_data_slice_u64}};

const ZERO_VECTOR_SIZE: usize = 0x10;

#[derive(Debug)]
pub(crate) struct EfiFirmwareVolume {
    zero_vector: [u8; 0x10],
    file_system_guid: String,
    fv_length: u64,
    signature: u32,
    attributes: EfiFvbAttributes2,
    header_length: u16,
    checksum: u16,
    ext_header_offset: u16,
    reserved: [u8; 0x1],
    revision: u8,
    block_map: Option<Vec<EfiFvBlockMapEntry>>
}

impl EfiFirmwareVolume {
    pub fn load(data: Vec<u8>) -> UefiFvlResult<Self> {

        let mut offset = 0;
        
        // Get Zero Vector
        let zv = read_data_slice_n(&data, offset, ZERO_VECTOR_SIZE);
        let Some(zero_vector) = zv.as_array() else {
            return Err(UefiFvError);
        };
        
        offset += zero_vector.len();

        // Get FS GUID
        let ga = read_data_slice_n(&data, offset, EFI_VOL_GUID_SZ);
        let Some(guid_array) = ga.as_array() else {
            return Err(UefiFvError);
        };
        let Ok(file_system_guid) = EfiGuid::new(*guid_array) else {
            return Err(UefiFvError);
        };

        offset += guid_array.len();

        let Some(fv_length) = read_data_slice_u64(&data, offset) else {
            return Err(UefiFvError);
        };

        offset += size_of::<u64>();

        let Some(signature) = read_data_slice_u32(&data, offset) else {
            return Err(UefiFvError);
        };

        offset += size_of::<u32>();

        let Some(attributes) = read_data_slice_u32(&data, offset) else {
            return Err(UefiFvError);
        };

        let attributes = EfiFvbAttributes2(attributes);

        offset += size_of::<u32>();

        let Some(header_length) = read_data_slice_u16(&data, offset) else {
            return Err(UefiFvError);
        };

        offset += size_of::<u16>();

        let Some(checksum) = read_data_slice_u16(&data, offset) else {
            return Err(UefiFvError);
        };

        offset += size_of::<u16>();

        let Some(ext_header_offset) = read_data_slice_u16(&data, offset) else {
            return Err(UefiFvError);
        };

        offset += size_of::<u16>();

        let Some(reserved) = read_data_slice_u8(&data, offset) else {
            return Err(UefiFvError);
        };

        let reserved = [ reserved ];

        offset += size_of::<u8>();

        let Some(revision) = read_data_slice_u8(&data, offset) else {
            return Err(UefiFvError);
        };

        offset += size_of::<u8>();

        let block_map = load_block_map_entries(&data, &mut offset, header_length);

        Ok(Self {
            zero_vector: *zero_vector,
            file_system_guid: file_system_guid.0.clone(),
            fv_length,
            signature,
            attributes,
            header_length,
            checksum,
            ext_header_offset,
            reserved,
            revision,
            block_map,
        })

    }
}


impl fmt::Display for EfiFirmwareVolume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use crate::uefi_pi::efi_volume::efi_firmware_volume::EfiFirmwareVolume;

    #[test]
    fn test_load_efi_fw_volume() {
        let data = std::fs::read("testbins/binary_0x9fa00000_0xfd001-abl.img").unwrap();
        let efi_vol = EfiFirmwareVolume::load(data).unwrap();

        println!("{efi_vol:#X?}");
    }
}