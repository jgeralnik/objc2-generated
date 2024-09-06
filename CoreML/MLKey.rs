//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MLKey;

    unsafe impl ClassType for MLKey {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for MLKey {}

unsafe impl NSCopying for MLKey {}

unsafe impl CopyingHelper for MLKey {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MLKey {}

unsafe impl NSSecureCoding for MLKey {}

extern_methods!(
    unsafe impl MLKey {
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other scope)]
        pub unsafe fn scope(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
