pub mod allocate;
pub mod to_ref;
pub mod defer;

#[macro_export]
macro_rules! cstr {
    ($s:literal) => {
        ::core::concat!($s, '\0').as_ptr() as *const std::ffi::c_char
    };
    ($s:expr) => {
        $s
    };
}
