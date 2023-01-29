use super::*;
use std::convert::TryInto;

#[derive(Debug)]
pub struct WasmModule {
    pub magic: u32,
    pub version: u32,
    pub types: Vec<FunctionType>,
    pub function_types: Vec<TypeIdx>,
    pub imports: Vec<Import>,
    pub globals: Vec<Global>,
    pub exports: Vec<Export>,
    pub elements: Vec<Element>,
    pub codes: Vec<Function>,
    pub memory: Vec<Limits>,
    pub data: Vec<Data>,
    pub tables: Vec<TableType>,
    pub custom_sections: Vec<CustomSection>,
}

impl WasmModule {
    fn handle_section(&mut self, section: Section) {
        // Decode the current function
        // TODO! here we should check if there is any reminder data and panic in that case.
        match section.section_type {
            // In the Type section are defined the "prototypes" of all the functions.
            // It's basically a vector of types, the types are just a magix byte 0x60
            // and the concatenation of two Vectors of types.
            SectionType::TypeSection => {
                let (_reiminder_data, types) = Vec::parse(section.section_data);
                self.types = types;
                println!("{:?}", self.types);
            }

            // The import section defines mainly the funcions needed from outside the sandbox.
            SectionType::ImportSection => {
                let (_reiminder_data, imports) = Vec::parse(section.section_data);
                self.imports = imports;
                println!("{:?}", self.imports);
            }

            // This is just a section with a vector which associates every
            // function to it's type index.
            SectionType::FunctionSection => {
                let (_reiminder_data, function_types) = Vec::parse(section.section_data);
                self.function_types = function_types;
                println!("{:?}", self.function_types);
            }

            SectionType::GlobalSection => {
                let (_reiminder_data, globals) = Vec::parse(section.section_data);
                self.globals = globals;
                println!("{:?}", self.globals);
            }

            SectionType::ExportSection => {
                let (_reminder_data, exports) = Vec::parse(section.section_data);
                self.exports = exports;
                println!("{:?}", self.exports);
            }

            SectionType::MemorySection => {
                let (_reminder_data, memory) = Vec::parse(section.section_data);
                self.memory = memory;
                println!("{:?}", self.memory);
            }

            SectionType::DataSection => {
                let (_reminder_data, data) = Vec::parse(section.section_data);
                self.data = data;
                println!("{:?}", self.data);
            }

            /*
            // TODO! figure out wtf this does
            // Skipping for now
            SectionType::ElementSection => {
                let (_reminder_data, elements) = Vec::parse(section.section_data);
                self.elements = elements;
                println!("{:?}", self.elements);
            }
            */

            SectionType::TableSection => {
                let (_reminder_data, tables) = Vec::parse(section.section_data);
                self.tables = tables;
                println!("{:?}", self.data);
            }

            SectionType::CustomSection => {
                // let (_reminder_data, custom_section) = CustomSection::parse(section.section_data);
                // println!("{:?}", custom_section);
                // self.custom_sections.push(custom_section);
            }

            SectionType::CodeSection => {
                let (_reminder_data, codes) = Vec::parse(section.section_data);
                self.codes = codes;
                println!("{:#4?}", self.codes);
            }

            _ => panic!("Section {:?} not implemented yet!", section.section_type),
        }
    }
}

impl<'a> Parse<'a> for WasmModule {
    fn parse(data: &[u8]) -> (&[u8], Self) {
        // Parse the metadata
        let (data, magic) = get_field!(data, u32, little);
        let (mut global_data, version) = get_field!(data, u32, little);

        // Initialize an empy wasm module
        let mut wasm_module = WasmModule {
            magic,
            version,
            function_types: Vec::new(),
            imports: Vec::new(),
            types: Vec::new(),
            globals: Vec::new(),
            exports: Vec::new(),
            elements: Vec::new(),
            codes: Vec::new(),
            memory: Vec::new(),
            data: Vec::new(),
            tables: Vec::new(),
            custom_sections: Vec::new(),
        };

        // while there is data, parse the sections.
        // since the parsing of a section is trivial but handling is slower
        // we could easily make this parallel.
        // Moreover, in the future we could jit in parallel all the functions.
        // the only dependancy is the function calls, which if done in an 
        // indirect way it's easy solvable. Basically every function has an unique
        // index and we could have a Jit vector of blocks Vec<Execblock>
        // and instead of just doing call rax we could do 
        // mov rax, [rax + 8*rbx]; call rax. where rax is the address of the vec
        // and rbx is the FunctionIdx, this way the Jitting is parallel
        // and does not requires that the called functions are actually jitted.
        while global_data.len() > 0 {
            let (new_data, section) = Section::parse(global_data);
            global_data = new_data;
            println!("Section: {:?}", section.section_type);
            wasm_module.handle_section(section);
        }

        (global_data, wasm_module)
    }
}
