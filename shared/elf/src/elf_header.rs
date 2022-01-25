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

impl<'a> Parse<ELFHeader> for Data<'a> {
    fn inner_parse(&mut self) -> ELFHeader {
        let magic: [u8; 4] = self.parse();
        assert_eq!(magic, [0x7F, 0x45, 0x4c, 0x46]);

        let ei_class = self.parse();

        // read ei_data and handle the endianess
        let ei_data = self.parse();
        match ei_data {
            ELFData::ELFDATANONE => {},
            ELFData::ELFDATA2LSB => self.set_little_endian(),
            ELFData::ELFDATA2MSB => self.set_big_endian(),
            ELFData::Unknown(x) => panic!("unknown endianess 0x{:02x}", x),
        };

        match ei_class {
            ELFClass::ELFCLASS64 =>
                ELFHeader{
                    magic,
                    ei_class,
                    ei_data,
                    ei_version:    self.parse(),
                    ei_osabi:      self.parse(),        
                    ei_abiversion: self.parse(),      
                    pad:           self.parse(),
                    e_type:        self.parse(),            
                    e_machine:     self.parse(),      
                    e_version:     self.parse(),     
                    e_entry:       self.parse(),               
                    e_phoff:       self.parse(),             
                    e_shoff:       self.parse(),               
                    e_flags:       self.parse(),              
                    e_ehsize:      self.parse(),              
                    e_phentsize:   self.parse(),           
                    e_phnum:       self.parse(),               
                    e_shentsize:   self.parse(),           
                    e_shnum:       self.parse(),               
                    e_shstrndx:    self.parse(),            
                },
            x @ _ => panic!("Elf class '{:?}' is not supported", x)
        }
    }
}
