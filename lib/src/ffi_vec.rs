#[repr(C)]
pub struct FFIVec<T> {
    pub data: *mut T,
    pub len: usize,
    pub capacity: usize,
}

impl<T: Clone> FFIVec<T> {
    /// # Safety
    pub unsafe fn into_vec(ptr: *mut FFIVec<T>) -> Vec<T> {
        let ffi_vec = Box::from_raw(ptr);
        let vec = Vec::from_raw_parts(ffi_vec.data, ffi_vec.len, ffi_vec.capacity);
        std::mem::forget(ffi_vec);
        vec
    }

    /// # Safety
    pub unsafe fn free_ffivec(ptr: *mut FFIVec<T>) {
        let _ = Box::from_raw(ptr);
    }
}
