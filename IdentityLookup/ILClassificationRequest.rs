//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ILClassificationRequest;
);

unsafe impl NSCoding for ILClassificationRequest {}

unsafe impl NSObjectProtocol for ILClassificationRequest {}

unsafe impl NSSecureCoding for ILClassificationRequest {}

extern_methods!(
    unsafe impl ILClassificationRequest {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ILClassificationRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
