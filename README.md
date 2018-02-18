# CCOMMON-RUST: Rust bindings for Twitter's ccommon library

## Setup

* Build and install `twitter/ccommon`
* Set the following environment variables, replacing the parts in curly braces:

```
RUSTFLAGS="-L {CCOMMON_INSTALL_DIR}/lib"
LD_LIBRARY_PATH={CCOMMON_INSTALL_DIR}/lib
C_INCLUDE_PATH={CCOMMON_SOURCE_DIR}/include:{CCOMMON_SOURCE_DIR}/{CCOMMON_BUILD_SUBDIR}
```

CCOMMON_BUILD_SUBDIR is the directory you created and typed `cmake` in.

* `cargo test` should succeed.
* To see the generated code: `find target -name bindings.rs -exec ln -sf '{}' ';'`

## TODO

* including `channel/cc_tcp.h` breaks the build.
* one of the generated layout tests in `bindings.rs` fails. 