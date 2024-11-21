//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSResponder")]
    pub struct NSWindowController;
);

#[cfg(feature = "NSResponder")]
unsafe impl NSCoding for NSWindowController {}

#[cfg(feature = "NSResponder")]
unsafe impl NSObjectProtocol for NSWindowController {}

#[cfg(all(feature = "NSResponder", feature = "NSStoryboardSegue"))]
unsafe impl NSSeguePerforming for NSWindowController {}

extern_methods!(
    #[cfg(feature = "NSResponder")]
    unsafe impl NSWindowController {
        #[cfg(feature = "NSWindow")]
        #[method_id(@__retain_semantics Init initWithWindow:)]
        pub unsafe fn initWithWindow(
            this: Allocated<Self>,
            window: Option<&NSWindow>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "NSNib")]
        #[method_id(@__retain_semantics Init initWithWindowNibName:)]
        pub unsafe fn initWithWindowNibName(
            this: Allocated<Self>,
            window_nib_name: &NSNibName,
        ) -> Retained<Self>;

        #[cfg(feature = "NSNib")]
        #[method_id(@__retain_semantics Init initWithWindowNibName:owner:)]
        pub unsafe fn initWithWindowNibName_owner(
            this: Allocated<Self>,
            window_nib_name: &NSNibName,
            owner: &AnyObject,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithWindowNibPath:owner:)]
        pub unsafe fn initWithWindowNibPath_owner(
            this: Allocated<Self>,
            window_nib_path: &NSString,
            owner: &AnyObject,
        ) -> Retained<Self>;

        #[cfg(feature = "NSNib")]
        #[method_id(@__retain_semantics Other windowNibName)]
        pub unsafe fn windowNibName(&self) -> Option<Retained<NSNibName>>;

        #[method_id(@__retain_semantics Other windowNibPath)]
        pub unsafe fn windowNibPath(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other owner)]
        pub unsafe fn owner(&self) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSWindow")]
        #[method_id(@__retain_semantics Other windowFrameAutosaveName)]
        pub unsafe fn windowFrameAutosaveName(&self) -> Retained<NSWindowFrameAutosaveName>;

        #[cfg(feature = "NSWindow")]
        #[method(setWindowFrameAutosaveName:)]
        pub unsafe fn setWindowFrameAutosaveName(
            &self,
            window_frame_autosave_name: &NSWindowFrameAutosaveName,
        );

        #[method(shouldCascadeWindows)]
        pub unsafe fn shouldCascadeWindows(&self) -> bool;

        #[method(setShouldCascadeWindows:)]
        pub unsafe fn setShouldCascadeWindows(&self, should_cascade_windows: bool);

        #[cfg(feature = "NSPreviewRepresentingActivityItem")]
        #[method_id(@__retain_semantics Other previewRepresentableActivityItems)]
        pub unsafe fn previewRepresentableActivityItems(
            &self,
        ) -> Option<Retained<NSArray<ProtocolObject<dyn NSPreviewRepresentableActivityItem>>>>;

        #[cfg(feature = "NSPreviewRepresentingActivityItem")]
        #[method(setPreviewRepresentableActivityItems:)]
        pub unsafe fn setPreviewRepresentableActivityItems(
            &self,
            preview_representable_activity_items: Option<
                &NSArray<ProtocolObject<dyn NSPreviewRepresentableActivityItem>>,
            >,
        );

        #[method_id(@__retain_semantics Other document)]
        pub unsafe fn document(&self) -> Option<Retained<AnyObject>>;

        #[method(setDocument:)]
        pub unsafe fn setDocument(&self, document: Option<&AnyObject>);

        #[method(setDocumentEdited:)]
        pub unsafe fn setDocumentEdited(&self, dirty_flag: bool);

        #[method(shouldCloseDocument)]
        pub unsafe fn shouldCloseDocument(&self) -> bool;

        #[method(setShouldCloseDocument:)]
        pub unsafe fn setShouldCloseDocument(&self, should_close_document: bool);

        #[method(synchronizeWindowTitleWithDocumentName)]
        pub unsafe fn synchronizeWindowTitleWithDocumentName(&self);

        #[method_id(@__retain_semantics Other windowTitleForDocumentDisplayName:)]
        pub unsafe fn windowTitleForDocumentDisplayName(
            &self,
            display_name: &NSString,
        ) -> Retained<NSString>;

        #[cfg(feature = "NSViewController")]
        #[method_id(@__retain_semantics Other contentViewController)]
        pub unsafe fn contentViewController(&self) -> Option<Retained<NSViewController>>;

        #[cfg(feature = "NSViewController")]
        #[method(setContentViewController:)]
        pub unsafe fn setContentViewController(
            &self,
            content_view_controller: Option<&NSViewController>,
        );

        #[cfg(feature = "NSWindow")]
        #[method_id(@__retain_semantics Other window)]
        pub unsafe fn window(&self) -> Option<Retained<NSWindow>>;

        #[cfg(feature = "NSWindow")]
        #[method(setWindow:)]
        pub unsafe fn setWindow(&self, window: Option<&NSWindow>);

        #[method(isWindowLoaded)]
        pub unsafe fn isWindowLoaded(&self) -> bool;

        #[method(windowWillLoad)]
        pub unsafe fn windowWillLoad(&self);

        #[method(windowDidLoad)]
        pub unsafe fn windowDidLoad(&self);

        #[method(loadWindow)]
        pub unsafe fn loadWindow(&self);

        #[method(close)]
        pub unsafe fn close(&self);

        #[method(showWindow:)]
        pub unsafe fn showWindow(&self, sender: Option<&AnyObject>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "NSResponder")]
    unsafe impl NSWindowController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSResponder")]
    unsafe impl NSWindowController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    /// NSWindowControllerStoryboardingMethods
    #[cfg(feature = "NSResponder")]
    unsafe impl NSWindowController {
        #[cfg(feature = "NSStoryboard")]
        #[method_id(@__retain_semantics Other storyboard)]
        pub unsafe fn storyboard(&self) -> Option<Retained<NSStoryboard>>;
    }
);

extern_methods!(
    /// NSWindowControllerDismissing
    #[cfg(feature = "NSResponder")]
    unsafe impl NSWindowController {
        #[method(dismissController:)]
        pub unsafe fn dismissController(&self, sender: Option<&AnyObject>);
    }
);
