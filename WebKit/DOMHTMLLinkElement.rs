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
    pub struct DOMHTMLLinkElement;
);

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMEventTarget",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl DOMEventTarget for DOMHTMLLinkElement {}

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSCopying for DOMHTMLLinkElement {}

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl CopyingHelper for DOMHTMLLinkElement {
    type Result = Self;
}

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSObjectProtocol for DOMHTMLLinkElement {}

extern_methods!(
    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMHTMLElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMHTMLLinkElement {
        #[deprecated]
        #[method(disabled)]
        pub unsafe fn disabled(&self) -> bool;

        #[deprecated]
        #[method(setDisabled:)]
        pub unsafe fn setDisabled(&self, disabled: bool);

        #[deprecated]
        #[method_id(@__retain_semantics Other charset)]
        pub unsafe fn charset(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setCharset:)]
        pub unsafe fn setCharset(&self, charset: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other href)]
        pub unsafe fn href(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setHref:)]
        pub unsafe fn setHref(&self, href: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other hreflang)]
        pub unsafe fn hreflang(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setHreflang:)]
        pub unsafe fn setHreflang(&self, hreflang: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other media)]
        pub unsafe fn media(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setMedia:)]
        pub unsafe fn setMedia(&self, media: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other rel)]
        pub unsafe fn rel(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setRel:)]
        pub unsafe fn setRel(&self, rel: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other rev)]
        pub unsafe fn rev(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setRev:)]
        pub unsafe fn setRev(&self, rev: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other type)]
        pub unsafe fn r#type(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setType:)]
        pub unsafe fn setType(&self, r#type: Option<&NSString>);

        #[cfg(feature = "DOMStyleSheet")]
        #[method_id(@__retain_semantics Other sheet)]
        pub unsafe fn sheet(&self) -> Option<Retained<DOMStyleSheet>>;

        #[method_id(@__retain_semantics Other absoluteLinkURL)]
        pub unsafe fn absoluteLinkURL(&self) -> Retained<NSURL>;
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
    unsafe impl DOMHTMLLinkElement {
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
    unsafe impl DOMHTMLLinkElement {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
