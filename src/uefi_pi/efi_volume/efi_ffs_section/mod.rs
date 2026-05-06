use std::collections::HashMap;

use crate::uefi_pi::{
    UefiFvError, UefiFvlResult,
    efi_volume::efi_ffs_section::{
        efi_common_section::EfiCommonSection, efi_guid_defined_section::EfiGuidDefinedSection,
        efi_section_type::EfiSectionTypeId,
    },
};

pub(crate) mod efi_common_section;
pub(crate) mod efi_guid_defined_section;
pub(crate) mod efi_section_type;

#[derive(Debug)]
pub enum EfiFfsSection {
    EfiGuidDefinedSection(EfiGuidDefinedSection),
    EfiSectionAll(Vec<u8>),
}

// The data input slice should ONLY be section data
pub fn load_efi_ffs_sections(data: &[u8]) -> UefiFvlResult<HashMap<usize, EfiFfsSection>> {
    let mut offset = 0;
    let mut section_index = 0;
    let mut sections = HashMap::new();

    loop {
        let Ok(common) = EfiCommonSection::load(data) else {
            // No more headers to parse
            println!("Failed to read EfiCommonSection!");
            return Err(UefiFvError::Other);
        };

        match common.get_efi_section_type() {
            EfiSectionTypeId::EFI_SECTION_ALL => {
                println!("Trying {:#?}", common.get_efi_section_type());
            }
            EfiSectionTypeId::EFI_SECTION_GUID_DEFINED => {
                println!("Trying {:#?}", common.get_efi_section_type());
                let guid_section = EfiGuidDefinedSection::load(data)?;
                offset += common.get_section_size();
                sections.insert(
                    section_index,
                    EfiFfsSection::EfiGuidDefinedSection(guid_section),
                );
                section_index += 1;
            }

            EfiSectionTypeId::EFI_SECTION_COMPRESSION => todo!(),
            EfiSectionTypeId::EFI_SECTION_DISPOSABLE => todo!(),
            EfiSectionTypeId::EFI_SECTION_PE32 => todo!(),
            EfiSectionTypeId::EFI_SECTION_PIC => todo!(),
            EfiSectionTypeId::EFI_SECTION_TE => todo!(),
            EfiSectionTypeId::EFI_SECTION_DXE_DEPEX => todo!(),
            EfiSectionTypeId::EFI_SECTION_VERSION => todo!(),
            EfiSectionTypeId::EFI_SECTION_USER_INTERFACE => todo!(),
            EfiSectionTypeId::EFI_SECTION_COMPATIBILITY16 => todo!(),
            EfiSectionTypeId::EFI_SECTION_FIRMWARE_VOLUME_IMAGE => todo!(),
            EfiSectionTypeId::EFI_SECTION_FREEFORM_SUBTYPE_GUID => todo!(),
            EfiSectionTypeId::EFI_SECTION_RAW => todo!(),
            EfiSectionTypeId::EFI_SECTION_PEI_DEPEX => todo!(),
            EfiSectionTypeId::EFI_SECTION_MM_DEPEX => todo!(),
            EfiSectionTypeId::EFI_SECTION_FAIL => {
                println!("Trying {:#?}", common.get_efi_section_type());
                println!("Got EfiSectionTypeId::EFI_SECTION_FAIL.. something is probably wrong..");
            }
        }

        if data.len() as usize == offset + 1 {
            println!("GFH header size offset reached");
            break;
        } else if offset >= data.len() {
            println!("offset > gfh_hdr_size... something went wrong..");
            break;
        }
    }
    Ok(sections)
}
