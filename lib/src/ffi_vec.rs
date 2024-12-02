#[repr(C)]
pub struct FFIVec<T> {
    pub data: *mut T,
    pub len: usize,
    pub capacity: usize,
}

impl<T: Clone> FFIVec<T> {
    /// # Safety
    pub unsafe extern "C" fn free_ffivec(ptr: *mut FFIVec<T>) {
        let _ = Box::from_raw(ptr);
    }
}
