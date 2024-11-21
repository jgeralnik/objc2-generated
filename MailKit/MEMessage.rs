//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MEMessageState(pub NSInteger);
impl MEMessageState {
    #[doc(alias = "MEMessageStateReceived")]
    pub const Received: Self = Self(0);
    #[doc(alias = "MEMessageStateDraft")]
    pub const Draft: Self = Self(1);
    #[doc(alias = "MEMessageStateSending")]
    pub const Sending: Self = Self(2);
}

unsafe impl Encode for MEMessageState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MEMessageState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MEMessageEncryptionState(pub NSInteger);
impl MEMessageEncryptionState {
    #[doc(alias = "MEMessageEncryptionStateUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "MEMessageEncryptionStateNotEncrypted")]
    pub const NotEncrypted: Self = Self(1);
    #[doc(alias = "MEMessageEncryptionStateEncrypted")]
    pub const Encrypted: Self = Self(2);
}

unsafe impl Encode for MEMessageEncryptionState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MEMessageEncryptionState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MEMessage;
);

unsafe impl NSCoding for MEMessage {}

unsafe impl NSObjectProtocol for MEMessage {}

unsafe impl NSSecureCoding for MEMessage {}

extern_methods!(
    unsafe impl MEMessage {
        #[method(state)]
        pub unsafe fn state(&self) -> MEMessageState;

        #[method(encryptionState)]
        pub unsafe fn encryptionState(&self) -> MEMessageEncryptionState;

        #[method_id(@__retain_semantics Other subject)]
        pub unsafe fn subject(&self) -> Retained<NSString>;

        #[cfg(feature = "MEEmailAddress")]
        #[method_id(@__retain_semantics Other fromAddress)]
        pub unsafe fn fromAddress(&self) -> Retained<MEEmailAddress>;

        #[cfg(feature = "MEEmailAddress")]
        #[method_id(@__retain_semantics Other toAddresses)]
        pub unsafe fn toAddresses(&self) -> Retained<NSArray<MEEmailAddress>>;

        #[cfg(feature = "MEEmailAddress")]
        #[method_id(@__retain_semantics Other ccAddresses)]
        pub unsafe fn ccAddresses(&self) -> Retained<NSArray<MEEmailAddress>>;

        #[cfg(feature = "MEEmailAddress")]
        #[method_id(@__retain_semantics Other bccAddresses)]
        pub unsafe fn bccAddresses(&self) -> Retained<NSArray<MEEmailAddress>>;

        #[cfg(feature = "MEEmailAddress")]
        #[method_id(@__retain_semantics Other replyToAddresses)]
        pub unsafe fn replyToAddresses(&self) -> Retained<NSArray<MEEmailAddress>>;

        #[cfg(feature = "MEEmailAddress")]
        #[method_id(@__retain_semantics Other allRecipientAddresses)]
        pub unsafe fn allRecipientAddresses(&self) -> Retained<NSArray<MEEmailAddress>>;

        #[method_id(@__retain_semantics Other dateSent)]
        pub unsafe fn dateSent(&self) -> Option<Retained<NSDate>>;

        #[method_id(@__retain_semantics Other dateReceived)]
        pub unsafe fn dateReceived(&self) -> Option<Retained<NSDate>>;

        #[method_id(@__retain_semantics Other headers)]
        pub unsafe fn headers(&self)
            -> Option<Retained<NSDictionary<NSString, NSArray<NSString>>>>;

        #[method_id(@__retain_semantics Other rawData)]
        pub unsafe fn rawData(&self) -> Option<Retained<NSData>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
