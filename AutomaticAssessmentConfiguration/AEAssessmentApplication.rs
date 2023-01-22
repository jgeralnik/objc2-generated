//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AutomaticAssessmentConfiguration::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AutomaticAssessmentConfiguration_AEAssessmentApplication")]
    pub struct AEAssessmentApplication;

    #[cfg(feature = "AutomaticAssessmentConfiguration_AEAssessmentApplication")]
    unsafe impl ClassType for AEAssessmentApplication {
        type Super = NSObject;
    }
);

#[cfg(feature = "AutomaticAssessmentConfiguration_AEAssessmentApplication")]
unsafe impl NSObjectProtocol for AEAssessmentApplication {}

extern_methods!(
    #[cfg(feature = "AutomaticAssessmentConfiguration_AEAssessmentApplication")]
    unsafe impl AEAssessmentApplication {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other bundleIdentifier)]
        pub unsafe fn bundleIdentifier(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other teamIdentifier)]
        pub unsafe fn teamIdentifier(&self) -> Option<Id<NSString, Shared>>;

        #[method(requiresSignatureValidation)]
        pub unsafe fn requiresSignatureValidation(&self) -> bool;

        #[method(setRequiresSignatureValidation:)]
        pub unsafe fn setRequiresSignatureValidation(&self, requires_signature_validation: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithBundleIdentifier:)]
        pub unsafe fn initWithBundleIdentifier(
            this: Option<Allocated<Self>>,
            bundle_identifier: &NSString,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithBundleIdentifier:teamIdentifier:)]
        pub unsafe fn initWithBundleIdentifier_teamIdentifier(
            this: Option<Allocated<Self>>,
            bundle_identifier: &NSString,
            team_identifier: Option<&NSString>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;
    }
);