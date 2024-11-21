//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NEVPNIKEv2EncryptionAlgorithm(pub NSInteger);
impl NEVPNIKEv2EncryptionAlgorithm {
    #[deprecated = "Use an encryption algorithm with 256-bit keys instead"]
    #[doc(alias = "NEVPNIKEv2EncryptionAlgorithmDES")]
    pub const DES: Self = Self(1);
    #[deprecated = "Use an encryption algorithm with 256-bit keys instead"]
    pub const NEVPNIKEv2EncryptionAlgorithm3DES: Self = Self(2);
    #[deprecated = "Use an encryption algorithm with 256-bit keys instead"]
    #[doc(alias = "NEVPNIKEv2EncryptionAlgorithmAES128")]
    pub const AES128: Self = Self(3);
    #[doc(alias = "NEVPNIKEv2EncryptionAlgorithmAES256")]
    pub const AES256: Self = Self(4);
    #[deprecated = "Use an encryption algorithm with 256-bit keys instead"]
    #[doc(alias = "NEVPNIKEv2EncryptionAlgorithmAES128GCM")]
    pub const AES128GCM: Self = Self(5);
    #[doc(alias = "NEVPNIKEv2EncryptionAlgorithmAES256GCM")]
    pub const AES256GCM: Self = Self(6);
    #[doc(alias = "NEVPNIKEv2EncryptionAlgorithmChaCha20Poly1305")]
    pub const ChaCha20Poly1305: Self = Self(7);
}

