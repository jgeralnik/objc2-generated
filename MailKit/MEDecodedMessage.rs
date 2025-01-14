//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mailkit/medecodedmessage?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MEDecodedMessage;
);

unsafe impl NSCoding for MEDecodedMessage {}

unsafe impl NSObjectProtocol for MEDecodedMessage {}

unsafe impl NSSecureCoding for MEDecodedMessage {}

extern_methods!(
    unsafe impl MEDecodedMessage {
        #[method_id(@__retain_semantics Other rawData)]
        pub unsafe fn rawData(&self) -> Option<Retained<NSData>>;

        #[cfg(feature = "MEMessageSecurityInformation")]
        #[method_id(@__retain_semantics Other securityInformation)]
        pub unsafe fn securityInformation(&self) -> Retained<MEMessageSecurityInformation>;

        #[method_id(@__retain_semantics Other context)]
        pub unsafe fn context(&self) -> Option<Retained<NSData>>;

        #[cfg(feature = "MEDecodedMessageBanner")]
        #[method_id(@__retain_semantics Other banner)]
        pub unsafe fn banner(&self) -> Option<Retained<MEDecodedMessageBanner>>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "MEMessageSecurityInformation")]
        #[method_id(@__retain_semantics Init initWithData:securityInformation:context:)]
        pub unsafe fn initWithData_securityInformation_context(
            this: Allocated<Self>,
            raw_data: Option<&NSData>,
            security_information: &MEMessageSecurityInformation,
            context: Option<&NSData>,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "MEDecodedMessageBanner",
            feature = "MEMessageSecurityInformation"
        ))]
        #[method_id(@__retain_semantics Init initWithData:securityInformation:context:banner:)]
        pub unsafe fn initWithData_securityInformation_context_banner(
            this: Allocated<Self>,
            raw_data: Option<&NSData>,
            security_information: &MEMessageSecurityInformation,
            context: Option<&NSData>,
            banner: Option<&MEDecodedMessageBanner>,
        ) -> Retained<Self>;
    }
);
