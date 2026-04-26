
#[derive(Debug)]
struct Checksum {
    header: u8,
    file: u8,
}

#[derive(Debug)]
pub struct EfiFfsIntegrityCheck {
    checksum: Checksum,
    checksum16: u16,
}