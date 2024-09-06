//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CNGroup")]
    pub struct CNMutableGroup;

    #[cfg(feature = "CNGroup")]
    unsafe impl ClassType for CNMutableGroup {
        #[inherits(NSObject)]
        type Super = CNGroup;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CNGroup")]
unsafe impl NSCoding for CNMutableGroup {}

#[cfg(feature = "CNGroup")]
unsafe impl NSCopying for CNMutableGroup {}

#[cfg(feature = "CNGroup")]
unsafe impl CopyingHelper for CNMutableGroup {
    type Result = Self;
}

#[cfg(feature = "CNGroup")]
unsafe impl NSMutableCopying for CNMutableGroup {}

#[cfg(feature = "CNGroup")]
unsafe impl MutableCopyingHelper for CNMutableGroup {
    type Result = Self;
}

#[cfg(feature = "CNGroup")]
unsafe impl NSObjectProtocol for CNMutableGroup {}

#[cfg(feature = "CNGroup")]
unsafe impl NSSecureCoding for CNMutableGroup {}

extern_methods!(
    #[cfg(feature = "CNGroup")]
    unsafe impl CNMutableGroup {
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[method(setName:)]
        pub unsafe fn setName(&self, name: &NSString);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CNGroup")]
    unsafe impl CNMutableGroup {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
