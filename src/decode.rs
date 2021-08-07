use std::convert::TryInto;
use image::GenericImageView;

use crate::*;

fn u32_from_lsb(bytes: &[u8; 32]) -> u32 {
    let mut v: BitVec<Lsb0, u8> = BitVec::new();
    for b in bytes {
        v.push(*b.view_bits::<Msb0>().last().unwrap());
    }
    u32::from_be_bytes(v.into_vec().try_into().unwrap())
}


pub fn text(input: String, output: String) -> Result {
    let img = image::open(input)?;
    info!(color = ?img.color(), dim = ?img.dimensions());
    let bytes = img.into_bytes();
    let header_bytes = &bytes[0..(4 * 8)];
    let len = u32_from_lsb(header_bytes.try_into()?);
    info!(bytes_len = bytes.len(), ?len);
    let mut v: BitVec<Lsb0, u8> = BitVec::with_capacity(len as usize * 8);
    let mut bytes = bytes.into_iter().skip(4 * 8);
    for _ in 0..len {
        for _ in 0..8 {
            v.push(*bytes.next().unwrap().view_bits::<Msb0>().last().unwrap());
        }
    }
    let st = String::from_utf8(v.into_vec())?;
    info!(?st);
    std::fs::write(output, st)?;
    
    Ok(())
}