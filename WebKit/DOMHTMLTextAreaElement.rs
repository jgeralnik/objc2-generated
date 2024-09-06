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
    pub struct DOMHTMLTextAreaElement;

    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMHTMLElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl ClassType for DOMHTMLTextAreaElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
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
unsafe impl DOMEventTarget for DOMHTMLTextAreaElement {}

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSCopying for DOMHTMLTextAreaElement {}

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl CopyingHelper for DOMHTMLTextAreaElement {
    type Result = Self;
}

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSObjectProtocol for DOMHTMLTextAreaElement {}

extern_methods!(
    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMHTMLElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMHTMLTextAreaElement {
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
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[deprecated]
        #[method(readOnly)]
        pub unsafe fn readOnly(&self) -> bool;

        #[deprecated]
        #[method(setReadOnly:)]
        pub unsafe fn setReadOnly(&self, read_only: bool);

        #[deprecated]
        #[method(rows)]
        pub unsafe fn rows(&self) -> c_int;

        #[deprecated]
        #[method(setRows:)]
        pub unsafe fn setRows(&self, rows: c_int);

        #[deprecated]
        #[method(cols)]
        pub unsafe fn cols(&self) -> c_int;

        #[deprecated]
        #[method(setCols:)]
        pub unsafe fn setCols(&self, cols: c_int);

        #[deprecated]
        #[method_id(@__retain_semantics Other type)]
        pub unsafe fn r#type(&self) -> Retained<NSString>;

        #[deprecated]
        #[method_id(@__retain_semantics Other defaultValue)]
        pub unsafe fn defaultValue(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setDefaultValue:)]
        pub unsafe fn setDefaultValue(&self, default_value: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other value)]
        pub unsafe fn value(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setValue:)]
        pub unsafe fn setValue(&self, value: Option<&NSString>);

        #[method(willValidate)]
        pub unsafe fn willValidate(&self) -> bool;

        #[method(selectionStart)]
        pub unsafe fn selectionStart(&self) -> c_int;

        #[method(setSelectionStart:)]
        pub unsafe fn setSelectionStart(&self, selection_start: c_int);

        #[method(selectionEnd)]
        pub unsafe fn selectionEnd(&self) -> c_int;

        #[method(setSelectionEnd:)]
        pub unsafe fn setSelectionEnd(&self, selection_end: c_int);

        #[deprecated]
        #[method_id(@__retain_semantics Other accessKey)]
        pub unsafe fn accessKey(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setAccessKey:)]
        pub unsafe fn setAccessKey(&self, access_key: Option<&NSString>);

        #[deprecated]
        #[method(select)]
        pub unsafe fn select(&self);

        #[method(setSelectionRange:end:)]
        pub unsafe fn setSelectionRange_end(&self, start: c_int, end: c_int);
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
    unsafe impl DOMHTMLTextAreaElement {
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
    unsafe impl DOMHTMLTextAreaElement {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
