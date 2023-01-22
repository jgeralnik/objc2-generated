//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMCSSImportRule")]
    #[deprecated]
    pub struct DOMCSSImportRule;

    #[cfg(feature = "WebKit_DOMCSSImportRule")]
    unsafe impl ClassType for DOMCSSImportRule {
        #[inherits(DOMObject, WebScriptObject, NSObject)]
        type Super = DOMCSSRule;
    }
);

extern_methods!(
    #[cfg(feature = "WebKit_DOMCSSImportRule")]
    unsafe impl DOMCSSImportRule {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other href)]
        pub unsafe fn href(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "WebKit_DOMMediaList")]
        #[method_id(@__retain_semantics Other media)]
        pub unsafe fn media(&self) -> Option<Id<DOMMediaList, Shared>>;

        #[cfg(feature = "WebKit_DOMCSSStyleSheet")]
        #[method_id(@__retain_semantics Other styleSheet)]
        pub unsafe fn styleSheet(&self) -> Option<Id<DOMCSSStyleSheet, Shared>>;
    }
);