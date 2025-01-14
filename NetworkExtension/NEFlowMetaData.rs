//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/networkextension/neflowmetadata?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NEFlowMetaData;
);

unsafe impl NSCoding for NEFlowMetaData {}

unsafe impl NSCopying for NEFlowMetaData {}

unsafe impl CopyingHelper for NEFlowMetaData {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NEFlowMetaData {}

unsafe impl NSSecureCoding for NEFlowMetaData {}

extern_methods!(
    unsafe impl NEFlowMetaData {
        #[method_id(@__retain_semantics Other sourceAppUniqueIdentifier)]
        pub unsafe fn sourceAppUniqueIdentifier(&self) -> Retained<NSData>;

        #[method_id(@__retain_semantics Other sourceAppSigningIdentifier)]
        pub unsafe fn sourceAppSigningIdentifier(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other sourceAppAuditToken)]
        pub unsafe fn sourceAppAuditToken(&self) -> Option<Retained<NSData>>;

        #[method_id(@__retain_semantics Other filterFlowIdentifier)]
        pub unsafe fn filterFlowIdentifier(&self) -> Option<Retained<NSUUID>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NEFlowMetaData {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
