//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKNavigationResponse;

    unsafe impl ClassType for WKNavigationResponse {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for WKNavigationResponse {}

extern_methods!(
    unsafe impl WKNavigationResponse {
        #[method(isForMainFrame)]
        pub unsafe fn isForMainFrame(&self) -> bool;

        #[method_id(@__retain_semantics Other response)]
        pub unsafe fn response(&self) -> Retained<NSURLResponse>;

        #[method(canShowMIMEType)]
        pub unsafe fn canShowMIMEType(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl WKNavigationResponse {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
