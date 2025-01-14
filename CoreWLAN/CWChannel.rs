//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/corewlan/cwchannel?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CWChannel;
);

unsafe impl NSCoding for CWChannel {}

unsafe impl NSCopying for CWChannel {}

unsafe impl CopyingHelper for CWChannel {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CWChannel {}

unsafe impl NSSecureCoding for CWChannel {}

extern_methods!(
    unsafe impl CWChannel {
        #[method(channelNumber)]
        pub unsafe fn channelNumber(&self) -> NSInteger;

        #[cfg(feature = "CoreWLANTypes")]
        #[method(channelWidth)]
        pub unsafe fn channelWidth(&self) -> CWChannelWidth;

        #[cfg(feature = "CoreWLANTypes")]
        #[method(channelBand)]
        pub unsafe fn channelBand(&self) -> CWChannelBand;

        #[method(isEqualToChannel:)]
        pub unsafe fn isEqualToChannel(&self, channel: &CWChannel) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CWChannel {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
