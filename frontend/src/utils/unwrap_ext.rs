use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

/*
 * debug switching between unwrap_throw() and unwrap()
 * just use unwrap_ji() and expect_ji() everywhere
 * potentially can add more instrumentaton for debugging here
 * TODO - get right line numbers!
 */

pub trait MyUnwrapExt<T>: Sized {
    #[track_caller]
    fn unwrap_ext(self) -> T {
        self.expect_ext("`unwrap_ext` failed")
    }

    #[track_caller]
    fn expect_ext(self, message: &str) -> T;
}

cfg_if! {
    if #[cfg(all(feature = "wasm-logger", feature = "console_error_panic_hook"))] {
        impl<T> MyUnwrapExt<T> for Option<T> {
            #[track_caller]
            fn expect_ext(self, message: &str) -> T {
                self.expect(message)
            }
        }

        impl<T, E> MyUnwrapExt<T> for Result<T, E>
        where
            E: core::fmt::Debug,
        {
            #[track_caller]
            fn expect_ext(self, message: &str) -> T {
                self.expect(message)
            }
        }
    } else {

        impl<T> MyUnwrapExt<T> for Option<T> {
            #[track_caller]
            fn expect_ext(self, message: &str) -> T {
                self.expect_throw(message)
            }
        }

        impl<T, E> MyUnwrapExt<T> for Result<T, E>
        where
            E: core::fmt::Debug,
        {
            #[track_caller]
            fn expect_ext(self, message: &str) -> T {
                self.expect_throw(message)
            }
        }
    }
}
