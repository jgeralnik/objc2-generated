//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/devicecheck/dcdevice?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct DCDevice;
);

unsafe impl NSObjectProtocol for DCDevice {}

extern_methods!(
    unsafe impl DCDevice {
        #[method_id(@__retain_semantics Other currentDevice)]
        pub unsafe fn currentDevice() -> Retained<DCDevice>;

        #[method(isSupported)]
        pub unsafe fn isSupported(&self) -> bool;

        #[cfg(feature = "block2")]
        #[method(generateTokenWithCompletionHandler:)]
        pub unsafe fn generateTokenWithCompletionHandler(
            &self,
            completion: &block2::Block<dyn Fn(*mut NSData, *mut NSError)>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl DCDevice {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
