//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/domnamednodemap?language=objc)
    #[unsafe(super(DOMObject, WebScriptObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    #[deprecated]
    pub struct DOMNamedNodeMap;
);

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl NSCopying for DOMNamedNodeMap {}

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl CopyingHelper for DOMNamedNodeMap {
    type Result = Self;
}

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl NSObjectProtocol for DOMNamedNodeMap {}

extern_methods!(
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMNamedNodeMap {
        #[deprecated]
        #[method(length)]
        pub unsafe fn length(&self) -> c_uint;

        #[cfg(feature = "DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other getNamedItem:)]
        pub unsafe fn getNamedItem(&self, name: Option<&NSString>) -> Option<Retained<DOMNode>>;

        #[cfg(feature = "DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other setNamedItem:)]
        pub unsafe fn setNamedItem(&self, node: Option<&DOMNode>) -> Option<Retained<DOMNode>>;

        #[cfg(feature = "DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other removeNamedItem:)]
        pub unsafe fn removeNamedItem(&self, name: Option<&NSString>) -> Option<Retained<DOMNode>>;

        #[cfg(feature = "DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other item:)]
        pub unsafe fn item(&self, index: c_uint) -> Option<Retained<DOMNode>>;

        #[cfg(feature = "DOMNode")]
        #[method_id(@__retain_semantics Other getNamedItemNS:localName:)]
        pub unsafe fn getNamedItemNS_localName(
            &self,
            namespace_uri: Option<&NSString>,
            local_name: Option<&NSString>,
        ) -> Option<Retained<DOMNode>>;

        #[cfg(feature = "DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other setNamedItemNS:)]
        pub unsafe fn setNamedItemNS(&self, node: Option<&DOMNode>) -> Option<Retained<DOMNode>>;

        #[cfg(feature = "DOMNode")]
        #[method_id(@__retain_semantics Other removeNamedItemNS:localName:)]
        pub unsafe fn removeNamedItemNS_localName(
            &self,
            namespace_uri: Option<&NSString>,
            local_name: Option<&NSString>,
        ) -> Option<Retained<DOMNode>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMNamedNodeMap {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMNamedNodeMap {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// DOMNamedNodeMapDeprecated
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMNamedNodeMap {
        #[cfg(feature = "DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other getNamedItemNS::)]
        pub unsafe fn getNamedItemNS(
            &self,
            namespace_uri: Option<&NSString>,
            local_name: Option<&NSString>,
        ) -> Option<Retained<DOMNode>>;

        #[cfg(feature = "DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other removeNamedItemNS::)]
        pub unsafe fn removeNamedItemNS(
            &self,
            namespace_uri: Option<&NSString>,
            local_name: Option<&NSString>,
        ) -> Option<Retained<DOMNode>>;
    }
);
