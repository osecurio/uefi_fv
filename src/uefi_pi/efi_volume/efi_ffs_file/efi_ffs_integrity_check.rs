use crate::{
    uefi_pi::{UefiFvError, UefiFvlResult},
    util::read_data_slice_u8,
};

#[derive(Debug)]
struct Checksum {
    header: u8,
    file: u8,
}

impl Checksum {
    pub fn to_chksum16(&self) -> u16 {
        let b = [self.header, self.file];
        u16::from_le_bytes(b)
    }
}
/*
 * In the UEFI PI spec this is a union however, in the Rust abstraction
 * I am exposing the union as a structure that holds both types.
 */
#[derive(Debug)]
pub struct EfiFfsIntegrityCheck {
    checksum: Checksum,
    checksum16: u16,
}

impl EfiFfsIntegrityCheck {
    pub fn new(data: &[u8]) -> UefiFvlResult<Self> {
        let mut offset = 0;
        let Some(header) = read_data_slice_u8(data, offset) else {
            return Err(UefiFvError::EfiFfsFileError(
                super::EfiFfsFileError::IntegrityCheckLoadFailure,
            ));
        };
        offset += size_of::<u8>();
        let Some(file) = read_data_slice_u8(data, offset) else {
            return Err(UefiFvError::EfiFfsFileError(
                super::EfiFfsFileError::IntegrityCheckLoadFailure,
            ));
        };
        let checksum = Checksum { header, file };

        let checksum16 = checksum.to_chksum16();

        Ok(Self {
            checksum,
            checksum16,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_chksum16() {
        let chksum = Checksum {
            header: 0x21,
            file: 0xaa,
        };
        assert_eq!(chksum.to_chksum16(), 0xaa21);
    }
}
