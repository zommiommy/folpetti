use crate::virtaddr::*;
use bitfields::*;
use alloc::alloc::{Allocator, Global, Layout, AllocError};
use core::sync::atomic::{AtomicPtr, Ordering};

pub struct PageTable<T, A = Global> {
    root: AtomicPtr<AtomicPtr<AtomicPtr<AtomicPtr<AtomicPtr<T>>>>>,
    allocator: A,
    _marker: core::marker::PhantomData<T>,
}

impl<T: Sized + Copy> PageTable<T, Global> {
    #[must_use]
    pub fn new() -> Result<Self, AllocError> {
        let root = unsafe{
            Global.allocate_zeroed( 
                Layout::new::<[usize; 1024]>().align_to(1024)?
            )?.as_ptr() as _
        };

        Ok(PageTable{
            root: AtomicPtr::new(root),
            allocator: Global,
            _marker: core::marker::PhantomData::default(),
        })
    }
}

impl<T: Sized + Copy, A: Allocator> PageTable<T, A> {
    #[must_use]
    pub fn new_with_allocator(allocator: A) -> Result<Self, AllocError> {
        let root = unsafe{
            allocator.allocate_zeroed( 
                Layout::new::<[usize; 1024]>().align_to(1024)?
            )?.as_ptr() as _
        };

        Ok(PageTable{
            root: AtomicPtr::new(root),
            _marker: core::marker::PhantomData::default(),
            allocator,
        })
    }
}

impl<T: core::fmt::Debug, A: Allocator> core::fmt::Debug for PageTable<T, A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> { unsafe {
        let l0 = self.root.load(Ordering::SeqCst) ;
        f.write_fmt(format_args!("{:p}\n", l0))?;

        debug_assert_ne!(l0 as usize, 0);
        for idx0 in 0..1024 {
            let l1 = (*l0.add(idx0)).load(Ordering::SeqCst);  
            // skip empty page directories
            if l1 as usize == 0 {
                continue;
            }
            f.write_fmt(format_args!("\t{:p}:{:p}\n", l0.add(idx0), l1))?;
            for idx1 in 0..1024 {
                let l2 = (*l1.add(idx1)).load(Ordering::SeqCst);  
                // skip empty page directories
                if l2 as usize == 0  {
                    continue;
                }
                f.write_fmt(format_args!("\t\t{:p}:{:p}\n", l1.add(idx1), l2))?;
                for idx2 in 0..1024 {
                    let l3 = (*l2.add(idx2)).load(Ordering::SeqCst);  
                    // skip empty page directories
                    if l3 as usize == 0 {
                        continue
                    }
                    f.write_fmt(format_args!("\t\t\t{:p}:{:p}\n", l2.add(idx2), l3))?;
                    for idx3 in 0..1024 {
                        let l4 = (*l3.add(idx3)).load(Ordering::SeqCst);  
                        // skip empty page directories
                        if l4 as usize == 0 {
                            continue
                        }
                        f.write_fmt(format_args!("\t\t\t\t{:p}:{:p}\n", l3.add(idx3), l4))?;
                        for idx4 in 0..1024 {
                            f.write_fmt(format_args!("\t\t\t\t\t{:p}:{:?}\n", l4.add(idx4), *l4.add(idx4)))?;
                        }      
                    }   
                }      
            }            
        }
        Ok(())
    } }
}

#[inline(always)]
fn get_offsets(virt_addr: VirtAddr) -> (usize, usize, usize, usize, usize) {
    // 4 levels paging with 4KByte pages, the higher bits HAVE TO BE all equals
    let higher_bits = virt_addr.0.extract_bitfield::<50, 64>();
    debug_assert!(
        higher_bits == 0 || ((higher_bits + 1) & 0b11111111111111) == 0
    );

    (
        virt_addr.0.extract_bitfield::<40, 50>() as usize,
        virt_addr.0.extract_bitfield::<30, 40>() as usize,
        virt_addr.0.extract_bitfield::<20, 30>() as usize,
        virt_addr.0.extract_bitfield::<10, 20>() as usize,
        virt_addr.0.extract_bitfield::< 0, 10>() as usize,
    )
}

impl<T: Sized + Copy, A: Allocator> PageTable<T, A> {

    const LayoutLevel0: Layout = unsafe{Layout::from_size_align_unchecked(core::mem::size_of::<[AtomicPtr<AtomicPtr<AtomicPtr<AtomicPtr<T>>>>; 1024]>(), 1024)};
    const LayoutLevel1: Layout = unsafe{Layout::from_size_align_unchecked(core::mem::size_of::<[AtomicPtr<AtomicPtr<AtomicPtr<T>>>; 1024]>(), 1024)};
    const LayoutLevel2: Layout = unsafe{Layout::from_size_align_unchecked(core::mem::size_of::<[AtomicPtr<AtomicPtr<T>>; 1024]>(), 1024)};
    const LayoutLevel3: Layout = unsafe{Layout::from_size_align_unchecked(core::mem::size_of::<[AtomicPtr<T>; 1024]>(), 1024)};
    const LayoutLevel4: Layout = unsafe{Layout::from_size_align_unchecked(core::mem::size_of::<[T; 1024]>(), 1024)};

