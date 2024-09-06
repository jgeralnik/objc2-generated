//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NEPacket;

    unsafe impl ClassType for NEPacket {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for NEPacket {}

unsafe impl NSCopying for NEPacket {}

unsafe impl CopyingHelper for NEPacket {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NEPacket {}

unsafe impl NSSecureCoding for NEPacket {}

extern_methods!(
    unsafe impl NEPacket {
        #[cfg(feature = "libc")]
        #[method_id(@__retain_semantics Init initWithData:protocolFamily:)]
        pub unsafe fn initWithData_protocolFamily(
            this: Allocated<Self>,
            data: &NSData,
            protocol_family: libc::sa_family_t,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other data)]
        pub unsafe fn data(&self) -> Retained<NSData>;

        #[cfg(feature = "libc")]
        #[method(protocolFamily)]
        pub unsafe fn protocolFamily(&self) -> libc::sa_family_t;

        #[cfg(feature = "NENetworkRule")]
        #[method(direction)]
        pub unsafe fn direction(&self) -> NETrafficDirection;

        #[cfg(feature = "NEFlowMetaData")]
        #[method_id(@__retain_semantics Other metadata)]
        pub unsafe fn metadata(&self) -> Option<Retained<NEFlowMetaData>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NEPacket {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
