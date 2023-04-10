//! Perform a massive ownage of your variables!
//!
//! For those who tired of manually cloning all the fancy arcs and boxes before passing them into a
//! closure/thread/future, but who's also against introducing another general purpose macro into
//! the codebase.
//!
//! The [`own`](crate::own) function tries to provide the golden mean between code ergonomics and
//! readability.
//!
//! Usage:
//! ```
//! use ownage::own;
//!
//! let string = String::new();
//! let str_ref = string.as_str();
//! let vec = Vec::<bool>::new();
//! let slice = vec.as_slice();
//! let arc = std::sync::Arc::new(42u8);
//! let u = 42u8;
//!
//! let answer = own((&string, str_ref, &vec, slice, &arc, &u), |s, s_ref, v, sl, arc, u| {
//!     std::thread::spawn(move || {
//!         // Do your dirty stuff here
//!         u
//!     })
//!     .join()
//!     .unwrap()
//! });
//!
//! assert_eq!(answer, 42);
//! ```

/// Takes a tuple of references (up to 12), clones them into their owned versions using [`ToOwned`]
/// and passes each new variable to the provided closure as an argument.
///
/// [`Closure`] is implemented for every [`FnOnce`] having a correct signature, so any compatible
/// fn pointer will also do.
///
/// If, for some reason, you want to own more than 12 variables at once - most likely you are doing
/// something wrong. Consider defining a custom struct and implementing your cloning logic there.
/// However, if you're sure that your usecase is valid, feel free to present it, so we can increase
/// the supported tuple arity.
pub fn own<R, C>(refs: R, closure: C) -> C::Output
where
    C: Closure<R>,
{
    closure.call(refs)
}

/// Closure trait, basically any [`FnOnce`] having up to 12 [`owned`](ToOwned::Owned) arguments.
pub trait Closure<R>: private::Closure<R> {}

impl<R, F> Closure<R> for F where F: private::Closure<R> {}

mod private {
    pub trait Closure<R> {
        type Output;

        fn call(self, variables: R) -> Self::Output;
    }
}

macro_rules! impl_closure {
    ($($idx:tt : $x:tt),+) => {
        impl<FN, Out $(, $x)+> private::Closure<($(&$x, )+)> for FN
            where FN: FnOnce($($x::Owned, )+) -> Out, $($x: ToOwned + ?Sized,)+
        {
            type Output = FN::Output;

            fn call(self, r: ($(&$x, )+)) -> Self::Output {
                self($(r.$idx.to_owned(),)+)
            }
        }
    };
}

impl_closure!(0: A);
impl_closure!(0: A, 1: B);
impl_closure!(0: A, 1: B, 2: C);
impl_closure!(0: A, 1: B, 2: C, 3: D);
impl_closure!(0: A, 1: B, 2: C, 3: D, 4: E);
impl_closure!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F);
impl_closure!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G);
impl_closure!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H);
impl_closure!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I);
impl_closure!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I, 9: J);
impl_closure!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I, 9: J, 10: K);
impl_closure!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I, 9: J, 10: K, 11: L);
