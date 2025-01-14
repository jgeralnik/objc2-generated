//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/networkextension/nefiltersettings?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NEFilterSettings;
);

unsafe impl NSCoding for NEFilterSettings {}

unsafe impl NSCopying for NEFilterSettings {}

unsafe impl CopyingHelper for NEFilterSettings {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NEFilterSettings {}

unsafe impl NSSecureCoding for NEFilterSettings {}

extern_methods!(
    unsafe impl NEFilterSettings {
        #[cfg(all(feature = "NEFilterProvider", feature = "NEFilterRule"))]
        #[method_id(@__retain_semantics Init initWithRules:defaultAction:)]
        pub unsafe fn initWithRules_defaultAction(
            this: Allocated<Self>,
            rules: &NSArray<NEFilterRule>,
            default_action: NEFilterAction,
        ) -> Retained<Self>;

        #[cfg(feature = "NEFilterRule")]
        #[method_id(@__retain_semantics Other rules)]
        pub unsafe fn rules(&self) -> Retained<NSArray<NEFilterRule>>;

        #[cfg(feature = "NEFilterProvider")]
        #[method(defaultAction)]
        pub unsafe fn defaultAction(&self) -> NEFilterAction;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NEFilterSettings {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
