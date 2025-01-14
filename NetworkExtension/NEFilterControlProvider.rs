//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/networkextension/nefiltercontrolprovider?language=objc)
    #[unsafe(super(NEFilterProvider, NEProvider, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NEFilterProvider", feature = "NEProvider"))]
    pub struct NEFilterControlProvider;
);

#[cfg(all(feature = "NEFilterProvider", feature = "NEProvider"))]
unsafe impl NSObjectProtocol for NEFilterControlProvider {}

extern_methods!(
    #[cfg(all(feature = "NEFilterProvider", feature = "NEProvider"))]
    unsafe impl NEFilterControlProvider {
        #[method_id(@__retain_semantics Other remediationMap)]
        pub unsafe fn remediationMap(
            &self,
        ) -> Option<Retained<NSDictionary<NSString, NSDictionary<NSString, NSObject>>>>;

        #[method(setRemediationMap:)]
        pub unsafe fn setRemediationMap(
            &self,
            remediation_map: Option<&NSDictionary<NSString, NSDictionary<NSString, NSObject>>>,
        );

        #[method_id(@__retain_semantics Other URLAppendStringMap)]
        pub unsafe fn URLAppendStringMap(
            &self,
        ) -> Option<Retained<NSDictionary<NSString, NSString>>>;

        #[method(setURLAppendStringMap:)]
        pub unsafe fn setURLAppendStringMap(
            &self,
            url_append_string_map: Option<&NSDictionary<NSString, NSString>>,
        );

        #[cfg(all(feature = "NEFilterFlow", feature = "block2"))]
        #[method(handleRemediationForFlow:completionHandler:)]
        pub unsafe fn handleRemediationForFlow_completionHandler(
            &self,
            flow: &NEFilterFlow,
            completion_handler: &block2::Block<dyn Fn(NonNull<NEFilterControlVerdict>)>,
        );

        #[cfg(all(feature = "NEFilterFlow", feature = "block2"))]
        #[method(handleNewFlow:completionHandler:)]
        pub unsafe fn handleNewFlow_completionHandler(
            &self,
            flow: &NEFilterFlow,
            completion_handler: &block2::Block<dyn Fn(NonNull<NEFilterControlVerdict>)>,
        );

        #[method(notifyRulesChanged)]
        pub unsafe fn notifyRulesChanged(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "NEFilterProvider", feature = "NEProvider"))]
    unsafe impl NEFilterControlProvider {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
