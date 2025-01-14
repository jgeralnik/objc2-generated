//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/domattr?language=objc)
    #[unsafe(super(DOMNode, DOMObject, WebScriptObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    #[deprecated]
    pub struct DOMAttr;
);

#[cfg(all(
    feature = "DOMEventTarget",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl DOMEventTarget for DOMAttr {}

#[cfg(all(
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSCopying for DOMAttr {}

#[cfg(all(
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl CopyingHelper for DOMAttr {
    type Result = Self;
}

#[cfg(all(
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSObjectProtocol for DOMAttr {}

extern_methods!(
    #[cfg(all(
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMAttr {
        #[deprecated]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(specified)]
        pub unsafe fn specified(&self) -> bool;

        #[deprecated]
        #[method_id(@__retain_semantics Other value)]
        pub unsafe fn value(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setValue:)]
        pub unsafe fn setValue(&self, value: Option<&NSString>);

        #[cfg(feature = "DOMElement")]
        #[deprecated]
        #[method_id(@__retain_semantics Other ownerElement)]
        pub unsafe fn ownerElement(&self) -> Option<Retained<DOMElement>>;

        #[cfg(feature = "DOMCSSStyleDeclaration")]
        #[method_id(@__retain_semantics Other style)]
        pub unsafe fn style(&self) -> Option<Retained<DOMCSSStyleDeclaration>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMAttr {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMAttr {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
