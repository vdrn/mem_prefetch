#![allow(internal_features)]
#![allow(unused)]
#![cfg_attr(feature = "nightly", feature(core_intrinsics))]

macro_rules! const_type_assert {
    ($t:ident, $c:expr, $($arg:tt)*) => {{
        struct CompileTimeCheck<$t>($t);
        impl<$t> CompileTimeCheck<$t> {
            const CHECK: () = assert!($c, $($arg)*);
        }
        let _ = CompileTimeCheck::<$t>::CHECK;
    }}
}


#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
unsafe fn prefetch_x86<const LOCALITY: i32>(ptr: *const u8) {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "x86_64")] {
            use std::arch::x86_64::*;
        } else if #[cfg(target_arch = "x86")] {
            use std::arch::x86::*;
       }
    }
    unsafe {
        match LOCALITY {
            0 => {
                // Fetch data using the non-temporal access (NTA) hint.
                // It may be a place closer than main memory but outside of the cache hierarchy.
                // This is used to reduce access latency without polluting the cache.
                _mm_prefetch::<_MM_HINT_NTA>(ptr as *const i8);
            }
            1 => {
                // Fetch into L3 and higher or an implementation-specific choice
                // (e.g., L2 if there is no L3).
                _mm_prefetch::<_MM_HINT_T2>(ptr as *const i8);
            }
            2 => {
                // Fetch into L2 and higher.
                _mm_prefetch::<_MM_HINT_T1>(ptr as *const i8);
            }
            3 => {
                // Fetch into all levels of the cache hierarchy.
                _mm_prefetch::<_MM_HINT_T0>(ptr as *const i8);
            }
            _ => {
                panic!("Invalid locality. Use values 0-3.");
            }
        }
    }
}

/// `LOCALITY` is a value between `0`-`3`, where `0` is the least local,
/// and `3` is the most local.
///
/// # Safety
///
/// It is the caller's responsibility to ensure that the `ptr` is valid.
///
pub unsafe fn prefetch_read_data_raw<T, const LOCALITY: i32>(ptr: *const T) {
    const_type_assert!(
        T,
        size_of::<T>() != 0,
        "It's pointless to prefetch a zero sized type."
    );
    unsafe {
        cfg_if::cfg_if! {
            if #[cfg(feature = "nightly")] {
                std::intrinsics::prefetch_read_data::<T, LOCALITY>(ptr);
            } else if #[cfg(any(target_arch = "x86_64", target_arch = "x86"))] {
                prefetch_x86::<LOCALITY>(ptr as *const u8);
            } else if #[cfg(feature = "fallback")] {
                // No prefetch instruction available, so just read a byte
                std::ptr::read_volatile(ptr as *const std::mem::MaybeUninit<u8>);
            }
          // TODO: aarch64 has prefetch, but it requires nightly, so its pointless to use it atm
        }
    }
}

/// `LOCALITY` is a value between `0`-`3`, where `0` is the least local,
/// and `3` is the most local.
///
/// # Safety
///
/// It is the caller's responsibility to ensure that the `ptr` is valid.
pub unsafe fn prefetch_write_data_raw<T, const LOCALITY: i32>(ptr: *const T) {
    const_type_assert!(
        T,
        size_of::<T>() != 0,
        "It's pointless to prefetch a zero sized type."
    );
    unsafe {
        cfg_if::cfg_if! {
            if #[cfg(feature = "nightly")] {
                std::intrinsics::prefetch_write_data::<T,LOCALITY>(ptr);
            } else if #[cfg(any(target_arch = "x86_64", target_arch = "x86"))] {
                prefetch_x86::<LOCALITY>(ptr as *const u8);
            } else if #[cfg(feature = "fallback")] {
                // No prefetch instruction available, so just read a byte
                std::ptr::read_volatile(ptr as *const std::mem::MaybeUninit<u8>);
            }
          // TODO: aarch64 has prefetch, but it requires nightly, so its pointless to use it atm
        }
    }
}

/// `LOCALITY` is a value between `0`-`3`, where `0` is the least local,
/// and `3` is the most local.
pub fn prefetch_read_data<T, const LOCALITY: i32>(data: &T) {
    unsafe {
        prefetch_read_data_raw::<_, LOCALITY>(data);
    }
}
/// `LOCALITY` is a value between `0`-`3`, where `0` is the least local,
/// and `3` is the most local.
pub fn prefetch_write_data<T, const LOCALITY: i32>(data: &T) {
    unsafe {
        prefetch_write_data_raw::<_, LOCALITY>(data);
    }
}
