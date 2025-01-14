//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/multipeerconnectivity/mcpeerid?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MCPeerID;
);

unsafe impl NSCoding for MCPeerID {}

unsafe impl NSCopying for MCPeerID {}

unsafe impl CopyingHelper for MCPeerID {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MCPeerID {}

unsafe impl NSSecureCoding for MCPeerID {}

extern_methods!(
    unsafe impl MCPeerID {
        #[method_id(@__retain_semantics Init initWithDisplayName:)]
        pub unsafe fn initWithDisplayName(
            this: Allocated<Self>,
            my_display_name: &NSString,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other displayName)]
        pub unsafe fn displayName(&self) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MCPeerID {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
