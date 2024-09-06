//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated]
    pub struct WebFrame;

    unsafe impl ClassType for WebFrame {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for WebFrame {}

extern_methods!(
    unsafe impl WebFrame {
        #[cfg(all(
            feature = "WebFrameView",
            feature = "WebView",
            feature = "objc2-app-kit"
        ))]
        #[cfg(target_os = "macos")]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithName:webFrameView:webView:)]
        pub unsafe fn initWithName_webFrameView_webView(
            this: Allocated<Self>,
            name: Option<&NSString>,
            view: Option<&WebFrameView>,
            web_view: Option<&WebView>,
        ) -> Option<Retained<Self>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[cfg(all(feature = "WebView", feature = "objc2-app-kit"))]
        #[cfg(target_os = "macos")]
        #[deprecated]
        #[method_id(@__retain_semantics Other webView)]
        pub unsafe fn webView(&self, mtm: MainThreadMarker) -> Option<Retained<WebView>>;

        #[cfg(all(feature = "WebFrameView", feature = "objc2-app-kit"))]
        #[cfg(target_os = "macos")]
        #[deprecated]
        #[method_id(@__retain_semantics Other frameView)]
        pub unsafe fn frameView(&self, mtm: MainThreadMarker) -> Option<Retained<WebFrameView>>;

        #[cfg(all(
            feature = "DOMDocument",
            feature = "DOMNode",
            feature = "DOMObject",
            feature = "WebScriptObject"
        ))]
        #[deprecated]
        #[method_id(@__retain_semantics Other DOMDocument)]
        pub unsafe fn DOMDocument(&self) -> Option<Retained<DOMDocument>>;

        #[cfg(all(
            feature = "DOMElement",
            feature = "DOMHTMLElement",
            feature = "DOMNode",
            feature = "DOMObject",
            feature = "WebScriptObject"
        ))]
        #[deprecated]
        #[method_id(@__retain_semantics Other frameElement)]
        pub unsafe fn frameElement(&self) -> Option<Retained<DOMHTMLElement>>;

        #[deprecated]
        #[method(loadRequest:)]
        pub unsafe fn loadRequest(&self, request: Option<&NSURLRequest>);

        #[deprecated]
        #[method(loadData:MIMEType:textEncodingName:baseURL:)]
        pub unsafe fn loadData_MIMEType_textEncodingName_baseURL(
            &self,
            data: Option<&NSData>,
            mime_type: Option<&NSString>,
            encoding_name: Option<&NSString>,
            url: Option<&NSURL>,
        );

        #[deprecated]
        #[method(loadHTMLString:baseURL:)]
        pub unsafe fn loadHTMLString_baseURL(&self, string: Option<&NSString>, url: Option<&NSURL>);

        #[deprecated]
        #[method(loadAlternateHTMLString:baseURL:forUnreachableURL:)]
        pub unsafe fn loadAlternateHTMLString_baseURL_forUnreachableURL(
            &self,
            string: Option<&NSString>,
            base_url: Option<&NSURL>,
            unreachable_url: Option<&NSURL>,
        );

        #[cfg(feature = "WebArchive")]
        #[deprecated]
        #[method(loadArchive:)]
        pub unsafe fn loadArchive(&self, archive: Option<&WebArchive>);

        #[cfg(feature = "WebDataSource")]
        #[deprecated]
        #[method_id(@__retain_semantics Other dataSource)]
        pub unsafe fn dataSource(&self) -> Option<Retained<WebDataSource>>;

        #[cfg(feature = "WebDataSource")]
        #[deprecated]
        #[method_id(@__retain_semantics Other provisionalDataSource)]
        pub unsafe fn provisionalDataSource(&self) -> Option<Retained<WebDataSource>>;

        #[deprecated]
        #[method(stopLoading)]
        pub unsafe fn stopLoading(&self);

        #[deprecated]
        #[method(reload)]
        pub unsafe fn reload(&self);

        #[deprecated]
        #[method(reloadFromOrigin)]
        pub unsafe fn reloadFromOrigin(&self);

        #[deprecated]
        #[method_id(@__retain_semantics Other findFrameNamed:)]
        pub unsafe fn findFrameNamed(&self, name: Option<&NSString>) -> Option<Retained<WebFrame>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other parentFrame)]
        pub unsafe fn parentFrame(&self) -> Option<Retained<WebFrame>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other childFrames)]
        pub unsafe fn childFrames(&self) -> Retained<NSArray>;

        #[cfg(feature = "WebScriptObject")]
        #[deprecated]
        #[method_id(@__retain_semantics Other windowObject)]
        pub unsafe fn windowObject(&self) -> Option<Retained<WebScriptObject>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl WebFrame {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
