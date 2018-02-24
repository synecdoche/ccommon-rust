#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::mem;
use std::ffi::CString;
use std::ptr;
use std::io;

/// Helper to convert borrows to pointers
fn to_ptr_mut<T>(val: &mut T) -> *mut T {
    val as *mut _ as *mut T
}

/// Helper to convert borrows to pointers
fn to_ptr<T>(val: &T) -> *const T {
    val as *const _ as *const T
}

impl metric {
    fn new(name: &str, type_: metric_type_e, description: &str) -> metric {
        metric {
            name: unsafe { mem::transmute(name.as_ptr()) },
            type_: type_,
            desc: unsafe { mem::transmute(description.as_ptr()) },
            __bindgen_anon_1: metric__bindgen_ty_1::default(),
        }
    }
}


impl log_metrics_st {
    pub fn new() -> log_metrics_st {
        log_metrics_st {
            log_create: metric::new("log_create", metric_type_METRIC_COUNTER, "# loggers created"),
            log_create_ex: metric::new("log_create_ex", metric_type_METRIC_COUNTER, "# log create errors"),
            log_destroy: metric::new("log_destroy", metric_type_METRIC_COUNTER, "# loggers destroyed"),
            log_curr: metric::new("log_curr", metric_type_METRIC_GAUGE, "current # loggers"),
            log_open: metric::new("log_open", metric_type_METRIC_COUNTER, "# files opened by loggers"),
            log_open_ex: metric::new("log_open_ex", metric_type_METRIC_COUNTER, "# logger open file errors"),
            log_write: metric::new("log_write", metric_type_METRIC_COUNTER, "# log messages written"),
            log_write_byte: metric::new("log_write_byte", metric_type_METRIC_COUNTER, "# bytes written by log"),
            log_write_ex: metric::new("log_write_ex", metric_type_METRIC_COUNTER, "# log write errors"),
            log_skip: metric::new("log_skip", metric_type_METRIC_COUNTER, "# messages not completely logged"),
            log_skip_byte: metric::new("log_skip_byte", metric_type_METRIC_COUNTER, "# bytes unable to be logged"),
            log_flush: metric::new("log_flush", metric_type_METRIC_COUNTER, "# log flushes to disk"),
            log_flush_ex: metric::new("log_flush_ex", metric_type_METRIC_COUNTER, "# errors flushing to disk"),
        }
    }
}

pub struct Logger {
    log_metrics: log_metrics_st,
    filename: String,
    logger: *mut logger,
}

impl Logger {
    pub fn new(filename: &str) -> Result<Logger, io::Error> {
        let mut log_metrics = log_metrics_st::new();
        unsafe { log_setup(to_ptr_mut(&mut log_metrics)) };
        let tmpfile_str = CString::new(filename).unwrap();
        let tmpfile: *mut i8 = unsafe { mem::transmute(tmpfile_str.as_ptr()) };
        let mut logger = unsafe { log_create(tmpfile, 0) };
        if logger.is_null() {
            return Err(io::Error::new(io::ErrorKind::Other,
                                      format!("failed to open file: {}", filename)))
        }
        Ok(Logger {
            log_metrics,
            filename: filename.to_string(),
            logger,
        })
    }

    pub fn write(&mut self, msg: &str) {
        let msg_len = msg.len() as u32;
        let msg_c: *mut i8 = unsafe { mem::transmute(msg.as_ptr()) };
        assert!( unsafe { log_write(self.logger, msg_c, msg_len) });
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        unsafe { log_destroy(to_ptr_mut(&mut self.logger)) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    use std::ptr;
    use std::os::raw::c_void;
    use std::ffi::CStr;
    use std::ffi::CString;

    #[test]
    fn ring_array_push_pop() {
        let ring_array = unsafe { ring_array_create(8, 64) };
        assert_ne!(ring_array.is_null(), true);
        let mut blort = 31337u64;
        let blort_ptr: *mut c_void = &mut blort as *mut _ as *mut c_void;
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


    #[test]
    #[should_panic]
    fn logging_unwritable_file() {
        let mut faillogger = Logger::new("/BLORT").unwrap();
    }

    #[test]
    fn logging() {
        let mut l = Logger::new("i-like-cats.log").unwrap();
        l.write("i like cats!!!");
    }
}

