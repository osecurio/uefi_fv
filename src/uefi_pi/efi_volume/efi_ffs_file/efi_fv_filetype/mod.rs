use crate::uefi_pi::efi_volume::efi_ffs_file::efi_fv_filetype::ft_firmware_volume_image::FtFirmwareVolumeImage;

use crate::uefi_pi::{
    UefiFvError, UefiFvlResult,
    efi_volume::efi_ffs_file::efi_fv_filetype::{ft_ffs_pad::FtFfsPad, ft_raw::FtRaw},
};

pub(crate) mod ft_ffs_pad;
pub(crate) mod ft_firmware_volume_image;
pub(crate) mod ft_raw;

#[derive(Debug)]
pub(crate) enum EfiFvFiletypeId {
    EFI_FV_FILETYPE_INVALID = 0x00,
    EFI_FV_FILETYPE_RAW = 0x01,
    EFI_FV_FILETYPE_FREEFORM = 0x02,
    EFI_FV_FILETYPE_SECURITY_CORE = 0x03,
    EFI_FV_FILETYPE_PEI_CORE = 0x04,
    EFI_FV_FILETYPE_DXE_CORE = 0x05,
    EFI_FV_FILETYPE_PEIM = 0x06,
    EFI_FV_FILETYPE_DRIVER = 0x07,
    EFI_FV_FILETYPE_COMBINED_PEIM_DRIVER = 0x08,
    EFI_FV_FILETYPE_APPLICATION = 0x09,
    EFI_FV_FILETYPE_MM = 0x0A,
    EFI_FV_FILETYPE_FIRMWARE_VOLUME_IMAGE = 0x0B,
    EFI_FV_FILETYPE_COMBINED_MM_DXE = 0x0C,
    EFI_FV_FILETYPE_MM_CORE = 0x0D,
    EFI_FV_FILETYPE_MM_STANDALONE = 0x0E,
    EFI_FV_FILETYPE_MM_CORE_STANDALONE = 0x0F,
    /*Implement these as num labels
        EFI_FV_FILETYPE_OEM_MIN = 0xC0,
        EFI_FV_FILETYPE_OEM_MAX = 0xDF,
        EFI_FV_FILETYPE_DEBUG_MIN = 0xE0,
        EFI_FV_FILETYPE_DEBUG_MAX = 0xEF,
        EFI_FV_FILETYPE_FFS_MIN = 0xF0,
        EFI_FV_FILETYPE_FFS_MAX = 0xFF,
    */
    EFI_FV_FILETYPE_FFS_PAD = 0xF0,
}

impl From<u8> for EfiFvFiletypeId {
    fn from(value: u8) -> Self {
        match value {
            0x01 => Self::EFI_FV_FILETYPE_RAW,
            0x02 => Self::EFI_FV_FILETYPE_FREEFORM,
            0x03 => Self::EFI_FV_FILETYPE_SECURITY_CORE,
            0x04 => Self::EFI_FV_FILETYPE_PEI_CORE,
            0x05 => Self::EFI_FV_FILETYPE_DXE_CORE,
            0x06 => Self::EFI_FV_FILETYPE_PEIM,
            0x07 => Self::EFI_FV_FILETYPE_DRIVER,
            0x08 => Self::EFI_FV_FILETYPE_COMBINED_PEIM_DRIVER,
            0x09 => Self::EFI_FV_FILETYPE_APPLICATION,
            0x0A => Self::EFI_FV_FILETYPE_MM,
            0x0B => Self::EFI_FV_FILETYPE_FIRMWARE_VOLUME_IMAGE,
            0x0C => Self::EFI_FV_FILETYPE_COMBINED_MM_DXE,
            0x0D => Self::EFI_FV_FILETYPE_MM_CORE,
            0x0E => Self::EFI_FV_FILETYPE_MM_STANDALONE,
            0x0F => Self::EFI_FV_FILETYPE_MM_CORE_STANDALONE,
            0xF0 => Self::EFI_FV_FILETYPE_FFS_PAD,
            _ => Self::EFI_FV_FILETYPE_INVALID,
        }
    }
}

pub(crate) enum EfiFvFileType {
    Raw(FtRaw),
    FfsPad(FtFfsPad),
    FirmwareVolumeImage(FtFirmwareVolumeImage),
}

/*
 * impl error enum for crate error type
 * pass offset linearly from volume load stage
 *
 */

pub fn load_file_data(
    data: &[u8],
    file_type: EfiFvFiletypeId,
    size: usize,
) -> UefiFvlResult<EfiFvFileType> {
    match file_type {
        EfiFvFiletypeId::EFI_FV_FILETYPE_RAW => Ok(EfiFvFileType::Raw(FtRaw::load(data, size))),
        EfiFvFiletypeId::EFI_FV_FILETYPE_FIRMWARE_VOLUME_IMAGE => Ok(
            EfiFvFileType::FirmwareVolumeImage(FtFirmwareVolumeImage::new(data).unwrap()),
        ),
        _ => Err(UefiFvError::Other),
    }
}
