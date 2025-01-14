//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationproviderextensionkerberosmapping?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationProviderExtensionKerberosMapping;
);

unsafe impl NSObjectProtocol for ASAuthorizationProviderExtensionKerberosMapping {}

extern_methods!(
    unsafe impl ASAuthorizationProviderExtensionKerberosMapping {
        #[method_id(@__retain_semantics Other ticketKeyPath)]
        pub unsafe fn ticketKeyPath(&self) -> Option<Retained<NSString>>;

        #[method(setTicketKeyPath:)]
        pub unsafe fn setTicketKeyPath(&self, ticket_key_path: Option<&NSString>);

        #[method_id(@__retain_semantics Other messageBufferKeyName)]
        pub unsafe fn messageBufferKeyName(&self) -> Option<Retained<NSString>>;

        #[method(setMessageBufferKeyName:)]
        pub unsafe fn setMessageBufferKeyName(&self, message_buffer_key_name: Option<&NSString>);

        #[method_id(@__retain_semantics Other realmKeyName)]
        pub unsafe fn realmKeyName(&self) -> Option<Retained<NSString>>;

        #[method(setRealmKeyName:)]
        pub unsafe fn setRealmKeyName(&self, realm_key_name: Option<&NSString>);

        #[method_id(@__retain_semantics Other serviceNameKeyName)]
        pub unsafe fn serviceNameKeyName(&self) -> Option<Retained<NSString>>;

        #[method(setServiceNameKeyName:)]
        pub unsafe fn setServiceNameKeyName(&self, service_name_key_name: Option<&NSString>);

        #[method_id(@__retain_semantics Other clientNameKeyName)]
        pub unsafe fn clientNameKeyName(&self) -> Option<Retained<NSString>>;

        #[method(setClientNameKeyName:)]
        pub unsafe fn setClientNameKeyName(&self, client_name_key_name: Option<&NSString>);

        #[method_id(@__retain_semantics Other encryptionKeyTypeKeyName)]
        pub unsafe fn encryptionKeyTypeKeyName(&self) -> Option<Retained<NSString>>;

        #[method(setEncryptionKeyTypeKeyName:)]
        pub unsafe fn setEncryptionKeyTypeKeyName(
            &self,
            encryption_key_type_key_name: Option<&NSString>,
        );

        #[method_id(@__retain_semantics Other sessionKeyKeyName)]
        pub unsafe fn sessionKeyKeyName(&self) -> Option<Retained<NSString>>;

        #[method(setSessionKeyKeyName:)]
        pub unsafe fn setSessionKeyKeyName(&self, session_key_key_name: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ASAuthorizationProviderExtensionKerberosMapping {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationproviderextensionfederationtype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ASAuthorizationProviderExtensionFederationType(pub NSInteger);
impl ASAuthorizationProviderExtensionFederationType {
    #[doc(alias = "ASAuthorizationProviderExtensionFederationTypeNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "ASAuthorizationProviderExtensionFederationTypeWSTrust")]
    pub const WSTrust: Self = Self(1);
    #[doc(alias = "ASAuthorizationProviderExtensionFederationTypeDynamicWSTrust")]
    pub const DynamicWSTrust: Self = Self(2);
}

unsafe impl Encode for ASAuthorizationProviderExtensionFederationType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for ASAuthorizationProviderExtensionFederationType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationproviderextensionusersecureenclavekeybiometricpolicy?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ASAuthorizationProviderExtensionUserSecureEnclaveKeyBiometricPolicy(pub NSUInteger);
bitflags::bitflags! {
    impl ASAuthorizationProviderExtensionUserSecureEnclaveKeyBiometricPolicy: NSUInteger {
        #[doc(alias = "ASAuthorizationProviderExtensionUserSecureEnclaveKeyBiometricPolicyNone")]
        const None = 0;
        #[doc(alias = "ASAuthorizationProviderExtensionUserSecureEnclaveKeyBiometricPolicyTouchIDOrWatchCurrentSet")]
        const TouchIDOrWatchCurrentSet = 1<<0;
        #[doc(alias = "ASAuthorizationProviderExtensionUserSecureEnclaveKeyBiometricPolicyTouchIDOrWatchAny")]
        const TouchIDOrWatchAny = 1<<1;
        #[doc(alias = "ASAuthorizationProviderExtensionUserSecureEnclaveKeyBiometricPolicyReuseDuringUnlock")]
        const ReuseDuringUnlock = 1<<2;
        #[doc(alias = "ASAuthorizationProviderExtensionUserSecureEnclaveKeyBiometricPolicyPasswordFallback")]
        const PasswordFallback = 1<<3;
    }
}

unsafe impl Encode for ASAuthorizationProviderExtensionUserSecureEnclaveKeyBiometricPolicy {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for ASAuthorizationProviderExtensionUserSecureEnclaveKeyBiometricPolicy {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationproviderextensionencryptionalgorithm?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
pub type ASAuthorizationProviderExtensionEncryptionAlgorithm = NSNumber;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationproviderextensionencryptionalgorithmecdhe_a256gcm?language=objc)
    pub static ASAuthorizationProviderExtensionEncryptionAlgorithmECDHE_A256GCM:
        &'static ASAuthorizationProviderExtensionEncryptionAlgorithm;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationproviderextensionencryptionalgorithmhpke_p256_sha256_aes_gcm_256?language=objc)
    pub static ASAuthorizationProviderExtensionEncryptionAlgorithmHPKE_P256_SHA256_AES_GCM_256:
        &'static ASAuthorizationProviderExtensionEncryptionAlgorithm;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationproviderextensionencryptionalgorithmhpke_p384_sha384_aes_gcm_256?language=objc)
    pub static ASAuthorizationProviderExtensionEncryptionAlgorithmHPKE_P384_SHA384_AES_GCM_256:
        &'static ASAuthorizationProviderExtensionEncryptionAlgorithm;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationproviderextensionencryptionalgorithmhpke_curve25519_sha256_chachapoly?language=objc)
    pub static ASAuthorizationProviderExtensionEncryptionAlgorithmHPKE_Curve25519_SHA256_ChachaPoly:
        &'static ASAuthorizationProviderExtensionEncryptionAlgorithm;
}

/// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationproviderextensionsigningalgorithm?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
pub type ASAuthorizationProviderExtensionSigningAlgorithm = NSNumber;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationproviderextensionsigningalgorithmes256?language=objc)
    pub static ASAuthorizationProviderExtensionSigningAlgorithmES256:
        &'static ASAuthorizationProviderExtensionSigningAlgorithm;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationproviderextensionsigningalgorithmes384?language=objc)
    pub static ASAuthorizationProviderExtensionSigningAlgorithmES384:
        &'static ASAuthorizationProviderExtensionSigningAlgorithm;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationproviderextensionsigningalgorithmed25519?language=objc)
    pub static ASAuthorizationProviderExtensionSigningAlgorithmEd25519:
        &'static ASAuthorizationProviderExtensionSigningAlgorithm;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationproviderextensionloginconfiguration?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationProviderExtensionLoginConfiguration;
);

unsafe impl NSObjectProtocol for ASAuthorizationProviderExtensionLoginConfiguration {}

extern_methods!(
    unsafe impl ASAuthorizationProviderExtensionLoginConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithClientID:issuer:tokenEndpointURL:jwksEndpointURL:audience:)]
        pub unsafe fn initWithClientID_issuer_tokenEndpointURL_jwksEndpointURL_audience(
            this: Allocated<Self>,
            client_id: &NSString,
            issuer: &NSString,
            token_endpoint_url: &NSURL,
            jwks_endpoint_url: &NSURL,
            audience: Option<&NSString>,
        ) -> Retained<Self>;

        #[cfg(feature = "block2")]
        #[method(configurationWithOpenIDConfigurationURL:clientID:issuer:completion:)]
        pub unsafe fn configurationWithOpenIDConfigurationURL_clientID_issuer_completion(
            open_id_configuration_url: &NSURL,
            client_id: &NSString,
            issuer: Option<&NSString>,
            completion: &block2::Block<
                dyn Fn(*mut ASAuthorizationProviderExtensionLoginConfiguration, *mut NSError),
            >,
        );

        #[method_id(@__retain_semantics Other invalidCredentialPredicate)]
        pub unsafe fn invalidCredentialPredicate(&self) -> Option<Retained<NSString>>;

        #[method(setInvalidCredentialPredicate:)]
        pub unsafe fn setInvalidCredentialPredicate(
            &self,
            invalid_credential_predicate: Option<&NSString>,
        );

        #[method_id(@__retain_semantics Other accountDisplayName)]
        pub unsafe fn accountDisplayName(&self) -> Option<Retained<NSString>>;

        #[method(setAccountDisplayName:)]
        pub unsafe fn setAccountDisplayName(&self, account_display_name: Option<&NSString>);

        #[method_id(@__retain_semantics Other clientID)]
        pub unsafe fn clientID(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other issuer)]
        pub unsafe fn issuer(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other audience)]
        pub unsafe fn audience(&self) -> Retained<NSString>;

        #[method(setAudience:)]
        pub unsafe fn setAudience(&self, audience: &NSString);

        #[method_id(@__retain_semantics Other tokenEndpointURL)]
        pub unsafe fn tokenEndpointURL(&self) -> Retained<NSURL>;

        #[method(setTokenEndpointURL:)]
        pub unsafe fn setTokenEndpointURL(&self, token_endpoint_url: &NSURL);

        #[method_id(@__retain_semantics Other jwksEndpointURL)]
        pub unsafe fn jwksEndpointURL(&self) -> Retained<NSURL>;

        #[method(setJwksEndpointURL:)]
        pub unsafe fn setJwksEndpointURL(&self, jwks_endpoint_url: &NSURL);

        #[method_id(@__retain_semantics Other jwksTrustedRootCertificates)]
        pub unsafe fn jwksTrustedRootCertificates(&self) -> Retained<NSArray>;

        #[method(setJwksTrustedRootCertificates:)]
        pub unsafe fn setJwksTrustedRootCertificates(
            &self,
            jwks_trusted_root_certificates: &NSArray,
        );

        #[method_id(@__retain_semantics Other deviceContext)]
        pub unsafe fn deviceContext(&self) -> Option<Retained<NSData>>;

        #[method(setDeviceContext:)]
        pub unsafe fn setDeviceContext(&self, device_context: Option<&NSData>);

        #[method(userSecureEnclaveKeyBiometricPolicy)]
        pub unsafe fn userSecureEnclaveKeyBiometricPolicy(
            &self,
        ) -> ASAuthorizationProviderExtensionUserSecureEnclaveKeyBiometricPolicy;

        #[method(setUserSecureEnclaveKeyBiometricPolicy:)]
        pub unsafe fn setUserSecureEnclaveKeyBiometricPolicy(
            &self,
            user_secure_enclave_key_biometric_policy: ASAuthorizationProviderExtensionUserSecureEnclaveKeyBiometricPolicy,
        );

        #[method_id(@__retain_semantics Other nonceEndpointURL)]
        pub unsafe fn nonceEndpointURL(&self) -> Retained<NSURL>;

        #[method(setNonceEndpointURL:)]
        pub unsafe fn setNonceEndpointURL(&self, nonce_endpoint_url: &NSURL);

        #[method_id(@__retain_semantics Other nonceResponseKeypath)]
        pub unsafe fn nonceResponseKeypath(&self) -> Retained<NSString>;

        #[method(setNonceResponseKeypath:)]
        pub unsafe fn setNonceResponseKeypath(&self, nonce_response_keypath: &NSString);

        #[method_id(@__retain_semantics Other serverNonceClaimName)]
        pub unsafe fn serverNonceClaimName(&self) -> Retained<NSString>;

        #[method(setServerNonceClaimName:)]
        pub unsafe fn setServerNonceClaimName(&self, server_nonce_claim_name: &NSString);

        #[method_id(@__retain_semantics Other customNonceRequestValues)]
        pub unsafe fn customNonceRequestValues(&self) -> Retained<NSArray<NSURLQueryItem>>;

        #[method(setCustomNonceRequestValues:)]
        pub unsafe fn setCustomNonceRequestValues(
            &self,
            custom_nonce_request_values: &NSArray<NSURLQueryItem>,
        );

        #[method(setCustomAssertionRequestHeaderClaims:returningError:_)]
        pub unsafe fn setCustomAssertionRequestHeaderClaims_returningError(
            &self,
            claims: &NSDictionary<NSString, AnyObject>,
        ) -> Result<(), Retained<NSError>>;

        #[method(setCustomAssertionRequestBodyClaims:returningError:_)]
        pub unsafe fn setCustomAssertionRequestBodyClaims_returningError(
            &self,
            claims: &NSDictionary<NSString, AnyObject>,
        ) -> Result<(), Retained<NSError>>;

        #[method_id(@__retain_semantics Other additionalScopes)]
        pub unsafe fn additionalScopes(&self) -> Retained<NSString>;

        #[method(setAdditionalScopes:)]
        pub unsafe fn setAdditionalScopes(&self, additional_scopes: &NSString);

        #[method_id(@__retain_semantics Other additionalAuthorizationScopes)]
        pub unsafe fn additionalAuthorizationScopes(&self) -> Option<Retained<NSString>>;

        #[method(setAdditionalAuthorizationScopes:)]
        pub unsafe fn setAdditionalAuthorizationScopes(
            &self,
            additional_authorization_scopes: Option<&NSString>,
        );

        #[method(includePreviousRefreshTokenInLoginRequest)]
        pub unsafe fn includePreviousRefreshTokenInLoginRequest(&self) -> bool;

        #[method(setIncludePreviousRefreshTokenInLoginRequest:)]
        pub unsafe fn setIncludePreviousRefreshTokenInLoginRequest(
            &self,
            include_previous_refresh_token_in_login_request: bool,
        );

        #[method_id(@__retain_semantics Other previousRefreshTokenClaimName)]
        pub unsafe fn previousRefreshTokenClaimName(&self) -> Retained<NSString>;

        #[method(setPreviousRefreshTokenClaimName:)]
        pub unsafe fn setPreviousRefreshTokenClaimName(
            &self,
            previous_refresh_token_claim_name: &NSString,
        );

        #[method_id(@__retain_semantics Other customRequestJWTParameterName)]
        pub unsafe fn customRequestJWTParameterName(&self) -> Option<Retained<NSString>>;

        #[method(setCustomRequestJWTParameterName:)]
        pub unsafe fn setCustomRequestJWTParameterName(
            &self,
            custom_request_jwt_parameter_name: Option<&NSString>,
        );

        #[method_id(@__retain_semantics Other customLoginRequestValues)]
        pub unsafe fn customLoginRequestValues(&self) -> Retained<NSArray<NSURLQueryItem>>;

        #[method(setCustomLoginRequestValues:)]
        pub unsafe fn setCustomLoginRequestValues(
            &self,
            custom_login_request_values: &NSArray<NSURLQueryItem>,
        );

        #[method(setCustomLoginRequestHeaderClaims:returningError:_)]
        pub unsafe fn setCustomLoginRequestHeaderClaims_returningError(
            &self,
            claims: &NSDictionary<NSString, AnyObject>,
        ) -> Result<(), Retained<NSError>>;

        #[method(setCustomLoginRequestBodyClaims:returningError:_)]
        pub unsafe fn setCustomLoginRequestBodyClaims_returningError(
            &self,
            claims: &NSDictionary<NSString, AnyObject>,
        ) -> Result<(), Retained<NSError>>;

        #[method_id(@__retain_semantics Other uniqueIdentifierClaimName)]
        pub unsafe fn uniqueIdentifierClaimName(&self) -> Option<Retained<NSString>>;

        #[method(setUniqueIdentifierClaimName:)]
        pub unsafe fn setUniqueIdentifierClaimName(
            &self,
            unique_identifier_claim_name: Option<&NSString>,
        );

        #[method_id(@__retain_semantics Other groupRequestClaimName)]
        pub unsafe fn groupRequestClaimName(&self) -> Option<Retained<NSString>>;

        #[method(setGroupRequestClaimName:)]
        pub unsafe fn setGroupRequestClaimName(&self, group_request_claim_name: Option<&NSString>);

        #[method_id(@__retain_semantics Other groupResponseClaimName)]
        pub unsafe fn groupResponseClaimName(&self) -> Option<Retained<NSString>>;

        #[method(setGroupResponseClaimName:)]
        pub unsafe fn setGroupResponseClaimName(
            &self,
            group_response_claim_name: Option<&NSString>,
        );

        #[method_id(@__retain_semantics Other kerberosTicketMappings)]
        pub unsafe fn kerberosTicketMappings(
            &self,
        ) -> Retained<NSArray<ASAuthorizationProviderExtensionKerberosMapping>>;

        #[method(setKerberosTicketMappings:)]
        pub unsafe fn setKerberosTicketMappings(
            &self,
            kerberos_ticket_mappings: &NSArray<ASAuthorizationProviderExtensionKerberosMapping>,
        );

        #[method_id(@__retain_semantics Other refreshEndpointURL)]
        pub unsafe fn refreshEndpointURL(&self) -> Option<Retained<NSURL>>;

        #[method(setRefreshEndpointURL:)]
        pub unsafe fn setRefreshEndpointURL(&self, refresh_endpoint_url: Option<&NSURL>);

        #[method_id(@__retain_semantics Other customRefreshRequestValues)]
        pub unsafe fn customRefreshRequestValues(&self) -> Retained<NSArray<NSURLQueryItem>>;

        #[method(setCustomRefreshRequestValues:)]
        pub unsafe fn setCustomRefreshRequestValues(
            &self,
            custom_refresh_request_values: &NSArray<NSURLQueryItem>,
        );

        #[method(setCustomRefreshRequestHeaderClaims:returningError:_)]
        pub unsafe fn setCustomRefreshRequestHeaderClaims_returningError(
            &self,
            claims: &NSDictionary<NSString, AnyObject>,
        ) -> Result<(), Retained<NSError>>;

        #[method(setCustomRefreshRequestBodyClaims:returningError:_)]
        pub unsafe fn setCustomRefreshRequestBodyClaims_returningError(
            &self,
            claims: &NSDictionary<NSString, AnyObject>,
        ) -> Result<(), Retained<NSError>>;

        #[method(federationType)]
        pub unsafe fn federationType(&self) -> ASAuthorizationProviderExtensionFederationType;

        #[method(setFederationType:)]
        pub unsafe fn setFederationType(
            &self,
            federation_type: ASAuthorizationProviderExtensionFederationType,
        );

        #[method_id(@__retain_semantics Other federationRequestURN)]
        pub unsafe fn federationRequestURN(&self) -> Option<Retained<NSString>>;

        #[method(setFederationRequestURN:)]
        pub unsafe fn setFederationRequestURN(&self, federation_request_urn: Option<&NSString>);

        #[method_id(@__retain_semantics Other federationMEXURL)]
        pub unsafe fn federationMEXURL(&self) -> Option<Retained<NSURL>>;

        #[method(setFederationMEXURL:)]
        pub unsafe fn setFederationMEXURL(&self, federation_mexurl: Option<&NSURL>);

        #[method_id(@__retain_semantics Other federationUserPreauthenticationURL)]
        pub unsafe fn federationUserPreauthenticationURL(&self) -> Option<Retained<NSURL>>;

        #[method(setFederationUserPreauthenticationURL:)]
        pub unsafe fn setFederationUserPreauthenticationURL(
            &self,
            federation_user_preauthentication_url: Option<&NSURL>,
        );

        #[method_id(@__retain_semantics Other federationMEXURLKeypath)]
        pub unsafe fn federationMEXURLKeypath(&self) -> Option<Retained<NSString>>;

        #[method(setFederationMEXURLKeypath:)]
        pub unsafe fn setFederationMEXURLKeypath(
            &self,
            federation_mexurl_keypath: Option<&NSString>,
        );

        #[method_id(@__retain_semantics Other federationPredicate)]
        pub unsafe fn federationPredicate(&self) -> Option<Retained<NSString>>;

        #[method(setFederationPredicate:)]
        pub unsafe fn setFederationPredicate(&self, federation_predicate: Option<&NSString>);

        #[method_id(@__retain_semantics Other customFederationUserPreauthenticationRequestValues)]
        pub unsafe fn customFederationUserPreauthenticationRequestValues(
            &self,
        ) -> Retained<NSArray<NSURLQueryItem>>;

        #[method(setCustomFederationUserPreauthenticationRequestValues:)]
        pub unsafe fn setCustomFederationUserPreauthenticationRequestValues(
            &self,
            custom_federation_user_preauthentication_request_values: &NSArray<NSURLQueryItem>,
        );

        #[method_id(@__retain_semantics Other loginRequestEncryptionAPVPrefix)]
        pub unsafe fn loginRequestEncryptionAPVPrefix(&self) -> Option<Retained<NSData>>;

        #[method(setLoginRequestEncryptionAPVPrefix:)]
        pub unsafe fn setLoginRequestEncryptionAPVPrefix(
            &self,
            login_request_encryption_apv_prefix: Option<&NSData>,
        );

        #[method_id(@__retain_semantics Other loginRequestEncryptionAlgorithm)]
        pub unsafe fn loginRequestEncryptionAlgorithm(
            &self,
        ) -> Retained<ASAuthorizationProviderExtensionEncryptionAlgorithm>;

        #[method(setLoginRequestEncryptionAlgorithm:)]
        pub unsafe fn setLoginRequestEncryptionAlgorithm(
            &self,
            login_request_encryption_algorithm: &ASAuthorizationProviderExtensionEncryptionAlgorithm,
        );

        #[method_id(@__retain_semantics Other loginRequestHPKEPreSharedKey)]
        pub unsafe fn loginRequestHPKEPreSharedKey(&self) -> Option<Retained<NSData>>;

        #[method(setLoginRequestHPKEPreSharedKey:)]
        pub unsafe fn setLoginRequestHPKEPreSharedKey(
            &self,
            login_request_hpke_pre_shared_key: Option<&NSData>,
        );

        #[method_id(@__retain_semantics Other loginRequestHPKEPreSharedKeyID)]
        pub unsafe fn loginRequestHPKEPreSharedKeyID(&self) -> Option<Retained<NSData>>;

        #[method(setLoginRequestHPKEPreSharedKeyID:)]
        pub unsafe fn setLoginRequestHPKEPreSharedKeyID(
            &self,
            login_request_hpke_pre_shared_key_id: Option<&NSData>,
        );

        #[method_id(@__retain_semantics Other keyEndpointURL)]
        pub unsafe fn keyEndpointURL(&self) -> Option<Retained<NSURL>>;

        #[method(setKeyEndpointURL:)]
        pub unsafe fn setKeyEndpointURL(&self, key_endpoint_url: Option<&NSURL>);

        #[method_id(@__retain_semantics Other customKeyExchangeRequestValues)]
        pub unsafe fn customKeyExchangeRequestValues(&self) -> Retained<NSArray<NSURLQueryItem>>;

        #[method(setCustomKeyExchangeRequestValues:)]
        pub unsafe fn setCustomKeyExchangeRequestValues(
            &self,
            custom_key_exchange_request_values: &NSArray<NSURLQueryItem>,
        );

        #[method(setCustomKeyExchangeRequestHeaderClaims:returningError:_)]
        pub unsafe fn setCustomKeyExchangeRequestHeaderClaims_returningError(
            &self,
            claims: &NSDictionary<NSString, AnyObject>,
        ) -> Result<(), Retained<NSError>>;

        #[method(setCustomKeyExchangeRequestBodyClaims:returningError:_)]
        pub unsafe fn setCustomKeyExchangeRequestBodyClaims_returningError(
            &self,
            claims: &NSDictionary<NSString, AnyObject>,
        ) -> Result<(), Retained<NSError>>;

        #[method_id(@__retain_semantics Other customKeyRequestValues)]
        pub unsafe fn customKeyRequestValues(&self) -> Retained<NSArray<NSURLQueryItem>>;

        #[method(setCustomKeyRequestValues:)]
        pub unsafe fn setCustomKeyRequestValues(
            &self,
            custom_key_request_values: &NSArray<NSURLQueryItem>,
        );

        #[method(setCustomKeyRequestHeaderClaims:returningError:_)]
        pub unsafe fn setCustomKeyRequestHeaderClaims_returningError(
            &self,
            claims: &NSDictionary<NSString, AnyObject>,
        ) -> Result<(), Retained<NSError>>;

        #[method(setCustomKeyRequestBodyClaims:returningError:_)]
        pub unsafe fn setCustomKeyRequestBodyClaims_returningError(
            &self,
            claims: &NSDictionary<NSString, AnyObject>,
        ) -> Result<(), Retained<NSError>>;

        #[method_id(@__retain_semantics Other hpkePreSharedKey)]
        pub unsafe fn hpkePreSharedKey(&self) -> Option<Retained<NSData>>;

        #[method(setHpkePreSharedKey:)]
        pub unsafe fn setHpkePreSharedKey(&self, hpke_pre_shared_key: Option<&NSData>);

        #[method_id(@__retain_semantics Other hpkePreSharedKeyID)]
        pub unsafe fn hpkePreSharedKeyID(&self) -> Option<Retained<NSData>>;

        #[method(setHpkePreSharedKeyID:)]
        pub unsafe fn setHpkePreSharedKeyID(&self, hpke_pre_shared_key_id: Option<&NSData>);
    }
);
