//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMDocumentFragment")]
    #[deprecated]
    pub struct DOMDocumentFragment;

    #[cfg(feature = "WebKit_DOMDocumentFragment")]
    unsafe impl ClassType for DOMDocumentFragment {
        #[inherits(DOMObject, WebScriptObject, NSObject)]
        type Super = DOMNode;
    }
);

extern_methods!(
    #[cfg(feature = "WebKit_DOMDocumentFragment")]
    unsafe impl DOMDocumentFragment {}
);