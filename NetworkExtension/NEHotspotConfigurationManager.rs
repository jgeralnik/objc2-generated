//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NEHotspotConfigurationEAPType(pub NSInteger);
impl NEHotspotConfigurationEAPType {
    #[doc(alias = "NEHotspotConfigurationEAPTypeEAPTLS")]
    pub const EAPTLS: Self = Self(13);
    #[doc(alias = "NEHotspotConfigurationEAPTypeEAPTTLS")]
    pub const EAPTTLS: Self = Self(21);
    #[doc(alias = "NEHotspotConfigurationEAPTypeEAPPEAP")]
    pub const EAPPEAP: Self = Self(25);
    #[doc(alias = "NEHotspotConfigurationEAPTypeEAPFAST")]
    pub const EAPFAST: Self = Self(43);
}

unsafe impl Encode for NEHotspotConfigurationEAPType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NEHotspotConfigurationEAPType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NEHotspotConfigurationTTLSInnerAuthenticationType(pub NSInteger);
impl NEHotspotConfigurationTTLSInnerAuthenticationType {
    pub const NEHotspotConfigurationEAPTTLSInnerAuthenticationPAP: Self = Self(0);
    pub const NEHotspotConfigurationEAPTTLSInnerAuthenticationCHAP: Self = Self(1);
    pub const NEHotspotConfigurationEAPTTLSInnerAuthenticationMSCHAP: Self = Self(2);
    pub const NEHotspotConfigurationEAPTTLSInnerAuthenticationMSCHAPv2: Self = Self(3);
    pub const NEHotspotConfigurationEAPTTLSInnerAuthenticationEAP: Self = Self(4);
}

unsafe impl Encode for NEHotspotConfigurationTTLSInnerAuthenticationType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NEHotspotConfigurationTTLSInnerAuthenticationType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NEHotspotConfigurationEAPTLSVersion(pub NSInteger);
impl NEHotspotConfigurationEAPTLSVersion {
    #[doc(alias = "NEHotspotConfigurationEAPTLSVersion_1_0")]
    pub const _1_0: Self = Self(0);
    #[doc(alias = "NEHotspotConfigurationEAPTLSVersion_1_1")]
    pub const _1_1: Self = Self(1);
    #[doc(alias = "NEHotspotConfigurationEAPTLSVersion_1_2")]
    pub const _1_2: Self = Self(2);
}

unsafe impl Encode for NEHotspotConfigurationEAPTLSVersion {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NEHotspotConfigurationEAPTLSVersion {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NEHotspotHS20Settings;

