use acridotheres_3ds::umsbt::{self, UmsbtFile, UmsbtFileSource};
use dh::recommended::*;

#[test]
fn umsbt_downtown() {
    let mut reader = dh::file::open_r("tests/samples/KO_Sp_Downtown.umsbt").unwrap();
    let metadata = umsbt::metadata(&mut reader).unwrap();

    assert_eq!(metadata.files.len(), 5);

    for file in metadata.files {
        let mut target = dh::data::write_empty();
        umsbt::extract(&mut reader, &mut target, &file, 1024).unwrap();
        let data = dh::data::close(target);

        assert!(data.starts_with(b"MsgStdBn"));
        assert_eq!(data.len() as i32, file.size);
    }
}

#[test]
fn umsbt_dream() {
    let mut reader = dh::file::open_r("tests/samples/NPC_Dream.umsbt").unwrap();
    let metadata = umsbt::metadata(&mut reader).unwrap();

    assert_eq!(metadata.files.len(), 5);

    for file in metadata.files {
        let mut target = dh::data::write_empty();
        umsbt::extract(&mut reader, &mut target, &file, 1024).unwrap();
        let data = dh::data::close(target);

        assert!(data.starts_with(b"MsgStdBn"));
        assert_eq!(data.len() as i32, file.size);
    }
}

#[test]
fn umsbt_secretary() {
    let mut reader = dh::file::open_r("tests/samples/NPC_Secretary.umsbt").unwrap();
    let metadata = umsbt::metadata(&mut reader).unwrap();

    assert_eq!(metadata.files.len(), 5);

    for file in metadata.files {
        let mut target = dh::data::write_empty();
        umsbt::extract(&mut reader, &mut target, &file, 1024).unwrap();
        let data = dh::data::close(target);

        assert!(data.starts_with(b"MsgStdBn"));
        assert_eq!(data.len() as i32, file.size);
    }
}

#[test]
fn custom_umsbt() {
    let mut file0 = dh::data::read_ref(b"Hello, world!");
    let mut file1 = dh::data::read_ref(b"Hello, world! 2");

    let files = vec![
        UmsbtFileSource {
            reader: &mut file1,
            metadata: UmsbtFile {
                offset: 0,
                size: 15,
                path: "00000001.msbt".to_string(),
            },
        },
        UmsbtFileSource {
            reader: &mut file0,
            metadata: UmsbtFile {
                offset: 0,
                size: 13,
                path: "00000000.msbt".to_string(),
            },
        },
    ];

    let mut target = dh::data::rw_empty();

    umsbt::create(files, &mut target, 1024).unwrap();

    target.rewind().unwrap();
    let metadata = umsbt::metadata(&mut target).unwrap();

    assert_eq!(metadata.files.len(), 2);

    assert_eq!(metadata.files[0].path, "00000000.msbt");
    assert_eq!(metadata.files[0].size, 13);
    let mut file0 = dh::data::write_empty();
    umsbt::extract(&mut target, &mut file0, &metadata.files[0], 1024).unwrap();
    assert_eq!(dh::data::close(file0), b"Hello, world!");

    assert_eq!(metadata.files[1].path, "00000001.msbt");
    assert_eq!(metadata.files[1].size, 15);
    let mut file1 = dh::data::write_empty();
    umsbt::extract(&mut target, &mut file1, &metadata.files[1], 1024).unwrap();
    assert_eq!(dh::data::close(file1), b"Hello, world! 2");
}
