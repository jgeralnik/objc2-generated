//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait ASAccountAuthenticationModificationControllerDelegate:
        NSObjectProtocol
    {
        #[cfg(feature = "ASAccountAuthenticationModificationRequest")]
        #[optional]
        #[method(accountAuthenticationModificationController:didSuccessfullyCompleteRequest:withUserInfo:)]
        unsafe fn accountAuthenticationModificationController_didSuccessfullyCompleteRequest_withUserInfo(
            &self,
            controller: &ASAccountAuthenticationModificationController,
            request: &ASAccountAuthenticationModificationRequest,
            user_info: Option<&NSDictionary>,
        );

        #[cfg(feature = "ASAccountAuthenticationModificationRequest")]
        #[optional]
        #[method(accountAuthenticationModificationController:didFailRequest:withError:)]
        unsafe fn accountAuthenticationModificationController_didFailRequest_withError(
            &self,
            controller: &ASAccountAuthenticationModificationController,
            request: &ASAccountAuthenticationModificationRequest,
            error: &NSError,
        );
    }

    unsafe impl ProtocolType for dyn ASAccountAuthenticationModificationControllerDelegate {}
);

extern_protocol!(
    pub unsafe trait ASAccountAuthenticationModificationControllerPresentationContextProviding:
        NSObjectProtocol + MainThreadOnly
    {
        #[cfg(feature = "ASFoundation")]
        #[cfg(target_os = "macos")]
        #[method_id(@__retain_semantics Other presentationAnchorForAccountAuthenticationModificationController:)]
        unsafe fn presentationAnchorForAccountAuthenticationModificationController(
            &self,
            controller: &ASAccountAuthenticationModificationController,
        ) -> Retained<ASPresentationAnchor>;
    }

    unsafe impl ProtocolType
        for dyn ASAccountAuthenticationModificationControllerPresentationContextProviding
    {
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAccountAuthenticationModificationController;

    unsafe impl ClassType for ASAccountAuthenticationModificationController {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for ASAccountAuthenticationModificationController {}

extern_methods!(
    unsafe impl ASAccountAuthenticationModificationController {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<
            Retained<ProtocolObject<dyn ASAccountAuthenticationModificationControllerDelegate>>,
        >;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<
                &ProtocolObject<dyn ASAccountAuthenticationModificationControllerDelegate>,
            >,
        );

        #[method_id(@__retain_semantics Other presentationContextProvider)]
        pub unsafe fn presentationContextProvider(
            &self,
            mtm: MainThreadMarker,
        ) -> Option<
            Retained<
                ProtocolObject<
                    dyn ASAccountAuthenticationModificationControllerPresentationContextProviding,
                >,
            >,
        >;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setPresentationContextProvider:)]
        pub unsafe fn setPresentationContextProvider(
            &self,
            presentation_context_provider: Option<
                &ProtocolObject<
                    dyn ASAccountAuthenticationModificationControllerPresentationContextProviding,
                >,
            >,
        );

        #[cfg(feature = "ASAccountAuthenticationModificationRequest")]
        #[method(performRequest:)]
        pub unsafe fn performRequest(&self, request: &ASAccountAuthenticationModificationRequest);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ASAccountAuthenticationModificationController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
