//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct LAPublicKey;
);

unsafe impl NSObjectProtocol for LAPublicKey {}

extern_methods!(
    unsafe impl LAPublicKey {
        #[cfg(feature = "block2")]
        #[method(exportBytesWithCompletion:)]
        pub unsafe fn exportBytesWithCompletion(
            &self,
            handler: &block2::Block<dyn Fn(*mut NSData, *mut NSError)>,
        );

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
