//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MLModelStructureProgramArgument;

    unsafe impl ClassType for MLModelStructureProgramArgument {
        type Super = NSObject;
    }
);

unsafe impl Send for MLModelStructureProgramArgument {}

unsafe impl Sync for MLModelStructureProgramArgument {}

unsafe impl NSObjectProtocol for MLModelStructureProgramArgument {}

extern_methods!(
    unsafe impl MLModelStructureProgramArgument {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "MLModelStructureProgramBinding")]
        #[method_id(@__retain_semantics Other bindings)]
        pub unsafe fn bindings(&self) -> Retained<NSArray<MLModelStructureProgramBinding>>;
    }
);
