//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/networkextension/nefilterpacketcontext?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NEFilterPacketContext;
);

unsafe impl NSObjectProtocol for NEFilterPacketContext {}

extern_methods!(
    unsafe impl NEFilterPacketContext {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NEFilterPacketContext {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/networkextension/nefilterpacketproviderverdict?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NEFilterPacketProviderVerdict(pub NSInteger);
impl NEFilterPacketProviderVerdict {
    #[doc(alias = "NEFilterPacketProviderVerdictAllow")]
    pub const Allow: Self = Self(0);
    #[doc(alias = "NEFilterPacketProviderVerdictDrop")]
    pub const Drop: Self = Self(1);
    #[doc(alias = "NEFilterPacketProviderVerdictDelay")]
    pub const Delay: Self = Self(2);
}

unsafe impl Encode for NEFilterPacketProviderVerdict {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NEFilterPacketProviderVerdict {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/networkextension/nefilterpacketprovider?language=objc)
    #[unsafe(super(NEFilterProvider, NEProvider, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NEFilterProvider", feature = "NEProvider"))]
    pub struct NEFilterPacketProvider;
);

#[cfg(all(feature = "NEFilterProvider", feature = "NEProvider"))]
unsafe impl NSObjectProtocol for NEFilterPacketProvider {}

extern_methods!(
    #[cfg(all(feature = "NEFilterProvider", feature = "NEProvider"))]
    unsafe impl NEFilterPacketProvider {
        #[cfg(feature = "NEPacket")]
        #[method_id(@__retain_semantics Other delayCurrentPacket:)]
        pub unsafe fn delayCurrentPacket(
            &self,
            context: &NEFilterPacketContext,
        ) -> Retained<NEPacket>;

        #[cfg(feature = "NEPacket")]
        #[method(allowPacket:)]
        pub unsafe fn allowPacket(&self, packet: &NEPacket);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "NEFilterProvider", feature = "NEProvider"))]
    unsafe impl NEFilterPacketProvider {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
