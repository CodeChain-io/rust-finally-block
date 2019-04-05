use std::ops::{Drop, FnOnce};

pub struct Finally<F>
where
    F: FnOnce(),
{
    f: Option<F>,
}

impl<F> Drop for Finally<F>
where
    F: FnOnce(),
{
    fn drop(&mut self) {
        if let Some(f) = self.f.take() {
            f()
        }
    }
}

pub fn finally<F>(f: F) -> Finally<F>
where
    F: FnOnce(),
{
    Finally { f: Some(f) }
}

#[cfg(test)]
mod tests {
    use super::finally;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn executed_on_drop() {
        let a = AtomicUsize::new(0);
        {
            let _f = finally(|| {
                a.fetch_add(1, Ordering::SeqCst);
            });
            assert_eq!(0, a.load(Ordering::SeqCst));
        }
        assert_eq!(1, a.load(Ordering::SeqCst));
    }
}
