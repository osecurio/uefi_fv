use core::fmt;
use std::collections::HashMap;

use crate::{
    uefi_pi::{
        UefiFvError, UefiFvlResult,
        efi_volume::{
            efi_ffs_file::EfiFfsFile,
            efi_fv_block_map::{EfiFvBlockMapEntry, load_block_map_entries},
            efi_fvb_attributes_2::EfiFvbAttributes2,
            efi_guid::{EFI_VOL_GUID_SZ, EfiGuid},
        },
    },
    util::{
        read_data_slice_n, read_data_slice_u8, read_data_slice_u16, read_data_slice_u32,
        read_data_slice_u64,
    },
};

#[derive(Debug, Clone)]
pub enum EfiVolError {
    ZeroVectorLoadFailure,
    FileSystemGuidLoadFailure,
    FvLengthLoadFailure,
    SignatureLoadFailure,
    AttributesLoadFailure,
    HeaderLengthLoadFailure,
    ChecksumLoadFailure,
    ExtHeaderOffsetLoadFailure,
    ReservedLoadFailure,
    RevisionLoadFailure,
    BlockmapLoadFailure,
}

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
    block_map: Option<Vec<EfiFvBlockMapEntry>>,
    efi_ffs_files: HashMap<String, EfiFfsFile>,
}

impl EfiFirmwareVolume {
    pub fn load(data: &[u8]) -> UefiFvlResult<Self> {
        let mut offset = 0;

        // Get Zero Vector
        let zv = read_data_slice_n(data, offset, ZERO_VECTOR_SIZE).unwrap();
        let Some(zero_vector) = zv.as_array() else {
            return Err(UefiFvError::EfiVolError(EfiVolError::ZeroVectorLoadFailure));
        };

        offset += zero_vector.len();

        // Get FS GUID
        let ga = read_data_slice_n(data, offset, EFI_VOL_GUID_SZ).unwrap();
        let Some(guid_array) = ga.as_array() else {
            return Err(UefiFvError::EfiVolError(
                EfiVolError::FileSystemGuidLoadFailure,
            ));
        };
        let Ok(file_system_guid) = EfiGuid::new(*guid_array) else {
            return Err(UefiFvError::EfiVolError(
                EfiVolError::FileSystemGuidLoadFailure,
            ));
        };

        offset += guid_array.len();

        let Some(fv_length) = read_data_slice_u64(data, offset) else {
            return Err(UefiFvError::EfiVolError(EfiVolError::FvLengthLoadFailure));
        };

        offset += size_of::<u64>();

        let Some(signature) = read_data_slice_u32(data, offset) else {
            return Err(UefiFvError::EfiVolError(EfiVolError::SignatureLoadFailure));
        };

        offset += size_of::<u32>();

        let Some(attributes) = read_data_slice_u32(data, offset) else {
            return Err(UefiFvError::EfiVolError(EfiVolError::AttributesLoadFailure));
        };

        let attributes = EfiFvbAttributes2(attributes);

        offset += size_of::<u32>();

        let Some(header_length) = read_data_slice_u16(data, offset) else {
            return Err(UefiFvError::EfiVolError(
                EfiVolError::HeaderLengthLoadFailure,
            ));
        };

        offset += size_of::<u16>();

        let Some(checksum) = read_data_slice_u16(data, offset) else {
            return Err(UefiFvError::EfiVolError(EfiVolError::ChecksumLoadFailure));
        };

        offset += size_of::<u16>();

        let Some(ext_header_offset) = read_data_slice_u16(data, offset) else {
            return Err(UefiFvError::EfiVolError(
                EfiVolError::ExtHeaderOffsetLoadFailure,
            ));
        };

        offset += size_of::<u16>();

        let Some(reserved) = read_data_slice_u8(data, offset) else {
            return Err(UefiFvError::EfiVolError(EfiVolError::ReservedLoadFailure));
        };

        let reserved = [reserved];

        offset += size_of::<u8>();

        let Some(revision) = read_data_slice_u8(data, offset) else {
            return Err(UefiFvError::EfiVolError(EfiVolError::RevisionLoadFailure));
        };

        offset += size_of::<u8>();

        let block_map = load_block_map_entries(data, &mut offset, header_length);

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
            efi_ffs_files: HashMap::new(),
        })
    }

    fn load_files(&mut self, data: &[u8]) -> UefiFvlResult<()> {
        let volume_file_data = &read_data_slice_n(
            data,
            self.header_length as usize,
            self.fv_length as usize - self.header_length as usize,
        )
        .unwrap()[..];
        let mut offset = 0;
        loop {
            let file_data = &volume_file_data[offset..];
            let file = EfiFfsFile::load(file_data)?;
            offset += file.get_file_size();
            self.efi_ffs_files.insert(file.get_guid_str(), file);

            if offset == data.len() - 1 {
                break;
            } else if offset > data.len() {
                println!("Ffs offset > data length!");
                return Err(UefiFvError::Other);
            }
        }
        Ok(())
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
        let efi_vol = EfiFirmwareVolume::load(&data).unwrap();

        //println!("{efi_vol:#X?}");
    }

    #[test]
    fn test_load_efi_volume_and_files() {
        let data = std::fs::read("testbins/binary_0x9fa00000_0x2489f-abl.img").unwrap();
        let mut efi_vol = EfiFirmwareVolume::load(&data).unwrap();
        efi_vol.load_files(&data).unwrap();
        println!("{efi_vol:#X?}");
    }
}
