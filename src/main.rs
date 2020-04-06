#[macro_use]
extern crate clap;
extern crate protoc_rust;
use clap::{App, Arg};
use protoc_rust::Customize;
use std::error;
use std::fs;
use std::path::Path;

fn main() {
    let matches = App::new("Protobuf based rust file generator")
        .version("0.1.0")
        .author("Guillaume Balaine <guillaumeb@adikteev.com>")
        .about("Generates rust model files from a protobuf schemas")
        .arg(
            Arg::with_name("files")
                .short("f")
                .long("files")
                .takes_value(true)
                .help("List of file paths"),
        )
        .arg(
            Arg::with_name("out-dir")
                .short("o")
                .long("out-dir")
                .takes_value(true)
                .help("Output directory"),
        )
        .get_matches();

    let files: Vec<String> = values_t!(matches.values_of("files"), String).unwrap();
    let input: Vec<&str> = files.iter().map(|s| s.as_str()).collect();
    let mut parent_dirs: Vec<&Path> = input
        .iter()
        .map(|f| Path::new(f).parent().unwrap())
        .collect();
    parent_dirs.dedup();
    let includes: Vec<&str> = parent_dirs.iter().map(|p| p.to_str().unwrap()).collect();
    let out_dir = matches.value_of("out-dir").unwrap();
    fs::create_dir_all(out_dir).unwrap();
    let mut args = protoc_rust::Args::new();
    args.out_dir(out_dir);
    args.inputs(input.clone());
    args.includes(includes);
    args.customize(Customize {
        serde_derive: Some(true),
        ..Default::default()
    });
    args.run().expect("protoc");
}
