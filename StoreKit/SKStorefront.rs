//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/storekit/skstorefront?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "Use Storefront"]
    pub struct SKStorefront;
);

unsafe impl Send for SKStorefront {}

unsafe impl Sync for SKStorefront {}

unsafe impl NSObjectProtocol for SKStorefront {}

extern_methods!(
    unsafe impl SKStorefront {
        #[deprecated = "Use 'Storefront.countryCode'"]
        #[method_id(@__retain_semantics Other countryCode)]
        pub unsafe fn countryCode(&self) -> Retained<NSString>;

        #[deprecated = "Use 'Storefront.id'"]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SKStorefront {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
