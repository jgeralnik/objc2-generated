//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationpublickeycredentialprfassertioninputvalues?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationPublicKeyCredentialPRFAssertionInputValues;
);

unsafe impl NSObjectProtocol for ASAuthorizationPublicKeyCredentialPRFAssertionInputValues {}

extern_methods!(
    unsafe impl ASAuthorizationPublicKeyCredentialPRFAssertionInputValues {
        #[method_id(@__retain_semantics Init initWithSaltInput1:saltInput2:)]
        pub unsafe fn initWithSaltInput1_saltInput2(
            this: Allocated<Self>,
            salt_input1: &NSData,
            salt_input2: Option<&NSData>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other saltInput1)]
        pub unsafe fn saltInput1(&self) -> Retained<NSData>;

        #[method_id(@__retain_semantics Other saltInput2)]
        pub unsafe fn saltInput2(&self) -> Option<Retained<NSData>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ASAuthorizationPublicKeyCredentialPRFAssertionInputValues {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationpublickeycredentialprfassertioninput?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationPublicKeyCredentialPRFAssertionInput;
);

unsafe impl NSObjectProtocol for ASAuthorizationPublicKeyCredentialPRFAssertionInput {}

extern_methods!(
    unsafe impl ASAuthorizationPublicKeyCredentialPRFAssertionInput {
        #[method_id(@__retain_semantics Init initWithInputValues:perCredentialInputValues:)]
        pub unsafe fn initWithInputValues_perCredentialInputValues(
            this: Allocated<Self>,
            input_values: Option<&ASAuthorizationPublicKeyCredentialPRFAssertionInputValues>,
            per_credential_input_values: Option<
                &NSDictionary<NSData, ASAuthorizationPublicKeyCredentialPRFAssertionInputValues>,
            >,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other inputValues)]
        pub unsafe fn inputValues(
            &self,
        ) -> Option<Retained<ASAuthorizationPublicKeyCredentialPRFAssertionInputValues>>;

        #[method_id(@__retain_semantics Other perCredentialInputValues)]
        pub unsafe fn perCredentialInputValues(
            &self,
        ) -> Option<
            Retained<
                NSDictionary<NSData, ASAuthorizationPublicKeyCredentialPRFAssertionInputValues>,
            >,
        >;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ASAuthorizationPublicKeyCredentialPRFAssertionInput {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
