use super::{UmsbtFile, UmsbtMetadata};
use dh::{recommended::*, Readable};
use std::io::Result;

/*

I could not find ANY specification for this format. I've taken a look at the
Kuriimu source code, if the devs made a mistake over there, I'm making the same
mistake here. Also, this was the first time I had to read C# code...

Due to the format being so simple, it was harder to find out how the file works
than to actually implement it.

*/

pub fn metadata(reader: &mut dyn Readable) -> Result<UmsbtMetadata> {
    let body_offset = reader.read_i32le()? as u64;
    reader.rewind()?;

    let mut files = vec![];
    let mut i = 0;
    while reader.pos()? < body_offset {
        let file = UmsbtFile {
            offset: reader.read_i32le()?,
            size: reader.read_i32le()?,
            path: format!("{i:08}.msbt"),
        };

        if file.size <= 0 {
            break;
        }

        files.push(file);
        i += 1;
    }

    Ok(UmsbtMetadata { files })
}
