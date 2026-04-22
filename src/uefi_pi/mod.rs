use core::fmt;

use crate::uefi_pi::efi_volume::{efi_firmware_volume::{EfiFirmwareVolume}, efi_fv_block_map::EfiFvBlockMapEntry};

pub(crate) mod efi_volume;
pub(crate) mod types;

pub(crate) type UefiFvlResult<T> = std::result::Result<T, UefiFvError>;

#[derive(Debug, Clone)]
pub struct UefiFvError;

impl fmt::Display for UefiFvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Uefi Firmware Volume loader failure")
    }
}

pub const EFI_FIRMWARE_VOLUME_HEADER_MAGIC: &'static str = "_FVH";
pub const EFI_FIRMWARE_VOLUME_HEADER_MAGIC_OFFSET: usize = 0x28;


#[derive(Debug)]
pub struct UefiFirmwareVolumeLoader {
    volume_entries: Vec<EfiFirmwareVolume>,

}

impl UefiFirmwareVolumeLoader {
    pub fn new(mut data: Vec<u8>) -> UefiFvlResult<Self> {

        let Some(magic_offset) = crate::util::find_magic_first(&data, EFI_FIRMWARE_VOLUME_HEADER_MAGIC.as_bytes()) else {
            return Err(UefiFvError);
        };

        let vol_offset = magic_offset - EFI_FIRMWARE_VOLUME_HEADER_MAGIC_OFFSET;

        let _drained_data = data.drain(0..vol_offset).as_slice().to_vec();

        let mut new_loader = Self {
            volume_entries: vec![],
        };

        let volume = EfiFirmwareVolume::load(data).unwrap();

        new_loader.volume_entries.push(volume);

        Ok(new_loader)
    }
}