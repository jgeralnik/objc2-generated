//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMHTMLElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    #[deprecated]
    pub struct DOMHTMLTitleElement;

    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMHTMLElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl ClassType for DOMHTMLTitleElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMEventTarget",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl DOMEventTarget for DOMHTMLTitleElement {}

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSCopying for DOMHTMLTitleElement {}

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl CopyingHelper for DOMHTMLTitleElement {
    type Result = Self;
}

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSObjectProtocol for DOMHTMLTitleElement {}

extern_methods!(
    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMHTMLElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMHTMLTitleElement {
        #[deprecated]
        #[method_id(@__retain_semantics Other text)]
        pub unsafe fn text(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setText:)]
        pub unsafe fn setText(&self, text: Option<&NSString>);
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
    unsafe impl DOMHTMLTitleElement {
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
    unsafe impl DOMHTMLTitleElement {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
