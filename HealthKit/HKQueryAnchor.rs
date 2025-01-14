//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkqueryanchor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKQueryAnchor;
);

unsafe impl Send for HKQueryAnchor {}

unsafe impl Sync for HKQueryAnchor {}

unsafe impl NSCoding for HKQueryAnchor {}

unsafe impl NSCopying for HKQueryAnchor {}

unsafe impl CopyingHelper for HKQueryAnchor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for HKQueryAnchor {}

unsafe impl NSSecureCoding for HKQueryAnchor {}

extern_methods!(
    unsafe impl HKQueryAnchor {
        #[method_id(@__retain_semantics Other anchorFromValue:)]
        pub unsafe fn anchorFromValue(value: NSUInteger) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HKQueryAnchor {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
