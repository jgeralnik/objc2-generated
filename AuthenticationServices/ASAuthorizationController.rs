//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationcontrollerdelegate?language=objc)
    pub unsafe trait ASAuthorizationControllerDelegate:
        NSObjectProtocol + MainThreadOnly
    {
        #[cfg(feature = "ASAuthorization")]
        #[optional]
        #[method(authorizationController:didCompleteWithAuthorization:)]
        unsafe fn authorizationController_didCompleteWithAuthorization(
            &self,
            controller: &ASAuthorizationController,
            authorization: &ASAuthorization,
        );

        #[optional]
        #[method(authorizationController:didCompleteWithError:)]
        unsafe fn authorizationController_didCompleteWithError(
            &self,
            controller: &ASAuthorizationController,
            error: &NSError,
        );

        #[cfg(feature = "ASAuthorizationCustomMethod")]
        #[optional]
        #[method(authorizationController:didCompleteWithCustomMethod:)]
        unsafe fn authorizationController_didCompleteWithCustomMethod(
            &self,
            controller: &ASAuthorizationController,
            method: &ASAuthorizationCustomMethod,
        );
    }

    unsafe impl ProtocolType for dyn ASAuthorizationControllerDelegate {}
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationcontrollerpresentationcontextproviding?language=objc)
    pub unsafe trait ASAuthorizationControllerPresentationContextProviding:
        NSObjectProtocol + MainThreadOnly
    {
        #[cfg(feature = "ASFoundation")]
        #[cfg(target_os = "macos")]
        #[method_id(@__retain_semantics Other presentationAnchorForAuthorizationController:)]
        unsafe fn presentationAnchorForAuthorizationController(
            &self,
            controller: &ASAuthorizationController,
        ) -> Retained<ASPresentationAnchor>;
    }

    unsafe impl ProtocolType for dyn ASAuthorizationControllerPresentationContextProviding {}
);

/// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationcontrollerrequestoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ASAuthorizationControllerRequestOptions(pub NSUInteger);
bitflags::bitflags! {
    impl ASAuthorizationControllerRequestOptions: NSUInteger {
        const ASAuthorizationControllerRequestOptionPreferImmediatelyAvailableCredentials = 1<<0;
    }
}

unsafe impl Encode for ASAuthorizationControllerRequestOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for ASAuthorizationControllerRequestOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationcontroller?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationController;
);

unsafe impl NSObjectProtocol for ASAuthorizationController {}

extern_methods!(
    unsafe impl ASAuthorizationController {
        #[cfg(feature = "ASAuthorizationRequest")]
        #[method_id(@__retain_semantics Other authorizationRequests)]
        pub unsafe fn authorizationRequests(&self) -> Retained<NSArray<ASAuthorizationRequest>>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
            mtm: MainThreadMarker,
        ) -> Option<Retained<ProtocolObject<dyn ASAuthorizationControllerDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn ASAuthorizationControllerDelegate>>,
        );

        #[method_id(@__retain_semantics Other presentationContextProvider)]
        pub unsafe fn presentationContextProvider(
            &self,
            mtm: MainThreadMarker,
        ) -> Option<
            Retained<ProtocolObject<dyn ASAuthorizationControllerPresentationContextProviding>>,
        >;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setPresentationContextProvider:)]
        pub unsafe fn setPresentationContextProvider(
            &self,
            presentation_context_provider: Option<
                &ProtocolObject<dyn ASAuthorizationControllerPresentationContextProviding>,
            >,
        );

        #[cfg(feature = "ASAuthorizationCustomMethod")]
        #[method_id(@__retain_semantics Other customAuthorizationMethods)]
        pub unsafe fn customAuthorizationMethods(
            &self,
        ) -> Retained<NSArray<ASAuthorizationCustomMethod>>;

        #[cfg(feature = "ASAuthorizationCustomMethod")]
        #[method(setCustomAuthorizationMethods:)]
        pub unsafe fn setCustomAuthorizationMethods(
            &self,
            custom_authorization_methods: &NSArray<ASAuthorizationCustomMethod>,
        );

        #[cfg(feature = "ASAuthorizationRequest")]
        #[method_id(@__retain_semantics Init initWithAuthorizationRequests:)]
        pub unsafe fn initWithAuthorizationRequests(
            this: Allocated<Self>,
            authorization_requests: &NSArray<ASAuthorizationRequest>,
        ) -> Retained<Self>;

        #[method(performRequests)]
        pub unsafe fn performRequests(&self);

        #[method(performAutoFillAssistedRequests)]
        pub unsafe fn performAutoFillAssistedRequests(&self);

        #[method(performRequestsWithOptions:)]
        pub unsafe fn performRequestsWithOptions(
            &self,
            options: ASAuthorizationControllerRequestOptions,
        );

        #[method(cancel)]
        pub unsafe fn cancel(&self);

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