    #[inline]
    fn atomic_alloc<X>(&self, root: *const AtomicPtr<X>) -> Result<*const X, AllocError> { unsafe{
        if *(root as *const usize) == 0 as _ {
            let ptr = self.allocator.allocate_zeroed( 
                Layout::new::<[X; 1024]>().align_to(1024).unwrap()
            )?.as_ptr();
            match (*root).compare_exchange(
                0 as _,
                ptr as _,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(_) => {
                    return ptr as _;
                }
                Err(new_ptr) => {
                    self.allocator.dealloc(ptr, Layout::new::<[X; 1024]>());
                    return new_ptr;
                }
            }
        }
    
        (*root).load(Ordering::SeqCst)
    } }

    pub fn get(&self, virt_addr: VirtAddr) -> Option<&T> {unsafe{
        let l0 = self.root.load(Ordering::SeqCst);

        let (o0, o1, o2, o3, o4) = get_offsets(virt_addr);

        let l1 = (*l0.add(o0)).load(Ordering::SeqCst);
        if l1 as usize == 0 {
            return None;
        }
        let l2 = (*l1.add(o1)).load(Ordering::SeqCst);
        if l2 as usize == 0 {
            return None;
        }
        let l3 = (*l2.add(o2)).load(Ordering::SeqCst);
        if l3 as usize == 0 {
            return None;
        }
        let l4 = (*l3.add(o3)).load(Ordering::SeqCst);
        if l4 as usize == 0 {
            return None;
        }
        let l5 = l4.add(o4);

        Some(&*l5)
    } } 

    pub fn get_allocate(&self, virt_addr: VirtAddr) -> Result<&T, AllocError> { unsafe{
        let l0 = self.root.load(Ordering::SeqCst);
        let (o0, o1, o2, o3, o4) = get_offsets(virt_addr);

        let l1 = self.atomic_alloc(l0.add(o0))?;
        let l2 = self.atomic_alloc(l1.add(o1))?;
        let l3 = self.atomic_alloc(l2.add(o2))?;
        let l4 = self.atomic_alloc(l3.add(o3))?;
        let l5 = l4.add(o4);

        Ok(&*l5)
    } }

    pub fn set_allocate(&mut self, virt_addr: VirtAddr, value: T) -> Result<(), AllocError> { unsafe{
        let l0 = self.root.load(Ordering::SeqCst);
        let (o0, o1, o2, o3, o4) = get_offsets(virt_addr);

        let l1 = self.atomic_alloc(l0.add(o0))?;
        let l2 = self.atomic_alloc(l1.add(o1))?;
        let l3 = self.atomic_alloc(l2.add(o2))?;
        let l4 = self.atomic_alloc(l3.add(o3))?;
        let l5 = l4.add(o4);

        *(l4 as *mut T) = value;

        Ok(())
    } }

    fn free(&mut self) -> Result<(), AllocError> {
        unsafe{
            let l0 = self.root.load(Ordering::SeqCst) ;
            if l0 as usize == 0 {
                return Ok(())
            }
            for idx in 0..1024 {
                let l1 = (*l0.add(idx)).load(Ordering::SeqCst);  
                // skip empty page directories
                if l1 as usize == 0 {
                    continue;
                }
                for idx in 0..1024 {
                    let l2 = (*l1.add(idx)).load(Ordering::SeqCst);  
                    // skip empty page directories
                    if l2 as usize == 0  {
                        continue;
                    }
                    for idx in 0..1024 {
                        let l3 = (*l2.add(idx)).load(Ordering::SeqCst);  
                        // skip empty page directories
                        if l3 as usize == 0  {
                            continue;
                        }
                        for idx in 0..1024 {
                            let l4 = (*l3.add(idx)).load(Ordering::SeqCst);  
                            // skip empty page directories
                            if l4 as usize != 0 {
                                self.allocator.deallocate(
                                    l4 as _, 
                                    Self::LayoutLevel4,
                                );
                            }
                        }      
                        self.allocator.deallocate(
                            l3 as _, 
                            Self::LayoutLevel3,
                        );
                    }            
                    self.allocator.deallocate(
                        l2 as _, 
                        Self::LayoutLevel2,
                    );
                }            
                self.allocator.deallocate(
                    l1 as _, 
                    Self::LayoutLevel1,
                );
            }
            self.allocator.deallocate(
                l0 as _, 
                Self::LayoutLevel0,
            );
        } 
        // reset the root pointer to avoid double frees
        self.root = AtomicPtr::new(core::ptr::null() as _);
        Ok(())
    }
}

impl<T, A> core::ops::Drop for PageTable<T, A> {
    fn drop(&mut self) {
        self.free().unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate std;
    use std::println;

    #[test]
    fn test() {
        let mut pt = PageTable::<u8>::new().unwrap();
        //println!("{:?}", pt);
        
        pt.set_allocate(VirtAddr(1024), 0xff);
        pt.set_allocate(VirtAddr(10), 0x69);
        pt.set_allocate(VirtAddr(u64::MAX), 0x69);
    
        assert_eq!(Some(&0xff), pt.get(VirtAddr(1024)));
        assert_eq!(Some(&0x69), pt.get(VirtAddr(10)));
        assert_eq!(Some(&0x69), pt.get(VirtAddr(u64::MAX)));
    
        println!("{:?}", pt);
    }
}
