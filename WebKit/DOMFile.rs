//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/domfile?language=objc)
    #[unsafe(super(DOMBlob, DOMObject, WebScriptObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "DOMBlob",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    #[deprecated]
    pub struct DOMFile;
);

#[cfg(all(
    feature = "DOMBlob",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSCopying for DOMFile {}

#[cfg(all(
    feature = "DOMBlob",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl CopyingHelper for DOMFile {
    type Result = Self;
}

#[cfg(all(
    feature = "DOMBlob",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSObjectProtocol for DOMFile {}

extern_methods!(
    #[cfg(all(
        feature = "DOMBlob",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMFile {
        #[deprecated]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(
        feature = "DOMBlob",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMFile {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "DOMBlob",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMFile {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
