//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VZKeyboardConfiguration")]
    pub struct VZUSBKeyboardConfiguration;

    #[cfg(feature = "VZKeyboardConfiguration")]
    unsafe impl ClassType for VZUSBKeyboardConfiguration {
        #[inherits(NSObject)]
        type Super = VZKeyboardConfiguration;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "VZKeyboardConfiguration")]
unsafe impl NSCopying for VZUSBKeyboardConfiguration {}

#[cfg(feature = "VZKeyboardConfiguration")]
unsafe impl CopyingHelper for VZUSBKeyboardConfiguration {
    type Result = Self;
}

#[cfg(feature = "VZKeyboardConfiguration")]
unsafe impl NSObjectProtocol for VZUSBKeyboardConfiguration {}

extern_methods!(
    #[cfg(feature = "VZKeyboardConfiguration")]
    unsafe impl VZUSBKeyboardConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `VZKeyboardConfiguration`
    #[cfg(feature = "VZKeyboardConfiguration")]
    unsafe impl VZUSBKeyboardConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
