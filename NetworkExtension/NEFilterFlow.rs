//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NEFilterFlow;
);

unsafe impl NSCoding for NEFilterFlow {}

unsafe impl NSCopying for NEFilterFlow {}

unsafe impl CopyingHelper for NEFilterFlow {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NEFilterFlow {}

unsafe impl NSSecureCoding for NEFilterFlow {}

extern_methods!(
    unsafe impl NEFilterFlow {
        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Option<Retained<NSURL>>;

        #[method_id(@__retain_semantics Other sourceAppUniqueIdentifier)]
        pub unsafe fn sourceAppUniqueIdentifier(&self) -> Option<Retained<NSData>>;

        #[method_id(@__retain_semantics Other sourceAppIdentifier)]
        pub unsafe fn sourceAppIdentifier(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other sourceAppVersion)]
        pub unsafe fn sourceAppVersion(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NENetworkRule")]
        #[method(direction)]
        pub unsafe fn direction(&self) -> NETrafficDirection;

        #[method_id(@__retain_semantics Other sourceAppAuditToken)]
        pub unsafe fn sourceAppAuditToken(&self) -> Option<Retained<NSData>>;

        #[method_id(@__retain_semantics Other sourceProcessAuditToken)]
        pub unsafe fn sourceProcessAuditToken(&self) -> Option<Retained<NSData>>;

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSUUID>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NEFilterFlow {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[unsafe(super(NEFilterFlow, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NEFilterBrowserFlow;
);

unsafe impl NSCoding for NEFilterBrowserFlow {}

unsafe impl NSCopying for NEFilterBrowserFlow {}

unsafe impl CopyingHelper for NEFilterBrowserFlow {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NEFilterBrowserFlow {}

unsafe impl NSSecureCoding for NEFilterBrowserFlow {}

extern_methods!(
    unsafe impl NEFilterBrowserFlow {
        #[method_id(@__retain_semantics Other request)]
        pub unsafe fn request(&self) -> Option<Retained<NSURLRequest>>;

        #[method_id(@__retain_semantics Other response)]
        pub unsafe fn response(&self) -> Option<Retained<NSURLResponse>>;

        #[method_id(@__retain_semantics Other parentURL)]
        pub unsafe fn parentURL(&self) -> Option<Retained<NSURL>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NEFilterBrowserFlow {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[unsafe(super(NEFilterFlow, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NEFilterSocketFlow;
);

unsafe impl NSCoding for NEFilterSocketFlow {}

unsafe impl NSCopying for NEFilterSocketFlow {}

unsafe impl CopyingHelper for NEFilterSocketFlow {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NEFilterSocketFlow {}

unsafe impl NSSecureCoding for NEFilterSocketFlow {}

extern_methods!(
    unsafe impl NEFilterSocketFlow {
        #[cfg(feature = "NWEndpoint")]
        #[deprecated]
        #[method_id(@__retain_semantics Other remoteEndpoint)]
        pub unsafe fn remoteEndpoint(&self) -> Option<Retained<NWEndpoint>>;

        #[method_id(@__retain_semantics Other remoteHostname)]
        pub unsafe fn remoteHostname(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NWEndpoint")]
        #[deprecated]
        #[method_id(@__retain_semantics Other localEndpoint)]
        pub unsafe fn localEndpoint(&self) -> Option<Retained<NWEndpoint>>;

        #[method(socketFamily)]
        pub unsafe fn socketFamily(&self) -> c_int;

        #[method(socketType)]
        pub unsafe fn socketType(&self) -> c_int;

        #[method(socketProtocol)]
        pub unsafe fn socketProtocol(&self) -> c_int;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NEFilterSocketFlow {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
