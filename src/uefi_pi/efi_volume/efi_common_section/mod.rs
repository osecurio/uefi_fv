use crate::{uefi_pi::{UefiFvError, UefiFvlResult, efi_volume::efi_common_section::efi_section_type::EfiSectionType}, util::{read_data_slice_n, read_data_slice_u8}};

pub(crate) mod efi_guid_defined_section;
pub(crate) mod efi_section_type;

pub(crate) struct EfiCommonSection {
    size: [u8; 0x3],
    efi_section_type: EfiSectionType,
}

impl EfiCommonSection {
    pub fn load(data: &[u8]) -> UefiFvlResult<Self> {
        let mut offset = 0;
        let sz = read_data_slice_n(data, offset, 0x3);
        let Some(size) = sz.as_array() else {
            return Err(UefiFvError);
        };

        offset += size.len();

        let Some(est) = read_data_slice_u8(data, offset) else {
            return Err(UefiFvError);
        };

        let efi_section_type = EfiSectionType::from(est);

        Ok(Self{
            size: *size,
            efi_section_type,
        })
    }
}