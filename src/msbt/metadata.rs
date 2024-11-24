use super::{
    hashtables::{parse_attr_block, parse_label_block, parse_text_block},
    MsbtFile, MsbtMetadata,
};
use dh::{recommended::*, Readable};
use std::io::{Error, ErrorKind, Result};

pub fn metadata(reader: &mut dyn Readable) -> Result<MsbtMetadata> {
    reader.jump(9)?;
    let big_endian = reader.read_u8()? == 0xff;
    reader.jump(4)?;
    let sections = reader.read_u16(big_endian)?;
    reader.jump(16)?;

    let mut files = vec![];

    let mut labels = None;
    let mut attr = None;
    let mut text = None;

    for _ in 0..sections {
        match reader.read_bytes(4).unwrap_or(vec![0; 4]).as_slice() {
            b"LBL1" => {
                labels = Some(parse_label_block(reader, big_endian)?);
            }
            b"ATR1" => {
                attr = Some(parse_attr_block(reader, big_endian)?);
            }
            b"TXT2" => {
                text = Some(parse_text_block(reader, big_endian)?);
            }
            _ => {
                dbg!(reader.pos()?);
                reader.jump(-4)?;
                dbg!(reader.read_utf8(4)?);
                return Err(Error::new(ErrorKind::InvalidData, "Invalid section"));
            }
        }
    }

    let labels = labels.unwrap_or_default();
    let attr = attr.unwrap_or((0, 0));
    let text = text.unwrap_or_default();

    for (i, (offset, size)) in text.iter().enumerate() {
        let path = labels
            .get(&(i as u32))
            .unwrap_or(&format!("nolabel-{i:03}").to_string())
            .clone();
        let ext = if path.contains(".") { "" } else { ".txt" };
        files.push(MsbtFile {
            path: path + ext,
            offset: *offset,
            size: *size as u64,
        });
    }

    Ok(MsbtMetadata {
        big_endian,
        files,
        attr_offset: attr.0,
        attr_size: attr.1,
    })
}
