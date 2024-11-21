//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSQueryGenerationToken;
);

unsafe impl NSCoding for NSQueryGenerationToken {}

unsafe impl NSCopying for NSQueryGenerationToken {}

unsafe impl CopyingHelper for NSQueryGenerationToken {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSQueryGenerationToken {}

unsafe impl NSSecureCoding for NSQueryGenerationToken {}

extern_methods!(
    unsafe impl NSQueryGenerationToken {
        #[method_id(@__retain_semantics Other currentQueryGenerationToken)]
        pub unsafe fn currentQueryGenerationToken() -> Retained<NSQueryGenerationToken>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSQueryGenerationToken {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
