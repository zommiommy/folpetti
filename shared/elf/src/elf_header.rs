use super::*;

/// ELFHeader class, this is exactly as the one defined in
/// elf.h so it could be parsed doing pointer tricks BUT
/// we choose the safe and correct way. (We also support
/// different endianess).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ELFHeader {
    /// magic bytes, some parser accept it even if they
    /// are not the first bytes of the file. 
    pub magic: [u8; 4],             
    
    /// 32 or 64 bit?
    pub ei_class: ELFClass,         
    
    ///
    pub ei_data: ELFData,
    
    ///     
    pub ei_version: ELFIntVersion,
    
    /// Abi type 
    pub ei_osabi: ELFOsAbi,        
    
    /// Abi version 
    pub ei_abiversion: u8,      
    
    /// Padding    
    pub ei_pad: [u8; 7],
    
    /// Object file type
    pub e_type: ELFType,            
    
    /// Architecture
    pub e_machine: ELFMachine,      
    
    /// Object file version
    pub e_version: ELFVersion,     
    
    /// Entry point virtual address 
    pub e_entry: u64,              
    
    /// Program header table file offset
    pub e_phoff: u64,              
    
    /// Section header table file offset
    pub e_shoff: u64,               
    
    /// Processors-specific flags
    pub e_flags: u32,               
    
    /// ELF header size in bytes
    pub e_ehsize: u16,              
    
    /// Program header table entry size
    pub e_phentsize: u16,           
    
    /// Program header table entry count
    pub e_phnum: u16,               
    
    /// Section header table entry size
    pub e_shentsize: u16,           
    
    /// Section header table entry count
    pub e_shnum: u16,               

    /// Section header string table index
    pub e_shstrndx: u16,            
}

impl ELFHeader {
    pub fn parse(data: &[u8]) -> ELFHeader {
        let (magic, data) = data.split_at(4);
        let magic: [u8; 4] = magic.try_into().unwrap();

        let (data, ei_class)      = get_field!(data);
        let (data, ei_data)       = get_field!(data);
        let (data, ei_version)    = get_field!(data);
        let (data, ei_osabi)      = get_field!(data);
        let (data, ei_abiversion) = get_field!(data);
        let (data, ei_pad0)       = get_field!(data);
        let (data, ei_pad1)       = get_field!(data);
        let (data, ei_pad2)       = get_field!(data);
        let (data, ei_pad3)       = get_field!(data);
        let (data, ei_pad4)       = get_field!(data);
        let (data, ei_pad5)       = get_field!(data);
        let (data, ei_pad6)       = get_field!(data);
        
        let ei_class   = ELFClass::from(ei_class);
        let ei_data    = ELFData::from(ei_data);
        let ei_version = ELFIntVersion::from(ei_version);
        let ei_osabi   = ELFOsAbi::from(ei_osabi);

        let (data, e_type)      = get_field!(data, u16, ei_data);
        let (data, e_machine)   = get_field!(data, u16, ei_data);
        let (data, e_version)   = get_field!(data, u32, ei_data);
        let (data, e_entry)     = get_field!(data, u64, ei_data);
        let (data, e_phoff)     = get_field!(data, u64, ei_data);
        let (data, e_shoff)     = get_field!(data, u64, ei_data);
        let (data, e_flags)     = get_field!(data, u32, ei_data);
        let (data, e_ehsize)    = get_field!(data, u16, ei_data);
        let (data, e_phentsize) = get_field!(data, u16, ei_data);
        let (data, e_phnum)     = get_field!(data, u16, ei_data);
        let (data, e_shentsize) = get_field!(data, u16, ei_data);
        let (data, e_shnum)     = get_field!(data, u16, ei_data);
        let (data, e_shstrndx)  = get_field!(data, u16, ei_data);
        let _ = data; // remove the warning about unused data because following the pattern is better 
        
        // Parse the enums
        let e_type    = ELFType::from(e_type);
        let e_machine = ELFMachine::from(e_machine);
        let e_version = ELFVersion::from(e_version);
        
        // Magic bytes should match
        assert!(magic == [0x7f, 0x45, 0x4c, 0x46]);
        // only support 64 bits
        assert_eq!(ei_class, ELFClass::ELFCLASS64);
        // little edian
        assert_eq!(ei_data,  ELFData::ELFDATA2LSB);
        
        ELFHeader{
            // e_init
            magic,
            ei_class,
            ei_data,
            ei_version,
            ei_osabi,
            ei_abiversion,
            ei_pad: [
                ei_pad0,
                ei_pad1,
                ei_pad2,
                ei_pad3,
                ei_pad4,
                ei_pad5,
                ei_pad6,
            ],
            // fields
            e_type,
            e_machine,
            e_version,
            e_entry,
            e_phoff,
            e_shoff,
            e_flags,
            e_ehsize,
            e_phentsize,
            e_phnum,
            e_shentsize,
            e_shnum,
            e_shstrndx,
        }
    }

    /// write the section to the start of the buffer.
    /// Therefore we expect to be already at the right position
    pub fn write(&self, buffer: &mut [u8]){
        // single bytes fields
        let buffer = write_field!(buffer, self.magic[0]);
        let buffer = write_field!(buffer, self.magic[1]);
        let buffer = write_field!(buffer, self.magic[2]);
        let buffer = write_field!(buffer, self.magic[3]);
        let buffer = write_field!(buffer, u8::from(self.ei_class));
        let buffer = write_field!(buffer, u8::from(self.ei_data));
        let buffer = write_field!(buffer, u8::from(self.ei_version));
        let buffer = write_field!(buffer, u8::from(self.ei_osabi));
        let buffer = write_field!(buffer, u8::from(self.ei_abiversion));
        let buffer = write_field!(buffer, u8::from(self.ei_pad[0]));
        let buffer = write_field!(buffer, u8::from(self.ei_pad[1]));
        let buffer = write_field!(buffer, u8::from(self.ei_pad[2]));
        let buffer = write_field!(buffer, u8::from(self.ei_pad[3]));
        let buffer = write_field!(buffer, u8::from(self.ei_pad[4]));
        let buffer = write_field!(buffer, u8::from(self.ei_pad[5]));
        let buffer = write_field!(buffer, u8::from(self.ei_pad[6]));
        
        // multibytes 
        let buffer = write_field!(buffer, u16, self.ei_data, u16::from(self.e_type));
        let buffer = write_field!(buffer, u16, self.ei_data, u16::from(self.e_machine));
        let buffer = write_field!(buffer, u32, self.ei_data, u32::from(self.e_version));
        let buffer = write_field!(buffer, u64, self.ei_data, self.e_entry);
        let buffer = write_field!(buffer, u64, self.ei_data, self.e_phoff);
        let buffer = write_field!(buffer, u64, self.ei_data, self.e_shoff);
        let buffer = write_field!(buffer, u32, self.ei_data, self.e_flags);
        let buffer = write_field!(buffer, u16, self.ei_data, self.e_ehsize);
        let buffer = write_field!(buffer, u16, self.ei_data, self.e_phentsize);
        let buffer = write_field!(buffer, u16, self.ei_data, self.e_phnum);
        let buffer = write_field!(buffer, u16, self.ei_data, self.e_shentsize);
        let buffer = write_field!(buffer, u16, self.ei_data, self.e_shnum);
        let buffer = write_field!(buffer, u16, self.ei_data, self.e_shstrndx);
        let _ = buffer;
    }
}
