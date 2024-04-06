use intmap::IntMap;
use std::mem::MaybeUninit;

static BUF: [usize; 256] = [0; 256];

#[repr(C)]
pub(super) struct Data {
    min: usize,
    max: usize,
    sum: usize,
    count: usize,
}

#[inline(always)]
pub(super) unsafe fn get_data(index: u8) -> Data {
    std::ptr::read_volatile(&BUF[index as usize] as *const usize as *const Data)
}

#[inline(always)]
#[allow(invalid_reference_casting)]
pub(super) unsafe fn set_data(index: u8, data: Data) {
    std::ptr::write_volatile(&BUF[index as usize] as *const usize as *mut Data, data)
}

#[inline(always)]
unsafe fn get_index_map() -> &'static mut IntMap<u8> {
    static mut MAP: MaybeUninit<IntMap<u8>> = MaybeUninit::uninit();
    if MAP.as_ptr().is_null() {
        MAP.as_mut_ptr().write(IntMap::with_capacity(256));
    }
    &mut *MAP.as_mut_ptr()
}

#[inline(always)]
pub(super) unsafe fn get_index(hash: u64) -> u8 {
    match get_index_map().entry(hash) {
        intmap::Entry::Occupied(entry) => *entry.get(),
        intmap::Entry::Vacant(entry) => {
            let length = get_index_map().len() as u8;
            entry.insert(length);
            set_data(
                length,
                Data {
                    min: usize::MAX,
                    max: usize::MIN,
                    sum: 0,
                    count: 0,
                },
            );
            length
        }
    }
}
