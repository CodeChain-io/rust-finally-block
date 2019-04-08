# finally-block [![Build Status](https://travis-ci.com/CodeChain-io/rust-finally-block.svg?branch=master)](https://travis-ci.com/CodeChain-io/rust-finally-block) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Finally block is a block that is executed when it's dropped.
It helps a user write the deferred statements that should be executed, even when some statements return early.

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
