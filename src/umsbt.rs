mod create;
mod extract;
mod metadata;

pub use create::create;
pub use extract::extract;
pub use metadata::metadata;

use dh::Readable;

#[derive(Debug)]
pub struct UmsbtMetadata {
    pub files: Vec<UmsbtFile>,
}

#[derive(Debug)]
pub struct UmsbtFile {
    pub offset: i32,
    pub size: i32,
    pub path: String,
}

pub struct UmsbtFileSource<'a> {
    pub reader: &'a mut dyn Readable<'a>,
    pub metadata: UmsbtFile,
}