    unsafe impl ClassType for NEHotspotHS20Settings {
        type Super = NSObject;
    }
);

unsafe impl NSCoding for NEHotspotHS20Settings {}

unsafe impl NSCopying for NEHotspotHS20Settings {}

unsafe impl CopyingHelper for NEHotspotHS20Settings {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NEHotspotHS20Settings {}

unsafe impl NSSecureCoding for NEHotspotHS20Settings {}

extern_methods!(
    unsafe impl NEHotspotHS20Settings {
        #[method_id(@__retain_semantics Other domainName)]
        pub unsafe fn domainName(&self) -> Retained<NSString>;

        #[method(isRoamingEnabled)]
        pub unsafe fn isRoamingEnabled(&self) -> bool;

        #[method(setRoamingEnabled:)]
        pub unsafe fn setRoamingEnabled(&self, roaming_enabled: bool);

        #[method_id(@__retain_semantics Other roamingConsortiumOIs)]
        pub unsafe fn roamingConsortiumOIs(&self) -> Retained<NSArray<NSString>>;

        #[method(setRoamingConsortiumOIs:)]
        pub unsafe fn setRoamingConsortiumOIs(&self, roaming_consortium_o_is: &NSArray<NSString>);

        #[method_id(@__retain_semantics Other naiRealmNames)]
        pub unsafe fn naiRealmNames(&self) -> Retained<NSArray<NSString>>;

        #[method(setNaiRealmNames:)]
        pub unsafe fn setNaiRealmNames(&self, nai_realm_names: &NSArray<NSString>);

        #[method_id(@__retain_semantics Other MCCAndMNCs)]
        pub unsafe fn MCCAndMNCs(&self) -> Retained<NSArray<NSString>>;

        #[method(setMCCAndMNCs:)]
        pub unsafe fn setMCCAndMNCs(&self, mcc_and_mn_cs: &NSArray<NSString>);

        #[method_id(@__retain_semantics Init initWithDomainName:roamingEnabled:)]
        pub unsafe fn initWithDomainName_roamingEnabled(
            this: Allocated<Self>,
            domain_name: &NSString,
            roaming_enabled: bool,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NEHotspotHS20Settings {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NEHotspotEAPSettings;

    unsafe impl ClassType for NEHotspotEAPSettings {
        type Super = NSObject;
    }
);

unsafe impl NSCoding for NEHotspotEAPSettings {}

unsafe impl NSCopying for NEHotspotEAPSettings {}

unsafe impl CopyingHelper for NEHotspotEAPSettings {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NEHotspotEAPSettings {}

unsafe impl NSSecureCoding for NEHotspotEAPSettings {}

extern_methods!(
    unsafe impl NEHotspotEAPSettings {
        #[method_id(@__retain_semantics Other supportedEAPTypes)]
        pub unsafe fn supportedEAPTypes(&self) -> Retained<NSArray<NSNumber>>;

        #[method(setSupportedEAPTypes:)]
        pub unsafe fn setSupportedEAPTypes(&self, supported_eap_types: &NSArray<NSNumber>);

        #[method_id(@__retain_semantics Other username)]
        pub unsafe fn username(&self) -> Retained<NSString>;

        #[method(setUsername:)]
        pub unsafe fn setUsername(&self, username: &NSString);

        #[method_id(@__retain_semantics Other outerIdentity)]
        pub unsafe fn outerIdentity(&self) -> Retained<NSString>;

        #[method(setOuterIdentity:)]
        pub unsafe fn setOuterIdentity(&self, outer_identity: &NSString);

        #[method(ttlsInnerAuthenticationType)]
        pub unsafe fn ttlsInnerAuthenticationType(
            &self,
        ) -> NEHotspotConfigurationTTLSInnerAuthenticationType;

        #[method(setTtlsInnerAuthenticationType:)]
        pub unsafe fn setTtlsInnerAuthenticationType(
            &self,
            ttls_inner_authentication_type: NEHotspotConfigurationTTLSInnerAuthenticationType,
        );

        #[method_id(@__retain_semantics Other password)]
        pub unsafe fn password(&self) -> Retained<NSString>;

        #[method(setPassword:)]
        pub unsafe fn setPassword(&self, password: &NSString);

        #[method_id(@__retain_semantics Other trustedServerNames)]
        pub unsafe fn trustedServerNames(&self) -> Retained<NSArray<NSString>>;

        #[method(setTrustedServerNames:)]
        pub unsafe fn setTrustedServerNames(&self, trusted_server_names: &NSArray<NSString>);

        #[method(isTLSClientCertificateRequired)]
        pub unsafe fn isTLSClientCertificateRequired(&self) -> bool;

        #[method(setTlsClientCertificateRequired:)]
        pub unsafe fn setTlsClientCertificateRequired(&self, tls_client_certificate_required: bool);

        #[method(preferredTLSVersion)]
        pub unsafe fn preferredTLSVersion(&self) -> NEHotspotConfigurationEAPTLSVersion;

        #[method(setPreferredTLSVersion:)]
        pub unsafe fn setPreferredTLSVersion(
            &self,
            preferred_tls_version: NEHotspotConfigurationEAPTLSVersion,
        );

        #[method(setTrustedServerCertificates:)]
        pub unsafe fn setTrustedServerCertificates(&self, certificates: &NSArray) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NEHotspotEAPSettings {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NEHotspotConfiguration;

    unsafe impl ClassType for NEHotspotConfiguration {
        type Super = NSObject;
    }
);

unsafe impl NSCoding for NEHotspotConfiguration {}

unsafe impl NSCopying for NEHotspotConfiguration {}

unsafe impl CopyingHelper for NEHotspotConfiguration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NEHotspotConfiguration {}

unsafe impl NSSecureCoding for NEHotspotConfiguration {}

extern_methods!(
    unsafe impl NEHotspotConfiguration {
        #[method_id(@__retain_semantics Other SSID)]
        pub unsafe fn SSID(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other SSIDPrefix)]
        pub unsafe fn SSIDPrefix(&self) -> Retained<NSString>;

        #[method(joinOnce)]
        pub unsafe fn joinOnce(&self) -> bool;

        #[method(setJoinOnce:)]
        pub unsafe fn setJoinOnce(&self, join_once: bool);

        #[method_id(@__retain_semantics Other lifeTimeInDays)]
        pub unsafe fn lifeTimeInDays(&self) -> Retained<NSNumber>;

        #[method(setLifeTimeInDays:)]
        pub unsafe fn setLifeTimeInDays(&self, life_time_in_days: &NSNumber);

        #[method(hidden)]
        pub unsafe fn hidden(&self) -> bool;

        #[method(setHidden:)]
        pub unsafe fn setHidden(&self, hidden: bool);

        #[method_id(@__retain_semantics Init initWithSSID:)]
        pub unsafe fn initWithSSID(this: Allocated<Self>, ssid: &NSString) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithSSID:passphrase:isWEP:)]
        pub unsafe fn initWithSSID_passphrase_isWEP(
            this: Allocated<Self>,
            ssid: &NSString,
            passphrase: &NSString,
            is_wep: bool,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithSSID:eapSettings:)]
        pub unsafe fn initWithSSID_eapSettings(
            this: Allocated<Self>,
            ssid: &NSString,
            eap_settings: &NEHotspotEAPSettings,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithHS20Settings:eapSettings:)]
        pub unsafe fn initWithHS20Settings_eapSettings(
            this: Allocated<Self>,
            hs20_settings: &NEHotspotHS20Settings,
            eap_settings: &NEHotspotEAPSettings,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithSSIDPrefix:)]
        pub unsafe fn initWithSSIDPrefix(
            this: Allocated<Self>,
            ssid_prefix: &NSString,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithSSIDPrefix:passphrase:isWEP:)]
        pub unsafe fn initWithSSIDPrefix_passphrase_isWEP(
            this: Allocated<Self>,
            ssid_prefix: &NSString,
            passphrase: &NSString,
            is_wep: bool,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NEHotspotConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    pub static NEHotspotConfigurationErrorDomain: &'static NSString;
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NEHotspotConfigurationError(pub NSInteger);
impl NEHotspotConfigurationError {
    #[doc(alias = "NEHotspotConfigurationErrorInvalid")]
    pub const Invalid: Self = Self(0);
    #[doc(alias = "NEHotspotConfigurationErrorInvalidSSID")]
    pub const InvalidSSID: Self = Self(1);
    #[doc(alias = "NEHotspotConfigurationErrorInvalidWPAPassphrase")]
    pub const InvalidWPAPassphrase: Self = Self(2);
    #[doc(alias = "NEHotspotConfigurationErrorInvalidWEPPassphrase")]
    pub const InvalidWEPPassphrase: Self = Self(3);
    #[doc(alias = "NEHotspotConfigurationErrorInvalidEAPSettings")]
    pub const InvalidEAPSettings: Self = Self(4);
    #[doc(alias = "NEHotspotConfigurationErrorInvalidHS20Settings")]
    pub const InvalidHS20Settings: Self = Self(5);
    #[doc(alias = "NEHotspotConfigurationErrorInvalidHS20DomainName")]
    pub const InvalidHS20DomainName: Self = Self(6);
    #[doc(alias = "NEHotspotConfigurationErrorUserDenied")]
    pub const UserDenied: Self = Self(7);
    #[doc(alias = "NEHotspotConfigurationErrorInternal")]
    pub const Internal: Self = Self(8);
    #[doc(alias = "NEHotspotConfigurationErrorPending")]
    pub const Pending: Self = Self(9);
    #[doc(alias = "NEHotspotConfigurationErrorSystemConfiguration")]
    pub const SystemConfiguration: Self = Self(10);
    #[doc(alias = "NEHotspotConfigurationErrorUnknown")]
    pub const Unknown: Self = Self(11);
    #[doc(alias = "NEHotspotConfigurationErrorJoinOnceNotSupported")]
    pub const JoinOnceNotSupported: Self = Self(12);
    #[doc(alias = "NEHotspotConfigurationErrorAlreadyAssociated")]
    pub const AlreadyAssociated: Self = Self(13);
    #[doc(alias = "NEHotspotConfigurationErrorApplicationIsNotInForeground")]
    pub const ApplicationIsNotInForeground: Self = Self(14);
    #[doc(alias = "NEHotspotConfigurationErrorInvalidSSIDPrefix")]
    pub const InvalidSSIDPrefix: Self = Self(15);
}

unsafe impl Encode for NEHotspotConfigurationError {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NEHotspotConfigurationError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NEHotspotConfigurationManager;

    unsafe impl ClassType for NEHotspotConfigurationManager {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for NEHotspotConfigurationManager {}

extern_methods!(
    unsafe impl NEHotspotConfigurationManager {
        #[method_id(@__retain_semantics Other sharedManager)]
        pub unsafe fn sharedManager() -> Retained<NEHotspotConfigurationManager>;

        #[cfg(feature = "block2")]
        #[method(applyConfiguration:completionHandler:)]
        pub unsafe fn applyConfiguration_completionHandler(
            &self,
            configuration: &NEHotspotConfiguration,
            completion_handler: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        );

        #[method(removeConfigurationForSSID:)]
        pub unsafe fn removeConfigurationForSSID(&self, ssid: &NSString);

        #[method(removeConfigurationForHS20DomainName:)]
        pub unsafe fn removeConfigurationForHS20DomainName(&self, domain_name: &NSString);

        #[cfg(feature = "block2")]
        #[method(getConfiguredSSIDsWithCompletionHandler:)]
        pub unsafe fn getConfiguredSSIDsWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(NonNull<NSArray<NSString>>)>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NEHotspotConfigurationManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
