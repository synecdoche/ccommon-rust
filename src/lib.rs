extern crate libc;

mod ffi;

#[cfg(test)]
mod tests {
    #[test]
    fn ring_array_push_pop() {
        use ffi;
        use std::mem;
        use libc::c_void;
        use std::ptr;

        let ring_array = unsafe { ffi::ring_array_create(8, 64) };
        assert_ne!(ring_array.is_null(), true);
        let mut blort = 31337u64;
        let blort_ptr: *mut c_void = &mut blort as *mut _ as *mut c_void;
        let result  = unsafe { ffi::ring_array_push(blort_ptr, ring_array) };
        assert_eq!(result, 0);
        let mut popped: u64 = 0;
        let popped_ptr: *mut c_void = &mut popped as *mut _ as *mut c_void;
        let result = unsafe { ffi::ring_array_pop(popped_ptr, ring_array) };
        assert_eq!(result, 0);
        assert_eq!(blort, popped);
        unsafe { ffi::ring_array_destroy(ring_array) };
    }
}

