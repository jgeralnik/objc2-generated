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
    pub struct DOMHTMLImageElement;

    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMHTMLElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl ClassType for DOMHTMLImageElement {
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
unsafe impl DOMEventTarget for DOMHTMLImageElement {}

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSCopying for DOMHTMLImageElement {}

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl CopyingHelper for DOMHTMLImageElement {
    type Result = Self;
}

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSObjectProtocol for DOMHTMLImageElement {}

extern_methods!(
    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMHTMLElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMHTMLImageElement {
        #[deprecated]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other align)]
        pub unsafe fn align(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setAlign:)]
        pub unsafe fn setAlign(&self, align: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other alt)]
        pub unsafe fn alt(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setAlt:)]
        pub unsafe fn setAlt(&self, alt: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other border)]
        pub unsafe fn border(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setBorder:)]
        pub unsafe fn setBorder(&self, border: Option<&NSString>);

        #[deprecated]
        #[method(height)]
        pub unsafe fn height(&self) -> c_int;

        #[deprecated]
        #[method(setHeight:)]
        pub unsafe fn setHeight(&self, height: c_int);

        #[deprecated]
        #[method(hspace)]
        pub unsafe fn hspace(&self) -> c_int;

        #[deprecated]
        #[method(setHspace:)]
        pub unsafe fn setHspace(&self, hspace: c_int);

        #[deprecated]
        #[method(isMap)]
        pub unsafe fn isMap(&self) -> bool;

        #[deprecated]
        #[method(setIsMap:)]
        pub unsafe fn setIsMap(&self, is_map: bool);

        #[deprecated]
        #[method_id(@__retain_semantics Other longDesc)]
        pub unsafe fn longDesc(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setLongDesc:)]
        pub unsafe fn setLongDesc(&self, long_desc: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other src)]
        pub unsafe fn src(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setSrc:)]
        pub unsafe fn setSrc(&self, src: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other useMap)]
        pub unsafe fn useMap(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setUseMap:)]
        pub unsafe fn setUseMap(&self, use_map: Option<&NSString>);

        #[deprecated]
        #[method(vspace)]
        pub unsafe fn vspace(&self) -> c_int;

        #[deprecated]
        #[method(setVspace:)]
        pub unsafe fn setVspace(&self, vspace: c_int);

        #[deprecated]
        #[method(width)]
        pub unsafe fn width(&self) -> c_int;

        #[deprecated]
        #[method(setWidth:)]
        pub unsafe fn setWidth(&self, width: c_int);

        #[method(complete)]
        pub unsafe fn complete(&self) -> bool;

        #[method_id(@__retain_semantics Other lowsrc)]
        pub unsafe fn lowsrc(&self) -> Retained<NSString>;

        #[method(setLowsrc:)]
        pub unsafe fn setLowsrc(&self, lowsrc: Option<&NSString>);

        #[method(naturalHeight)]
        pub unsafe fn naturalHeight(&self) -> c_int;

        #[method(naturalWidth)]
        pub unsafe fn naturalWidth(&self) -> c_int;

        #[method(x)]
        pub unsafe fn x(&self) -> c_int;

        #[method(y)]
        pub unsafe fn y(&self) -> c_int;

        #[method_id(@__retain_semantics Other altDisplayString)]
        pub unsafe fn altDisplayString(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other absoluteImageURL)]
        pub unsafe fn absoluteImageURL(&self) -> Retained<NSURL>;
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
    unsafe impl DOMHTMLImageElement {
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
    unsafe impl DOMHTMLImageElement {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
