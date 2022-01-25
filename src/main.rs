#![no_std]
#![no_main]

mod psf;

use bootloader::{entry_point, BootInfo};
use crate::psf::*;
use core::{convert::TryInto, panic::PanicInfo};

entry_point!(kernel_main);

fn kernel_main(boot: &'static mut BootInfo) -> ! {
    if let Some(fb) = boot.framebuffer.as_mut() {
        let buffer = include_bytes!("terminus.psf");
        let font = PSFFont::parse(buffer).unwrap();
        let glyph = font.glyph(97).unwrap();

        let size = font.glyph_size();
        let stride = ((size.0 + 7) / 8) as usize;

        let info = fb.info();
        let buff = fb.buffer_mut();
        for y in 0..info.horizontal_resolution.min(size.1 as usize) {
            let out_scanline = &mut buff[(y + 0) * info.stride..(y + 1) * info.stride];
            for x in 0..info.vertical_resolution.min(size.0.try_into().unwrap()) {
                let byte = y * stride + (x / 8);
                let bit = 7 - (x % 8);
                let fill = (glyph[byte as usize] & (1 << bit)) != 0;
                let fill = if fill { 255 } else { 0 };

                let out_pixel = &mut out_scanline
                    [(x + 0) * info.bytes_per_pixel..(x + 1) * info.bytes_per_pixel];
                out_pixel.fill(fill);
            }
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
