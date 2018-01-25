use core::mem::size_of;


#[repr(packed)]
pub struct FramebufferTag
{
    typ: u32,
    size: u32,
    pub framebuffer_addr: u64,
    pub framebuffer_pitch: u32,
    pub framebuffer_width: u32,
    pub framebuffer_height: u32,
    pub framebuffer_bpp: u8,
    pub framebuffer_type: u8,
    reserved: u8,
    color_info: ColorUnion,
}

impl FramebufferTag
{
    pub fn get_indexed_color(&self) -> Option<&IndexedColor>
    {
        match self.framebuffer_type
            {
                0 => unsafe{Some(&self.color_info.zero)},
                _ => None,
            }
    }

    pub fn get_direct_rgb_color(&self) -> Option<&DirectRGB>
    {
        match self.framebuffer_type
            {
                1 => unsafe{Some(&self.color_info.one)},
                _ => None,
            }
    }
}

#[repr(packed)]
union ColorUnion
{
    zero: IndexedColor,
    one: DirectRGB,
}

#[derive(Debug, Clone, Copy)]
#[repr(packed)]
pub struct DirectRGB
{
    pub framebuffer_red_field_position: u8,
    pub framebuffer_red_mask_size: u8,
    pub framebuffer_green_field_position: u8,
    pub framebuffer_green_mask_size: u8,
    pub framebuffer_blue_field_position: u8,
    pub framebuffer_blue_mask_size: u8,
}

#[derive(Debug, Clone, Copy)]
#[repr(packed)]
pub struct IndexedColor
{
    framebuffer_palette_num_colors: u32,
    framebuffer_palette: Palette,
}

#[derive(Debug, Clone, Copy)]
#[repr(packed)]
pub struct Palette
{
    pub red_value: u8,
    pub green_value: u8,
    pub blue_value: u8,
}

impl IndexedColor
{
    pub fn get_size(&self) -> u32
    {
        self.framebuffer_palette_num_colors
    }

    pub fn get(&self, index: usize) -> &Palette
    {
        assert!(!(index > self.framebuffer_palette_num_colors as usize - 1));
        let ptr = &self.framebuffer_palette as *const Palette;
        let ptr = ptr as u64 + (index * size_of::<Palette>()) as u64;
        unsafe{&*(ptr as *const Palette)}
    }
}