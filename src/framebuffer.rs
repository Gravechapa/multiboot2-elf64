#[derive(Debug)]
#[repr(packed)]
pub struct FramebufferTag
{
    typ: u32,
    size: u32,
    framebuffer_addr: u64,
    framebuffer_pitch: u32,
    framebuffer_width: u32,
    framebuffer_height: u32,
    framebuffer_bpp: u8,
    framebuffer_type: u8,
    reserved: u8,
    color_info: u8,
}



#[derive(Debug)]
#[repr(packed)]
struct IndexedColor{}