use io::prelude::*;
use seedle_gen::Parser;
use std::path::PathBuf;
use std::{error, fs, io};
use walkdir::{DirEntry, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn is_dir(entry: &DirEntry) -> bool {
    entry.metadata().map(|meta| meta.is_dir()).unwrap_or(false)
}

fn clean() -> Result<(), io::Error> {
    let mut path = PathBuf::new();
    path.push("tests");
    path.push("__generated__");
    let walker = WalkDir::new(path).into_iter().filter_map(|e| e.ok());
    for entry in walker.filter(|e| !is_dir(e) && !is_hidden(e)) {
        fs::remove_file(entry.path())?;
    }
    Ok(())
}

fn ready_file(file_name: &str) -> Result<fs::File, Box<dyn error::Error>> {
    let mut path = PathBuf::new();
    path.push("tests");
    path.push("__generated__");
    path.push(file_name);
    println!("{}", path.display());
    fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(path)
        .map_err(|e| e.into())
}

fn render_cddl(mut file: fs::File) -> Result<fs::File, Box<dyn error::Error>> {
    let mut parser = Parser::build()?;
    parser.load_cddl("types", CDDL)?;
    parser
        .render_to(&mut file, &seedle_gen::templates::C)
        .map(|_| file)
        .map_err(|e| e.into())
}

// https://github.com/dtolnay/trybuild/issues/210
fn render_trybuild_hack(mut file: fs::File) -> Result<fs::File, Box<dyn error::Error>> {
    file.write(b"\nfn main () {}")
        .map(|_| file)
        .map_err(|e| e.into())
}

fn flush(mut file: fs::File) -> Result<fs::File, Box<dyn error::Error>> {
    file.flush().map(|_| file).map_err(|e| e.into())
}

#[test]
fn test_build() {
    clean().unwrap();
    ready_file("c.rs")
        .and_then(render_cddl)
        .and_then(render_trybuild_hack)
        .and_then(flush)
        .unwrap();
    let runner = trybuild::TestCases::new();
    runner.pass("tests/__generated__/c.rs");
}

static CDDL: &'static str = r#"
groupa-literal-three = 3
groupa-literal-char = "C"
groupa-literal-four = 4
u8 = uint .size 1
u16 = uint .size 2
u32 = uint .size 4
u64 = uint .size 8
i8 = int .size 1
i16 = int .size 2
i32 = int .size 4
i64 = int .size 8
ip-addr = tstr .size 16
port = { 
    http: u16, 
    label: tstr .size 32 
}
ints = (
    a: u8,
    b: u16,
    c: u32,
    d: u64,
    e: i8,
    f: i16,
    g: i32,
    h: i64,
)
network = {
    dhcp: bool,
    ip: ip-addr,
    sn: ip-addr,
    gw: ip-addr,
    mac: [ 6*6 u8 ],
    ids: [ 2*2 u32 ]
}
thing = {
    ints,
    update: [ 4096*4096 u8 ],
    net: network,
    ports: [ 4*4 port ]
}
"#;
