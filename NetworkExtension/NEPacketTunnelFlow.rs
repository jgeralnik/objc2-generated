//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/networkextension/nepackettunnelflow?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NEPacketTunnelFlow;
);

unsafe impl NSObjectProtocol for NEPacketTunnelFlow {}

extern_methods!(
    unsafe impl NEPacketTunnelFlow {
        #[cfg(feature = "block2")]
        #[method(readPacketsWithCompletionHandler:)]
        pub unsafe fn readPacketsWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<
                dyn Fn(NonNull<NSArray<NSData>>, NonNull<NSArray<NSNumber>>),
            >,
        );

        #[method(writePackets:withProtocols:)]
        pub unsafe fn writePackets_withProtocols(
            &self,
            packets: &NSArray<NSData>,
            protocols: &NSArray<NSNumber>,
        ) -> bool;

        #[cfg(all(feature = "NEPacket", feature = "block2"))]
        #[method(readPacketObjectsWithCompletionHandler:)]
        pub unsafe fn readPacketObjectsWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(NonNull<NSArray<NEPacket>>)>,
        );

        #[cfg(feature = "NEPacket")]
        #[method(writePacketObjects:)]
        pub unsafe fn writePacketObjects(&self, packets: &NSArray<NEPacket>) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NEPacketTunnelFlow {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
