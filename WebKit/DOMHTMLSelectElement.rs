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
    pub struct DOMHTMLSelectElement;

    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMHTMLElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl ClassType for DOMHTMLSelectElement {
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
unsafe impl DOMEventTarget for DOMHTMLSelectElement {}

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSCopying for DOMHTMLSelectElement {}

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl CopyingHelper for DOMHTMLSelectElement {
    type Result = Self;
}

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSObjectProtocol for DOMHTMLSelectElement {}

extern_methods!(
    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMHTMLElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMHTMLSelectElement {
        #[method(autofocus)]
        pub unsafe fn autofocus(&self) -> bool;

        #[method(setAutofocus:)]
        pub unsafe fn setAutofocus(&self, autofocus: bool);

        #[deprecated]
        #[method(disabled)]
        pub unsafe fn disabled(&self) -> bool;

        #[deprecated]
        #[method(setDisabled:)]
        pub unsafe fn setDisabled(&self, disabled: bool);

        #[cfg(feature = "DOMHTMLFormElement")]
        #[deprecated]
        #[method_id(@__retain_semantics Other form)]
        pub unsafe fn form(&self) -> Option<Retained<DOMHTMLFormElement>>;

        #[deprecated]
        #[method(multiple)]
        pub unsafe fn multiple(&self) -> bool;

        #[deprecated]
        #[method(setMultiple:)]
        pub unsafe fn setMultiple(&self, multiple: bool);

        #[deprecated]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[deprecated]
        #[method(size)]
        pub unsafe fn size(&self) -> c_int;

        #[deprecated]
        #[method(setSize:)]
        pub unsafe fn setSize(&self, size: c_int);

        #[deprecated]
        #[method_id(@__retain_semantics Other type)]
        pub unsafe fn r#type(&self) -> Retained<NSString>;

        #[cfg(feature = "DOMHTMLOptionsCollection")]
        #[deprecated]
        #[method_id(@__retain_semantics Other options)]
        pub unsafe fn options(&self) -> Option<Retained<DOMHTMLOptionsCollection>>;

        #[deprecated]
        #[method(length)]
        pub unsafe fn length(&self) -> c_int;

        #[deprecated]
        #[method(selectedIndex)]
        pub unsafe fn selectedIndex(&self) -> c_int;

        #[deprecated]
        #[method(setSelectedIndex:)]
        pub unsafe fn setSelectedIndex(&self, selected_index: c_int);

        #[deprecated]
        #[method_id(@__retain_semantics Other value)]
        pub unsafe fn value(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setValue:)]
        pub unsafe fn setValue(&self, value: Option<&NSString>);

        #[method(willValidate)]
        pub unsafe fn willValidate(&self) -> bool;

        #[method_id(@__retain_semantics Other item:)]
        pub unsafe fn item(&self, index: c_uint) -> Option<Retained<DOMNode>>;

        #[method_id(@__retain_semantics Other namedItem:)]
        pub unsafe fn namedItem(&self, name: Option<&NSString>) -> Option<Retained<DOMNode>>;

        #[method(add:before:)]
        pub unsafe fn add_before(
            &self,
            element: Option<&DOMHTMLElement>,
            before: Option<&DOMHTMLElement>,
        );

        #[deprecated]
        #[method(remove:)]
        pub unsafe fn remove(&self, index: c_int);
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
    unsafe impl DOMHTMLSelectElement {
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
    unsafe impl DOMHTMLSelectElement {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// DOMHTMLSelectElementDeprecated
    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMHTMLElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMHTMLSelectElement {
        #[deprecated]
        #[method(add::)]
        pub unsafe fn add(&self, element: Option<&DOMHTMLElement>, before: Option<&DOMHTMLElement>);
    }
);
