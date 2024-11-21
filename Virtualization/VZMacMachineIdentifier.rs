//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZMacMachineIdentifier;
);

unsafe impl NSCopying for VZMacMachineIdentifier {}

unsafe impl CopyingHelper for VZMacMachineIdentifier {
    type Result = Self;
}

unsafe impl NSObjectProtocol for VZMacMachineIdentifier {}

extern_methods!(
    unsafe impl VZMacMachineIdentifier {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDataRepresentation:)]
        pub unsafe fn initWithDataRepresentation(
            this: Allocated<Self>,
            data_representation: &NSData,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other dataRepresentation)]
        pub unsafe fn dataRepresentation(&self) -> Retained<NSData>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl VZMacMachineIdentifier {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
