//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MKShape;

    unsafe impl ClassType for MKShape {
        type Super = NSObject;
    }
);

#[cfg(feature = "MKAnnotation")]
unsafe impl MKAnnotation for MKShape {}

unsafe impl NSObjectProtocol for MKShape {}

extern_methods!(
    unsafe impl MKShape {
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Retained<NSString>>;

        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[method_id(@__retain_semantics Other subtitle)]
        pub unsafe fn subtitle(&self) -> Option<Retained<NSString>>;

        #[method(setSubtitle:)]
        pub unsafe fn setSubtitle(&self, subtitle: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MKShape {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
