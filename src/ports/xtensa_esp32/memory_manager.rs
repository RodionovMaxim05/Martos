use core::mem::MaybeUninit;
use esp_alloc as _;

/// Heap initialization.
/// For more information see https://github.com/esp-rs/esp-alloc/.
pub fn init_heap() {
    const HEAP_SIZE: usize = 64 * 1024;
    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

    unsafe {
        esp_alloc::HEAP.add_region(esp_alloc::HeapRegion::new(
            HEAP.as_mut_ptr() as *mut u8,
            HEAP_SIZE,
            esp_alloc::MemoryCapability::Internal.into(),
        ));
    }
}
