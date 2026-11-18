#![no_std]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! { core::arch::wasm32::unreachable() }

const CAP: usize = 65536; // 64KB scratchpad (adjust to max payload)
#[repr(align(8))]
struct Scratch([u8; CAP]);
static mut ARENA: Scratch = Scratch([0; CAP]);
static mut HEAD: usize = 0;

#[no_mangle]
pub extern "C" fn reset() { unsafe { HEAD = 0; } }

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut u8 {
    unsafe {
        let aligned = (HEAD + 7) & !7; // 8-byte alignment for structs/f64
        if aligned + size > CAP { core::arch::wasm32::unreachable(); }
        let ptr = ARENA.0.as_mut_ptr().add(aligned);
        HEAD = aligned + size;
        ptr
    }
}

/// Packs (32-bit ptr, 32-bit len) into one u64 so any Wasm host reads structs/strings cleanly
#[inline]
pub fn pack_slice(ptr: *const u8, len: usize) -> u64 {
    ((ptr as usize as u64) << 32) | (len as u64)
}
