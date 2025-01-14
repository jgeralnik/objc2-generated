//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mailkit/meoutgoingmessageencodingstatus?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MEOutgoingMessageEncodingStatus;
);

unsafe impl NSCoding for MEOutgoingMessageEncodingStatus {}

unsafe impl NSObjectProtocol for MEOutgoingMessageEncodingStatus {}

unsafe impl NSSecureCoding for MEOutgoingMessageEncodingStatus {}

extern_methods!(
    unsafe impl MEOutgoingMessageEncodingStatus {
        #[method(canSign)]
        pub unsafe fn canSign(&self) -> bool;

        #[method(canEncrypt)]
        pub unsafe fn canEncrypt(&self) -> bool;

        #[method_id(@__retain_semantics Other securityError)]
        pub unsafe fn securityError(&self) -> Option<Retained<NSError>>;

        #[cfg(feature = "MEEmailAddress")]
        #[method_id(@__retain_semantics Other addressesFailingEncryption)]
        pub unsafe fn addressesFailingEncryption(&self) -> Retained<NSArray<MEEmailAddress>>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "MEEmailAddress")]
        #[method_id(@__retain_semantics Init initWithCanSign:canEncrypt:securityError:addressesFailingEncryption:)]
        pub unsafe fn initWithCanSign_canEncrypt_securityError_addressesFailingEncryption(
            this: Allocated<Self>,
            can_sign: bool,
            can_encrypt: bool,
            security_error: Option<&NSError>,
            addresses_failing_encryption: &NSArray<MEEmailAddress>,
        ) -> Retained<Self>;
    }
);
