use crate::{
    uefi_pi::{
        UefiFvError, UefiFvlResult, efi_volume::efi_ffs_section::efi_section_type::EfiSectionTypeId,
    },
    util::{read_data_slice_n, read_data_slice_u8},
};

#[derive(Debug)]
pub(crate) struct EfiCommonSection {
    size: [u8; 0x3],
    efi_section_type: EfiSectionTypeId,
}

impl EfiCommonSection {
    pub fn load(data: &[u8]) -> UefiFvlResult<Self> {
        let mut offset = 0;
        let sz = read_data_slice_n(data, offset, 0x3).unwrap();
        let Some(size) = sz.as_array() else {
            return Err(UefiFvError::Other);
        };

        offset += size.len();

        let Some(est) = read_data_slice_u8(data, offset) else {
            return Err(UefiFvError::Other);
        };

        let efi_section_type = EfiSectionTypeId::from(est);

        Ok(Self {
            size: *size,
            efi_section_type,
        })
    }

    pub fn get_efi_section_type(&self) -> EfiSectionTypeId {
        self.efi_section_type
    }

    pub fn get_section_size(&self) -> usize {
        let mut tmp_size = self.size.to_vec();
        // Push 8 bits since size is 24 bit
        tmp_size.push(0x00);
        u32::from_le_bytes(*tmp_size.as_array().unwrap()) as usize
    }
}
