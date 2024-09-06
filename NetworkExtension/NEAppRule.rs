//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NEAppRule;

    unsafe impl ClassType for NEAppRule {
        type Super = NSObject;
    }
);

unsafe impl NSCoding for NEAppRule {}

unsafe impl NSCopying for NEAppRule {}

unsafe impl CopyingHelper for NEAppRule {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NEAppRule {}

unsafe impl NSSecureCoding for NEAppRule {}

extern_methods!(
    unsafe impl NEAppRule {
        #[method_id(@__retain_semantics Init initWithSigningIdentifier:)]
        pub unsafe fn initWithSigningIdentifier(
            this: Allocated<Self>,
            signing_identifier: &NSString,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithSigningIdentifier:designatedRequirement:)]
        pub unsafe fn initWithSigningIdentifier_designatedRequirement(
            this: Allocated<Self>,
            signing_identifier: &NSString,
            designated_requirement: &NSString,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other matchSigningIdentifier)]
        pub unsafe fn matchSigningIdentifier(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other matchDesignatedRequirement)]
        pub unsafe fn matchDesignatedRequirement(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other matchPath)]
        pub unsafe fn matchPath(&self) -> Option<Retained<NSString>>;

        #[method(setMatchPath:)]
        pub unsafe fn setMatchPath(&self, match_path: Option<&NSString>);

        #[method_id(@__retain_semantics Other matchDomains)]
        pub unsafe fn matchDomains(&self) -> Option<Retained<NSArray>>;

        #[method(setMatchDomains:)]
        pub unsafe fn setMatchDomains(&self, match_domains: Option<&NSArray>);

        #[method_id(@__retain_semantics Other matchTools)]
        pub unsafe fn matchTools(&self) -> Option<Retained<NSArray<NEAppRule>>>;

        #[method(setMatchTools:)]
        pub unsafe fn setMatchTools(&self, match_tools: Option<&NSArray<NEAppRule>>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NEAppRule {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
