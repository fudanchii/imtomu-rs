use form::create_directory_structure;
use rustfmt_nightly::{Config, Input, Session};
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::PathBuf;
use svd2rust::{generate, Generation, Target::CortexM};

pub fn main() -> () {
    //parse xml
    let xml = read_to_string("svd/EFM32HG309F64.svd").unwrap();
    let Generation {
        lib_rs,
        device_specific,
        ..
    } = generate(&xml, &CortexM, true).unwrap();

    let device_specific = device_specific.unwrap();

    //save other files
    writeln!(
        File::create("device.x").unwrap(),
        "{}",
        device_specific.device_x
    )
    .unwrap();
    writeln!(
        File::create("build.rs").unwrap(),
        "{}",
        device_specific.build_rs
    )
    .unwrap();

    //form lib.rs and save
    create_directory_structure("src/", lib_rs).unwrap();

    //rustfmt saved files
    let files = ["build.rs", "src/lib.rs"].iter().map(|a| PathBuf::from(a));
    let mut buf = Vec::<u8>::new();
    let mut session = Session::new(Config::default(), Some(&mut buf));

    for path in files {
        session.format(Input::File(path)).unwrap();
    }
}
