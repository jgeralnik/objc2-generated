//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

#[deprecated]
pub const DOM_ANY_TYPE: c_uint = 0;
#[deprecated]
pub const DOM_NUMBER_TYPE: c_uint = 1;
#[deprecated]
pub const DOM_STRING_TYPE: c_uint = 2;
#[deprecated]
pub const DOM_BOOLEAN_TYPE: c_uint = 3;
#[deprecated]
pub const DOM_UNORDERED_NODE_ITERATOR_TYPE: c_uint = 4;
#[deprecated]
pub const DOM_ORDERED_NODE_ITERATOR_TYPE: c_uint = 5;
#[deprecated]
pub const DOM_UNORDERED_NODE_SNAPSHOT_TYPE: c_uint = 6;
#[deprecated]
pub const DOM_ORDERED_NODE_SNAPSHOT_TYPE: c_uint = 7;
#[deprecated]
pub const DOM_ANY_UNORDERED_NODE_TYPE: c_uint = 8;
#[deprecated]
pub const DOM_FIRST_ORDERED_NODE_TYPE: c_uint = 9;

extern_class!(
    #[unsafe(super(DOMObject, WebScriptObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    #[deprecated]
    pub struct DOMXPathResult;
);

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl NSCopying for DOMXPathResult {}

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl CopyingHelper for DOMXPathResult {
    type Result = Self;
}

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl NSObjectProtocol for DOMXPathResult {}

extern_methods!(
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMXPathResult {
        #[deprecated]
        #[method(resultType)]
        pub unsafe fn resultType(&self) -> c_ushort;

        #[deprecated]
        #[method(numberValue)]
        pub unsafe fn numberValue(&self) -> c_double;

        #[deprecated]
        #[method_id(@__retain_semantics Other stringValue)]
        pub unsafe fn stringValue(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(booleanValue)]
        pub unsafe fn booleanValue(&self) -> bool;

        #[cfg(feature = "DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other singleNodeValue)]
        pub unsafe fn singleNodeValue(&self) -> Option<Retained<DOMNode>>;

        #[deprecated]
        #[method(invalidIteratorState)]
        pub unsafe fn invalidIteratorState(&self) -> bool;

        #[deprecated]
        #[method(snapshotLength)]
        pub unsafe fn snapshotLength(&self) -> c_uint;

        #[cfg(feature = "DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other iterateNext)]
        pub unsafe fn iterateNext(&self) -> Option<Retained<DOMNode>>;

        #[cfg(feature = "DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other snapshotItem:)]
        pub unsafe fn snapshotItem(&self, index: c_uint) -> Option<Retained<DOMNode>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMXPathResult {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMXPathResult {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
