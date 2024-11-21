//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSURLCredentialPersistence(pub NSUInteger);
impl NSURLCredentialPersistence {
    #[doc(alias = "NSURLCredentialPersistenceNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "NSURLCredentialPersistenceForSession")]
    pub const ForSession: Self = Self(1);
    #[doc(alias = "NSURLCredentialPersistencePermanent")]
    pub const Permanent: Self = Self(2);
    #[doc(alias = "NSURLCredentialPersistenceSynchronizable")]
    pub const Synchronizable: Self = Self(3);
}

unsafe impl Encode for NSURLCredentialPersistence {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSURLCredentialPersistence {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSURLCredential;
);

unsafe impl Send for NSURLCredential {}

unsafe impl Sync for NSURLCredential {}

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSURLCredential {}

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSURLCredential {}

#[cfg(feature = "NSObject")]
unsafe impl CopyingHelper for NSURLCredential {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSURLCredential {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSURLCredential {}

extern_methods!(
    unsafe impl NSURLCredential {
        #[method(persistence)]
        pub unsafe fn persistence(&self) -> NSURLCredentialPersistence;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSURLCredential {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// NSInternetPassword
    unsafe impl NSURLCredential {
        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Init initWithUser:password:persistence:)]
        pub unsafe fn initWithUser_password_persistence(
            this: Allocated<Self>,
            user: &NSString,
            password: &NSString,
            persistence: NSURLCredentialPersistence,
        ) -> Retained<Self>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other credentialWithUser:password:persistence:)]
        pub unsafe fn credentialWithUser_password_persistence(
            user: &NSString,
            password: &NSString,
            persistence: NSURLCredentialPersistence,
        ) -> Retained<NSURLCredential>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other user)]
        pub unsafe fn user(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other password)]
        pub unsafe fn password(&self) -> Option<Retained<NSString>>;

        #[method(hasPassword)]
        pub unsafe fn hasPassword(&self) -> bool;
    }
);

extern_methods!(
    /// NSClientCertificate
    unsafe impl NSURLCredential {
        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Other certificates)]
        pub unsafe fn certificates(&self) -> Retained<NSArray>;
    }
);

extern_methods!(
    /// NSServerTrust
    unsafe impl NSURLCredential {}
);
