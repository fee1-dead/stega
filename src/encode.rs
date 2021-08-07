use image::GenericImageView;

use crate::*;

pub fn text(input: String, output: String, txt: String) -> Result {
    let img = image::open(input)?;
    let color = img.color();
    let (x, y) = img.dimensions();
    let can_encode_bytes = x * y * (color.bytes_per_pixel() as u32) / 8;
    let requested_bytes = txt.len() as u32;
    info!(?x, ?y, ?color, ?can_encode_bytes, ?requested_bytes);
    if requested_bytes + 4 > can_encode_bytes {
        Err("Too much data to fit in lsb")?;
    }
    let mut bytes = img.into_bytes();

    for b in bytes.iter_mut() {
        *b &= 0b11111110; // clear all lsb;
    }
    let mut c = 0;
    for b in requested_bytes.to_be_bytes().iter().copied().chain(txt.bytes()) {
        for bit in b.view_bits::<Lsb0>() {
            let to = &mut bytes[c];
            debug!("setting bytes[{}] to {}", c, bit);
            to.view_bits_mut::<Msb0>().last_mut().unwrap().replace(*bit);
            // ^ msb0 here because we want the last bit to be least sig
            c += 1;
        }
    }

    image::save_buffer(output, &bytes, x, y, color)?;
    Ok(())
} 