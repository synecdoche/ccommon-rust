#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    use std::ptr;
    use std::os::raw::c_void;

    #[test]
    fn ring_array_push_pop() {
        let ring_array = unsafe { ring_array_create(8, 64) };
        assert_ne!(ring_array.is_null(), true);
        let blort = 31337u64;
        let blort_ptr: *const c_void = &blort as *const _ as *const c_void;
        let result = unsafe { ring_array_push(blort_ptr, ring_array) };
        assert_eq!(result, 0);
        let mut popped: u64 = 0;
        let popped_ptr: *mut c_void = &mut popped as *mut _ as *mut c_void;
        let result = unsafe { ring_array_pop(popped_ptr, ring_array) };
        assert_eq!(result, 0);
        assert_eq!(blort, popped);
        unsafe { ring_array_destroy(ring_array) };
    }

    #[test]
    fn array_push_pop_one() {
        unsafe {
            let mut options = option {
                name: ptr::null_mut(),
                set: true,
                type_: option_type_OPTION_TYPE_UINT,
                default_val: option_val {vuint: 0},
                val: option_val {vuint: 8u64},
                description: ptr::null_mut(),
            };

            array_setup(mem::transmute(&mut options));

            let initial_nalloc = 4u32;
            let times = 3u32;
            let expected_nalloc = 4u32;
            let size = 8;
            let mut arr = 0 as *mut array;
            let arr_ptr = &mut arr as *mut _ as *mut *mut array;
            array_create(arr_ptr, initial_nalloc, size);
            // array_nalloc() is inline
            (*arr).nalloc = initial_nalloc;
            let elem = array_push(arr) as *mut u32;
            (*elem) = 31337u32;

            assert_ne!(arr, 0 as *mut array);
        }
    }
}

