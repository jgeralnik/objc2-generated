//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIScreenMode;

    unsafe impl ClassType for UIScreenMode {
        type Super = NSObject;
    }
);

unsafe impl Send for UIScreenMode {}

unsafe impl Sync for UIScreenMode {}

unsafe impl NSObjectProtocol for UIScreenMode {}

extern_methods!(
    unsafe impl UIScreenMode {
        #[method(size)]
        pub fn size(&self) -> CGSize;

        #[method(pixelAspectRatio)]
        pub unsafe fn pixelAspectRatio(&self) -> CGFloat;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIScreenMode {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
