//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZLinuxRosettaCachingOptions;

    unsafe impl ClassType for VZLinuxRosettaCachingOptions {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for VZLinuxRosettaCachingOptions {}

extern_methods!(
    unsafe impl VZLinuxRosettaCachingOptions {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
