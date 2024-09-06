//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NEAppProxyFlow")]
    pub struct NEAppProxyUDPFlow;

    #[cfg(feature = "NEAppProxyFlow")]
    unsafe impl ClassType for NEAppProxyUDPFlow {
        #[inherits(NSObject)]
        type Super = NEAppProxyFlow;
    }
);

#[cfg(feature = "NEAppProxyFlow")]
unsafe impl NSObjectProtocol for NEAppProxyUDPFlow {}

extern_methods!(
    #[cfg(feature = "NEAppProxyFlow")]
    unsafe impl NEAppProxyUDPFlow {
        #[cfg(all(feature = "NWEndpoint", feature = "block2"))]
        #[method(readDatagramsWithCompletionHandler:)]
        pub unsafe fn readDatagramsWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<
                dyn Fn(*mut NSArray<NSData>, *mut NSArray<NWEndpoint>, *mut NSError),
            >,
        );

        #[cfg(all(feature = "NWEndpoint", feature = "block2"))]
        #[method(writeDatagrams:sentByEndpoints:completionHandler:)]
        pub unsafe fn writeDatagrams_sentByEndpoints_completionHandler(
            &self,
            datagrams: &NSArray<NSData>,
            remote_endpoints: &NSArray<NWEndpoint>,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "NWEndpoint")]
        #[method_id(@__retain_semantics Other localEndpoint)]
        pub unsafe fn localEndpoint(&self) -> Option<Retained<NWEndpoint>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NEAppProxyFlow")]
    unsafe impl NEAppProxyUDPFlow {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
