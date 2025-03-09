//! Shared macros for use inside this crate

// Implement the wrapper for a newtype
#[macro_export]
macro_rules! impl_wrapper_on_newtype {
    ($name:ty, $kind:expr) => {
        impl $crate::object::Wrapper for $name {
            const KIND: $crate::object::Kind = $kind;

            fn from_ptr_unchecked(ptr: ::std::ptr::NonNull<::std::ffi::c_void>) -> Self {
                Self(ptr)
            }

            fn as_xhandle(&self) -> ::rxegy_sys::xhandle {
                self.0.as_ptr()
            }
        }
    };
}
