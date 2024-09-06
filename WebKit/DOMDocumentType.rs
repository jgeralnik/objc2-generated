//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    #[deprecated]
    pub struct DOMDocumentType;

    #[cfg(all(
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl ClassType for DOMDocumentType {
        #[inherits(DOMObject, WebScriptObject, NSObject)]
        type Super = DOMNode;
    }
);

#[cfg(all(
    feature = "DOMEventTarget",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl DOMEventTarget for DOMDocumentType {}

#[cfg(all(
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSCopying for DOMDocumentType {}

#[cfg(all(
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl CopyingHelper for DOMDocumentType {
    type Result = Self;
}

#[cfg(all(
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSObjectProtocol for DOMDocumentType {}

extern_methods!(
    #[cfg(all(
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMDocumentType {
        #[deprecated]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[cfg(feature = "DOMNamedNodeMap")]
        #[deprecated]
        #[method_id(@__retain_semantics Other entities)]
        pub unsafe fn entities(&self) -> Option<Retained<DOMNamedNodeMap>>;

        #[cfg(feature = "DOMNamedNodeMap")]
        #[deprecated]
        #[method_id(@__retain_semantics Other notations)]
        pub unsafe fn notations(&self) -> Option<Retained<DOMNamedNodeMap>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other publicId)]
        pub unsafe fn publicId(&self) -> Retained<NSString>;

        #[deprecated]
        #[method_id(@__retain_semantics Other systemId)]
        pub unsafe fn systemId(&self) -> Retained<NSString>;

        #[deprecated]
        #[method_id(@__retain_semantics Other internalSubset)]
        pub unsafe fn internalSubset(&self) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMDocumentType {
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
    unsafe impl DOMDocumentType {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
