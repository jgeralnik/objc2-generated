//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/identitylookup/ilnetworkresponse?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ILNetworkResponse;
);

unsafe impl NSCoding for ILNetworkResponse {}

unsafe impl NSObjectProtocol for ILNetworkResponse {}

unsafe impl NSSecureCoding for ILNetworkResponse {}

extern_methods!(
    unsafe impl ILNetworkResponse {
        #[method_id(@__retain_semantics Other urlResponse)]
        pub unsafe fn urlResponse(&self) -> Retained<NSHTTPURLResponse>;

        #[method_id(@__retain_semantics Other data)]
        pub unsafe fn data(&self) -> Retained<NSData>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ILNetworkResponse {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
