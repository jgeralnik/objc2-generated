//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AEAssessmentSession;

    unsafe impl ClassType for AEAssessmentSession {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for AEAssessmentSession {}

extern_methods!(
    unsafe impl AEAssessmentSession {
        #[method(supportsMultipleParticipants)]
        pub unsafe fn supportsMultipleParticipants() -> bool;

        #[method(supportsConfigurationUpdates)]
        pub unsafe fn supportsConfigurationUpdates() -> bool;

        #[cfg(feature = "AEAssessmentSessionDelegate")]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn AEAssessmentSessionDelegate>>>;

        #[cfg(feature = "AEAssessmentSessionDelegate")]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn AEAssessmentSessionDelegate>>,
        );

        #[cfg(feature = "AEAssessmentConfiguration")]
        #[method_id(@__retain_semantics Other configuration)]
        pub unsafe fn configuration(&self) -> Id<AEAssessmentConfiguration>;

        #[method(isActive)]
        pub unsafe fn isActive(&self) -> bool;

        #[cfg(feature = "AEAssessmentConfiguration")]
        #[method_id(@__retain_semantics Init initWithConfiguration:)]
        pub unsafe fn initWithConfiguration(
            this: Allocated<Self>,
            configuration: &AEAssessmentConfiguration,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method(begin)]
        pub unsafe fn begin(&self);

        #[method(end)]
        pub unsafe fn end(&self);

        #[cfg(feature = "AEAssessmentConfiguration")]
        #[method(updateToConfiguration:)]
        pub unsafe fn updateToConfiguration(&self, configuration: &AEAssessmentConfiguration);
    }
);
