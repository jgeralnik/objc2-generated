//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uidocumentinteractioncontroller?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIDocumentInteractionController;
);

unsafe impl NSObjectProtocol for UIDocumentInteractionController {}

#[cfg(feature = "UIActionSheet")]
unsafe impl UIActionSheetDelegate for UIDocumentInteractionController {}

extern_methods!(
    unsafe impl UIDocumentInteractionController {
        #[method_id(@__retain_semantics Other interactionControllerWithURL:)]
        pub unsafe fn interactionControllerWithURL(
            url: &NSURL,
            mtm: MainThreadMarker,
        ) -> Retained<UIDocumentInteractionController>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UIDocumentInteractionControllerDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn UIDocumentInteractionControllerDelegate>>,
        );

        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Option<Retained<NSURL>>;

        #[method(setURL:)]
        pub unsafe fn setURL(&self, url: Option<&NSURL>);

        #[method_id(@__retain_semantics Other UTI)]
        pub unsafe fn UTI(&self) -> Option<Retained<NSString>>;

        #[method(setUTI:)]
        pub unsafe fn setUTI(&self, uti: Option<&NSString>);

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Retained<NSString>>;

        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other icons)]
        pub unsafe fn icons(&self) -> Retained<NSArray<UIImage>>;

        #[method_id(@__retain_semantics Other annotation)]
        pub unsafe fn annotation(&self) -> Option<Retained<AnyObject>>;

        #[method(setAnnotation:)]
        pub unsafe fn setAnnotation(&self, annotation: Option<&AnyObject>);

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method(presentOptionsMenuFromRect:inView:animated:)]
        pub unsafe fn presentOptionsMenuFromRect_inView_animated(
            &self,
            rect: CGRect,
            view: &UIView,
            animated: bool,
        ) -> bool;

        #[cfg(all(feature = "UIBarButtonItem", feature = "UIBarItem"))]
        #[method(presentOptionsMenuFromBarButtonItem:animated:)]
        pub unsafe fn presentOptionsMenuFromBarButtonItem_animated(
            &self,
            item: &UIBarButtonItem,
            animated: bool,
        ) -> bool;

        #[method(presentPreviewAnimated:)]
        pub unsafe fn presentPreviewAnimated(&self, animated: bool) -> bool;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method(presentOpenInMenuFromRect:inView:animated:)]
        pub unsafe fn presentOpenInMenuFromRect_inView_animated(
            &self,
            rect: CGRect,
            view: &UIView,
            animated: bool,
        ) -> bool;

        #[cfg(all(feature = "UIBarButtonItem", feature = "UIBarItem"))]
        #[method(presentOpenInMenuFromBarButtonItem:animated:)]
        pub unsafe fn presentOpenInMenuFromBarButtonItem_animated(
            &self,
            item: &UIBarButtonItem,
            animated: bool,
        ) -> bool;

        #[method(dismissPreviewAnimated:)]
        pub unsafe fn dismissPreviewAnimated(&self, animated: bool);

        #[method(dismissMenuAnimated:)]
        pub unsafe fn dismissMenuAnimated(&self, animated: bool);

        #[cfg(feature = "UIGestureRecognizer")]
        #[method_id(@__retain_semantics Other gestureRecognizers)]
        pub unsafe fn gestureRecognizers(&self) -> Retained<NSArray<UIGestureRecognizer>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIDocumentInteractionController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uidocumentinteractioncontrollerdelegate?language=objc)
    pub unsafe trait UIDocumentInteractionControllerDelegate: NSObjectProtocol {
        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method_id(@__retain_semantics Other documentInteractionControllerViewControllerForPreview:)]
        unsafe fn documentInteractionControllerViewControllerForPreview(
            &self,
            controller: &UIDocumentInteractionController,
        ) -> Retained<UIViewController>;

        #[optional]
        #[method(documentInteractionControllerRectForPreview:)]
        unsafe fn documentInteractionControllerRectForPreview(
            &self,
            controller: &UIDocumentInteractionController,
        ) -> CGRect;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[method_id(@__retain_semantics Other documentInteractionControllerViewForPreview:)]
        unsafe fn documentInteractionControllerViewForPreview(
            &self,
            controller: &UIDocumentInteractionController,
        ) -> Option<Retained<UIView>>;

        #[optional]
        #[method(documentInteractionControllerWillBeginPreview:)]
        unsafe fn documentInteractionControllerWillBeginPreview(
            &self,
            controller: &UIDocumentInteractionController,
        );

        #[optional]
        #[method(documentInteractionControllerDidEndPreview:)]
        unsafe fn documentInteractionControllerDidEndPreview(
            &self,
            controller: &UIDocumentInteractionController,
        );

        #[optional]
        #[method(documentInteractionControllerWillPresentOptionsMenu:)]
        unsafe fn documentInteractionControllerWillPresentOptionsMenu(
            &self,
            controller: &UIDocumentInteractionController,
        );

        #[optional]
        #[method(documentInteractionControllerDidDismissOptionsMenu:)]
        unsafe fn documentInteractionControllerDidDismissOptionsMenu(
            &self,
            controller: &UIDocumentInteractionController,
        );

        #[optional]
        #[method(documentInteractionControllerWillPresentOpenInMenu:)]
        unsafe fn documentInteractionControllerWillPresentOpenInMenu(
            &self,
            controller: &UIDocumentInteractionController,
        );

        #[optional]
        #[method(documentInteractionControllerDidDismissOpenInMenu:)]
        unsafe fn documentInteractionControllerDidDismissOpenInMenu(
            &self,
            controller: &UIDocumentInteractionController,
        );

        #[optional]
        #[method(documentInteractionController:willBeginSendingToApplication:)]
        unsafe fn documentInteractionController_willBeginSendingToApplication(
            &self,
            controller: &UIDocumentInteractionController,
            application: Option<&NSString>,
        );

        #[optional]
        #[method(documentInteractionController:didEndSendingToApplication:)]
        unsafe fn documentInteractionController_didEndSendingToApplication(
            &self,
            controller: &UIDocumentInteractionController,
            application: Option<&NSString>,
        );

        #[deprecated]
        #[optional]
        #[method(documentInteractionController:canPerformAction:)]
        unsafe fn documentInteractionController_canPerformAction(
            &self,
            controller: &UIDocumentInteractionController,
            action: Option<Sel>,
        ) -> bool;

        #[deprecated]
        #[optional]
        #[method(documentInteractionController:performAction:)]
        unsafe fn documentInteractionController_performAction(
            &self,
            controller: &UIDocumentInteractionController,
            action: Option<Sel>,
        ) -> bool;
    }

    unsafe impl ProtocolType for dyn UIDocumentInteractionControllerDelegate {}
);
