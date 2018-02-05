#![allow(non_camel_case_types)]

use libc::{size_t, c_int, uint32_t, c_void, uint8_t};

pub type rstatus_i = c_int;

#[repr(C)]
pub struct ring_array {
    pub elem_size: size_t,
    pub cap: uint32_t,
    pub rpos: uint32_t,
    pub wpos: uint32_t,
    // TODO: force alignment at native word boundry
    pub data: [uint8_t; 1],

}

// #[link(name = "ccommon-1.2.0", kind = "static")]
#[link(name = "ccommon")]
extern {
    pub fn ring_array_push(elem: *const c_void, ring_array: *mut ring_array) -> rstatus_i;
    pub fn ring_array_pop(elem: *mut c_void, ring_array: *mut ring_array) -> rstatus_i;
    pub fn ring_array_create(elem_size: size_t, cap: uint32_t) -> *mut ring_array;
    pub fn ring_array_destroy(arr: *mut ring_array);
}
