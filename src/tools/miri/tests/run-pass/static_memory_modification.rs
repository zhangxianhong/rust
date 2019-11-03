use std::sync::atomic::{Ordering, AtomicUsize};

static mut X: usize = 5;
static Y: AtomicUsize = AtomicUsize::new(5);

fn main() {
    unsafe {
        X = 6;
        assert_eq!(X, 6);
    }

    Y.store(6, Ordering::Relaxed);
    assert_eq!(Y.load(Ordering::Relaxed), 6);
}
