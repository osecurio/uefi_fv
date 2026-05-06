use crate::{
    uefi_pi::efi_volume::efi_ffs_section::{
        efi_common_section::EfiCommonSection, efi_guid_defined_section::EfiGuidDefinedSection,
        efi_section_type::EfiSectionTypeId,
    },
    util::read_data_slice_u8,
};
use std::{collections::HashMap, fmt};

use crate::{
    uefi_pi::{
        UefiFvError, UefiFvlResult,
        efi_volume::{
            efi_ffs_file::{
                efi_ffs_file_attributes::EfiFfsFileAttributes, efi_ffs_file_state::EfiFfsFileState,
                efi_ffs_integrity_check::EfiFfsIntegrityCheck, efi_fv_filetype::EfiFvFiletypeId,
            },
            efi_ffs_section::EfiFfsSection,
            efi_guid::EfiGuid,
        },
    },
    util::read_data_slice_n,
};

pub(crate) mod efi_ffs_file_attributes;
pub(crate) mod efi_ffs_file_state;
pub(crate) mod efi_ffs_integrity_check;
pub(crate) mod efi_fv_filetype;

#[derive(Debug, Clone)]
pub enum EfiFfsFileError {
    GuidLoadFailure,
    IntegrityCheckLoadFailure,
    FiletypeIdLoadFailure,
    FileAttributesLoadFailure,
    FileSizeLoadFailure,
    FileStateLoadFailure,
}

/*
 * A trait for FFS Files loading?
 *
 */

#[derive(Debug)]
pub struct EfiFfsFile {
    internal_file_data: Vec<u8>,
    internal_file_size: u32,
    name: String,
    integrity_check: EfiFfsIntegrityCheck,
    efi_fv_filetype: EfiFvFiletypeId,
    efi_ffs_file_attributes: EfiFfsFileAttributes,
    size: [u8; 0x3],
    efi_ffs_file_state: EfiFfsFileState,
    // Data of file
    // <Index, EfiSection
    efi_sections: HashMap<usize, EfiFfsSection>,
}

impl EfiFfsFile {
    pub fn load(data: &[u8]) -> UefiFvlResult<Self> {
        let mut offset = 0;

        // Get File Name GUID
        let ga = read_data_slice_n(&data, offset, 0x10).unwrap();
        let Some(guid_array) = ga.as_array() else {
            return Err(UefiFvError::EfiFfsFileError(
                EfiFfsFileError::GuidLoadFailure,
            ));
        };
        let Ok(name) = EfiGuid::new(*guid_array) else {
            return Err(UefiFvError::EfiFfsFileError(
                EfiFfsFileError::GuidLoadFailure,
            ));
        };

        let name = name.0;

        offset += guid_array.len();

        let ic = read_data_slice_n(data, offset, 0x2).unwrap();
        let integrity_check = EfiFfsIntegrityCheck::new(&ic)?;

        offset += size_of::<u16>();

        let Some(ftid) = read_data_slice_u8(data, offset) else {
            return Err(UefiFvError::EfiFfsFileError(
                EfiFfsFileError::FiletypeIdLoadFailure,
            ));
        };
        let efi_fv_filetype = EfiFvFiletypeId::from(ftid);

        offset += size_of::<u8>();

        let Some(fa) = read_data_slice_u8(data, offset) else {
            return Err(UefiFvError::EfiFfsFileError(
                EfiFfsFileError::FileAttributesLoadFailure,
            ));
        };

        let efi_ffs_file_attributes = EfiFfsFileAttributes(fa);

        offset += size_of::<u8>();

        let sz = read_data_slice_n(data, offset, 0x3).unwrap();
        let Some(size) = sz.as_array() else {
            return Err(UefiFvError::EfiFfsFileError(
                EfiFfsFileError::FileSizeLoadFailure,
            ));
        };
        let size: [u8; 0x3] = *size;

        offset += size.len();

        let Some(fstate) = read_data_slice_u8(data, offset) else {
            return Err(UefiFvError::EfiFfsFileError(
                EfiFfsFileError::FileStateLoadFailure,
            ));
        };
        let efi_ffs_file_state = EfiFfsFileState(fstate);

        let mut tmp_size = size.to_vec();
        // Push 8 bits since size is 24 bit
        tmp_size.push(0x00);
        let internal_file_size = u32::from_le_bytes(*tmp_size.as_array().unwrap());
        println!("{internal_file_size:#X} , {:#X}", data.len());
        let internal_file_data = read_data_slice_n(data, 0, internal_file_size as usize).unwrap();

        Ok(Self {
            internal_file_data,
            internal_file_size,
            name,
            integrity_check,
            efi_fv_filetype,
            efi_ffs_file_attributes,
            size,
            efi_ffs_file_state,
            efi_sections: HashMap::new(),
        })
    }

    pub fn get_state(&self) -> u8 {
        self.efi_ffs_file_state.get_state()
    }

    pub fn get_file_size(&self) -> usize {
        self.internal_file_size as usize
    }

    pub fn get_guid_str(&self) -> String {
        self.name.clone()
    }
}
