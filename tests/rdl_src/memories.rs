use memories::{
    access::{R, RW, W},
    components::memories::Memories,
    mem::{MemEntry, Memory},
};

/// A block of memory used for simulating hardware registers.
///
/// A forced alignment of 16 bytes allows access to internal registers as
/// primitive types.
#[repr(align(16))]
pub(crate) struct MockMemory<const N: usize>([u8; N]);

impl<const N: usize> MockMemory<N> {
    pub const fn new_zeroed() -> Self {
        MockMemory([0; N])
    }

    pub const fn as_mut_ptr(&mut self) -> *mut u8 {
        self.0.as_mut_ptr()
    }
}

const SIZE: usize = Memories::SIZE;
static mut MOCK_MEM: MockMemory<SIZE> = MockMemory::new_zeroed();
#[allow(static_mut_refs)]
const TOP: Memories = unsafe { Memories::from_ptr(MOCK_MEM.as_mut_ptr() as _) };

#[test]
fn test_memory() {
    let mem = TOP.mem_5_64_rw();
    assert_eq!(mem.num_entries(), 5);
    assert_eq!(mem.width(), 64);
    let mut first_entry = mem.index(0);
    first_entry.write(0x0123_4567_89ab_cdef);
    assert_eq!(first_entry.read(), 0x0123_4567_89ab_cdef);
}

#[test]
fn test_memory_iter() {
    let mem = TOP.mem_5_64_rw();
    for (i, mut entry) in mem.iter().enumerate() {
        entry.write(i as u64);
    }
    // forward iteration
    for (i, entry) in mem.iter().enumerate() {
        assert_eq!(entry.read(), i as u64);
    }
    // backward iteration
    for (i, entry) in mem.iter().enumerate().rev() {
        assert_eq!(entry.read(), i as u64);
    }
    // forward slice
    for (i, entry) in mem.slice(2..).enumerate() {
        assert_eq!(entry.read(), (i as u64) + 2);
    }
    // backward slice
    for (i, entry) in mem.slice(..4).enumerate().rev() {
        assert_eq!(entry.read(), i as u64);
    }
}

#[test]
fn test_memory_access() {
    let _: MemEntry<u32, R> = TOP.mem_2_32_r().index(0);
    let _: MemEntry<u32, W> = TOP.mem_2_32_w().index(0);
    let _: MemEntry<u32, RW> = TOP.mem_2_32_rw().index(0);
}
