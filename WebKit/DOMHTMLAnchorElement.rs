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
    pub struct DOMHTMLAnchorElement;

    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMHTMLElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl ClassType for DOMHTMLAnchorElement {
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
unsafe impl DOMEventTarget for DOMHTMLAnchorElement {}

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSCopying for DOMHTMLAnchorElement {}

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl CopyingHelper for DOMHTMLAnchorElement {
    type Result = Self;
}

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSObjectProtocol for DOMHTMLAnchorElement {}

extern_methods!(
    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMHTMLElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMHTMLAnchorElement {
        #[deprecated]
        #[method_id(@__retain_semantics Other charset)]
        pub unsafe fn charset(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setCharset:)]
        pub unsafe fn setCharset(&self, charset: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other coords)]
        pub unsafe fn coords(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setCoords:)]
        pub unsafe fn setCoords(&self, coords: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other hreflang)]
        pub unsafe fn hreflang(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setHreflang:)]
        pub unsafe fn setHreflang(&self, hreflang: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

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
        #[method_id(@__retain_semantics Other shape)]
        pub unsafe fn shape(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setShape:)]
        pub unsafe fn setShape(&self, shape: Option<&NSString>);

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

        #[deprecated]
        #[method_id(@__retain_semantics Other accessKey)]
        pub unsafe fn accessKey(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setAccessKey:)]
        pub unsafe fn setAccessKey(&self, access_key: Option<&NSString>);

        #[method_id(@__retain_semantics Other text)]
        pub unsafe fn text(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other absoluteLinkURL)]
        pub unsafe fn absoluteLinkURL(&self) -> Retained<NSURL>;

        #[deprecated]
        #[method_id(@__retain_semantics Other href)]
        pub unsafe fn href(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setHref:)]
        pub unsafe fn setHref(&self, href: Option<&NSString>);

        #[method_id(@__retain_semantics Other protocol)]
        pub unsafe fn protocol(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other host)]
        pub unsafe fn host(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other hostname)]
        pub unsafe fn hostname(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other port)]
        pub unsafe fn port(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other pathname)]
        pub unsafe fn pathname(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other search)]
        pub unsafe fn search(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other hashName)]
        pub unsafe fn hashName(&self) -> Retained<NSString>;
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
    unsafe impl DOMHTMLAnchorElement {
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
    unsafe impl DOMHTMLAnchorElement {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
