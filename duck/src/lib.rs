#![no_std]

mod vec;

pub use vec::Vec;

#[derive(Debug)]
pub enum Error {
    Locked,
}

/// Defines a statically allocated vector-like instance of `duck::vec::Vec`.
#[macro_export]
macro_rules! static_vec {
    ($module_name:ident, $item_type:ty, $size:literal, $default:literal) => {
        mod $module_name {
            static mut BUFFER: [$item_type; $size] = [$default; $size];
            static mut BUFFER_LOCK_FLAG: core::sync::atomic::AtomicBool =
                core::sync::atomic::AtomicBool::new(false);

            pub fn vec() -> Result<duck::Vec<$item_type>, duck::Error> {
                unsafe {
                    let seq_cst = core::sync::atomic::Ordering::SeqCst;
                    match BUFFER_LOCK_FLAG.compare_exchange(false, true, seq_cst, seq_cst) {
                        Err(_) => Err(duck::Error::Locked),
                        _ => Ok(duck::Vec::<$item_type>::new(&mut BUFFER, &BUFFER_LOCK_FLAG)),
                    }
                }
            }
        }
    };
}
