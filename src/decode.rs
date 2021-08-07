use crate::*;

pub enum DecodeType {
    Text,
    File,
}

impl DecodeType {
    fn transform(self, mut bytes: Vec<u8>) -> Result<Vec<u8>> {
        match self {
            DecodeType::Text => {
                let st = String::from_utf8(bytes)?;
                info!(?st);
                bytes = st.into_bytes();
            }
            _ => {}
        }
        Ok(bytes)
    }
}

fn u32_from_lsb(bytes: &[u8; 32]) -> u32 {
    let mut v: BitVec<Lsb0, u8> = BitVec::new();
    for b in bytes {
        v.push(*b.view_bits::<Lsb0>().first().unwrap());
    }
    u32::from_be_bytes(v.into_vec().try_into().unwrap())
}

pub fn text(input: &str, output: &str, ty: DecodeType) -> Result {
    let img = image::open(input)?;
    info!(color = ?img.color(), dim = ?img.dimensions());
    let bytes = img.into_bytes();
    let header_bytes = &bytes[0..32];
    let len = u32_from_lsb(header_bytes.try_into()?);
    info!(bytes_len = bytes.len(), ?len);
    let mut v: BitVec<Lsb0, u8> = BitVec::with_capacity(len as usize * 8);
    let mut bytes = bytes.into_iter().skip(32);
    for _ in 0..len {
        for _ in 0..8 {
            v.push(*bytes.next().unwrap().view_bits::<Lsb0>().first().unwrap());
        }
    }
    std::fs::write(output, ty.transform(v.into_vec())?)?;

    Ok(())
}
