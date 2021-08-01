//! Sometimes it just takes a small nudge for the compiler to generate the code you want.

#![cfg_attr(feature = "nightly", feature(core_intrinsics))]
#![cfg_attr(not(feature = "std"), no_std)]

mod internal {
    cfg_if::cfg_if! {
        if #[cfg(feature = "nightly")] {
            #[inline(always)]
            pub unsafe fn assume(b: bool) {
                core::intrinsics::assume(b)
            }

            #[inline(always)]
            pub fn unlikely(b: bool) -> bool {
                core::intrinsics::unlikely(b)
            }

            #[inline(always)]
            pub fn likely(b: bool) -> bool {
                core::intrinsics::likely(b)
            }
        } else {
            #[inline(always)]
            #[cold]
            fn cold() {}

            #[inline(always)]
            pub unsafe fn assume(b: bool) {
                if !b { crate::unreach() }
            }

            #[inline(always)]
            pub fn unlikely(b: bool) -> bool {
                if b {
                    cold()
                }
                b
            }

            #[inline(always)]
            pub fn likely(b: bool) -> bool {
                if !b {
                    cold()
                }
                b
            }
        }
    }

    // extern "C" gives us nounwind without having to use lto=fat
    #[cold]
    pub extern "C" fn nounwind_abort() -> ! {
        cfg_if::cfg_if! {
            if #[cfg(feature = "std")] {
                #[inline(always)]
                fn abort_impl() -> ! {
                    std::process::abort()
                }
            } else {
                #[inline(always)]
                fn abort_impl() -> ! {
                    // extern "C" prevents panic from escaping
                    panic!()
                }
            }
        }
        abort_impl()
    }

    #[inline(always)]
    pub unsafe extern "C" fn assume_nopanic<F: FnOnce() -> T, T>(f: F) -> T {
        struct NoPanic;
        impl Drop for NoPanic {
            #[inline(always)]
            fn drop(&mut self) {
                unsafe { crate::unreach() };
            }
        }

        let no_panic = NoPanic;
        let r = f();
        core::mem::forget(no_panic);
        r
    }
}

/// Unsafely assumes the value of an expression to be `true`.
///
/// The compiler is sometimes able to use this to optimize better, but it often backfires.
///
/// This generally requires the `nightly` feature to work.
#[inline(always)]
pub unsafe fn assume(b: bool) {
    crate::internal::assume(b)
}

/// Tells the compiler this `bool` is probably `false`.
///
/// This generally requires the `nightly` feature to work.
#[inline(always)]
pub fn unlikely(b: bool) -> bool {
    crate::internal::unlikely(b)
}

/// Tells the compiler this `bool` is probably `true`.
///
/// This generally requires the `nightly` feature to work.
#[inline(always)]
pub fn likely(b: bool) -> bool {
    crate::internal::likely(b)
}

/// Tells the compiler this code is unreachable.
///
/// This is identical to `core::hint::unreachable_unchecked` and is provided only for completeness.
#[inline(always)]
pub unsafe fn unreach() -> ! {
    core::hint::unreachable_unchecked()
}

/// The same as `std::process::abort`, but annotated as `#[cold]`, `nounwind`, and usable in `no_std` environments.
///
/// In a `no_std` environment this generates a trap instruction.
#[cold]
#[inline(always)]
pub fn abort() -> ! {
    crate::internal::nounwind_abort()
}

/// Assumes a closure will not panic.
///
/// Calls to `core::panicking` functions will still be generated; however, this function is nounwind and panics will cause UB.
#[inline]
pub unsafe fn assume_nopanic<F: FnOnce() -> T, T>(f: F) -> T {
    crate::internal::assume_nopanic(f)
}
