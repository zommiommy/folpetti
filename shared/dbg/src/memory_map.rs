use super::*;
use std::fs;
use std::io::Read;
use std::collections::HashMap;


/// Structs that collects the permission of a given section
#[derive(Debug, Clone, Copy)]
pub struct SectionPermissions {
    /// The start address of this section
    start_address: usize,

    /// The end address of this section
    end_address: usize,
    
    /// If the current section is readable
    read: bool,

    /// If the current section is writable
    write: bool,

    /// If the current section is executable
    execute: bool,
}        



/// This collects the informations about the memory maps of the process
/// This is needed to be able to set relative breakpoints and in general
/// resolve relative addresses.
#[derive(Debug, Clone)]
pub struct MemoryMap {
    files: HashMap<String, ELF>,
    functions: HashMap<String, Address>,
    memory_map: HashMap<String, SectionPermissions>,
}

impl Default for MemoryMap {
    fn default() -> Self {
        MemoryMap{
            files: HashMap::new(),
            functions: HashMap::new(),
            memory_map: HashMap::new(),
        }
    }
}

impl MemoryMap {
    pub fn new(pid: Pid) -> MemoryMap {
        // Follow the /proc/<PID>/exe symlink to get the absolute path to the
        // executable

        let current_executable_path = fs::canonicalize(
            format!("/proc/{}/exe", pid)
        ).expect("Could not read the path to the current executable")
            .to_str().unwrap().to_string();

        //  Read the memory maps of the running process
        let proc_maps = fs::read_to_string(format!("/proc/{}/maps", pid))
            .expect("Could not read the memory map of the child process");

        println!("{}", proc_maps);

        let mut files = HashMap::new();
        let mut memory_map = HashMap::new();

        for line in proc_maps.split("\n") {
            if line.trim().is_empty() {
                continue;
            }
            let (path , offset, permissions) = parse_proc_maps_line(line);

            println!("{} {:x} {:#4x?}", path, offset, permissions);
            // Some segments have no path so we will ignore them because we don't
            // have any symbol to reference them
            if path.is_empty() {
                continue;
            }

            // Some segments are not elfs e.g:
            // [stack], [heap], [vvar], [vdso], [vsyscall]
            // so we will just load them as is because we don't need to resolve
            // symbols.
            if path.starts_with("[") {
                memory_map.insert(path, permissions);
                continue;
            }

            // we have a segment of an ELF so we need to load it so we can
            // associate the segments to each sections, so it's easy to reference
            // them.

            // get the elf associated to the current memory map
            // if it's not present load it.
            let elf = files.entry(path.to_string()).or_insert_with(|| {
                let mut f = fs::File::open(&path).expect("no file found");
                let metadata = fs::metadata(&path).expect("unable to read metadata");
                let mut buffer = vec![1; metadata.len() as usize];
                f.read(&mut buffer).expect("buffer overflow");
                ELF::parse(&buffer)
            });

            // find all the sections that maps to the current segment
            let sections = elf.find_sections(offset);
            
            // Add each section to the memory map
            for section in sections {
                // Skip sections that are not actually loaded
                if section.sh_type != ELFSectionType::SHT_PROGBITS {
                    continue;
                }

                // Compute the boundary addresses of the section 
                // in the current loading (this is needed for ALSR)
                let start_address = permissions.start_address 
                    + section.sh_addr as usize;
                let end_address = start_address 
                    + section.sh_size as usize;

                // the name will be the original file concatenated with the 
                // section name, so for a file `/bin/cat` the `.text` section will
                // have name `/bin/cat.text`.
                // While for the current executable text it will just be:
                // `.text` 
                let section_name = elf.get_section_name(section).unwrap();
                let section_name = if path != current_executable_path {
                    format!("{}{}", path, section_name)
                } else {
                    section_name.to_string()
                };

                // Add the section to the memory map,
                let section_permissions = SectionPermissions{
                    start_address,
                    end_address,
                    read: permissions.read,
                    write: permissions.write,
                    execute: permissions.execute,
                };
                println!("{} {:x} {:#4x?}", section_name, section.sh_offset ,section_permissions);
                memory_map.insert(
                    section_name, 
                    section_permissions,
                );
            }
        }

        // TODO!: Parse the debug symbols, if present so that we can reference 
        // functions. This requires a dwarf parser tho

        MemoryMap{
            files,
            memory_map,
            functions: HashMap::new(),
        }
    }

    /// This function resolve addresses.
    /// The syntax for the relative addresses is:
    /// `.text` to reference the text section of the main executable
    /// `libc-2.33.so.text` to reference the text section of the library `libc-2.33.so`
    pub fn resolve_address(&self, address: &Address) -> usize {
        match address {
            Address::Absolute(addr) => *addr,
            Address::Section(section_name, offset) => {
                self.memory_map.get(section_name).unwrap().start_address 
                    + offset
            },
            _ => unimplemented!("The address function relative is not yet supported"),
        }
    }


}

/// Convert a line to a section permissions, this expects line in the format:
/// `56187cdca000-56187cdcc000 r--p 00000000 103:02 7733675                   /usr/bin/cat`
fn parse_proc_maps_line(line: &str) -> (String, usize, SectionPermissions) {
    let (address, line) = line.split_once(" ").unwrap();
    let (start_address, end_address) = address.split_once("-").unwrap();
    let start_address = usize::from_str_radix(start_address, 16).unwrap();
    let end_address   = usize::from_str_radix(end_address, 16).unwrap();

    let (permissions, line) = line.split_once(" ").unwrap();
    let read = &permissions[1..2] == "r";
    let write = &permissions[2..3] == "w";
    let execute= &permissions[3..4] == "x";


    let (offset, line) = line.split_once(" ").unwrap();
    let offset = usize::from_str_radix(offset, 16).unwrap();

    let (_device, line) = line.split_once(" ").unwrap();
    let (_idk, line) = line.split_once(" ").unwrap();


    let path = line.trim().to_string();


    (
        path, 
        offset, 
        SectionPermissions{
            start_address,
            end_address,
            read,
            write,
            execute,
        }
    )
}

#[cfg(test)]
mod test {
    use super::parse_proc_maps_line;

    #[test]
    fn test_parse_proc_maps_line() {
        let test_line = "56187cdca000-56187cdcc000 r--p 00000420 103:02 7733675                   /usr/bin/cat";
        let (
            path,
            offset,
            permissions,
        ) = parse_proc_maps_line(test_line);

        assert_eq!(path, "/usr/bin/cat");
        assert_eq!(offset, 0x00000420);
        assert_eq!(permissions.start_address, 0x56187cdca000);
        assert_eq!(permissions.end_address,   0x56187cdcc000);
        assert_eq!(permissions.read,   true);
        assert_eq!(permissions.write,   false);
        assert_eq!(permissions.execute,   false);
    }
}