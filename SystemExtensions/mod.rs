// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

#![allow(unused_imports)]
#![allow(deprecated)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(missing_docs)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::identity_op)]
#![allow(clippy::missing_safety_doc)]

#[link(name = "SystemExtensions", kind = "framework")]
extern "C" {}

use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    pub static OSSystemExtensionErrorDomain: &'static NSErrorDomain;
}

extern "C" {
    pub static OSBundleUsageDescriptionKey: &'static NSString;
}

extern "C" {
    pub static NSSystemExtensionUsageDescriptionKey: &'static NSString;
}

extern "C" {
    pub static OSRelatedKernelExtensionKey: &'static NSString;
}

// NS_ERROR_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OSSystemExtensionErrorCode(pub NSInteger);
impl OSSystemExtensionErrorCode {
    pub const OSSystemExtensionErrorUnknown: Self = Self(1);
    pub const OSSystemExtensionErrorMissingEntitlement: Self = Self(2);
    pub const OSSystemExtensionErrorUnsupportedParentBundleLocation: Self = Self(3);
    pub const OSSystemExtensionErrorExtensionNotFound: Self = Self(4);
    pub const OSSystemExtensionErrorExtensionMissingIdentifier: Self = Self(5);
    pub const OSSystemExtensionErrorDuplicateExtensionIdentifer: Self = Self(6);
    pub const OSSystemExtensionErrorUnknownExtensionCategory: Self = Self(7);
    #[doc(alias = "OSSystemExtensionErrorCodeSignatureInvalid")]
    pub const SignatureInvalid: Self = Self(8);
    pub const OSSystemExtensionErrorValidationFailed: Self = Self(9);
    pub const OSSystemExtensionErrorForbiddenBySystemPolicy: Self = Self(10);
    pub const OSSystemExtensionErrorRequestCanceled: Self = Self(11);
    pub const OSSystemExtensionErrorRequestSuperseded: Self = Self(12);
    pub const OSSystemExtensionErrorAuthorizationRequired: Self = Self(13);
}

unsafe impl Encode for OSSystemExtensionErrorCode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for OSSystemExtensionErrorCode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OSSystemExtensionReplacementAction(pub NSInteger);
impl OSSystemExtensionReplacementAction {
    #[doc(alias = "OSSystemExtensionReplacementActionCancel")]
    pub const Cancel: Self = Self(0);
    #[doc(alias = "OSSystemExtensionReplacementActionReplace")]
    pub const Replace: Self = Self(1);
}

unsafe impl Encode for OSSystemExtensionReplacementAction {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for OSSystemExtensionReplacementAction {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OSSystemExtensionRequestResult(pub NSInteger);
impl OSSystemExtensionRequestResult {
    pub const OSSystemExtensionRequestCompleted: Self = Self(0);
    pub const OSSystemExtensionRequestWillCompleteAfterReboot: Self = Self(1);
}

unsafe impl Encode for OSSystemExtensionRequestResult {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for OSSystemExtensionRequestResult {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct OSSystemExtensionRequest;

    unsafe impl ClassType for OSSystemExtensionRequest {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for OSSystemExtensionRequest {}

extern_methods!(
    unsafe impl OSSystemExtensionRequest {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn OSSystemExtensionRequestDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn OSSystemExtensionRequestDelegate>>,
        );

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl OSSystemExtensionRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct OSSystemExtensionProperties;

    unsafe impl ClassType for OSSystemExtensionProperties {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for OSSystemExtensionProperties {}

extern_methods!(
    unsafe impl OSSystemExtensionProperties {
        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Retained<NSURL>;

        #[method_id(@__retain_semantics Other bundleIdentifier)]
        pub unsafe fn bundleIdentifier(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other bundleVersion)]
        pub unsafe fn bundleVersion(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other bundleShortVersion)]
        pub unsafe fn bundleShortVersion(&self) -> Retained<NSString>;

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(isAwaitingUserApproval)]
        pub unsafe fn isAwaitingUserApproval(&self) -> bool;

        #[method(isUninstalling)]
        pub unsafe fn isUninstalling(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl OSSystemExtensionProperties {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    pub unsafe trait OSSystemExtensionRequestDelegate: NSObjectProtocol {
        #[method(request:actionForReplacingExtension:withExtension:)]
        unsafe fn request_actionForReplacingExtension_withExtension(
            &self,
            request: &OSSystemExtensionRequest,
            existing: &OSSystemExtensionProperties,
            ext: &OSSystemExtensionProperties,
        ) -> OSSystemExtensionReplacementAction;

        #[method(requestNeedsUserApproval:)]
        unsafe fn requestNeedsUserApproval(&self, request: &OSSystemExtensionRequest);

        #[method(request:didFinishWithResult:)]
        unsafe fn request_didFinishWithResult(
            &self,
            request: &OSSystemExtensionRequest,
            result: OSSystemExtensionRequestResult,
        );

        #[method(request:didFailWithError:)]
        unsafe fn request_didFailWithError(
            &self,
            request: &OSSystemExtensionRequest,
            error: &NSError,
        );

        #[optional]
        #[method(request:foundProperties:)]
        unsafe fn request_foundProperties(
            &self,
            request: &OSSystemExtensionRequest,
            properties: &NSArray<OSSystemExtensionProperties>,
        );
    }

    unsafe impl ProtocolType for dyn OSSystemExtensionRequestDelegate {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct OSSystemExtensionManager;

    unsafe impl ClassType for OSSystemExtensionManager {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for OSSystemExtensionManager {}

extern_methods!(
    unsafe impl OSSystemExtensionManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(&self) -> Retained<Self>;

        #[method_id(@__retain_semantics Other sharedManager)]
        pub unsafe fn sharedManager() -> Retained<OSSystemExtensionManager>;

        #[method(submitRequest:)]
        pub unsafe fn submitRequest(&self, request: &OSSystemExtensionRequest);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl OSSystemExtensionManager {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new_class() -> Retained<Self>;
    }
);
