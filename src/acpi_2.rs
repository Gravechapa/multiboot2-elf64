
#[derive(Debug)]
#[repr(packed)]
pub struct ACPI2Tag
{
    typ: u32,
    size: u32,
    rsdp_2: RSDP2,
}

#[derive(Debug)]
#[repr(packed)]
pub struct RSDP2
{
    signature: [u8; 8],
    checksum: u8,
    oem_id: [u8; 6],
    revision: u8,
    rsdt_address: u32,
    //v2 part
    length: u32,
    xsdt_address: u64,
    extended_checksum: u8,
    reserved: [u8; 3],
}