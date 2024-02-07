//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum WKMediaPlaybackState {
        #[doc(alias = "WKMediaPlaybackStateNone")]
        None = 0,
        #[doc(alias = "WKMediaPlaybackStatePlaying")]
        Playing = 1,
        #[doc(alias = "WKMediaPlaybackStatePaused")]
        Paused = 2,
        #[doc(alias = "WKMediaPlaybackStateSuspended")]
        Suspended = 3,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum WKMediaCaptureState {
        #[doc(alias = "WKMediaCaptureStateNone")]
        None = 0,
        #[doc(alias = "WKMediaCaptureStateActive")]
        Active = 1,
        #[doc(alias = "WKMediaCaptureStateMuted")]
        Muted = 2,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum WKFullscreenState {
        #[doc(alias = "WKFullscreenStateNotInFullscreen")]
        NotInFullscreen = 0,
        #[doc(alias = "WKFullscreenStateEnteringFullscreen")]
        EnteringFullscreen = 1,
        #[doc(alias = "WKFullscreenStateInFullscreen")]
        InFullscreen = 2,
        #[doc(alias = "WKFullscreenStateExitingFullscreen")]
        ExitingFullscreen = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_WKWebView")]
    pub struct WKWebView;

    #[cfg(feature = "WebKit_WKWebView")]
    unsafe impl ClassType for WKWebView {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "WebKit_WKWebView")]
unsafe impl NSAccessibility for WKWebView {}

#[cfg(feature = "WebKit_WKWebView")]
unsafe impl NSAccessibilityElementProtocol for WKWebView {}

#[cfg(feature = "WebKit_WKWebView")]
unsafe impl NSAnimatablePropertyContainer for WKWebView {}

#[cfg(feature = "WebKit_WKWebView")]
unsafe impl NSAppearanceCustomization for WKWebView {}

#[cfg(feature = "WebKit_WKWebView")]
unsafe impl NSCoding for WKWebView {}

#[cfg(feature = "WebKit_WKWebView")]
unsafe impl NSDraggingDestination for WKWebView {}

#[cfg(feature = "WebKit_WKWebView")]
unsafe impl NSObjectProtocol for WKWebView {}

#[cfg(feature = "WebKit_WKWebView")]
unsafe impl NSUserInterfaceItemIdentification for WKWebView {}

extern_methods!(
    #[cfg(feature = "WebKit_WKWebView")]
    unsafe impl WKWebView {
        #[cfg(feature = "WebKit_WKWebViewConfiguration")]
        #[method_id(@__retain_semantics Other configuration)]
        pub unsafe fn configuration(&self) -> Id<WKWebViewConfiguration>;

        #[method_id(@__retain_semantics Other navigationDelegate)]
        pub unsafe fn navigationDelegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn WKNavigationDelegate>>>;

        #[method(setNavigationDelegate:)]
        pub unsafe fn setNavigationDelegate(
            &self,
            navigation_delegate: Option<&ProtocolObject<dyn WKNavigationDelegate>>,
        );

        #[method_id(@__retain_semantics Other UIDelegate)]
        pub unsafe fn UIDelegate(&self) -> Option<Id<ProtocolObject<dyn WKUIDelegate>>>;

        #[method(setUIDelegate:)]
        pub unsafe fn setUIDelegate(&self, ui_delegate: Option<&ProtocolObject<dyn WKUIDelegate>>);

        #[cfg(feature = "WebKit_WKBackForwardList")]
        #[method_id(@__retain_semantics Other backForwardList)]
        pub unsafe fn backForwardList(&self) -> Id<WKBackForwardList>;

        #[cfg(feature = "WebKit_WKWebViewConfiguration")]
        #[method_id(@__retain_semantics Init initWithFrame:configuration:)]
        pub unsafe fn initWithFrame_configuration(
            this: Allocated<Self>,
            frame: CGRect,
            configuration: &WKWebViewConfiguration,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;

        #[cfg(all(feature = "Foundation_NSURLRequest", feature = "WebKit_WKNavigation"))]
        #[method_id(@__retain_semantics Other loadRequest:)]
        pub unsafe fn loadRequest(&self, request: &NSURLRequest) -> Option<Id<WKNavigation>>;

        #[cfg(all(feature = "Foundation_NSURL", feature = "WebKit_WKNavigation"))]
        #[method_id(@__retain_semantics Other loadFileURL:allowingReadAccessToURL:)]
        pub unsafe fn loadFileURL_allowingReadAccessToURL(
            &self,
            url: &NSURL,
            read_access_url: &NSURL,
        ) -> Option<Id<WKNavigation>>;

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL",
            feature = "WebKit_WKNavigation"
        ))]
        #[method_id(@__retain_semantics Other loadHTMLString:baseURL:)]
        pub unsafe fn loadHTMLString_baseURL(
            &self,
            string: &NSString,
            base_url: Option<&NSURL>,
        ) -> Option<Id<WKNavigation>>;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL",
            feature = "WebKit_WKNavigation"
        ))]
        #[method_id(@__retain_semantics Other loadData:MIMEType:characterEncodingName:baseURL:)]
        pub unsafe fn loadData_MIMEType_characterEncodingName_baseURL(
            &self,
            data: &NSData,
            mime_type: &NSString,
            character_encoding_name: &NSString,
            base_url: &NSURL,
        ) -> Option<Id<WKNavigation>>;

        #[cfg(all(
            feature = "WebKit_WKBackForwardListItem",
            feature = "WebKit_WKNavigation"
        ))]
        #[method_id(@__retain_semantics Other goToBackForwardListItem:)]
        pub unsafe fn goToBackForwardListItem(
            &self,
            item: &WKBackForwardListItem,
        ) -> Option<Id<WKNavigation>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Option<Id<NSURL>>;

        #[method(isLoading)]
        pub unsafe fn isLoading(&self) -> bool;

        #[method(estimatedProgress)]
        pub unsafe fn estimatedProgress(&self) -> c_double;

        #[method(hasOnlySecureContent)]
        pub unsafe fn hasOnlySecureContent(&self) -> bool;

        #[method(canGoBack)]
        pub unsafe fn canGoBack(&self) -> bool;

        #[method(canGoForward)]
        pub unsafe fn canGoForward(&self) -> bool;

        #[cfg(feature = "WebKit_WKNavigation")]
        #[method_id(@__retain_semantics Other goBack)]
        pub unsafe fn goBack(&self) -> Option<Id<WKNavigation>>;

        #[cfg(feature = "WebKit_WKNavigation")]
        #[method_id(@__retain_semantics Other goForward)]
        pub unsafe fn goForward(&self) -> Option<Id<WKNavigation>>;

        #[cfg(feature = "WebKit_WKNavigation")]
        #[method_id(@__retain_semantics Other reload)]
        pub unsafe fn reload(&self) -> Option<Id<WKNavigation>>;

        #[cfg(feature = "WebKit_WKNavigation")]
        #[method_id(@__retain_semantics Other reloadFromOrigin)]
        pub unsafe fn reloadFromOrigin(&self) -> Option<Id<WKNavigation>>;

        #[method(stopLoading)]
        pub unsafe fn stopLoading(&self);

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(evaluateJavaScript:completionHandler:)]
        pub unsafe fn evaluateJavaScript_completionHandler(
            &self,
            java_script_string: &NSString,
            completion_handler: Option<&Block<dyn Fn(*mut AnyObject, *mut NSError)>>,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "WebKit_WKContentWorld",
            feature = "WebKit_WKFrameInfo"
        ))]
        #[method(evaluateJavaScript:inFrame:inContentWorld:completionHandler:)]
        pub unsafe fn evaluateJavaScript_inFrame_inContentWorld_completionHandler(
            &self,
            java_script_string: &NSString,
            frame: Option<&WKFrameInfo>,
            content_world: &WKContentWorld,
            completion_handler: Option<&Block<dyn Fn(*mut AnyObject, *mut NSError)>>,
        );

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "WebKit_WKContentWorld",
            feature = "WebKit_WKFrameInfo"
        ))]
        #[method(callAsyncJavaScript:arguments:inFrame:inContentWorld:completionHandler:)]
        pub unsafe fn callAsyncJavaScript_arguments_inFrame_inContentWorld_completionHandler(
            &self,
            function_body: &NSString,
            arguments: Option<&NSDictionary<NSString, AnyObject>>,
            frame: Option<&WKFrameInfo>,
            content_world: &WKContentWorld,
            completion_handler: Option<&Block<dyn Fn(*mut AnyObject, *mut NSError)>>,
        );

        #[method(closeAllMediaPresentationsWithCompletionHandler:)]
        pub unsafe fn closeAllMediaPresentationsWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<dyn Fn()>>,
        );

        #[deprecated]
        #[method(closeAllMediaPresentations)]
        pub unsafe fn closeAllMediaPresentations(&self);

        #[method(pauseAllMediaPlaybackWithCompletionHandler:)]
        pub unsafe fn pauseAllMediaPlaybackWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<dyn Fn()>>,
        );

        #[deprecated]
        #[method(pauseAllMediaPlayback:)]
        pub unsafe fn pauseAllMediaPlayback(&self, completion_handler: Option<&Block<dyn Fn()>>);

        #[method(setAllMediaPlaybackSuspended:completionHandler:)]
        pub unsafe fn setAllMediaPlaybackSuspended_completionHandler(
            &self,
            suspended: bool,
            completion_handler: Option<&Block<dyn Fn()>>,
        );

        #[deprecated]
        #[method(resumeAllMediaPlayback:)]
        pub unsafe fn resumeAllMediaPlayback(&self, completion_handler: Option<&Block<dyn Fn()>>);

        #[deprecated]
        #[method(suspendAllMediaPlayback:)]
        pub unsafe fn suspendAllMediaPlayback(&self, completion_handler: Option<&Block<dyn Fn()>>);

        #[method(requestMediaPlaybackStateWithCompletionHandler:)]
        pub unsafe fn requestMediaPlaybackStateWithCompletionHandler(
            &self,
            completion_handler: &Block<dyn Fn(WKMediaPlaybackState)>,
        );

        #[deprecated]
        #[method(requestMediaPlaybackState:)]
        pub unsafe fn requestMediaPlaybackState(
            &self,
            completion_handler: &Block<dyn Fn(WKMediaPlaybackState)>,
        );

        #[method(cameraCaptureState)]
        pub unsafe fn cameraCaptureState(&self) -> WKMediaCaptureState;

        #[method(microphoneCaptureState)]
        pub unsafe fn microphoneCaptureState(&self) -> WKMediaCaptureState;

        #[method(setCameraCaptureState:completionHandler:)]
        pub unsafe fn setCameraCaptureState_completionHandler(
            &self,
            state: WKMediaCaptureState,
            completion_handler: Option<&Block<dyn Fn()>>,
        );

        #[method(setMicrophoneCaptureState:completionHandler:)]
        pub unsafe fn setMicrophoneCaptureState_completionHandler(
            &self,
            state: WKMediaCaptureState,
            completion_handler: Option<&Block<dyn Fn()>>,
        );

        #[cfg(all(
            feature = "AppKit_NSImage",
            feature = "Foundation_NSError",
            feature = "WebKit_WKSnapshotConfiguration"
        ))]
        #[method(takeSnapshotWithConfiguration:completionHandler:)]
        pub unsafe fn takeSnapshotWithConfiguration_completionHandler(
            &self,
            snapshot_configuration: Option<&WKSnapshotConfiguration>,
            completion_handler: &Block<dyn Fn(*mut NSImage, *mut NSError)>,
        );

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "WebKit_WKPDFConfiguration"
        ))]
        #[method(createPDFWithConfiguration:completionHandler:)]
        pub unsafe fn createPDFWithConfiguration_completionHandler(
            &self,
            pdf_configuration: Option<&WKPDFConfiguration>,
            completion_handler: &Block<dyn Fn(*mut NSData, *mut NSError)>,
        );

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSError"))]
        #[method(createWebArchiveDataWithCompletionHandler:)]
        pub unsafe fn createWebArchiveDataWithCompletionHandler(
            &self,
            completion_handler: &Block<dyn Fn(NonNull<NSData>, NonNull<NSError>)>,
        );

        #[method(allowsBackForwardNavigationGestures)]
        pub unsafe fn allowsBackForwardNavigationGestures(&self) -> bool;

        #[method(setAllowsBackForwardNavigationGestures:)]
        pub unsafe fn setAllowsBackForwardNavigationGestures(
            &self,
            allows_back_forward_navigation_gestures: bool,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other customUserAgent)]
        pub unsafe fn customUserAgent(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCustomUserAgent:)]
        pub unsafe fn setCustomUserAgent(&self, custom_user_agent: Option<&NSString>);

        #[method(allowsLinkPreview)]
        pub unsafe fn allowsLinkPreview(&self) -> bool;

        #[method(setAllowsLinkPreview:)]
        pub unsafe fn setAllowsLinkPreview(&self, allows_link_preview: bool);

        #[method(allowsMagnification)]
        pub unsafe fn allowsMagnification(&self) -> bool;

        #[method(setAllowsMagnification:)]
        pub unsafe fn setAllowsMagnification(&self, allows_magnification: bool);

        #[method(magnification)]
        pub unsafe fn magnification(&self) -> CGFloat;

        #[method(setMagnification:)]
        pub unsafe fn setMagnification(&self, magnification: CGFloat);

        #[method(setMagnification:centeredAtPoint:)]
        pub unsafe fn setMagnification_centeredAtPoint(
            &self,
            magnification: CGFloat,
            point: CGPoint,
        );

        #[method(pageZoom)]
        pub unsafe fn pageZoom(&self) -> CGFloat;

        #[method(setPageZoom:)]
        pub unsafe fn setPageZoom(&self, page_zoom: CGFloat);

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "WebKit_WKFindConfiguration",
            feature = "WebKit_WKFindResult"
        ))]
        #[method(findString:withConfiguration:completionHandler:)]
        pub unsafe fn findString_withConfiguration_completionHandler(
            &self,
            string: &NSString,
            configuration: Option<&WKFindConfiguration>,
            completion_handler: &Block<dyn Fn(NonNull<WKFindResult>)>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(handlesURLScheme:)]
        pub unsafe fn handlesURLScheme(url_scheme: &NSString, mtm: MainThreadMarker) -> bool;

        #[cfg(all(feature = "Foundation_NSURLRequest", feature = "WebKit_WKDownload"))]
        #[method(startDownloadUsingRequest:completionHandler:)]
        pub unsafe fn startDownloadUsingRequest_completionHandler(
            &self,
            request: &NSURLRequest,
            completion_handler: &Block<dyn Fn(NonNull<WKDownload>)>,
        );

        #[cfg(all(feature = "Foundation_NSData", feature = "WebKit_WKDownload"))]
        #[method(resumeDownloadFromResumeData:completionHandler:)]
        pub unsafe fn resumeDownloadFromResumeData_completionHandler(
            &self,
            resume_data: &NSData,
            completion_handler: &Block<dyn Fn(NonNull<WKDownload>)>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other mediaType)]
        pub unsafe fn mediaType(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setMediaType:)]
        pub unsafe fn setMediaType(&self, media_type: Option<&NSString>);

        #[method_id(@__retain_semantics Other interactionState)]
        pub unsafe fn interactionState(&self) -> Option<Id<AnyObject>>;

        #[method(setInteractionState:)]
        pub unsafe fn setInteractionState(&self, interaction_state: Option<&AnyObject>);

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSURLRequest",
            feature = "Foundation_NSURLResponse",
            feature = "WebKit_WKNavigation"
        ))]
        #[method_id(@__retain_semantics Other loadSimulatedRequest:response:responseData:)]
        pub unsafe fn loadSimulatedRequest_response_responseData(
            &self,
            request: &NSURLRequest,
            response: &NSURLResponse,
            data: &NSData,
        ) -> Id<WKNavigation>;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSURLRequest",
            feature = "Foundation_NSURLResponse",
            feature = "WebKit_WKNavigation"
        ))]
        #[deprecated]
        #[method_id(@__retain_semantics Other loadSimulatedRequest:withResponse:responseData:)]
        pub unsafe fn loadSimulatedRequest_withResponse_responseData(
            &self,
            request: &NSURLRequest,
            response: &NSURLResponse,
            data: &NSData,
        ) -> Id<WKNavigation>;

        #[cfg(all(
            feature = "Foundation_NSURL",
            feature = "Foundation_NSURLRequest",
            feature = "WebKit_WKNavigation"
        ))]
        #[method_id(@__retain_semantics Other loadFileRequest:allowingReadAccessToURL:)]
        pub unsafe fn loadFileRequest_allowingReadAccessToURL(
            &self,
            request: &NSURLRequest,
            read_access_url: &NSURL,
        ) -> Id<WKNavigation>;

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "Foundation_NSURLRequest",
            feature = "WebKit_WKNavigation"
        ))]
        #[method_id(@__retain_semantics Other loadSimulatedRequest:responseHTMLString:)]
        pub unsafe fn loadSimulatedRequest_responseHTMLString(
            &self,
            request: &NSURLRequest,
            string: &NSString,
        ) -> Id<WKNavigation>;

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "Foundation_NSURLRequest",
            feature = "WebKit_WKNavigation"
        ))]
        #[deprecated]
        #[method_id(@__retain_semantics Other loadSimulatedRequest:withResponseHTMLString:)]
        pub unsafe fn loadSimulatedRequest_withResponseHTMLString(
            &self,
            request: &NSURLRequest,
            string: &NSString,
        ) -> Id<WKNavigation>;

        #[cfg(all(feature = "AppKit_NSPrintInfo", feature = "AppKit_NSPrintOperation"))]
        #[method_id(@__retain_semantics Other printOperationWithPrintInfo:)]
        pub unsafe fn printOperationWithPrintInfo(
            &self,
            print_info: &NSPrintInfo,
        ) -> Id<NSPrintOperation>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other themeColor)]
        pub unsafe fn themeColor(&self) -> Option<Id<NSColor>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other underPageBackgroundColor)]
        pub unsafe fn underPageBackgroundColor(&self) -> Id<NSColor>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setUnderPageBackgroundColor:)]
        pub unsafe fn setUnderPageBackgroundColor(
            &self,
            under_page_background_color: Option<&NSColor>,
        );

        #[method(fullscreenState)]
        pub unsafe fn fullscreenState(&self) -> WKFullscreenState;

        #[method(minimumViewportInset)]
        pub unsafe fn minimumViewportInset(&self) -> NSEdgeInsets;

        #[method(maximumViewportInset)]
        pub unsafe fn maximumViewportInset(&self) -> NSEdgeInsets;

        #[method(setMinimumViewportInset:maximumViewportInset:)]
        pub unsafe fn setMinimumViewportInset_maximumViewportInset(
            &self,
            minimum_viewport_inset: NSEdgeInsets,
            maximum_viewport_inset: NSEdgeInsets,
        );

        #[method(isInspectable)]
        pub unsafe fn isInspectable(&self) -> bool;

        #[method(setInspectable:)]
        pub unsafe fn setInspectable(&self, inspectable: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "WebKit_WKWebView")]
    unsafe impl WKWebView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "WebKit_WKWebView")]
    unsafe impl WKWebView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_WKWebView")]
    unsafe impl WKWebView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_methods!(
    /// WKIBActions
    #[cfg(feature = "WebKit_WKWebView")]
    unsafe impl WKWebView {
        #[method(goBack:)]
        pub unsafe fn goBack_(&self, sender: Option<&AnyObject>);

        #[method(goForward:)]
        pub unsafe fn goForward_(&self, sender: Option<&AnyObject>);

        #[method(reload:)]
        pub unsafe fn reload_(&self, sender: Option<&AnyObject>);

        #[method(reloadFromOrigin:)]
        pub unsafe fn reloadFromOrigin_(&self, sender: Option<&AnyObject>);

        #[method(stopLoading:)]
        pub unsafe fn stopLoading_(&self, sender: Option<&AnyObject>);
    }
);

#[cfg(feature = "WebKit_WKWebView")]
unsafe impl NSUserInterfaceValidations for WKWebView {}

extern_methods!(
    /// WKNSTextFinderClient
    #[cfg(feature = "WebKit_WKWebView")]
    unsafe impl WKWebView {}
);

#[cfg(feature = "WebKit_WKWebView")]
unsafe impl NSTextFinderClient for WKWebView {}

extern_methods!(
    /// WKDeprecated
    #[cfg(feature = "WebKit_WKWebView")]
    unsafe impl WKWebView {
        #[cfg(feature = "Foundation_NSArray")]
        #[deprecated]
        #[method_id(@__retain_semantics Other certificateChain)]
        pub unsafe fn certificateChain(&self) -> Id<NSArray>;
    }
);
