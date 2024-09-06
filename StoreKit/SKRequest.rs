//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SKRequest;

    unsafe impl ClassType for SKRequest {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for SKRequest {}

extern_methods!(
    unsafe impl SKRequest {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Retained<ProtocolObject<dyn SKRequestDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn SKRequestDelegate>>);

        #[method(cancel)]
        pub unsafe fn cancel(&self);

        #[method(start)]
        pub unsafe fn start(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SKRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    pub unsafe trait SKRequestDelegate: NSObjectProtocol {
        #[optional]
        #[method(requestDidFinish:)]
        unsafe fn requestDidFinish(&self, request: &SKRequest);

        #[optional]
        #[method(request:didFailWithError:)]
        unsafe fn request_didFailWithError(&self, request: &SKRequest, error: &NSError);
    }

    unsafe impl ProtocolType for dyn SKRequestDelegate {}
);
