//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MEEmailAddress;

    unsafe impl ClassType for MEEmailAddress {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for MEEmailAddress {}

unsafe impl NSCopying for MEEmailAddress {}

unsafe impl CopyingHelper for MEEmailAddress {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MEEmailAddress {}

unsafe impl NSSecureCoding for MEEmailAddress {}

extern_methods!(
    unsafe impl MEEmailAddress {
        #[method_id(@__retain_semantics Other rawString)]
        pub unsafe fn rawString(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other addressString)]
        pub unsafe fn addressString(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithRawString:)]
        pub unsafe fn initWithRawString(
            this: Allocated<Self>,
            raw_string: &NSString,
        ) -> Retained<Self>;
    }
);
