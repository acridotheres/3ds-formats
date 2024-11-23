use super::UmsbtFileSource;
use dh::{recommended::*, Writable};
use std::io::Result;

pub fn create<'a>(
    mut files: Vec<UmsbtFileSource<'a>>,
    target: &mut dyn Writable<'a>,
    buffer_size: u64,
) -> Result<()> {
    let body_offset = files.len() * 8;
    target.write_bytes(vec![0; body_offset].as_slice())?;

    files.sort_by(|a, b| a.metadata.path.cmp(&b.metadata.path));

    for (i, file) in files.into_iter().enumerate() {
        let offset = target.pos()? as i32;
        file.reader.copy_at(
            file.metadata.offset as u64,
            file.metadata.size as u64,
            target,
            buffer_size,
        )?;

        target.write_i32le_at((i * 8) as u64, offset)?;
        target.write_i32le_at((i * 8 + 4) as u64, file.metadata.size)?;
    }

    Ok(())
}
