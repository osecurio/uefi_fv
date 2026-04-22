use crate::{uefi_pi::UefiFvlResult, util::{read_data_slice_n, read_data_slice_u16, read_data_slice_u32}};


const PART0_OFF: usize = 0x0;
const PART0_SZ: usize = 0x4;

const PART1_OFF: usize = 0x4;
const PART1_SZ: usize = 0x2;

const PART2_OFF: usize = 0x6;
const PART2_SZ: usize = 0x2;

const PART3_OFF: usize = 0x8;
const PART3_SZ: usize = 0x8;

pub const EFI_VOL_GUID_SZ: usize = 0x10;

pub struct EfiGuid (pub String);

impl EfiGuid {
    pub fn new(guid_data: [u8; 0x10]) -> UefiFvlResult<Self> {
        let part0 = read_data_slice_u32(&guid_data, PART0_OFF).unwrap();
        let part1 = read_data_slice_u16(&guid_data, PART1_OFF).unwrap();
        let part2 = read_data_slice_u16(&guid_data, PART2_OFF).unwrap();
        let part3 = read_data_slice_n(&guid_data, PART3_OFF, PART3_SZ);
        let part3: Vec<String> = part3.iter().map(|b| format!("{b:02X}")).collect();
        let part3_hex_str = part3.join("");
        let guid_str = format!("{part0:X}-{part1:X}-{part2:X}-{part3_hex_str}");

        Ok(Self(guid_str))
    }
}

#[cfg(test)]
mod tests {
    use crate::uefi_pi::efi_volume::efi_guid::EfiGuid;

    #[test]
    fn test_guid_byte_parse() {
        let data: [u8; 0x10] = [
	        0x98, 0x58, 0x4e, 0xee, 0x14, 0x39, 0x59, 0x42, 0x9d, 0x6e, 0xdc, 0x7b, 0xd7, 0x94, 0x03, 0xcf
        ];

        let efi_guid = EfiGuid::new(data).unwrap().0;
        assert_eq!("EE4E5898-3914-4259-9D6EDC7BD79403CF", efi_guid);

        println!(
            "{efi_guid}"
        );
    }
}