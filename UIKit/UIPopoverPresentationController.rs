//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipopoverpresentationcontrollerdelegate?language=objc)
    #[cfg(feature = "UIPresentationController")]
    pub unsafe trait UIPopoverPresentationControllerDelegate:
        UIAdaptivePresentationControllerDelegate + MainThreadOnly
    {
        #[optional]
        #[method(prepareForPopoverPresentation:)]
        unsafe fn prepareForPopoverPresentation(
            &self,
            popover_presentation_controller: &UIPopoverPresentationController,
        );

        #[deprecated]
        #[optional]
        #[method(popoverPresentationControllerShouldDismissPopover:)]
        unsafe fn popoverPresentationControllerShouldDismissPopover(
            &self,
            popover_presentation_controller: &UIPopoverPresentationController,
        ) -> bool;

        #[deprecated]
        #[optional]
        #[method(popoverPresentationControllerDidDismissPopover:)]
        unsafe fn popoverPresentationControllerDidDismissPopover(
            &self,
            popover_presentation_controller: &UIPopoverPresentationController,
        );

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[method(popoverPresentationController:willRepositionPopoverToRect:inView:)]
        unsafe fn popoverPresentationController_willRepositionPopoverToRect_inView(
            &self,
            popover_presentation_controller: &UIPopoverPresentationController,
            rect: NonNull<CGRect>,
            view: &mut Retained<UIView>,
        );
    }

    #[cfg(feature = "UIPresentationController")]
    unsafe impl ProtocolType for dyn UIPopoverPresentationControllerDelegate {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipopoverpresentationcontroller?language=objc)
    #[unsafe(super(UIPresentationController, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIPresentationController")]
    pub struct UIPopoverPresentationController;
);

#[cfg(feature = "UIPresentationController")]
unsafe impl NSObjectProtocol for UIPopoverPresentationController {}

#[cfg(all(feature = "UIAppearance", feature = "UIPresentationController"))]
unsafe impl UIAppearanceContainer for UIPopoverPresentationController {}

#[cfg(all(feature = "UIPresentationController", feature = "UIViewController"))]
unsafe impl UIContentContainer for UIPopoverPresentationController {}

#[cfg(all(feature = "UIFocus", feature = "UIPresentationController"))]
unsafe impl UIFocusEnvironment for UIPopoverPresentationController {}

#[cfg(all(feature = "UIPresentationController", feature = "UITraitCollection"))]
unsafe impl UITraitEnvironment for UIPopoverPresentationController {}

extern_methods!(
    #[cfg(feature = "UIPresentationController")]
    unsafe impl UIPopoverPresentationController {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UIPopoverPresentationControllerDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn UIPopoverPresentationControllerDelegate>>,
        );

        #[cfg(feature = "UIPopoverSupport")]
        #[method(permittedArrowDirections)]
        pub unsafe fn permittedArrowDirections(&self) -> UIPopoverArrowDirection;

        #[cfg(feature = "UIPopoverSupport")]
        #[method(setPermittedArrowDirections:)]
        pub unsafe fn setPermittedArrowDirections(
            &self,
            permitted_arrow_directions: UIPopoverArrowDirection,
        );

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Other sourceView)]
        pub unsafe fn sourceView(&self) -> Option<Retained<UIView>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method(setSourceView:)]
        pub unsafe fn setSourceView(&self, source_view: Option<&UIView>);

        #[method(sourceRect)]
        pub unsafe fn sourceRect(&self) -> CGRect;

        #[method(setSourceRect:)]
        pub unsafe fn setSourceRect(&self, source_rect: CGRect);

        #[method(canOverlapSourceViewRect)]
        pub unsafe fn canOverlapSourceViewRect(&self) -> bool;

        #[method(setCanOverlapSourceViewRect:)]
        pub unsafe fn setCanOverlapSourceViewRect(&self, can_overlap_source_view_rect: bool);

        #[cfg(feature = "UIPopoverPresentationControllerSourceItem")]
        #[method_id(@__retain_semantics Other sourceItem)]
        pub unsafe fn sourceItem(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UIPopoverPresentationControllerSourceItem>>>;

        #[cfg(feature = "UIPopoverPresentationControllerSourceItem")]
        #[method(setSourceItem:)]
        pub unsafe fn setSourceItem(
            &self,
            source_item: Option<&ProtocolObject<dyn UIPopoverPresentationControllerSourceItem>>,
        );

        #[cfg(all(feature = "UIBarButtonItem", feature = "UIBarItem"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other barButtonItem)]
        pub unsafe fn barButtonItem(&self) -> Option<Retained<UIBarButtonItem>>;

        #[cfg(all(feature = "UIBarButtonItem", feature = "UIBarItem"))]
        #[deprecated]
        #[method(setBarButtonItem:)]
        pub unsafe fn setBarButtonItem(&self, bar_button_item: Option<&UIBarButtonItem>);

        #[cfg(feature = "UIPopoverSupport")]
        #[method(arrowDirection)]
        pub unsafe fn arrowDirection(&self) -> UIPopoverArrowDirection;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Other passthroughViews)]
        pub unsafe fn passthroughViews(&self) -> Option<Retained<NSArray<UIView>>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method(setPassthroughViews:)]
        pub unsafe fn setPassthroughViews(&self, passthrough_views: Option<&NSArray<UIView>>);

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, background_color: Option<&UIColor>);

        #[cfg(feature = "UIGeometry")]
        #[method(popoverLayoutMargins)]
        pub unsafe fn popoverLayoutMargins(&self) -> UIEdgeInsets;

        #[cfg(feature = "UIGeometry")]
        #[method(setPopoverLayoutMargins:)]
        pub unsafe fn setPopoverLayoutMargins(&self, popover_layout_margins: UIEdgeInsets);

        #[cfg(feature = "UIPopoverBackgroundView")]
        #[method(popoverBackgroundViewClass)]
        pub unsafe fn popoverBackgroundViewClass(&self) -> Option<&'static AnyClass>;

        #[cfg(feature = "UIPopoverBackgroundView")]
        #[method(setPopoverBackgroundViewClass:)]
        pub unsafe fn setPopoverBackgroundViewClass(
            &self,
            popover_background_view_class: Option<&AnyClass>,
        );

        #[cfg(feature = "UISheetPresentationController")]
        #[method_id(@__retain_semantics Other adaptiveSheetPresentationController)]
        pub unsafe fn adaptiveSheetPresentationController(
            &self,
        ) -> Retained<UISheetPresentationController>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UIPresentationController`
    #[cfg(feature = "UIPresentationController")]
    unsafe impl UIPopoverPresentationController {
        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[method_id(@__retain_semantics Init initWithPresentedViewController:presentingViewController:)]
        pub unsafe fn initWithPresentedViewController_presentingViewController(
            this: Allocated<Self>,
            presented_view_controller: &UIViewController,
            presenting_view_controller: Option<&UIViewController>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UIPresentationController")]
    unsafe impl UIPopoverPresentationController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
