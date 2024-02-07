//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSURLCredentialPersistence {
        #[doc(alias = "NSURLCredentialPersistenceNone")]
        None = 0,
        #[doc(alias = "NSURLCredentialPersistenceForSession")]
        ForSession = 1,
        #[doc(alias = "NSURLCredentialPersistencePermanent")]
        Permanent = 2,
        #[doc(alias = "NSURLCredentialPersistenceSynchronizable")]
        Synchronizable = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSURLCredential")]
    pub struct NSURLCredential;

    #[cfg(feature = "Foundation_NSURLCredential")]
    unsafe impl ClassType for NSURLCredential {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSURLCredential")]
unsafe impl Send for NSURLCredential {}

#[cfg(feature = "Foundation_NSURLCredential")]
unsafe impl Sync for NSURLCredential {}

#[cfg(feature = "Foundation_NSURLCredential")]
unsafe impl NSCoding for NSURLCredential {}

#[cfg(feature = "Foundation_NSURLCredential")]
unsafe impl NSCopying for NSURLCredential {}

#[cfg(feature = "Foundation_NSURLCredential")]
unsafe impl NSObjectProtocol for NSURLCredential {}

#[cfg(feature = "Foundation_NSURLCredential")]
unsafe impl NSSecureCoding for NSURLCredential {}

extern_methods!(
    #[cfg(feature = "Foundation_NSURLCredential")]
    unsafe impl NSURLCredential {
        #[method(persistence)]
        pub unsafe fn persistence(&self) -> NSURLCredentialPersistence;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSURLCredential")]
    unsafe impl NSURLCredential {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSInternetPassword
    #[cfg(feature = "Foundation_NSURLCredential")]
    unsafe impl NSURLCredential {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithUser:password:persistence:)]
        pub unsafe fn initWithUser_password_persistence(
            this: Allocated<Self>,
            user: &NSString,
            password: &NSString,
            persistence: NSURLCredentialPersistence,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other credentialWithUser:password:persistence:)]
        pub unsafe fn credentialWithUser_password_persistence(
            user: &NSString,
            password: &NSString,
            persistence: NSURLCredentialPersistence,
        ) -> Id<NSURLCredential>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other user)]
        pub unsafe fn user(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other password)]
        pub unsafe fn password(&self) -> Option<Id<NSString>>;

        #[method(hasPassword)]
        pub unsafe fn hasPassword(&self) -> bool;
    }
);

extern_methods!(
    /// NSClientCertificate
    #[cfg(feature = "Foundation_NSURLCredential")]
    unsafe impl NSURLCredential {
        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other certificates)]
        pub unsafe fn certificates(&self) -> Id<NSArray>;
    }
);

extern_methods!(
    /// NSServerTrust
    #[cfg(feature = "Foundation_NSURLCredential")]
    unsafe impl NSURLCredential {}
);
