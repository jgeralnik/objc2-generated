//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/networkextension/neappproxyflowerror?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NEAppProxyFlowError(pub NSInteger);
impl NEAppProxyFlowError {
    #[doc(alias = "NEAppProxyFlowErrorNotConnected")]
    pub const NotConnected: Self = Self(1);
    #[doc(alias = "NEAppProxyFlowErrorPeerReset")]
    pub const PeerReset: Self = Self(2);
    #[doc(alias = "NEAppProxyFlowErrorHostUnreachable")]
    pub const HostUnreachable: Self = Self(3);
    #[doc(alias = "NEAppProxyFlowErrorInvalidArgument")]
    pub const InvalidArgument: Self = Self(4);
    #[doc(alias = "NEAppProxyFlowErrorAborted")]
    pub const Aborted: Self = Self(5);
    #[doc(alias = "NEAppProxyFlowErrorRefused")]
    pub const Refused: Self = Self(6);
    #[doc(alias = "NEAppProxyFlowErrorTimedOut")]
    pub const TimedOut: Self = Self(7);
    #[doc(alias = "NEAppProxyFlowErrorInternal")]
    pub const Internal: Self = Self(8);
    #[doc(alias = "NEAppProxyFlowErrorDatagramTooLarge")]
    pub const DatagramTooLarge: Self = Self(9);
    #[doc(alias = "NEAppProxyFlowErrorReadAlreadyPending")]
    pub const ReadAlreadyPending: Self = Self(10);
}

unsafe impl Encode for NEAppProxyFlowError {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NEAppProxyFlowError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/networkextension/neappproxyerrordomain?language=objc)
    pub static NEAppProxyErrorDomain: &'static NSString;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/networkextension/neappproxyflow?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NEAppProxyFlow;
);

unsafe impl NSObjectProtocol for NEAppProxyFlow {}

extern_methods!(
    unsafe impl NEAppProxyFlow {
        #[cfg(all(feature = "NWEndpoint", feature = "NWHostEndpoint", feature = "block2"))]
        #[deprecated]
        #[method(openWithLocalEndpoint:completionHandler:)]
        pub unsafe fn openWithLocalEndpoint_completionHandler(
            &self,
            local_endpoint: Option<&NWHostEndpoint>,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[method(closeReadWithError:)]
        pub unsafe fn closeReadWithError(&self, error: Option<&NSError>);

        #[method(closeWriteWithError:)]
        pub unsafe fn closeWriteWithError(&self, error: Option<&NSError>);

        #[cfg(feature = "NEFlowMetaData")]
        #[method_id(@__retain_semantics Other metaData)]
        pub unsafe fn metaData(&self) -> Retained<NEFlowMetaData>;

        #[method_id(@__retain_semantics Other remoteHostname)]
        pub unsafe fn remoteHostname(&self) -> Option<Retained<NSString>>;

        #[method(isBound)]
        pub unsafe fn isBound(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NEAppProxyFlow {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
