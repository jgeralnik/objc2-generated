//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MKLookAroundSnapshot;

    unsafe impl ClassType for MKLookAroundSnapshot {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for MKLookAroundSnapshot {}

extern_methods!(
    unsafe impl MKLookAroundSnapshot {
        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Retained<NSImage>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MKLookAroundSnapshot {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
