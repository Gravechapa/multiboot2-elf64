
#[derive(Debug)]
#[repr(packed)]
pub struct ACPI2Tag
{
    typ: u32,
    size: u32,
    rsdp_2: RSDP2,
}

#[derive(Debug, Clone, Copy)]
#[repr(packed)]
pub struct RSDP2
{
    pub signature: [u8; 8],
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub revision: u8,
    pub rsdt_address: u32,
    //v2 part
    pub length: u32,
    pub xsdt_address: u64,
    pub extended_checksum: u8,
    pub reserved: [u8; 3],
}

impl ACPI2Tag
{
    pub fn get_rsdp(&self) -> RSDP2
    {
        self.rsdp_2.clone()
    }
}