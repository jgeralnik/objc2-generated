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
    pub struct DOMHTMLTableRowElement;

    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMHTMLElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl ClassType for DOMHTMLTableRowElement {
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
unsafe impl DOMEventTarget for DOMHTMLTableRowElement {}

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSCopying for DOMHTMLTableRowElement {}

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl CopyingHelper for DOMHTMLTableRowElement {
    type Result = Self;
}

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSObjectProtocol for DOMHTMLTableRowElement {}

extern_methods!(
    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMHTMLElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMHTMLTableRowElement {
        #[deprecated]
        #[method(rowIndex)]
        pub unsafe fn rowIndex(&self) -> c_int;

        #[deprecated]
        #[method(sectionRowIndex)]
        pub unsafe fn sectionRowIndex(&self) -> c_int;

        #[cfg(feature = "DOMHTMLCollection")]
        #[deprecated]
        #[method_id(@__retain_semantics Other cells)]
        pub unsafe fn cells(&self) -> Option<Retained<DOMHTMLCollection>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other align)]
        pub unsafe fn align(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setAlign:)]
        pub unsafe fn setAlign(&self, align: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other bgColor)]
        pub unsafe fn bgColor(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setBgColor:)]
        pub unsafe fn setBgColor(&self, bg_color: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other ch)]
        pub unsafe fn ch(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setCh:)]
        pub unsafe fn setCh(&self, ch: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other chOff)]
        pub unsafe fn chOff(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setChOff:)]
        pub unsafe fn setChOff(&self, ch_off: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other vAlign)]
        pub unsafe fn vAlign(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setVAlign:)]
        pub unsafe fn setVAlign(&self, v_align: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other insertCell:)]
        pub unsafe fn insertCell(&self, index: c_int) -> Option<Retained<DOMHTMLElement>>;

        #[deprecated]
        #[method(deleteCell:)]
        pub unsafe fn deleteCell(&self, index: c_int);
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
    unsafe impl DOMHTMLTableRowElement {
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
    unsafe impl DOMHTMLTableRowElement {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
