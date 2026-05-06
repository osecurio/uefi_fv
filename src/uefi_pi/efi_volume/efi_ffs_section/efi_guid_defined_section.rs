use crate::uefi_pi::efi_volume::efi_guid::EfiGuid;
use crate::uefi_pi::{UefiFvError, UefiFvlResult, efi_volume::efi_ffs_section::EfiCommonSection};
use crate::util::{read_data_slice_n, read_data_slice_u16};

pub(crate) const EFI_GUID_DEFINED_SECTION_SIZE: usize = 0x17;

#[derive(Debug)]
pub struct EfiGuidDefinedSection {
    efi_common_section: EfiCommonSection,
    section_definition_guid: String,
    data_offset: u16,
    attributes: u16,
}

impl EfiGuidDefinedSection {
    pub fn load(data: &[u8]) -> UefiFvlResult<Self> {
        let mut offset = 0;

        let efi_common_section = EfiCommonSection::load(data)?;

        // Get section definition GUID
        let ga = read_data_slice_n(&data, offset, 0x10).unwrap();
        let Some(guid_array) = ga.as_array() else {
            return Err(UefiFvError::Other);
        };
        let Ok(sdg) = EfiGuid::new(*guid_array) else {
            return Err(UefiFvError::Other);
        };

        let section_definition_guid = sdg.0;

        offset += guid_array.len();

        let Some(data_offset) = read_data_slice_u16(data, offset) else {
            return Err(UefiFvError::Other);
        };

        offset += size_of::<u16>();

        let Some(attributes) = read_data_slice_u16(data, offset) else {
            return Err(UefiFvError::Other);
        };

        Ok(Self {
            efi_common_section,
            section_definition_guid,
            data_offset,
            attributes,
        })
    }
}
