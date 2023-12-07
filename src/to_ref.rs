/// Accessing nested pointers is so annoying in Rust. Use for to make process a lot less annoying.
pub trait ToReference<T> {
    unsafe fn to_ref(self) -> T;
}

impl<'a, T> ToReference<&'a T> for *const T {
    #[inline]
    unsafe fn to_ref(self) -> &'a T {
        &*self
    }
}

impl<'a, T> ToReference<&'a mut T> for *mut T {
    #[inline]
    unsafe fn to_ref(self) -> &'a mut T {
        &mut *self
    }
}

impl<'a, T> ToReference<&'a [T]> for *const [T] {
    #[inline]
    unsafe fn to_ref(self) -> &'a [T] {
        &*self
    }
}

impl<'a, T> ToReference<&'a mut [T]> for *mut [T] {
    #[inline]
    unsafe fn to_ref(self) -> &'a mut [T] {
        &mut *self
    }
}
