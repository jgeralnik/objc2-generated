//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ASCredentialServiceIdentifierType(pub NSInteger);
impl ASCredentialServiceIdentifierType {
    #[doc(alias = "ASCredentialServiceIdentifierTypeDomain")]
    pub const Domain: Self = Self(0);
    #[doc(alias = "ASCredentialServiceIdentifierTypeURL")]
    pub const URL: Self = Self(1);
}

unsafe impl Encode for ASCredentialServiceIdentifierType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for ASCredentialServiceIdentifierType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASCredentialServiceIdentifier;

    unsafe impl ClassType for ASCredentialServiceIdentifier {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for ASCredentialServiceIdentifier {}

unsafe impl NSCopying for ASCredentialServiceIdentifier {}

unsafe impl CopyingHelper for ASCredentialServiceIdentifier {
    type Result = Self;
}

unsafe impl NSObjectProtocol for ASCredentialServiceIdentifier {}

unsafe impl NSSecureCoding for ASCredentialServiceIdentifier {}

extern_methods!(
    unsafe impl ASCredentialServiceIdentifier {
        #[method_id(@__retain_semantics Init initWithIdentifier:type:)]
        pub unsafe fn initWithIdentifier_type(
            this: Allocated<Self>,
            identifier: &NSString,
            r#type: ASCredentialServiceIdentifierType,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSString>;

        #[method(type)]
        pub unsafe fn r#type(&self) -> ASCredentialServiceIdentifierType;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ASCredentialServiceIdentifier {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
