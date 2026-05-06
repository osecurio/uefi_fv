use crate::uefi_pi::{EfiFirmwareVolume, UefiFvlResult};

pub struct FtFirmwareVolumeImage {
    data: Vec<u8>,
    firmware_volume_image: EfiFirmwareVolume,
}

impl FtFirmwareVolumeImage {
    pub fn new(data: &[u8]) -> UefiFvlResult<Self> {
        let v = EfiFirmwareVolume::load(data)?;
        Ok(FtFirmwareVolumeImage {
            data: data.to_vec(),
            firmware_volume_image: v,
        })
    }
}
