//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTintConfiguration;
);

unsafe impl NSCoding for NSTintConfiguration {}

unsafe impl NSCopying for NSTintConfiguration {}

unsafe impl CopyingHelper for NSTintConfiguration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSTintConfiguration {}

unsafe impl NSSecureCoding for NSTintConfiguration {}

extern_methods!(
    unsafe impl NSTintConfiguration {
        #[method_id(@__retain_semantics Other defaultTintConfiguration)]
        pub unsafe fn defaultTintConfiguration() -> Retained<NSTintConfiguration>;

        #[method_id(@__retain_semantics Other monochromeTintConfiguration)]
        pub unsafe fn monochromeTintConfiguration() -> Retained<NSTintConfiguration>;

        #[cfg(feature = "NSColor")]
        #[method_id(@__retain_semantics Other tintConfigurationWithPreferredColor:)]
        pub unsafe fn tintConfigurationWithPreferredColor(color: &NSColor) -> Retained<Self>;

        #[cfg(feature = "NSColor")]
        #[method_id(@__retain_semantics Other tintConfigurationWithFixedColor:)]
        pub unsafe fn tintConfigurationWithFixedColor(color: &NSColor) -> Retained<Self>;

        #[cfg(feature = "NSColor")]
        #[method_id(@__retain_semantics Other baseTintColor)]
        pub unsafe fn baseTintColor(&self) -> Option<Retained<NSColor>>;

        #[cfg(feature = "NSColor")]
        #[method_id(@__retain_semantics Other equivalentContentTintColor)]
        pub unsafe fn equivalentContentTintColor(&self) -> Option<Retained<NSColor>>;

        #[method(adaptsToUserAccentColor)]
        pub unsafe fn adaptsToUserAccentColor(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSTintConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
