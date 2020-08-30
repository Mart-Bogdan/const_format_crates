#[macro_use]
#[cfg(feature = "fmt")]
mod call_debug_fmt;

#[macro_use]
mod constructors;

#[macro_use]
mod helper_macros;

#[macro_use]
mod fmt_macros;

#[macro_use]
#[cfg(feature = "fmt")]
mod impl_fmt;

/// For returning early on an error, otherwise evaluating to `()`.
#[cfg(feature = "fmt")]
#[macro_export]
macro_rules! try_ {
    ($e:expr) => {
        if let $crate::pmr::Err(e) = $e {
            return $crate::pmr::Err(e);
        }
    };
}

/// Equivalent to `Result::unwrap`, for use with [`const_format::Error`] errors.
///
/// [`const_format::Error`]: ./fmt/enum.Error.html
#[cfg(feature = "fmt")]
#[macro_export]
macro_rules! unwrap {
    ($e:expr $(,)*) => {
        match $e {
            $crate::pmr::Ok(x) => x,
            $crate::pmr::Err(error) => $crate::Error::unwrap(error),
        }
    };
}

/// Equivalent to `Result::unwrap_or_else` but allows returning from the enclosing function.
#[cfg(feature = "fmt")]
#[macro_export]
macro_rules! unwrap_or_else {
    ($e:expr, |$error:ident| $orelse:expr ) => {
        match $e {
            $crate::pmr::Ok(x) => x,
            $crate::pmr::Err($error) => $orelse,
        }
    };
}

/// Coerces a reference to a type that has a `const_*_fmt` method.
///
/// # Behavior
///
/// For arrays it coerces them into a slice, and wraps them in a [`PWrapper`].
///
/// For std types, it wraps them in a [`PWrapper`], which implements the
/// `const_*_fmt` methods.
///
/// For std types, it just returns back the same reference.
///
/// [`PWrapper`]: ./
#[cfg(feature = "fmt")]
#[macro_export]
macro_rules! coerce_to_fmt {
    ($reference:expr) => {{
        match $reference {
            ref reference => {
                let marker = $crate::pmr::IsAFormatMarker::NEW;
                if false {
                    marker.infer_type(reference);
                }
                marker.coerce(marker.unreference(reference))
            }
        }
    }};
}

/// Converts a `&'static StrWriter` to a `&'static str`, in a `const`/`static` initializer.
///
/// This is usable in `const` or `static` initializers,
/// but not inside of `const fn`s.
///
/// # Runtime
///
/// If the "constant_time_as_str" feature is disabled,
/// thich takes time proportional to `$expr.capacity() - $expr.len()`.
///
/// If the "constant_time_as_str" feature is enabled, it takes constant time to run,
/// but uses a few additional nightly features.
///
/// # Example
///
/// ```rust
/// #![feature(const_mut_refs)]
///
/// use const_format::StrWriter;
/// use const_format::{strwriter_as_str, unwrap, writec};
///
///
/// const CAP: usize = 128;
///
/// // Can't use mutable references in `const`s yet, the `const fn` is a workaround
/// const fn formatted() -> StrWriter<[u8; CAP]> {
///     let mut writer =  StrWriter::new([0; CAP]);
///     unwrap!(writec!(writer, "{:x}", [3u32, 5, 8, 13, 21, 34]));
///     writer
/// }
///
/// const STR: &str = strwriter_as_str!(&formatted());
///
/// fn main() {
///     assert_eq!(STR, "[3, 5, 8, D, 15, 22]");
/// }
/// ```
///
#[cfg(feature = "fmt")]
#[macro_export]
macro_rules! strwriter_as_str {
    ($expr:expr) => {
        unsafe {
            let writer: &'static $crate::StrWriter = $expr;
            $crate::pmr::transmute::<&'static [u8], &'static str>(writer.as_bytes_alt())
        }
    };
}

macro_rules! conditionally_const {
    (
        feature = $feature:literal;
        $(
            $( #[$meta:meta] )*
            $vis:vis fn $fn_name:ident ($($params:tt)*) -> $ret:ty $block:block
        )*
    ) => (
        $(
            $(#[$meta])*
            #[cfg(feature = $feature)]
            $vis const fn $fn_name ($($params)*) -> $ret $block

            $(#[$meta])*
            #[cfg(not(feature = $feature))]
            $vis fn $fn_name ($($params)*) -> $ret $block
        )*
    )
}

macro_rules! std_kind_impl {
    (
        impl[$($impl:tt)*] $self:ty
        $(where[ $($where_:tt)* ])?
    )=>{
        impl<$($impl)*> $crate::pmr::FormatMarker for $self
        where
            $($($where_)*)?
        {
            type Kind = $crate::pmr::IsStdKind;
            type This = Self;
        }

        impl<$($impl)* __T> $crate::pmr::IsAFormatMarker<$crate::pmr::IsStdKind, $self, __T>
        where
            $($($where_)*)?
        {
            #[inline(always)]
            pub const fn coerce(self, reference: &$self) -> $crate::pmr::PWrapper<$self> {
                $crate::pmr::PWrapper(*reference)
            }
        }
    }
}
