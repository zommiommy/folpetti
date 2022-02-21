pub trait BitExtract {
    fn extract_bits<const START: usize, const END: usize>(&self) -> Self;
    fn extract_bit<const IDX: usize>(&self) -> Self;
    fn set_bits<const START: usize, const END: usize>(&mut self, value: Self);
}

impl BitExtract for u32 {
    #[inline(always)]
    /// Extract a bit range form a word, START and END are **inclusive**
    fn extract_bits<const START: usize, const END: usize>(&self) -> u32 {
        // check that the range is reasonable
        debug_assert!(START <  8 * core::mem::size_of::<u32>());
        debug_assert!(END   <= 8 * core::mem::size_of::<u32>());   
        debug_assert!(START <= END); 
        
        (self >> START) & (1_u32 << (END - START)).wrapping_sub(1)
    }

    #[inline(always)]
    fn extract_bit<const IDX: usize>(&self) -> u32 {
        // check that the idx is reasonable
        debug_assert!(IDX <  8 * core::mem::size_of::<u32>());
        
        (self >> IDX) & 1
    }

    #[inline(always)]
    fn set_bits<const START: usize, const END: usize>(&mut self, value: u32) {
        unimplemented!() // TODO!
    }
}