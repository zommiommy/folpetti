#[allow(non_camel_case_types)]
use super::*;

use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

/// ELF class, the goal is to be able to read an elf
/// modifiy it and re-write it. This might be useful for
/// writing polyglots files. All the fields should be 
/// freely modifiable by the user and thus checked for 
/// correctness when writing the file
#[derive(Debug, PartialEq, Clone)]
pub struct ELF {
    /// ELF Header
    pub header: ELFHeader,
    
    /// Vector of sections
    pub sections: Vec<Section>,

    /// Map of <Index of section, Name>
    pub sections_names: HashMap<u32, String>,

    /// Vector of the segments
    pub segments: Vec<Segment>,
}

/// ELF parsing methods
impl ELF {
    /// Take a buffer of bytes and parse it as an ELF.
    pub fn parse(data: &[u8]) -> ELF {
        let mut result = ELF{
            header: ELFHeader::parse(data),
            sections: Vec::new(),
            sections_names: HashMap::new(),
            segments: Vec::new(),
        };
        result.parse_sections(data);
        result.parse_sections_names(data);
        result.parse_segments(data);
        result
    }

    /// Parse the sections, once the ELF header is parsed
    fn parse_sections(
        &mut self,
        data: &[u8],
    ) {
        self.sections = (0..self.header.e_shnum).map(|i| {
            Section::parse(
                data,
                self.header.e_shoff as u64 + 
                    i  as u64 * self.header.e_shentsize as u64,
                self.header.ei_data,
            )
        }).collect::<Vec<Section>>();
    }

    /// Parse the segments, once the ELF header is parsed
    fn parse_segments(
        &mut self,
        data: &[u8],
    ) {
        self.segments = (0..self.header.e_phnum).map(|i| {
            Segment::parse(
                data,
                self.header.e_phoff as u64 + 
                    i  as u64 * self.header.e_phentsize as u64,
                self.header.ei_data,
            )
        }).collect::<Vec<Segment>>();
    }

    /// Parse the sections names table, once the ELF header AND THE SECTIONS 
    /// are parsed
    fn parse_sections_names(
        &mut self,
        data: &[u8],
    ) {
        let shstrtab = &self.sections[self.header.e_shstrndx as usize];

        let strs = &data[
            shstrtab.sh_offset as usize + 1 
            .. 
            shstrtab.sh_offset as usize + shstrtab.sh_size as usize
        ];

        let mut sections_names = HashMap::new();

        for section in &self.sections[1..]{
            let mut name = String::new();

            // sections with sh_name == 0 have no name
            if  section.sh_name == 0 {
                sections_names.insert(section.sh_name, "".to_string());
                continue;
            }

            let mut index = section.sh_name - 1;
            while strs[index as usize] != b'\0' {
                name.push( strs[index as usize] as char);
                index += 1;
            }
            sections_names.insert(section.sh_name, name);
            
        }

        self.sections_names = sections_names;
    }
}

/// ELF class getters and utils methods
impl ELF {
    /// Given a section returns its name (THIS MIGHT HAVE DUPLICATES)
    pub fn get_section_name(&self, section: &Section) -> Option<&str> {
        self.sections_names.get(&section.sh_name).map(|x| x.as_str())
    }

    /// Given the name of a section return the index OF THE FIRST SECTION with 
    /// that name
    pub fn get_section_index(&self, name: &str) -> Result<usize> {
        for (i, section) in self.sections.iter().enumerate() {
            if let Some(sec_name) = self.get_section_name(&section){
                if sec_name == name {
                    return Ok(i);
                }
            }
        }
        Err(Error::SectionNotFound{
            section_name:name.to_string()
        })
    }

    /// Given the name of a section return the FIRST SECTION with that name
    pub fn get_section_by_name(&self, name: &str) -> Result<&Section> {
        Ok(&self.sections[self.get_section_index(name)?])
    }
    /// Given the name of a section return the FIRST SECTION with that name
    pub fn get_section_by_name_mut(&mut self, name: &str) 
        -> Result<&mut Section> {
        let index = self.get_section_index(name)?;
        Ok(&mut self.sections[index])
    }

    /// Move the section 
    pub fn move_section_by_name(&mut self, name:&str, new_offset: usize) 
        -> Result<()> {
        let section = self.get_section_by_name_mut(name)?;
        section.sh_offset = new_offset as u64;
        // TODO!: Fix the segments else we get a pagefault because of permissions
        unimplemented!("TODO!:");
        Ok(())
    }

