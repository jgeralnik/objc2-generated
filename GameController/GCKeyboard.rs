//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    pub static GCKeyboardDidConnectNotification: &'static NSString;
}

extern "C" {
    pub static GCKeyboardDidDisconnectNotification: &'static NSString;
}

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GCKeyboard;
);

#[cfg(feature = "GCDevice")]
unsafe impl GCDevice for GCKeyboard {}

unsafe impl NSObjectProtocol for GCKeyboard {}

extern_methods!(
    unsafe impl GCKeyboard {
        #[cfg(all(feature = "GCKeyboardInput", feature = "GCPhysicalInputProfile"))]
        #[method_id(@__retain_semantics Other keyboardInput)]
        pub unsafe fn keyboardInput(&self) -> Option<Retained<GCKeyboardInput>>;

        #[method_id(@__retain_semantics Other coalescedKeyboard)]
        pub unsafe fn coalescedKeyboard() -> Option<Retained<GCKeyboard>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl GCKeyboard {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
