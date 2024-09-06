//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    #[deprecated]
    pub struct DOMRect;

    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl ClassType for DOMRect {
        #[inherits(WebScriptObject, NSObject)]
        type Super = DOMObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl NSCopying for DOMRect {}

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl CopyingHelper for DOMRect {
    type Result = Self;
}

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl NSObjectProtocol for DOMRect {}

extern_methods!(
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMRect {
        #[cfg(all(feature = "DOMCSSPrimitiveValue", feature = "DOMCSSValue"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other top)]
        pub unsafe fn top(&self) -> Option<Retained<DOMCSSPrimitiveValue>>;

        #[cfg(all(feature = "DOMCSSPrimitiveValue", feature = "DOMCSSValue"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other right)]
        pub unsafe fn right(&self) -> Option<Retained<DOMCSSPrimitiveValue>>;

        #[cfg(all(feature = "DOMCSSPrimitiveValue", feature = "DOMCSSValue"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other bottom)]
        pub unsafe fn bottom(&self) -> Option<Retained<DOMCSSPrimitiveValue>>;

        #[cfg(all(feature = "DOMCSSPrimitiveValue", feature = "DOMCSSValue"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other left)]
        pub unsafe fn left(&self) -> Option<Retained<DOMCSSPrimitiveValue>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMRect {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMRect {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
