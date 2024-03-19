//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

#[cfg(feature = "Foundation_NSString")]
typed_extensible_enum!(
    pub type ASAuthorizationSecurityKeyPublicKeyCredentialDescriptorTransport = NSString;
);

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static ASAuthorizationSecurityKeyPublicKeyCredentialDescriptorTransportUSB:
        &'static ASAuthorizationSecurityKeyPublicKeyCredentialDescriptorTransport;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static ASAuthorizationSecurityKeyPublicKeyCredentialDescriptorTransportNFC:
        &'static ASAuthorizationSecurityKeyPublicKeyCredentialDescriptorTransport;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static ASAuthorizationSecurityKeyPublicKeyCredentialDescriptorTransportBluetooth:
        &'static ASAuthorizationSecurityKeyPublicKeyCredentialDescriptorTransport;
}

extern "C" {
    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
    pub fn ASAuthorizationAllSupportedPublicKeyCredentialDescriptorTransports(
    ) -> NonNull<NSArray<ASAuthorizationSecurityKeyPublicKeyCredentialDescriptorTransport>>;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor;

    unsafe impl ClassType for ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "AuthenticationServices_ASAuthorizationPublicKeyCredentialDescriptor",
    feature = "Foundation_NSObject"
))]
unsafe impl ASAuthorizationPublicKeyCredentialDescriptor
    for ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor
{
}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor {}

unsafe impl NSObjectProtocol for ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor {}

extern_methods!(
    unsafe impl ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSData",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithCredentialID:transports:)]
        pub unsafe fn initWithCredentialID_transports(
            this: Allocated<Self>,
            credential_id: &NSData,
            allowed_transports: &NSArray<
                ASAuthorizationSecurityKeyPublicKeyCredentialDescriptorTransport,
            >,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other transports)]
        pub unsafe fn transports(
            &self,
        ) -> Id<NSArray<ASAuthorizationSecurityKeyPublicKeyCredentialDescriptorTransport>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setTransports:)]
        pub unsafe fn setTransports(
            &self,
            transports: &NSArray<ASAuthorizationSecurityKeyPublicKeyCredentialDescriptorTransport>,
        );

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);
