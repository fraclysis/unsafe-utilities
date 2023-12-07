use std::alloc::*;

pub unsafe fn allocate<T>() -> *mut T {
    alloc(Layout::new::<T>()) as *mut T
}

pub unsafe fn allocate_zeroed<T>() -> *mut T {
    alloc_zeroed(Layout::new::<T>()) as *mut T
}

pub unsafe fn deallocate<T>(ptr: *mut T) {
    dealloc(ptr as _, Layout::new::<T>())
}

pub unsafe fn allocate_array<T>(n: usize) -> *mut [T] {
    let data = alloc(Layout::array::<T>(n).unwrap()) as *mut T;
    std::slice::from_raw_parts_mut(data, n)
}

pub unsafe fn allocate_array_zeroed<T>(n: usize) -> *mut [T] {
    let data = alloc_zeroed(Layout::array::<T>(n).unwrap()) as *mut T;
    std::slice::from_raw_parts_mut(data, n)
}

pub unsafe fn deallocate_array<T>(ptr: *mut [T]) {
    let slice = &mut *ptr;
    dealloc(
        slice.as_mut_ptr() as _,
        Layout::array::<T>(slice.len()).unwrap(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alloc_single() {
        unsafe { deallocate(allocate::<usize>()) }
    }

    #[test]
    fn alloc_array() {
        unsafe { deallocate_array(allocate_array::<usize>(10)) }
    }
}
