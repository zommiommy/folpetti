use std::env;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::process::exit;
use elf::ELF;

const SECTIONS_TO_REMOVE: [&str; 3] = [
    ".comment",
    ".note.ABI-tag",
    ".note.gnu.build-id",
    //".gnu.version_r"
];

fn main() {  
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Usage: strip filename");
        exit(1);
    }

    // read the file
    let filename = args[1].as_str();
    let mut f = File::open(filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![1; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    // parse it
    let mut elf = ELF::parse(&buffer);

    let segsec = elf.get_sections_in_each_segment();

    // Remove the sections
    for section_name in &SECTIONS_TO_REMOVE {
        let _ = elf.remove_section_by_name(section_name);
    }

    // re-build the section string tab
    elf.build_shstrtab();
    
    // STILL BUGGED
    elf.compact_sections();
    
    // write the file
    let mut dst_filename = filename.to_string();
    dst_filename.push_str("_stripped");
    println!("{}", dst_filename);
    elf.write_file(&dst_filename);
}
