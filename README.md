# finally-block [![Build Status](https://travis-ci.com/CodeChain-io/rust-finally-block.svg?branch=master)](https://travis-ci.com/CodeChain-io/rust-finally-block) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Final block is a block that is executed when it dropped.
It helps a user to write the deferred statements that should be executed even some statements return early.

```rust
function f(flag: &AtomicBool) -> Option<()> {
    if some_condition {
        let _f = finally(|| {
            flag.store(true, Ordering::SeqCst);
        });
        some_function(flag)?;
    } else {
        another_function()?;
    }
}
```
