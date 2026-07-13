//! A minimal single-future executor, so Phase 0 runs fully offline with no
//! async-runtime dependency. The port is async because real shells do I/O, but
//! Phase 0's only backend is the fixture-backed fake, whose futures are always
//! immediately ready. M6 (the real network adapter) swaps in a real runtime.

use std::future::Future;
use std::pin::pin;
use std::ptr;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

static VTABLE: RawWakerVTable = RawWakerVTable::new(
    |_| RawWaker::new(ptr::null(), &VTABLE),
    |_| {},
    |_| {},
    |_| {},
);

fn noop_waker() -> Waker {
    // SAFETY: the vtable's methods are all no-ops and ignore the data pointer.
    unsafe { Waker::from_raw(RawWaker::new(ptr::null(), &VTABLE)) }
}

/// Drive a future to completion. Phase 0's futures complete on the first poll,
/// so we poll exactly once and fail loudly if one pends — that can only mean a
/// genuinely async backend (the real adapter) has arrived and a real runtime is
/// now needed. Failing fast beats a silent CPU-spinning loop.
pub fn block_on<F: Future>(fut: F) -> F::Output {
    let waker = noop_waker();
    let mut cx = Context::from_waker(&waker);
    let mut fut = pin!(fut);
    match fut.as_mut().poll(&mut cx) {
        Poll::Ready(value) => value,
        Poll::Pending => panic!(
            "Phase 0 executor only supports immediately-ready futures; a pending \
             future means a real async runtime is needed (the real adapter has arrived)"
        ),
    }
}
