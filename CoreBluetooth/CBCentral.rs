//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CBPeer")]
    pub struct CBCentral;

    #[cfg(feature = "CBPeer")]
    unsafe impl ClassType for CBCentral {
        #[inherits(NSObject)]
        type Super = CBPeer;
    }
);

#[cfg(feature = "CBPeer")]
unsafe impl NSCopying for CBCentral {}

#[cfg(feature = "CBPeer")]
unsafe impl CopyingHelper for CBCentral {
    type Result = Self;
}

#[cfg(feature = "CBPeer")]
unsafe impl NSObjectProtocol for CBCentral {}

extern_methods!(
    #[cfg(feature = "CBPeer")]
    unsafe impl CBCentral {
        #[method(maximumUpdateValueLength)]
        pub unsafe fn maximumUpdateValueLength(&self) -> NSUInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `CBPeer`
    #[cfg(feature = "CBPeer")]
    unsafe impl CBCentral {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CBPeer")]
    unsafe impl CBCentral {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
