use crate::uefi_pi::{UefiFvError, UefiFvlResult, efi_volume::efi_common_section::EfiCommonSection};



pub struct EfiGuidDefinedSection {
    efi_common_section: EfiCommonSection,
    section_definition_guid: String,
    data_offset: u16,
    attributes: u16,
}

impl EfiGuidDefinedSection {
    pub fn load(data: &[u8]) -> UefiFvlResult<Self> {

        

        todo!()
    }
}