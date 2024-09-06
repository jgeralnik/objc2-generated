//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CBAttribute;

    unsafe impl ClassType for CBAttribute {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for CBAttribute {}

extern_methods!(
    unsafe impl CBAttribute {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "CBUUID")]
        #[method_id(@__retain_semantics Other UUID)]
        pub unsafe fn UUID(&self) -> Retained<CBUUID>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CBAttribute {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
