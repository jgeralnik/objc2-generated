//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LARightState(pub NSInteger);
impl LARightState {
    #[doc(alias = "LARightStateUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "LARightStateAuthorizing")]
    pub const Authorizing: Self = Self(1);
    #[doc(alias = "LARightStateAuthorized")]
    pub const Authorized: Self = Self(2);
    #[doc(alias = "LARightStateNotAuthorized")]
    pub const NotAuthorized: Self = Self(3);
}

unsafe impl Encode for LARightState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for LARightState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct LARight;

    unsafe impl ClassType for LARight {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for LARight {}

extern_methods!(
    unsafe impl LARight {
        #[method(state)]
        pub unsafe fn state(&self) -> LARightState;

        #[method(tag)]
        pub unsafe fn tag(&self) -> NSInteger;

        #[method(setTag:)]
        pub unsafe fn setTag(&self, tag: NSInteger);

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "LARequirement")]
        #[method_id(@__retain_semantics Init initWithRequirement:)]
        pub unsafe fn initWithRequirement(
            this: Allocated<Self>,
            requirement: &LAAuthenticationRequirement,
        ) -> Retained<Self>;

        #[cfg(feature = "block2")]
        #[method(authorizeWithLocalizedReason:completion:)]
        pub unsafe fn authorizeWithLocalizedReason_completion(
            &self,
            localized_reason: &NSString,
            handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "block2")]
        #[method(checkCanAuthorizeWithCompletion:)]
        pub unsafe fn checkCanAuthorizeWithCompletion(
            &self,
            handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "block2")]
        #[method(deauthorizeWithCompletion:)]
        pub unsafe fn deauthorizeWithCompletion(&self, handler: &block2::Block<dyn Fn()>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl LARight {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
