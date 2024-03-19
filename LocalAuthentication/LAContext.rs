//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::LocalAuthentication::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum LAPolicy {
        #[doc(alias = "LAPolicyDeviceOwnerAuthenticationWithBiometrics")]
        DeviceOwnerAuthenticationWithBiometrics = 1,
        #[doc(alias = "LAPolicyDeviceOwnerAuthentication")]
        DeviceOwnerAuthentication = 2,
        #[doc(alias = "LAPolicyDeviceOwnerAuthenticationWithWatch")]
        DeviceOwnerAuthenticationWithWatch = 3,
        #[doc(alias = "LAPolicyDeviceOwnerAuthenticationWithBiometricsOrWatch")]
        DeviceOwnerAuthenticationWithBiometricsOrWatch = 4,
        #[doc(alias = "LAPolicyDeviceOwnerAuthenticationWithWristDetection")]
        DeviceOwnerAuthenticationWithWristDetection = 5,
    }
);

extern "C" {
    #[cfg(feature = "Foundation_NSDate")]
    pub static LATouchIDAuthenticationMaximumAllowableReuseDuration: NSTimeInterval;
}

ns_enum!(
    #[underlying(NSInteger)]
    pub enum LACredentialType {
        #[doc(alias = "LACredentialTypeApplicationPassword")]
        ApplicationPassword = 0,
        #[doc(alias = "LACredentialTypeSmartCardPIN")]
        SmartCardPIN = -3,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum LAAccessControlOperation {
        #[doc(alias = "LAAccessControlOperationCreateItem")]
        CreateItem = 0,
        #[doc(alias = "LAAccessControlOperationUseItem")]
        UseItem = 1,
        #[doc(alias = "LAAccessControlOperationCreateKey")]
        CreateKey = 2,
        #[doc(alias = "LAAccessControlOperationUseKeySign")]
        UseKeySign = 3,
        #[doc(alias = "LAAccessControlOperationUseKeyDecrypt")]
        UseKeyDecrypt = 4,
        #[doc(alias = "LAAccessControlOperationUseKeyKeyExchange")]
        UseKeyKeyExchange = 5,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum LABiometryType {
        #[doc(alias = "LABiometryTypeNone")]
        None = 0,
        #[deprecated]
        LABiometryNone = LABiometryType::None.0,
        #[doc(alias = "LABiometryTypeTouchID")]
        TouchID = 1,
        #[doc(alias = "LABiometryTypeFaceID")]
        FaceID = 2,
        #[doc(alias = "LABiometryTypeOpticID")]
        OpticID = 4,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct LAContext;

    unsafe impl ClassType for LAContext {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for LAContext {}

extern_methods!(
    unsafe impl LAContext {
        #[cfg(feature = "Foundation_NSError")]
        #[method(canEvaluatePolicy:error:_)]
        pub unsafe fn canEvaluatePolicy_error(&self, policy: LAPolicy) -> Result<(), Id<NSError>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(evaluatePolicy:localizedReason:reply:)]
        pub unsafe fn evaluatePolicy_localizedReason_reply(
            &self,
            policy: LAPolicy,
            localized_reason: &NSString,
            reply: &Block<dyn Fn(Bool, *mut NSError)>,
        );

        #[method(invalidate)]
        pub unsafe fn invalidate(&self);

        #[cfg(feature = "Foundation_NSData")]
        #[method(setCredential:type:)]
        pub unsafe fn setCredential_type(
            &self,
            credential: Option<&NSData>,
            r#type: LACredentialType,
        ) -> bool;

        #[method(isCredentialSet:)]
        pub unsafe fn isCredentialSet(&self, r#type: LACredentialType) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedFallbackTitle)]
        pub unsafe fn localizedFallbackTitle(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLocalizedFallbackTitle:)]
        pub unsafe fn setLocalizedFallbackTitle(&self, localized_fallback_title: Option<&NSString>);

        #[cfg(feature = "Foundation_NSValue")]
        #[deprecated = "No longer supported"]
        #[method_id(@__retain_semantics Other maxBiometryFailures)]
        pub unsafe fn maxBiometryFailures(&self) -> Option<Id<NSNumber>>;

        #[cfg(feature = "Foundation_NSValue")]
        #[deprecated = "No longer supported"]
        #[method(setMaxBiometryFailures:)]
        pub unsafe fn setMaxBiometryFailures(&self, max_biometry_failures: Option<&NSNumber>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedCancelTitle)]
        pub unsafe fn localizedCancelTitle(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLocalizedCancelTitle:)]
        pub unsafe fn setLocalizedCancelTitle(&self, localized_cancel_title: Option<&NSString>);

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other evaluatedPolicyDomainState)]
        pub unsafe fn evaluatedPolicyDomainState(&self) -> Option<Id<NSData>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method(touchIDAuthenticationAllowableReuseDuration)]
        pub unsafe fn touchIDAuthenticationAllowableReuseDuration(&self) -> NSTimeInterval;

        #[cfg(feature = "Foundation_NSDate")]
        #[method(setTouchIDAuthenticationAllowableReuseDuration:)]
        pub unsafe fn setTouchIDAuthenticationAllowableReuseDuration(
            &self,
            touch_id_authentication_allowable_reuse_duration: NSTimeInterval,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedReason)]
        pub unsafe fn localizedReason(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLocalizedReason:)]
        pub unsafe fn setLocalizedReason(&self, localized_reason: &NSString);

        #[method(interactionNotAllowed)]
        pub unsafe fn interactionNotAllowed(&self) -> bool;

        #[method(setInteractionNotAllowed:)]
        pub unsafe fn setInteractionNotAllowed(&self, interaction_not_allowed: bool);

        #[method(biometryType)]
        pub unsafe fn biometryType(&self) -> LABiometryType;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl LAContext {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
