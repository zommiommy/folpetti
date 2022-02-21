/// An explicit representaton of an address.
#[derive(Debug, Clone)]
pub enum Address {
    /// Just a raw virtual address in the process
    Absolute(usize),

    /// Address relative to a given section in memory
    Section(String, usize),

    /// Address relative to a function start (requries dwarf info)
    Function(String, usize),
}