    pub fn remove_section_by_name(&mut self, name: &str) -> Result<Section> {
        let section = self.sections.remove(
            self.get_section_index(name)?
        );
        self.header.e_shnum = self.header.e_shnum.saturating_sub(1);
        Ok(section)
    }

    /// Return an hashmap with the section name as key
    /// and a tuple with (Start, End) offsets in the file.
    pub fn get_layout(&self) -> HashMap<&str, (usize, usize)> {
        let mut result = HashMap::new();

        result.insert("header", (0, 64));
        result.insert("sections_table", (
            self.header.e_shoff as usize,
            self.header.e_shoff as usize + (
                self.header.e_shentsize * self.header.e_shnum
            ) as usize
        ));
        result.insert("segments_table", (
            self.header.e_phoff as usize,
            self.header.e_phoff as usize + (
                self.header.e_phentsize * self.header.e_phnum
            ) as usize
        ));

        for section in self.sections[1..].iter() {
            result.insert(
                self.get_section_name(section).unwrap(),
                (
                    section.sh_offset as usize, 
                    (section.sh_offset + section.sh_size) as usize
                )
            );
        }

        result
    }

    /// Return the layout as the sorted vector of:
    /// `(Section name, Start offset, End offset, size)`.
    pub fn get_layout_vec(&self) -> Vec<(&str, usize, usize, usize)> {
        let mut vector = self.get_layout().iter().map(
            |(name, (start, end))| 
                (*name, *start, *end, *end - *start)
        ).collect::<Vec<(&str, usize, usize, usize)>>();

        vector.sort_by(|
            (_name1, _start1, _end1, _size1), 
            (_name2, _start2, _end2, _size2)
            | _start1.cmp(_start2)
        );

        vector
    }

    /// Get the size of the final ELF if written in a buffer or file
    pub fn len(&self) -> usize {
        self.get_layout().iter().map(|(_, (start, size))|{
            start + size
        }).max().unwrap()
    }

    /// Return a vector with which sections are in each segment.
    pub fn get_sections_in_each_segment(&self) -> Vec<Vec<usize>> {
        let mut result = Vec::new();
        for segment in &self.segments {
            let range = segment.p_offset..(segment.p_offset + segment.p_filesz);
            let mut mid = Vec::new();
            for (i, section) in self.sections.iter().enumerate() {
                if range.contains(&section.sh_offset) {
                    mid.push(i);
                }
            }
            result.push(mid);
        }
        result
    }

    /// Given the offset of a segment, returns a vector with all the 
    /// sections defined in it.
    pub fn find_sections(&self, segment_offset: usize) -> Vec<&Section> {

        // clear the lowest values to align to a page boundary
        let segment_offset = segment_offset & (!0xfff);

        // find the referenced segment
        let mut right_segment = None;
        for segment in self.segments.iter() {
            // Compute the portion of the file where the segment is.
            // Since the loader will always load segments in separate pages
            // (thus aligned to 0x1000) to correctly check the range we must
            // round to the previous page start while the end must be round
            // to the NEXT page start. This is tricky to me so this might have
            // bugs.
            let segment_range = (segment.p_offset  & (!0xfff))
                ..
                (0x1000 + (segment.p_offset + segment.p_filesz - 1) & (!0xfff));

            if segment_range.contains(&(segment_offset as _)) {
                right_segment = Some(segment);
                break
            }
        }

        if right_segment.is_none() {
            panic!(
                "Cannot find a segment that starts at {:016x}", 
                segment_offset
            );
        }

        let right_segment = right_segment.unwrap();

        // compute the boundary of the given segment
        let start_off = right_segment.p_offset;
        let end_off   = start_off + right_segment.p_filesz;

        // find all the sections in this file portion
        let mut result = Vec::new();
        for section in self.sections.iter() {
            let section_start = section.sh_offset;
            let section_end = section_start + section.sh_size;

            if section_start >= start_off 
                && section_end < end_off 
                && section.sh_type != ELFSectionType::SHT_NULL {
                result.push(section);
            }
        }

        result
    }
}

