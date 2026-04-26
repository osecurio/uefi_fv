#[derive(Debug)]
#[repr(u8)]
pub(crate) enum EfiSectionType {
//************************************************************
// The section type EFI_SECTION_ALL is a pseudo type. It is
// used as a wild card when retrieving sections. The section
// type EFI_SECTION_ALL matches all section types.
//************************************************************
    EFI_SECTION_ALL = 0x00,
//************************************************************
// Encapsulation section Type values
//************************************************************
    EFI_SECTION_COMPRESSION = 0x01,
    EFI_SECTION_GUID_DEFINED = 0x02,
    EFI_SECTION_DISPOSABLE = 0x03,
//************************************************************
// Leaf section Type values
//************************************************************
    EFI_SECTION_PE32 = 0x10,
    EFI_SECTION_PIC = 0x11,
    EFI_SECTION_TE = 0x12,
    EFI_SECTION_DXE_DEPEX = 0x13,
    EFI_SECTION_VERSION = 0x14,
    EFI_SECTION_USER_INTERFACE = 0x15,
    EFI_SECTION_COMPATIBILITY16 = 0x16,
    EFI_SECTION_FIRMWARE_VOLUME_IMAGE = 0x17,
    EFI_SECTION_FREEFORM_SUBTYPE_GUID = 0x18,
    EFI_SECTION_RAW = 0x19,
    EFI_SECTION_PEI_DEPEX = 0x1B,
    EFI_SECTION_MM_DEPEX = 0x1C,

    // NOT PART OF THE SPEC
    EFI_SECTION_FAIL = 0xFF,
}

impl From<u8> for EfiSectionType {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::EFI_SECTION_ALL,
            0x01 => Self::EFI_SECTION_COMPRESSION,
            0x02 => Self::EFI_SECTION_GUID_DEFINED,
            0x03 => Self::EFI_SECTION_DISPOSABLE,
            0x10 => Self::EFI_SECTION_PE32,
            0x11 => Self::EFI_SECTION_PIC,
            0x12 => Self::EFI_SECTION_TE,
            0x13 => Self::EFI_SECTION_DXE_DEPEX,
            0x14 => Self::EFI_SECTION_VERSION,
            0x15 => Self::EFI_SECTION_USER_INTERFACE,
            0x16 => Self::EFI_SECTION_COMPATIBILITY16,
            0x17 => Self::EFI_SECTION_FIRMWARE_VOLUME_IMAGE,
            0x18 => Self::EFI_SECTION_FREEFORM_SUBTYPE_GUID,
            0x19 => Self::EFI_SECTION_RAW,
            0x1B => Self::EFI_SECTION_PEI_DEPEX,
            0x1C => Self::EFI_SECTION_MM_DEPEX,
            _ => Self::EFI_SECTION_FAIL,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::uefi_pi::efi_volume::efi_common_section::efi_section_type::EfiSectionType;

    #[test]
    fn test_efi_section_type_u8_repr() {
        assert_eq!(EfiSectionType::EFI_SECTION_MM_DEPEX as u8, 0x1C);
    }
}