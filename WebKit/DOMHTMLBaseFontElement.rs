//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(
        DOMHTMLElement,
        DOMElement,
        DOMNode,
        DOMObject,
        WebScriptObject,
        NSObject
    ))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMHTMLElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    #[deprecated]
    pub struct DOMHTMLBaseFontElement;
);

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMEventTarget",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl DOMEventTarget for DOMHTMLBaseFontElement {}

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSCopying for DOMHTMLBaseFontElement {}

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl CopyingHelper for DOMHTMLBaseFontElement {
    type Result = Self;
}

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSObjectProtocol for DOMHTMLBaseFontElement {}

extern_methods!(
    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMHTMLElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMHTMLBaseFontElement {
        #[deprecated]
        #[method_id(@__retain_semantics Other color)]
        pub unsafe fn color(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setColor:)]
        pub unsafe fn setColor(&self, color: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other face)]
        pub unsafe fn face(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setFace:)]
        pub unsafe fn setFace(&self, face: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other size)]
        pub unsafe fn size(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setSize:)]
        pub unsafe fn setSize(&self, size: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMHTMLElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMHTMLBaseFontElement {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMHTMLElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMHTMLBaseFontElement {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
