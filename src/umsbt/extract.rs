use super::UmsbtFile;
use dh::{recommended::*, Readable, Writable};
use std::io::Result;

pub fn extract<'a>(
    reader: &mut dyn Readable<'a>,
    target: &mut dyn Writable<'a>,
    file: &UmsbtFile,
    buffer_size: u64,
) -> Result<()> {
    reader.copy_at(file.offset as u64, file.size as u64, target, buffer_size)
}
