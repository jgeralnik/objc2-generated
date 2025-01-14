//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/automator/amworkspace?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AMWorkspace;
);

unsafe impl NSObjectProtocol for AMWorkspace {}

extern_methods!(
    unsafe impl AMWorkspace {
        #[method_id(@__retain_semantics Other sharedWorkspace)]
        pub unsafe fn sharedWorkspace() -> Option<Retained<AMWorkspace>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AMWorkspace {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
