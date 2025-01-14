//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/webkit/wkpermissiondecision?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WKPermissionDecision(pub NSInteger);
impl WKPermissionDecision {
    #[doc(alias = "WKPermissionDecisionPrompt")]
    pub const Prompt: Self = Self(0);
    #[doc(alias = "WKPermissionDecisionGrant")]
    pub const Grant: Self = Self(1);
    #[doc(alias = "WKPermissionDecisionDeny")]
    pub const Deny: Self = Self(2);
}

unsafe impl Encode for WKPermissionDecision {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for WKPermissionDecision {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/webkit/wkmediacapturetype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WKMediaCaptureType(pub NSInteger);
impl WKMediaCaptureType {
    #[doc(alias = "WKMediaCaptureTypeCamera")]
    pub const Camera: Self = Self(0);
    #[doc(alias = "WKMediaCaptureTypeMicrophone")]
    pub const Microphone: Self = Self(1);
    #[doc(alias = "WKMediaCaptureTypeCameraAndMicrophone")]
    pub const CameraAndMicrophone: Self = Self(2);
}

unsafe impl Encode for WKMediaCaptureType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for WKMediaCaptureType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/webkit/wkdialogresult?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WKDialogResult(pub NSInteger);
impl WKDialogResult {
    #[doc(alias = "WKDialogResultShowDefault")]
    pub const ShowDefault: Self = Self(1);
    #[doc(alias = "WKDialogResultAskAgain")]
    pub const AskAgain: Self = Self(2);
    #[doc(alias = "WKDialogResultHandled")]
    pub const Handled: Self = Self(3);
}

unsafe impl Encode for WKDialogResult {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for WKDialogResult {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/wkuidelegate?language=objc)
    pub unsafe trait WKUIDelegate: NSObjectProtocol + MainThreadOnly {
        #[cfg(all(
            feature = "WKNavigationAction",
            feature = "WKWebView",
            feature = "WKWebViewConfiguration",
            feature = "WKWindowFeatures",
            feature = "objc2-app-kit"
        ))]
        #[cfg(target_os = "macos")]
        #[optional]
        #[method_id(@__retain_semantics Other webView:createWebViewWithConfiguration:forNavigationAction:windowFeatures:)]
        unsafe fn webView_createWebViewWithConfiguration_forNavigationAction_windowFeatures(
            &self,
            web_view: &WKWebView,
            configuration: &WKWebViewConfiguration,
            navigation_action: &WKNavigationAction,
            window_features: &WKWindowFeatures,
        ) -> Option<Retained<WKWebView>>;

        #[cfg(all(feature = "WKWebView", feature = "objc2-app-kit"))]
        #[cfg(target_os = "macos")]
        #[optional]
        #[method(webViewDidClose:)]
        unsafe fn webViewDidClose(&self, web_view: &WKWebView);

        #[cfg(all(
            feature = "WKFrameInfo",
            feature = "WKWebView",
            feature = "block2",
            feature = "objc2-app-kit"
        ))]
        #[cfg(target_os = "macos")]
        #[optional]
        #[method(webView:runJavaScriptAlertPanelWithMessage:initiatedByFrame:completionHandler:)]
        unsafe fn webView_runJavaScriptAlertPanelWithMessage_initiatedByFrame_completionHandler(
            &self,
            web_view: &WKWebView,
            message: &NSString,
            frame: &WKFrameInfo,
            completion_handler: &block2::Block<dyn Fn()>,
        );

        #[cfg(all(
            feature = "WKFrameInfo",
            feature = "WKWebView",
            feature = "block2",
            feature = "objc2-app-kit"
        ))]
        #[cfg(target_os = "macos")]
        #[optional]
        #[method(webView:runJavaScriptConfirmPanelWithMessage:initiatedByFrame:completionHandler:)]
        unsafe fn webView_runJavaScriptConfirmPanelWithMessage_initiatedByFrame_completionHandler(
            &self,
            web_view: &WKWebView,
            message: &NSString,
            frame: &WKFrameInfo,
            completion_handler: &block2::Block<dyn Fn(Bool)>,
        );

        #[cfg(all(
            feature = "WKFrameInfo",
            feature = "WKWebView",
            feature = "block2",
            feature = "objc2-app-kit"
        ))]
        #[cfg(target_os = "macos")]
        #[optional]
        #[method(webView:runJavaScriptTextInputPanelWithPrompt:defaultText:initiatedByFrame:completionHandler:)]
        unsafe fn webView_runJavaScriptTextInputPanelWithPrompt_defaultText_initiatedByFrame_completionHandler(
            &self,
            web_view: &WKWebView,
            prompt: &NSString,
            default_text: Option<&NSString>,
            frame: &WKFrameInfo,
            completion_handler: &block2::Block<dyn Fn(*mut NSString)>,
        );

        #[cfg(all(
            feature = "WKFrameInfo",
            feature = "WKSecurityOrigin",
            feature = "WKWebView",
            feature = "block2",
            feature = "objc2-app-kit"
        ))]
        #[cfg(target_os = "macos")]
        #[optional]
        #[method(webView:requestMediaCapturePermissionForOrigin:initiatedByFrame:type:decisionHandler:)]
        unsafe fn webView_requestMediaCapturePermissionForOrigin_initiatedByFrame_type_decisionHandler(
            &self,
            web_view: &WKWebView,
            origin: &WKSecurityOrigin,
            frame: &WKFrameInfo,
            r#type: WKMediaCaptureType,
            decision_handler: &block2::Block<dyn Fn(WKPermissionDecision)>,
        );

        #[cfg(all(
            feature = "WKFrameInfo",
            feature = "WKSecurityOrigin",
            feature = "WKWebView",
            feature = "block2",
            feature = "objc2-app-kit"
        ))]
        #[cfg(target_os = "macos")]
        #[optional]
        #[method(webView:requestDeviceOrientationAndMotionPermissionForOrigin:initiatedByFrame:decisionHandler:)]
        unsafe fn webView_requestDeviceOrientationAndMotionPermissionForOrigin_initiatedByFrame_decisionHandler(
            &self,
            web_view: &WKWebView,
            origin: &WKSecurityOrigin,
            frame: &WKFrameInfo,
            decision_handler: &block2::Block<dyn Fn(WKPermissionDecision)>,
        );

        #[cfg(all(
            feature = "WKFrameInfo",
            feature = "WKOpenPanelParameters",
            feature = "WKWebView",
            feature = "block2",
            feature = "objc2-app-kit"
        ))]
        #[cfg(target_os = "macos")]
        #[optional]
        #[method(webView:runOpenPanelWithParameters:initiatedByFrame:completionHandler:)]
        unsafe fn webView_runOpenPanelWithParameters_initiatedByFrame_completionHandler(
            &self,
            web_view: &WKWebView,
            parameters: &WKOpenPanelParameters,
            frame: &WKFrameInfo,
            completion_handler: &block2::Block<dyn Fn(*mut NSArray<NSURL>)>,
        );
    }

    unsafe impl ProtocolType for dyn WKUIDelegate {}
);
