//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZMACAddress;

    unsafe impl ClassType for VZMACAddress {
        type Super = NSObject;
    }
);

unsafe impl NSCopying for VZMACAddress {}

unsafe impl CopyingHelper for VZMACAddress {
    type Result = Self;
}

unsafe impl NSObjectProtocol for VZMACAddress {}

extern_methods!(
    unsafe impl VZMACAddress {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithString:)]
        pub unsafe fn initWithString(
            this: Allocated<Self>,
            string: &NSString,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other randomLocallyAdministeredAddress)]
        pub unsafe fn randomLocallyAdministeredAddress() -> Retained<Self>;

        #[method_id(@__retain_semantics Other string)]
        pub unsafe fn string(&self) -> Retained<NSString>;

        #[method(isBroadcastAddress)]
        pub unsafe fn isBroadcastAddress(&self) -> bool;

        #[method(isMulticastAddress)]
        pub unsafe fn isMulticastAddress(&self) -> bool;

        #[method(isUnicastAddress)]
        pub unsafe fn isUnicastAddress(&self) -> bool;

        #[method(isLocallyAdministeredAddress)]
        pub unsafe fn isLocallyAdministeredAddress(&self) -> bool;

        #[method(isUniversallyAdministeredAddress)]
        pub unsafe fn isUniversallyAdministeredAddress(&self) -> bool;
    }
);
