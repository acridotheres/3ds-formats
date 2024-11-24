use super::MsbtFile;
use dh::{recommended::*, Readable, Writable};
use std::io::Result;

pub fn extract<'a>(
    reader: &mut dyn Readable<'a>,
    target: &mut dyn Writable<'a>,
    file: &MsbtFile,
) -> Result<()> {
    let mut bytes = reader.read_bytes_at(file.offset, file.size)?;
    bytes.pop();

    let mut i = 0;
    let mut new_bytes = vec![];
    while i < bytes.len() {
        if bytes[i] == 0x0E && bytes[i + 1] == 0x00 {
            let tag_group_id = bytes[i + 2] as u16 | (bytes[i + 3] as u16) << 8;
            let tag_id = bytes[i + 4] as u16 | (bytes[i + 5] as u16) << 8;
            let data_size = bytes[i + 6] as u16 | (bytes[i + 7] as u16) << 8;
            let data = &bytes[i + 8..i + 8 + data_size as usize];

            let string = format!(
                "<{{{:04x},{:04x},{}}}>",
                tag_group_id,
                tag_id,
                data.iter()
                    .fold(String::new(), |acc, x| acc + &format!("{:02x}", x))
            );
            let u16 = string.encode_utf16().collect::<Vec<u16>>();
            let u8 = u16.iter().fold(vec![], |mut acc, x| {
                acc.extend_from_slice(&x.to_ne_bytes());
                acc
            });
            new_bytes.push(u8);

            i += 8 + data_size as usize;
        } else {
            new_bytes.push(vec![bytes[i]]);
            i += 1;
        }
    }

    let bytes = new_bytes.iter().fold(vec![], |mut acc: Vec<u8>, x| {
        acc.extend(x);
        acc
    });

    let u16: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|a| u16::from_ne_bytes([a[0], a[1]]))
        .collect();
    let string = String::from_utf16(&u16).unwrap_or("".to_string());

    target.write_utf8(&string)?;
    Ok(())
}
