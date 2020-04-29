#![no_std]

mod vec;

pub use vec::Vec;

#[derive(Debug)]
pub enum Error {
    Locked,
}

/// Statically allocate a fixed size chunk of memory to be handled by a `duck::vec::Vec`.
///
/// This macro will create a static chunk of `$count` instances of `$type`.
///
/// Access to the data is made possible by exposing a private module
/// called `$module_name` that contains a public function `vec()` that
/// can be used to get a `duck::vec::Vec` to work with the memory chunk.
///
/// - At any given time at most one `duck::vec::Vec` referencing the memory can exist
/// - Any calls to `vec()` when a reference already exists will cause a panic
/// - You may call `vec()` again when the referencing `Vec` has been dropped
///
/// # Examples
///
/// ```no_run
/// // statically allocate 512 bytes and initialize them to 0xFF
/// static_vec!(some_bytes, u8, 512, 0xFF);
///
/// fn do_something() {
///     let mut bytes = some_bytes::vec().expect("failed to lock some_bytes");
///     bytes.push(0);
///     assert_eq!(bytes.len(), 1);
///
///     // this would cause a panic
///     // let mut bytes_again = some_bytes::vec().expect("failed to lock some_bytes");
/// }
/// ```
#[macro_export]
macro_rules! static_vec {
    ($module_name:ident, $type:ty, $count:literal, $default:literal) => {
        mod $module_name {
            static mut BUFFER: [$type; $count] = [$default; $count];
            static mut BUFFER_LOCK_FLAG: core::sync::atomic::AtomicBool =
                core::sync::atomic::AtomicBool::new(false);

            pub fn vec() -> Result<duck::Vec<$type>, duck::Error> {
                unsafe {
                    let seq_cst = core::sync::atomic::Ordering::SeqCst;
                    match BUFFER_LOCK_FLAG.compare_exchange(false, true, seq_cst, seq_cst) {
                        Err(_) => Err(duck::Error::Locked),
                        _ => Ok(duck::Vec::<$type>::new(&mut BUFFER, &BUFFER_LOCK_FLAG)),
                    }
                }
            }
        }
    };
}
