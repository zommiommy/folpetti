//use mmu::*;

fn main() {
    /*
    let mut mmu = Mmu::<
        256,    // dirty block size
        true,   // RAW
        true,   // TAINT
    >::new(0x10_000).unwrap();

    // We did not write anything so we should not be able to read it
    let null = VirtAddr(0);
    assert!(mmu.read::<u32>(null).is_err());
    assert!(mmu.write::<u64>(null, 0x1337).is_err());

    // allocate memory the allocations start from address 0x1000
    let addr = mmu.allocate(0x100).unwrap();

    // error on uninitalized memory
    assert!(mmu.read::<u16>(addr).is_err());

    // write to the memory and read it back
    mmu.write::<u64>(addr, 1337).unwrap();
    assert_eq!(1337, mmu.read::<u64>(addr).unwrap());
    // even one byte out of bound will cause to err
    assert!(mmu.read::<u64>(addr + 1).is_err());
    assert!(mmu.read::<u64>(addr - 1).is_err());

    // create a new mmu forking the current state
    let mut mmu2 = mmu.fork();
    assert_eq!(1337, mmu2.read::<u64>(addr).unwrap());

    // modify the state
    mmu2.write::<u16>(addr + 8, 420).unwrap();
    assert_eq!(420, mmu2.read::<u16>(addr + 8).unwrap());

    // reset differentially the mmu to the fork state
    mmu2.reset(&mmu);

    // check that the state was **actually** resetted
    assert!(mmu2.read::<u16>(addr + 8).is_err());
    assert_eq!(1337, mmu.read::<u64>(addr).unwrap());
     */
}


