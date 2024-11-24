mod create;
mod extract;
mod metadata;

pub(crate) mod hashtables;

pub use create::create;
pub use extract::extract;
pub use metadata::metadata;

use dh::Readable;

#[derive(Debug)]
pub struct MsbtMetadata {
    pub big_endian: bool,
    pub files: Vec<MsbtFile>,
    pub attr_offset: u64,
    pub attr_size: u64,
}

#[derive(Debug)]
pub struct MsbtFile {
    pub path: String,
    pub offset: u64,
    pub size: u64,
}

pub struct MsbtFileSource<'a> {
    pub reader: &'a mut dyn Readable<'a>,
    pub metadata: MsbtFile,
}
