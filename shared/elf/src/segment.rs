use super::*;

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
/// An executable or shared object file's program header table is an array of s
/// tructures, each describing a segment or other information the system needs 
/// to prepare the program for execution. An object file segment contains one or 
/// more sections. Program headers are meaningful only for executable and shared
///  object files. A file specifies its own program header size with the ELF 
/// header's e_phentsize and e_phnum members.
pub struct Segment {
    /// This member tells what kind of segment this array element describes or 
    /// how to interpret the array element's information.
    pub p_type: SegmentType,             
    
    /// This member gives flags relevant to the segment.
    pub p_flags: SegmentFlags,    

    /// This member gives the offset from the beginning of the file at which the
    /// first byte of the segment resides.
    pub p_offset: u64,              
    
    /// This member gives the virtual address at which the first byte of the 
    /// segment resides in memory.
    pub p_vaddr: u64,     

    /// On systems for which physical addressing is relevant, this member is 
    /// reserved for the segment's physical address. Because System V ignores 
    /// physical addressing for application programs, this member has 
    /// unspecified contents for executable files and shared objects.
    pub p_addr: u64,
    
    /// This member gives the number of bytes in the file image of the segment; 
    /// it may be zero.
    pub p_filesz: u64,
    
    /// This member gives the number of bytes in the memory image of the segment;
    /// it may be zero.
    pub p_memsz: u64,
    
    /// As ``Program Loading'' describes in this chapter of the processor 
    /// supplement, loadable process segments must have congruent values for 
    /// p_vaddr and p_offset, modulo the page size. This member gives the value 
    /// to which the segments are aligned in memory and in the file. Values 0 
    /// and 1 mean no alignment is required. Otherwise, p_align should be a 
    /// positive, integral power of 2, and p_vaddr should equal p_offset, 
    /// modulo p_align.
    pub p_align: u64,
}

impl Segment {
    /// Parse the section header.
    /// # Arguments
    /// data: &[u8] a reference to the slice of data
    /// e_shoff: u64 offset from the start of the file of the section header
    /// e_shentsize: u64 dimesion of the section header
    /// e_shnum: u64 number of section in the elf
    pub fn parse(
        data: &[u8],
        e_phentsize: u64,
        ei_data: ELFData,
    ) -> Segment {
        let seg_data = &data[e_phentsize as usize ..];

        let (seg_data, p_type)   = get_field!(seg_data, u32, ei_data);
        let (seg_data, p_flags)  = get_field!(seg_data, u32, ei_data);
        let (seg_data, p_offset) = get_field!(seg_data, u64, ei_data);
        let (seg_data, p_vaddr)  = get_field!(seg_data, u64, ei_data);
        let (seg_data, p_addr)   = get_field!(seg_data, u64, ei_data);
        let (seg_data, p_filesz) = get_field!(seg_data, u64, ei_data);
        let (seg_data, p_memsz)  = get_field!(seg_data, u64, ei_data);
        let (seg_data, p_align)  = get_field!(seg_data, u64, ei_data);
        let _ = seg_data;
        
        let p_type = SegmentType::from(p_type);
        let p_flags = SegmentFlags::from(p_flags);

        let result = Segment{
            p_type,
            p_flags,
            p_offset,
            p_vaddr,
            p_addr,
            p_filesz,
            p_memsz,
            p_align,
        };

        result
    }


    /// write the section to the start of the buffer
    pub fn write(&self, buffer: &mut [u8], endianess: ELFData){
        let buffer = write_field!(buffer, u32, endianess, u32::from(self.p_type));
        let buffer = write_field!(buffer, u32, endianess, u32::from(self.p_flags.clone()));
        let buffer = write_field!(buffer, u64, endianess, self.p_offset);
        let buffer = write_field!(buffer, u64, endianess, self.p_vaddr);
        let buffer = write_field!(buffer, u64, endianess, self.p_addr);
        let buffer = write_field!(buffer, u64, endianess, self.p_filesz);
        let buffer = write_field!(buffer, u64, endianess, self.p_memsz);
        let buffer = write_field!(buffer, u64, endianess, self.p_align);
        let _ = buffer;
    }
}