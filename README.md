# CCOMMON-RUST: Rust bindings for Twitter's ccommon library

## Getting Started

* `cargo test` 

This does the work of pulling `ccommon` from GitHub, building it, and running `bindgen` to generate the Rust bindings.

## Usage

This package uses `rust-bindgen` to generate all the bindings for `ccommon`. Note that these bindings are the core low level FFI bindings needed to call into the C layer. They are not ergonomic Rust, i.e. you have to structure your calls just like you would in C. For example:

```rust
// Create a ccommon ring array with an item size of 8 bytes, holding 64 elements.
let ring_array = unsafe { ring_array_create(8, 64) };
// Push a value onto it.
let blort = 31337u64;
let blort_ptr: *const c_void = &blort as *const _ as *const c_void;
let result = unsafe { ring_array_push(blort_ptr, ring_array) };
// Pop a value into a variable.
let mut popped: u64 = 0;
let popped_ptr: *mut c_void = &mut popped as *mut _ as *mut c_void;
let result = unsafe { ring_array_pop(popped_ptr, ring_array) };
// Make sure to free it when you're done with it to avoid memory leaks!
unsafe { ring_array_destroy(ring_array) };
```

An ergonomic wrapper on top of the generated bindings might look a little more like this:

```rust
struct ring_array {
    pub ra: *mut ring_array,
}

impl ring_array {
    pub fn new<T>(cap: u32) -> ring_array {
        let ra = unsafe { ring_array_create(std::mem::size_of::<T>(), cap) };
        ring_array {ra}
    }

    // This would probably return something higher level.
    pub fn push<T>(&mut self, item: &T) -> rstatus_i {
        let t_ptr: *const c_void = item as *const _ as *const c_void;
        unsafe { ring_array_push(t_ptr, self.ra) }
    }

    // And here you would simply return the item directly rather than passing a parameter,
    // but you get my drift. 
    pub fn pop<T>(&mut self, item: &mut T) -> rstatus_i {
        let t_ptr: *mut c_void = item as *mut _ as *mut c_void;
        unsafe { ring_array_pop(t_ptr, self.ra) }
    }
}

impl Drop for ring_array {
    fn drop(&mut self) {
        unsafe { ring_array_destroy(self.ra) }
    }
}

```

As of this writing, bindings are generated for all of `ccommon`.

## Implementation

The `rust-bindgen` project's documentation goes into detail on how to wrap a project, but here is a quick overview of key files in this project.

### `build.rs`

This is a hook into `cargo` that does the work of grabbing `ccommon` from GitHub, configuring and building it with `cmake`, and generating the bindings.

### `wrapper.h`

This C header file gets passed to `rust-bindgen`. It serves as the top level include that pulls in all of the other include files from `ccommon`.

### `src/lib.rs`

This is the top level file that contains all of the generated code, as well as a couple of rudimentary tests.

### `target/[debug|release]/build/ccommon-${BUILD_ID}/out/bindings.rs`

The bindings themselves are in this file. Look here to see the actual method signatures. Pro tip: to see the generated code in your editor, create a symlink since the `$BUILD_ID` will change: `find target -name bindings.rs -exec ln -sf '{}' ';'` 

## TODO

* Write ergonomic wrappers!
