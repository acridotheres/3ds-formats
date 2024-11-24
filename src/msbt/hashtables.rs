use dh::{recommended::*, Readable};
use std::{collections::HashMap, io::Result};

// https://zeldamods.org/wiki/Msbt#Labels_Section
pub fn parse_label_block(
    reader: &mut dyn Readable,
    big_endian: bool,
) -> Result<HashMap<u32, String>> {
    let size = reader.read_u32(big_endian)?;
    reader.jump(8)?;

    let table_start = reader.pos()?;
    let offset_count = reader.read_u32(big_endian)?;

    let mut offsets = vec![];
    for _ in 0..offset_count {
        let count = reader.read_u32(big_endian)?;
        let offset = reader.read_u32(big_endian)?;
        offsets.push((count, offset));
    }

    let mut labels = HashMap::new();
    for offset in offsets {
        reader.to(table_start + offset.1 as u64)?;
        if reader.pos()? >= table_start + size as u64 {
            continue;
        }
        let length = reader.read_u8()?;
        let string = reader.read_utf8(length as u64)?;
        let index = reader.read_u32(big_endian)?;
        labels.insert(index, string);
    }

    reader.to(table_start + size as u64)?;
    let pos = reader.pos()?;
    reader.jump(16 - pos as i64 % 16)?;

    Ok(labels)
}

// TODO: Parse attributes
pub fn parse_attr_block(reader: &mut dyn Readable, big_endian: bool) -> Result<(u64, u64)> {
    let size = reader.read_u32(big_endian)?;
    reader.jump(8)?;
    let start = reader.pos()?;
    reader.jump(size as i64)?;
    let pos = reader.pos()?;
    reader.jump(16 - pos as i64 % 16)?;

    Ok((start, size as u64))
}

pub fn parse_text_block(reader: &mut dyn Readable, big_endian: bool) -> Result<Vec<(u64, u32)>> {
    let size = reader.read_u32(big_endian)?;
    reader.jump(8)?;
    let start = reader.pos()?;
    let offset_count = reader.read_u32(big_endian)?;
    let mut offsets = vec![];
    for _ in 0..offset_count {
        let offset = reader.read_u32(big_endian)?;
        offsets.push(offset);
    }
    offsets.push(size);

    let mut files = vec![];
    for i in 0..offsets.len() - 1 {
        let length = offsets[i + 1] - offsets[i];
        files.push((start + offsets[i] as u64, length));
    }

    Ok(files)
}