unsafe impl Encode for NEVPNIKEv2EncryptionAlgorithm {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NEVPNIKEv2EncryptionAlgorithm {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NEVPNIKEv2IntegrityAlgorithm(pub NSInteger);
impl NEVPNIKEv2IntegrityAlgorithm {
    #[deprecated = "Use SHA-2 for integrity protection instead"]
    #[doc(alias = "NEVPNIKEv2IntegrityAlgorithmSHA96")]
    pub const SHA96: Self = Self(1);
    #[deprecated = "Use SHA-2 for integrity protection instead"]
    #[doc(alias = "NEVPNIKEv2IntegrityAlgorithmSHA160")]
    pub const SHA160: Self = Self(2);
    #[doc(alias = "NEVPNIKEv2IntegrityAlgorithmSHA256")]
    pub const SHA256: Self = Self(3);
    #[doc(alias = "NEVPNIKEv2IntegrityAlgorithmSHA384")]
    pub const SHA384: Self = Self(4);
    #[doc(alias = "NEVPNIKEv2IntegrityAlgorithmSHA512")]
    pub const SHA512: Self = Self(5);
}

unsafe impl Encode for NEVPNIKEv2IntegrityAlgorithm {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NEVPNIKEv2IntegrityAlgorithm {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NEVPNIKEv2DeadPeerDetectionRate(pub NSInteger);
impl NEVPNIKEv2DeadPeerDetectionRate {
    #[doc(alias = "NEVPNIKEv2DeadPeerDetectionRateNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "NEVPNIKEv2DeadPeerDetectionRateLow")]
    pub const Low: Self = Self(1);
    #[doc(alias = "NEVPNIKEv2DeadPeerDetectionRateMedium")]
    pub const Medium: Self = Self(2);
    #[doc(alias = "NEVPNIKEv2DeadPeerDetectionRateHigh")]
    pub const High: Self = Self(3);
}

unsafe impl Encode for NEVPNIKEv2DeadPeerDetectionRate {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NEVPNIKEv2DeadPeerDetectionRate {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NEVPNIKEv2DiffieHellmanGroup(pub NSInteger);
impl NEVPNIKEv2DiffieHellmanGroup {
    #[doc(alias = "NEVPNIKEv2DiffieHellmanGroupInvalid")]
    pub const Invalid: Self = Self(0);
    #[deprecated = "Use Diffie Hellman group 14 or greater instead"]
    pub const NEVPNIKEv2DiffieHellmanGroup1: Self = Self(1);
    #[deprecated = "Use Diffie Hellman group 14 or greater instead"]
    pub const NEVPNIKEv2DiffieHellmanGroup2: Self = Self(2);
    #[deprecated = "Use Diffie Hellman group 14 or greater instead"]
    pub const NEVPNIKEv2DiffieHellmanGroup5: Self = Self(5);
    pub const NEVPNIKEv2DiffieHellmanGroup14: Self = Self(14);
    pub const NEVPNIKEv2DiffieHellmanGroup15: Self = Self(15);
    pub const NEVPNIKEv2DiffieHellmanGroup16: Self = Self(16);
    pub const NEVPNIKEv2DiffieHellmanGroup17: Self = Self(17);
    pub const NEVPNIKEv2DiffieHellmanGroup18: Self = Self(18);
    pub const NEVPNIKEv2DiffieHellmanGroup19: Self = Self(19);
    pub const NEVPNIKEv2DiffieHellmanGroup20: Self = Self(20);
    pub const NEVPNIKEv2DiffieHellmanGroup21: Self = Self(21);
    pub const NEVPNIKEv2DiffieHellmanGroup31: Self = Self(31);
    pub const NEVPNIKEv2DiffieHellmanGroup32: Self = Self(32);
}

unsafe impl Encode for NEVPNIKEv2DiffieHellmanGroup {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NEVPNIKEv2DiffieHellmanGroup {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NEVPNIKEv2CertificateType(pub NSInteger);
impl NEVPNIKEv2CertificateType {
    #[doc(alias = "NEVPNIKEv2CertificateTypeRSA")]
    pub const RSA: Self = Self(1);
    #[doc(alias = "NEVPNIKEv2CertificateTypeECDSA256")]
    pub const ECDSA256: Self = Self(2);
    #[doc(alias = "NEVPNIKEv2CertificateTypeECDSA384")]
    pub const ECDSA384: Self = Self(3);
    #[doc(alias = "NEVPNIKEv2CertificateTypeECDSA521")]
    pub const ECDSA521: Self = Self(4);
    #[doc(alias = "NEVPNIKEv2CertificateTypeEd25519")]
    pub const Ed25519: Self = Self(5);
    #[doc(alias = "NEVPNIKEv2CertificateTypeRSAPSS")]
    pub const RSAPSS: Self = Self(6);
}

unsafe impl Encode for NEVPNIKEv2CertificateType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NEVPNIKEv2CertificateType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NEVPNIKEv2TLSVersion(pub NSInteger);
impl NEVPNIKEv2TLSVersion {
    #[doc(alias = "NEVPNIKEv2TLSVersionDefault")]
    pub const Default: Self = Self(0);
    pub const NEVPNIKEv2TLSVersion1_0: Self = Self(1);
    pub const NEVPNIKEv2TLSVersion1_1: Self = Self(2);
    pub const NEVPNIKEv2TLSVersion1_2: Self = Self(3);
}

unsafe impl Encode for NEVPNIKEv2TLSVersion {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NEVPNIKEv2TLSVersion {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NEVPNIKEv2SecurityAssociationParameters;
);

unsafe impl NSCoding for NEVPNIKEv2SecurityAssociationParameters {}

unsafe impl NSCopying for NEVPNIKEv2SecurityAssociationParameters {}

unsafe impl CopyingHelper for NEVPNIKEv2SecurityAssociationParameters {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NEVPNIKEv2SecurityAssociationParameters {}

unsafe impl NSSecureCoding for NEVPNIKEv2SecurityAssociationParameters {}

extern_methods!(
    unsafe impl NEVPNIKEv2SecurityAssociationParameters {
        #[method(encryptionAlgorithm)]
        pub unsafe fn encryptionAlgorithm(&self) -> NEVPNIKEv2EncryptionAlgorithm;

        #[method(setEncryptionAlgorithm:)]
        pub unsafe fn setEncryptionAlgorithm(
            &self,
            encryption_algorithm: NEVPNIKEv2EncryptionAlgorithm,
        );

        #[method(integrityAlgorithm)]
        pub unsafe fn integrityAlgorithm(&self) -> NEVPNIKEv2IntegrityAlgorithm;

        #[method(setIntegrityAlgorithm:)]
        pub unsafe fn setIntegrityAlgorithm(
            &self,
            integrity_algorithm: NEVPNIKEv2IntegrityAlgorithm,
        );

        #[method(diffieHellmanGroup)]
        pub unsafe fn diffieHellmanGroup(&self) -> NEVPNIKEv2DiffieHellmanGroup;

        #[method(setDiffieHellmanGroup:)]
        pub unsafe fn setDiffieHellmanGroup(
            &self,
            diffie_hellman_group: NEVPNIKEv2DiffieHellmanGroup,
        );

        #[method(lifetimeMinutes)]
        pub unsafe fn lifetimeMinutes(&self) -> i32;

        #[method(setLifetimeMinutes:)]
        pub unsafe fn setLifetimeMinutes(&self, lifetime_minutes: i32);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NEVPNIKEv2SecurityAssociationParameters {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NEVPNIKEv2PPKConfiguration;
);

unsafe impl NSCopying for NEVPNIKEv2PPKConfiguration {}

unsafe impl CopyingHelper for NEVPNIKEv2PPKConfiguration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NEVPNIKEv2PPKConfiguration {}

extern_methods!(
    unsafe impl NEVPNIKEv2PPKConfiguration {
        #[method_id(@__retain_semantics Init initWithIdentifier:keychainReference:)]
        pub unsafe fn initWithIdentifier_keychainReference(
            this: Allocated<Self>,
            identifier: &NSString,
            keychain_reference: &NSData,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other keychainReference)]
        pub unsafe fn keychainReference(&self) -> Retained<NSData>;

        #[method(isMandatory)]
        pub unsafe fn isMandatory(&self) -> bool;

        #[method(setIsMandatory:)]
        pub unsafe fn setIsMandatory(&self, is_mandatory: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NEVPNIKEv2PPKConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[unsafe(super(NEVPNProtocolIPSec, NEVPNProtocol, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NEVPNProtocol", feature = "NEVPNProtocolIPSec"))]
    pub struct NEVPNProtocolIKEv2;
);

#[cfg(all(feature = "NEVPNProtocol", feature = "NEVPNProtocolIPSec"))]
unsafe impl NSCoding for NEVPNProtocolIKEv2 {}

#[cfg(all(feature = "NEVPNProtocol", feature = "NEVPNProtocolIPSec"))]
unsafe impl NSCopying for NEVPNProtocolIKEv2 {}

#[cfg(all(feature = "NEVPNProtocol", feature = "NEVPNProtocolIPSec"))]
unsafe impl CopyingHelper for NEVPNProtocolIKEv2 {
    type Result = Self;
}

#[cfg(all(feature = "NEVPNProtocol", feature = "NEVPNProtocolIPSec"))]
unsafe impl NSObjectProtocol for NEVPNProtocolIKEv2 {}

#[cfg(all(feature = "NEVPNProtocol", feature = "NEVPNProtocolIPSec"))]
unsafe impl NSSecureCoding for NEVPNProtocolIKEv2 {}

extern_methods!(
    #[cfg(all(feature = "NEVPNProtocol", feature = "NEVPNProtocolIPSec"))]
    unsafe impl NEVPNProtocolIKEv2 {
        #[method(deadPeerDetectionRate)]
        pub unsafe fn deadPeerDetectionRate(&self) -> NEVPNIKEv2DeadPeerDetectionRate;

        #[method(setDeadPeerDetectionRate:)]
        pub unsafe fn setDeadPeerDetectionRate(
            &self,
            dead_peer_detection_rate: NEVPNIKEv2DeadPeerDetectionRate,
        );

        #[method_id(@__retain_semantics Other serverCertificateIssuerCommonName)]
        pub unsafe fn serverCertificateIssuerCommonName(&self) -> Option<Retained<NSString>>;

        #[method(setServerCertificateIssuerCommonName:)]
        pub unsafe fn setServerCertificateIssuerCommonName(
            &self,
            server_certificate_issuer_common_name: Option<&NSString>,
        );

        #[method_id(@__retain_semantics Other serverCertificateCommonName)]
        pub unsafe fn serverCertificateCommonName(&self) -> Option<Retained<NSString>>;

        #[method(setServerCertificateCommonName:)]
        pub unsafe fn setServerCertificateCommonName(
            &self,
            server_certificate_common_name: Option<&NSString>,
        );

        #[method(certificateType)]
        pub unsafe fn certificateType(&self) -> NEVPNIKEv2CertificateType;

        #[method(setCertificateType:)]
        pub unsafe fn setCertificateType(&self, certificate_type: NEVPNIKEv2CertificateType);

        #[method(useConfigurationAttributeInternalIPSubnet)]
        pub unsafe fn useConfigurationAttributeInternalIPSubnet(&self) -> bool;

        #[method(setUseConfigurationAttributeInternalIPSubnet:)]
        pub unsafe fn setUseConfigurationAttributeInternalIPSubnet(
            &self,
            use_configuration_attribute_internal_ip_subnet: bool,
        );

        #[method_id(@__retain_semantics Other IKESecurityAssociationParameters)]
        pub unsafe fn IKESecurityAssociationParameters(
            &self,
        ) -> Retained<NEVPNIKEv2SecurityAssociationParameters>;

        #[method_id(@__retain_semantics Other childSecurityAssociationParameters)]
        pub unsafe fn childSecurityAssociationParameters(
            &self,
        ) -> Retained<NEVPNIKEv2SecurityAssociationParameters>;

        #[method(disableMOBIKE)]
        pub unsafe fn disableMOBIKE(&self) -> bool;

        #[method(setDisableMOBIKE:)]
        pub unsafe fn setDisableMOBIKE(&self, disable_mobike: bool);

        #[method(disableRedirect)]
        pub unsafe fn disableRedirect(&self) -> bool;

        #[method(setDisableRedirect:)]
        pub unsafe fn setDisableRedirect(&self, disable_redirect: bool);

        #[method(enablePFS)]
        pub unsafe fn enablePFS(&self) -> bool;

        #[method(setEnablePFS:)]
        pub unsafe fn setEnablePFS(&self, enable_pfs: bool);

        #[method(enableRevocationCheck)]
        pub unsafe fn enableRevocationCheck(&self) -> bool;

        #[method(setEnableRevocationCheck:)]
        pub unsafe fn setEnableRevocationCheck(&self, enable_revocation_check: bool);

        #[method(strictRevocationCheck)]
        pub unsafe fn strictRevocationCheck(&self) -> bool;

        #[method(setStrictRevocationCheck:)]
        pub unsafe fn setStrictRevocationCheck(&self, strict_revocation_check: bool);

        #[method(minimumTLSVersion)]
        pub unsafe fn minimumTLSVersion(&self) -> NEVPNIKEv2TLSVersion;

        #[method(setMinimumTLSVersion:)]
        pub unsafe fn setMinimumTLSVersion(&self, minimum_tls_version: NEVPNIKEv2TLSVersion);

        #[method(maximumTLSVersion)]
        pub unsafe fn maximumTLSVersion(&self) -> NEVPNIKEv2TLSVersion;

        #[method(setMaximumTLSVersion:)]
        pub unsafe fn setMaximumTLSVersion(&self, maximum_tls_version: NEVPNIKEv2TLSVersion);

        #[method(enableFallback)]
        pub unsafe fn enableFallback(&self) -> bool;

        #[method(setEnableFallback:)]
        pub unsafe fn setEnableFallback(&self, enable_fallback: bool);

        #[method(mtu)]
        pub unsafe fn mtu(&self) -> NSUInteger;

        #[method(setMtu:)]
        pub unsafe fn setMtu(&self, mtu: NSUInteger);

        #[method_id(@__retain_semantics Other ppkConfiguration)]
        pub unsafe fn ppkConfiguration(&self) -> Option<Retained<NEVPNIKEv2PPKConfiguration>>;

        #[method(setPpkConfiguration:)]
        pub unsafe fn setPpkConfiguration(
            &self,
            ppk_configuration: Option<&NEVPNIKEv2PPKConfiguration>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "NEVPNProtocol", feature = "NEVPNProtocolIPSec"))]
    unsafe impl NEVPNProtocolIKEv2 {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
