//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CNContactProperty;
);

unsafe impl NSCoding for CNContactProperty {}

unsafe impl NSCopying for CNContactProperty {}

unsafe impl CopyingHelper for CNContactProperty {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CNContactProperty {}

unsafe impl NSSecureCoding for CNContactProperty {}

extern_methods!(
    unsafe impl CNContactProperty {
        #[cfg(feature = "CNContact")]
        #[method_id(@__retain_semantics Other contact)]
        pub unsafe fn contact(&self) -> Retained<CNContact>;

        #[method_id(@__retain_semantics Other key)]
        pub unsafe fn key(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other value)]
        pub unsafe fn value(&self) -> Option<Retained<AnyObject>>;

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other label)]
        pub unsafe fn label(&self) -> Option<Retained<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CNContactProperty {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