impl ELF {
    /// Move the sections so that they are adiacent in the file.
    ///
    /// THIS ASSUMES THAT SH_SIZE IS RIGHT.
    /// 
    /// For simplicity the elf will always have the following order: 
    /// ```ignore
    /// +--------------------+
    /// |       Header       |
    /// +--------------------+
    /// |   Segments table   |
    /// +--------------------+
    /// |       Sections     |
    /// +--------------------+
    /// |   Sections table   |
    /// +--------------------+
    ///
    /// If needed we can change this without problems.
    /// And the sections will be grouped by segment
    ///
    /// TO FIX!!!! TODO
    /// ```
    pub fn compact_sections(&mut self) {
        let new_addresses: Vec<u64> = Vec::new();

        // Order the header and the main tables.

        // The header is always at the top.
        let mut counter = self.header.e_ehsize as u64;

        // Set the segments table just after the header
        self.header.e_phoff = counter;
        counter += (self.header.e_phentsize * self.header.e_phnum) as u64;

        // just set each section in an adiacent manner
        let segsec = self.get_sections_in_each_segment();

        for (i, sections) in segsec.into_iter().enumerate() {
            // set the start of the segment
            self.segments[i].p_offset = counter;

            // update the sections in the current segment
            for section_idx in sections {
                self.sections[section_idx].sh_offset = counter;
                counter += self.sections[section_idx].sh_size;
            }

            self.segments[i].p_filesz = counter - self.segments[i].p_offset;
        }

        // Set the section table just after the segment table
        self.header.e_shoff = counter;
        counter += (self.header.e_shentsize * self.header.e_shnum) as u64;
    }

    /// Build the shstrtab section.
    /// This method create the sections' string table for the current sections.
    /// This is meant to be called after removing sections. 
    /// 
    /// Additionally it will compact the secitons names,
    /// so the sh_name field of each section will be modified!
    /// 
    /// TODO: This method is naive and doesn't do any fancy thing like
    /// storing .got.plt and .plt in the same string.
    pub fn build_shstrtab(&mut self) -> Result<()>{
        // New shstrab content
        let mut buffer : Vec<u8> = Vec::new();
        // new hashmap with the updated indices
        let mut names_map= HashMap::new();

        // The first string (index 0) is always empty
        buffer.push(0); 

        // Now for each section, write its name and update its name index
        // This will probably change the order of the sections and we don't care
        // but its possible to write them orderly
        let mut shstrtab_index = 0;
        let mut i = 1;
        let mut len = 1;
        for section in self.sections.iter_mut() {
            let section_name = self.sections_names.remove(&section.sh_name);
            // Skip not-named sections
            if section_name.is_none() {
                continue;
            }
            let section_name = section_name.unwrap();

            // save the shstrtab index to put it in the header
            if section_name == ".shstrtab" {
                shstrtab_index = i;
            }

            buffer.extend_from_slice(section_name.as_bytes());
            buffer.push(0);

            // fix the index
            let name_length = section_name.len() as u32;
            section.sh_name = len;
            names_map.insert(len, section_name);
            i += 1;
            len += name_length + 1;
        }

        self.sections_names = names_map;
        let shstrtab = self.get_section_by_name_mut(".shstrtab")?;
        shstrtab.sh_size = buffer.len() as u64;
        shstrtab.data = Some(buffer);
        self.header.e_shstrndx = shstrtab_index as u16;
        Ok(())
    }

    /// Write the elf to a buffer which can then be written to a file if needed.
    pub fn write(&self, buffer: &mut [u8]) {
        self.header.write(buffer);

        // write the sections
        for section in &self.sections {
            section.write_data(
                buffer
            );
        }

        // write the sections table
        // TODO: the sections might overlap!!!!! we probably want to compute
        // the result anyway and just return a Result.
        for (i, section) in self.sections.iter().enumerate() {
            let start = self.header.e_shoff as usize + i * self.header.e_shentsize as usize;
            section.write(
                &mut buffer[
                    start
                    ..
                    start + self.header.e_shentsize as usize
                ],
                self.header.ei_data
            );
        }
        
        
        // write the segments table
        for (i, segment) in self.segments.iter().enumerate() {
            let start = self.header.e_phoff as usize 
                + i * self.header.e_phentsize as usize;
            
            segment.write(
                &mut buffer[
                    start
                    ..
                    start + self.header.e_phentsize as usize
                ],
                self.header.ei_data
            );
        }
        // WTF TUTTO QUI????   
    }

    pub fn write_file(&self, filename: &str) {
        let mut buffer = vec![0; self.len()];
        self.write(&mut buffer);

        let mut file = File::create(filename).expect("Can't create the file");
        file.write_all(&buffer).expect("Can't write to file");
    }
}