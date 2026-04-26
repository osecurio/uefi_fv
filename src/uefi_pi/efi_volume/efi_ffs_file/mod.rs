use crate::{uefi_pi::{UefiFvError, UefiFvlResult, efi_volume::{efi_ffs_file::{
    efi_ffs_file_attributes::EfiFfsFileAttributes, efi_ffs_file_state::EfiFfsFileState,
    efi_ffs_integrity_check::EfiFfsIntegrityCheck, efi_fv_filetype::EfiFvFiletype,
}, efi_guid::EfiGuid}}, util::read_data_slice_n};

pub(crate) mod efi_ffs_file_attributes;
pub(crate) mod efi_ffs_file_state;
pub(crate) mod efi_ffs_integrity_check;
pub(crate) mod efi_fv_filetype;

/*
 * A trait for FFS Files loading?
 * 
 */

#[derive(Debug)]
pub struct EfiFfsFile {
    name: String,
    integrity_check: EfiFfsIntegrityCheck,
    efi_fv_filetype: EfiFvFiletype,
    efi_ffs_file_attributes: EfiFfsFileAttributes,
    size: [u8; 0x3],
    efi_ffs_file_state: EfiFfsFileState,
    // Data of file
}

impl EfiFfsFile {
    pub fn load(data: &[u8]) -> UefiFvlResult<Self> {
        let mut offset = 0;

        // Get File Name GUID
        let ga = read_data_slice_n(&data, offset, 0x10);
        let Some(guid_array) = ga.as_array() else {
            return Err(UefiFvError);
        };
        let Ok(name) = EfiGuid::new(*guid_array) else {
            return Err(UefiFvError);
        };

        offset += guid_array.len();

        

        todo!()
    }
}