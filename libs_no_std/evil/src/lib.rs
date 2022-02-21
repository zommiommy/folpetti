//! # Experimental Vectorized Intermediate Language
//! ah ah my shitty code goes brrrrrrr

#![no_std]
extern crate alloc;
use alloc::vec::{self, Vec};

mod word;
pub use word::*;


/// A trait for values that can be used as indices
pub trait IndexTrait: TryFrom<usize> + Into<usize> + Copy + Clone 
    + core::fmt::Display + core::fmt::Debug {}


/// A strongly typed index, this is just an index in the vector of values and
/// instructions.
#[derive(Clone, Copy)]
pub struct Ref<IndexType: IndexTrait>(IndexType);

#[derive(Debug, Clone, Copy)]
pub struct Register<IndexType: IndexTrait>(IndexType);


#[derive(Clone, Copy)]
pub enum Instruction<I: IndexTrait> {
    Nop,
    Add(Ref<I>, Ref<I>, Ref<I>),
    Sub(Ref<I>, Ref<I>, Ref<I>),
}


pub struct Graph {
    //instructions: Vec<usize>,
    //values: Vec<Word>,
}
