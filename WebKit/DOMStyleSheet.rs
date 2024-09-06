//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    #[deprecated]
    pub struct DOMStyleSheet;

    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl ClassType for DOMStyleSheet {
        #[inherits(WebScriptObject, NSObject)]
        type Super = DOMObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl NSCopying for DOMStyleSheet {}

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl CopyingHelper for DOMStyleSheet {
    type Result = Self;
}

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl NSObjectProtocol for DOMStyleSheet {}

extern_methods!(
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMStyleSheet {
        #[deprecated]
        #[method_id(@__retain_semantics Other type)]
        pub unsafe fn r#type(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(disabled)]
        pub unsafe fn disabled(&self) -> bool;

        #[deprecated]
        #[method(setDisabled:)]
        pub unsafe fn setDisabled(&self, disabled: bool);

        #[cfg(feature = "DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other ownerNode)]
        pub unsafe fn ownerNode(&self) -> Option<Retained<DOMNode>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other parentStyleSheet)]
        pub unsafe fn parentStyleSheet(&self) -> Option<Retained<DOMStyleSheet>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other href)]
        pub unsafe fn href(&self) -> Retained<NSString>;

        #[deprecated]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Retained<NSString>;

        #[cfg(feature = "DOMMediaList")]
        #[deprecated]
        #[method_id(@__retain_semantics Other media)]
        pub unsafe fn media(&self) -> Option<Retained<DOMMediaList>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMStyleSheet {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMStyleSheet {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
