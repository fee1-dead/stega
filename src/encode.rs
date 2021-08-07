use crate::*;

pub fn bytes<It, Byte>(
    input: &str,
    output: &str,
    input_bytes: It,
    len: usize,
) -> Result
where
    It: Iterator<Item = Byte>,
    Byte: Try<Output = u8>,
    Result: FromResidual<<Byte as Try>::Residual>,
{
    let img = image::open(input)?;
    let color = img.color();
    let (x, y) = img.dimensions();
    let can_encode_bytes = x * y * (color.bytes_per_pixel() as u32) / 8;
    let requested_bytes = len as u32;
    info!(?x, ?y, ?color, ?can_encode_bytes, ?requested_bytes);
    if requested_bytes + 4 > can_encode_bytes {
        Err("Too much data to fit in lsb")?;
    }
    let mut bytes = img.into_bytes();

    for b in bytes.iter_mut() {
        *b &= 0b11111110; // clear all lsb;
    }
    let mut c = 0;
    let mut write_bit = |b: u8| {
        for bit in b.view_bits::<Lsb0>() {
            let to = &mut bytes[c];
            debug!("setting bytes[{}] to {}", c, bit);
            to.view_bits_mut::<Lsb0>().first_mut().unwrap().replace(*bit);
            // ^ msb0 here because we want the last bit to be least sig
            c += 1;
        }
    };
    requested_bytes
        .to_be_bytes()
        .iter()
        .copied()
        .for_each(&mut write_bit);
    for b in input_bytes {
        write_bit(b?);
    }

    image::save_buffer(output, &bytes, x, y, color)?;
    Ok(())
}
