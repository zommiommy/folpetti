use super::*;

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Section {
    /// Index of the section name in the string tbl .shstrtab
    pub sh_name: u32,             

    /// Section type  
    pub sh_type: ELFSectionType,   
    
    /// Section flags
    pub sh_flags: ELFSectionAttributeFlags,  

    /// Section virtual address at execution
    pub sh_addr: u64,               

    /// Section file offset
    pub sh_offset: u64,

    /// Section size in bytes
    pub sh_size: u64,

    /// Link to another section. This is useful for 
    /// SHT_SYMTAB, SHT_DYNSYM, or SHT_DYNAMIC section
    pub sh_link: u32,

    /// Additional section information
    pub sh_info: u32,

    /// Section alignment
    pub sh_addralign: u64,

    /// Entry size if section holds table
    /// Throws an error if the sh_size % sh_entsize != 0
    pub sh_entsize: u64,

    // Extra fields
    pub data: Option<Vec<u8>>,
}

impl Section {
    /// Parse the section header.
    /// # Arguments
    /// * `data` : &[u8] - a reference to the slice of data
    /// * `e_shoff` : u64 - offset from the start of the file of the section header
    /// * `e_shentsize` : u64 - dimesion of the section header
    /// * `e_shnum` : u64 - number of section in the elf
    pub fn parse(
        data: &[u8],
        sh_offset: u64,
        ei_data: ELFData,
    ) -> Section {
        let sec_data = &data[sh_offset as usize ..];

        let (sec_data, sh_name)      = get_field!(sec_data, u32, ei_data);
        let (sec_data, sh_type)      = get_field!(sec_data, u32, ei_data);
        let (sec_data, sh_flags)     = get_field!(sec_data, u64, ei_data);
        let (sec_data, sh_addr)      = get_field!(sec_data, u64, ei_data);
        let (sec_data, sh_offset)    = get_field!(sec_data, u64, ei_data);
        let (sec_data, sh_size)      = get_field!(sec_data, u64, ei_data);
        let (sec_data, sh_link)      = get_field!(sec_data, u32, ei_data);
        let (sec_data, sh_info)      = get_field!(sec_data, u32, ei_data);
        let (sec_data, sh_addralign) = get_field!(sec_data, u64, ei_data);
        let (sec_data, sh_entsize)   = get_field!(sec_data, u64, ei_data);
        let _ = sec_data;
        
        let sh_type = ELFSectionType::from(sh_type);
        let sh_flags = ELFSectionAttributeFlags::from(sh_flags);

        let mut result = Section{
            sh_name,
            sh_type,
            sh_flags,
            sh_addr,
            sh_offset,
            sh_size,
            sh_link,
            sh_info,
            sh_addralign,
            sh_entsize,
            data: None,
        };

        result.data = match sh_type {
            // Identifies the section header as inactive. This section header 
            // does not have an associated section.
            ELFSectionType::SHT_NULL => None,
            // Identifies a section that occupies no space in the file but 
            // otherwise resembles SHT_PROGBITS.
            ELFSectionType::SHT_NOBITS => None,
            // Unknown data is not meaningfull
            ELFSectionType::UNKNOWN(_) => None,
            // Load the data
            _ => {
                Some(data[
                    sh_offset as usize 
                    .. 
                    sh_offset as usize + sh_size as usize
                ].to_vec())
            }
        };

        result
    }

    /// write the section to the start of the buffer.
    /// Therefore we expect to be already at the right position
    pub fn write(&self, buffer: &mut [u8], endianess: ELFData){
        let buffer = write_field!(buffer, u32, endianess, self.sh_name);
        let buffer = write_field!(buffer, u32, endianess, u32::from(self.sh_type));
        let buffer = write_field!(buffer, u64, endianess, u64::from(self.sh_flags.clone()));
        let buffer = write_field!(buffer, u64, endianess, self.sh_addr);
        let buffer = write_field!(buffer, u64, endianess, self.sh_offset);
        let buffer = write_field!(buffer, u64, endianess, self.sh_size);
        let buffer = write_field!(buffer, u32, endianess, self.sh_link);
        let buffer = write_field!(buffer, u32, endianess, self.sh_info);
        let buffer = write_field!(buffer, u64, endianess, self.sh_addralign);
        let buffer = write_field!(buffer, u64, endianess, self.sh_entsize);
        let _ = buffer;
    }

    /// Write the data, since this is arbitrary and due to the fields
    /// we need the WHOLE BUFFER not a reference given by the layout
    pub fn write_data(&self, whole_buffer: &mut [u8]){
        if let Some(bytes) = &self.data {
            whole_buffer[
                self.sh_offset as usize
                ..
                (self.sh_offset + self.sh_size) as usize
            ].clone_from_slice(bytes);
        }
    }
}