use super::MsbtFileSource;
use dh::{recommended::*, Readable, Writable};
use std::io::Result;

pub fn create<'a>(
    mut files: Vec<MsbtFileSource<'a>>,
    attribute_source: &mut dyn Readable<'a>,
    target: &mut dyn Writable<'a>,
    buffer_size: u64,
) -> Result<()> {
    // main file header
    target.write_bytes(b"MsgStdBn")?;
    target.write_u16le(0xfeff)?; // TODO: Add big endian support
    target.write_u16be(0x0000)?;
    target.write_u16be(0x0103)?;

    let attr_size = attribute_source.size()?;
    let has_attr_section = attr_size > 0;

    if has_attr_section {
        target.write_u16le(0x0003)?;
    } else {
        target.write_u16le(0x0002)?;
    }

    target.write_u32le(0x0000)?; // Placeholder for the size of the ENTIRE file, including everything!
    target.write_bytes(&[0; 12])?;

    // labels section
    target.write_bytes(b"LBL1")?;
    let labels_size_pos = target.pos()?;
    target.write_u32le(0x0000)?;
    target.write_bytes(&[0; 8])?;
    let table_pos = target.pos()?;
    target.write_u32le(files.len() as u32)?;

    let mut offset_offsets = vec![];

    for _ in &files {
        target.write_u32le(0x0001)?;
        offset_offsets.push(target.pos()?);
        target.write_u32le(0x0000)?;
    }

    for (i, file) in files.iter().enumerate() {
        let pos = target.pos()? as u32;
        target.write_u32le_at(offset_offsets[i], pos - table_pos as u32)?;
        let path = if file.metadata.path.ends_with(".txt") {
            &file.metadata.path[..file.metadata.path.len() - 4]
        } else {
            &file.metadata.path
        };
        target.write_u8(path.len() as u8)?;
        target.write_utf8(path)?;
        target.write_u32le(i as u32)?;
    }

    let pos = target.pos()?;
    let labels_size = pos - table_pos;
    target.write_u32le_at(labels_size_pos, labels_size as u32)?;
    target.write_bytes(&vec![0xab; 16 - pos as usize % 16])?;

    // attributes section
    if has_attr_section {
        attribute_source.copy(attr_size, target, buffer_size)?;
    }

    // text section
    target.write_bytes(b"TXT2")?;
    let text_size_pos = target.pos()?;
    target.write_u32le(0x0000)?;
    target.write_bytes(&[0; 8])?;
    let table_pos = target.pos()?;

    target.write_u32le(files.len() as u32)?;

    let mut offset_offsets = vec![];

    for _ in &files {
        offset_offsets.push(target.pos()?);
        target.write_u32le(0x0000)?;
    }

    for file in &mut files {
        let pos = target.pos()? as u32;
        target.write_u32le_at(offset_offsets.remove(0), pos - table_pos as u32)?;
        let string = file.reader.read_utf8(file.metadata.size)?;

        let mut i = 0;
        let chars = string.chars().collect::<Vec<char>>();
        while i < string.len() {
            if chars.get(i).unwrap_or(&'\0') == &'<'
                && chars.get(i + 1).unwrap_or(&'\0') == &'{'
                && chars.get(i + 6).unwrap_or(&'\0') == &','
                && chars.get(i + 11).unwrap_or(&'\0') == &','
            {
                let tag_group_id = u16::from_str_radix(&string[i + 2..i + 6], 16).unwrap();
                let tag_id = u16::from_str_radix(&string[i + 7..i + 11], 16).unwrap();
                let mut data_chars = vec![];
                let mut j = i + 12;
                while j < string.len() {
                    if chars.get(j).unwrap_or(&'\0') == &'}'
                        && chars.get(j + 1).unwrap_or(&'\0') == &'>'
                    {
                        break;
                    }
                    data_chars.push(chars[j]);
                    j += 1;
                }
                let data = data_chars
                    .chunks_exact(2)
                    .map(|a| u8::from_str_radix(&a.iter().collect::<String>(), 16).unwrap())
                    .collect::<Vec<u8>>();

                target.write_u8(0x0e)?;
                target.write_u8(0x00)?;
                target.write_u16le(tag_group_id)?;
                target.write_u16le(tag_id)?;
                target.write_u16le(data.len() as u16)?;
                target.write_bytes(&data)?;

                i += 14 + data_chars.len();
            } else {
                let u16 = chars[i].to_string().encode_utf16().collect::<Vec<u16>>();
                let u8 = u16.iter().fold(vec![], |mut acc, x| {
                    acc.extend_from_slice(&x.to_ne_bytes());
                    acc
                });
                target.write_bytes(&u8)?;
                i += 1;
            }
        }

        target.write_u8(0)?;
    }

    let pos = target.pos()?;
    let text_size = pos - table_pos;
    target.write_u32le_at(text_size_pos, text_size as u32)?;
    target.write_bytes(&vec![0xab; 16 - pos as usize % 16])?;
    Ok(())
}
