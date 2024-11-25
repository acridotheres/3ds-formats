use acridotheres_3ds::msbt::{self, MsbtFile, MsbtFileSource};
use dh::recommended::*;

#[test]
fn msbt_downtown() {
    let mut reader = dh::file::open_r("tests/samples/KO_Sp_Downtown.msbt").unwrap();
    let metadata = msbt::metadata(&mut reader).unwrap();
    assert_eq!(metadata.files.len(), 16);

    let mut target = dh::data::write_empty();
    msbt::extract(&mut reader, &mut target, &metadata.files[8]).unwrap();
    let string = String::from_utf8(dh::data::close(target)).unwrap();
    assert!(string.starts_with("<{0003,0017,}>You came all this way to shop?"));
}

#[test]
fn msbt_dream() {
    let mut reader = dh::file::open_r("tests/samples/NPC_Dream.msbt").unwrap();
    let metadata = msbt::metadata(&mut reader).unwrap();
    assert_eq!(metadata.files.len(), 117);

    let mut target = dh::data::write_empty();
    msbt::extract(&mut reader, &mut target, &metadata.files[31]).unwrap();
    let string = String::from_utf8(dh::data::close(target)).unwrap();
    assert!(string.starts_with("<{0003,0025,}><{000e,0003,}><{0005,0005,}>'s Dream Address"));
}

#[test]
fn msbt_secretary() {
    let mut reader = dh::file::open_r("tests/samples/NPC_Secretary.msbt").unwrap();
    let metadata = msbt::metadata(&mut reader).unwrap();
    assert_eq!(metadata.files.len(), 209);

    let mut target = dh::data::write_empty();
    msbt::extract(&mut reader, &mut target, &metadata.files[31]).unwrap();
    let string = String::from_utf8(dh::data::close(target)).unwrap();
    assert!(string.starts_with("Sure thing!"));
}

#[test]
fn msbt_an_3p_fu() {
    let mut reader = dh::file::open_r("tests/samples/AN_3P_Fu.msbt").unwrap();
    let metadata = msbt::metadata(&mut reader).unwrap();
    assert_eq!(metadata.files.len(), 56);

    let mut target = dh::data::write_empty();
    msbt::extract(&mut reader, &mut target, &metadata.files[31]).unwrap();
    let string = String::from_utf8(dh::data::close(target)).unwrap();
    assert!(string.starts_with("<{0003,0016,}>Oh!<{0007,0000,14000000}> That might work!"));
}

#[test]
fn custom_msbt() {
    let mut file0 = dh::data::read_ref(b"Hello, world!");
    let mut file1 = dh::data::read_ref(b"Hello, world! 2");
    let mut file2 = dh::data::read_ref(b"<{ABCD,EF01,23456789}>Test data<{9876,5432,10FEDCBA}>");

    let files = vec![
        MsbtFileSource {
            reader: &mut file0,
            metadata: MsbtFile {
                offset: 0,
                size: 13,
                path: "test.txt".to_string(),
            },
        },
        MsbtFileSource {
            reader: &mut file1,
            metadata: MsbtFile {
                offset: 0,
                size: 15,
                path: "test2.txt".to_string(),
            },
        },
        MsbtFileSource {
            reader: &mut file2,
            metadata: MsbtFile {
                offset: 0,
                size: 53,
                path: "test3.bin".to_string(),
            },
        },
    ];

    let mut target = dh::data::rw_empty();

    msbt::create(files, &mut dh::data::rw_empty(), &mut target, 1024).unwrap();

    target.rewind().unwrap();
    let metadata = msbt::metadata(&mut target).unwrap();

    assert_eq!(metadata.files.len(), 3);

    assert_eq!(metadata.files[0].path, "test.txt");
    assert_eq!(metadata.files[0].size, 27);
    let mut file0 = dh::data::write_empty();
    msbt::extract(&mut target, &mut file0, &metadata.files[0]).unwrap();
    assert_eq!(dh::data::close(file0), b"Hello, world!");

    assert_eq!(metadata.files[1].path, "test2.txt");
    assert_eq!(metadata.files[1].size, 31);
    let mut file1 = dh::data::write_empty();
    msbt::extract(&mut target, &mut file1, &metadata.files[1]).unwrap();
    assert_eq!(dh::data::close(file1), b"Hello, world! 2");

    assert_eq!(metadata.files[2].path, "test3.bin");
    assert_eq!(metadata.files[2].size, 43);
    let mut file2 = dh::data::write_empty();
    msbt::extract(&mut target, &mut file2, &metadata.files[2]).unwrap();
    assert_eq!(
        dh::data::close(file2),
        b"<{abcd,ef01,23456789}>Test data<{9876,5432,10fedcba}>"
    );
}
