//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum ASAuthorizationProviderExtensionKeyType {
        #[doc(alias = "ASAuthorizationProviderExtensionKeyTypeUserDeviceSigning")]
        UserDeviceSigning = 1,
        #[doc(alias = "ASAuthorizationProviderExtensionKeyTypeUserDeviceEncryption")]
        UserDeviceEncryption = 2,
        #[doc(alias = "ASAuthorizationProviderExtensionKeyTypeUserSecureEnclaveKey")]
        UserSecureEnclaveKey = 3,
        #[doc(alias = "ASAuthorizationProviderExtensionKeyTypeSharedDeviceSigning")]
        SharedDeviceSigning = 4,
        #[doc(alias = "ASAuthorizationProviderExtensionKeyTypeSharedDeviceEncryption")]
        SharedDeviceEncryption = 5,
        #[doc(alias = "ASAuthorizationProviderExtensionKeyTypeCurrentDeviceSigning")]
        CurrentDeviceSigning = 10,
        #[doc(alias = "ASAuthorizationProviderExtensionKeyTypeCurrentDeviceEncryption")]
        CurrentDeviceEncryption = 11,
        #[doc(alias = "ASAuthorizationProviderExtensionKeyTypeUserSmartCard")]
        UserSmartCard = 20,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASAuthorizationProviderExtensionLoginManager")]
    pub struct ASAuthorizationProviderExtensionLoginManager;

    #[cfg(feature = "AuthenticationServices_ASAuthorizationProviderExtensionLoginManager")]
    unsafe impl ClassType for ASAuthorizationProviderExtensionLoginManager {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AuthenticationServices_ASAuthorizationProviderExtensionLoginManager")]
unsafe impl NSObjectProtocol for ASAuthorizationProviderExtensionLoginManager {}

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASAuthorizationProviderExtensionLoginManager")]
    unsafe impl ASAuthorizationProviderExtensionLoginManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method(isDeviceRegistered)]
        pub unsafe fn isDeviceRegistered(&self) -> bool;

        #[method(isUserRegistered)]
        pub unsafe fn isUserRegistered(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other registrationToken)]
        pub unsafe fn registrationToken(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other extensionData)]
        pub unsafe fn extensionData(&self) -> Id<NSDictionary>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other loginUserName)]
        pub unsafe fn loginUserName(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setLoginUserName:)]
        pub unsafe fn setLoginUserName(&self, login_user_name: Option<&NSString>);

        #[cfg(
            feature = "AuthenticationServices_ASAuthorizationProviderExtensionUserLoginConfiguration"
        )]
        #[method_id(@__retain_semantics Other userLoginConfiguration)]
        pub unsafe fn userLoginConfiguration(
            &self,
        ) -> Option<Id<ASAuthorizationProviderExtensionUserLoginConfiguration>>;

        #[cfg(all(
            feature = "AuthenticationServices_ASAuthorizationProviderExtensionUserLoginConfiguration",
            feature = "Foundation_NSError"
        ))]
        #[method(saveUserLoginConfiguration:error:_)]
        pub unsafe fn saveUserLoginConfiguration_error(
            &self,
            user_login_configuration: &ASAuthorizationProviderExtensionUserLoginConfiguration,
        ) -> Result<(), Id<NSError>>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other ssoTokens)]
        pub unsafe fn ssoTokens(&self) -> Option<Id<NSDictionary>>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method(setSsoTokens:)]
        pub unsafe fn setSsoTokens(&self, sso_tokens: Option<&NSDictionary>);

        #[cfg(
            feature = "AuthenticationServices_ASAuthorizationProviderExtensionLoginConfiguration"
        )]
        #[method_id(@__retain_semantics Other loginConfiguration)]
        pub unsafe fn loginConfiguration(
            &self,
        ) -> Option<Id<ASAuthorizationProviderExtensionLoginConfiguration>>;

        #[cfg(all(
            feature = "AuthenticationServices_ASAuthorizationProviderExtensionLoginConfiguration",
            feature = "Foundation_NSError"
        ))]
        #[method(saveLoginConfiguration:error:_)]
        pub unsafe fn saveLoginConfiguration_error(
            &self,
            login_configuration: &ASAuthorizationProviderExtensionLoginConfiguration,
        ) -> Result<(), Id<NSError>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(userNeedsReauthenticationWithCompletion:)]
        pub unsafe fn userNeedsReauthenticationWithCompletion(
            &self,
            completion: &Block<dyn Fn(*mut NSError)>,
        );

        #[method(deviceRegistrationsNeedsRepair)]
        pub unsafe fn deviceRegistrationsNeedsRepair(&self);

        #[method(userRegistrationsNeedsRepair)]
        pub unsafe fn userRegistrationsNeedsRepair(&self);

        #[method(decryptionKeysNeedRepair)]
        pub unsafe fn decryptionKeysNeedRepair(&self);

        #[method(resetKeys)]
        pub unsafe fn resetKeys(&self);

        #[method(resetDeviceKeys)]
        pub unsafe fn resetDeviceKeys(&self);

        #[method(resetUserSecureEnclaveKey)]
        pub unsafe fn resetUserSecureEnclaveKey(&self);

        #[cfg(feature = "Foundation_NSError")]
        #[method(presentRegistrationViewControllerWithCompletion:)]
        pub unsafe fn presentRegistrationViewControllerWithCompletion(
            &self,
            completion: &Block<dyn Fn(*mut NSError)>,
        );
    }
);
