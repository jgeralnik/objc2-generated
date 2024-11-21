//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MLImageConstraint;
);

unsafe impl NSCoding for MLImageConstraint {}

unsafe impl NSObjectProtocol for MLImageConstraint {}

unsafe impl NSSecureCoding for MLImageConstraint {}

extern_methods!(
    unsafe impl MLImageConstraint {
        #[method(pixelsHigh)]
        pub unsafe fn pixelsHigh(&self) -> NSInteger;

        #[method(pixelsWide)]
        pub unsafe fn pixelsWide(&self) -> NSInteger;

        #[method(pixelFormatType)]
        pub unsafe fn pixelFormatType(&self) -> OSType;

        #[cfg(feature = "MLImageSizeConstraint")]
        #[method_id(@__retain_semantics Other sizeConstraint)]
        pub unsafe fn sizeConstraint(&self) -> Retained<MLImageSizeConstraint>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MLImageConstraint {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
