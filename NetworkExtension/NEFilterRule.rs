//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NEFilterRule;
);

unsafe impl NSCoding for NEFilterRule {}

unsafe impl NSCopying for NEFilterRule {}

unsafe impl CopyingHelper for NEFilterRule {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NEFilterRule {}

unsafe impl NSSecureCoding for NEFilterRule {}

extern_methods!(
    unsafe impl NEFilterRule {
        #[cfg(all(feature = "NEFilterProvider", feature = "NENetworkRule"))]
        #[method_id(@__retain_semantics Init initWithNetworkRule:action:)]
        pub unsafe fn initWithNetworkRule_action(
            this: Allocated<Self>,
            network_rule: &NENetworkRule,
            action: NEFilterAction,
        ) -> Retained<Self>;

        #[cfg(feature = "NENetworkRule")]
        #[method_id(@__retain_semantics Other networkRule)]
        pub unsafe fn networkRule(&self) -> Retained<NENetworkRule>;

        #[cfg(feature = "NEFilterProvider")]
        #[method(action)]
        pub unsafe fn action(&self) -> NEFilterAction;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NEFilterRule {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
