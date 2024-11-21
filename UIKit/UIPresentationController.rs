//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait UIAdaptivePresentationControllerDelegate:
        NSObjectProtocol + MainThreadOnly
    {
        #[cfg(feature = "UIViewController")]
        #[optional]
        #[method(adaptivePresentationStyleForPresentationController:)]
        unsafe fn adaptivePresentationStyleForPresentationController(
            &self,
            controller: &UIPresentationController,
        ) -> UIModalPresentationStyle;

        #[cfg(all(feature = "UITraitCollection", feature = "UIViewController"))]
        #[optional]
        #[method(adaptivePresentationStyleForPresentationController:traitCollection:)]
        unsafe fn adaptivePresentationStyleForPresentationController_traitCollection(
            &self,
            controller: &UIPresentationController,
            trait_collection: &UITraitCollection,
        ) -> UIModalPresentationStyle;

        #[optional]
        #[method(presentationController:prepareAdaptivePresentationController:)]
        unsafe fn presentationController_prepareAdaptivePresentationController(
            &self,
            presentation_controller: &UIPresentationController,
            adaptive_presentation_controller: &UIPresentationController,
        );

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method_id(@__retain_semantics Other presentationController:viewControllerForAdaptivePresentationStyle:)]
        unsafe fn presentationController_viewControllerForAdaptivePresentationStyle(
            &self,
            controller: &UIPresentationController,
            style: UIModalPresentationStyle,
        ) -> Option<Retained<UIViewController>>;

        #[cfg(all(
            feature = "UIViewController",
            feature = "UIViewControllerTransitionCoordinator"
        ))]
        #[optional]
        #[method(presentationController:willPresentWithAdaptiveStyle:transitionCoordinator:)]
        unsafe fn presentationController_willPresentWithAdaptiveStyle_transitionCoordinator(
            &self,
            presentation_controller: &UIPresentationController,
            style: UIModalPresentationStyle,
            transition_coordinator: Option<
                &ProtocolObject<dyn UIViewControllerTransitionCoordinator>,
            >,
        );

        #[optional]
        #[method(presentationControllerShouldDismiss:)]
        unsafe fn presentationControllerShouldDismiss(
            &self,
            presentation_controller: &UIPresentationController,
        ) -> bool;

        #[optional]
        #[method(presentationControllerWillDismiss:)]
        unsafe fn presentationControllerWillDismiss(
            &self,
            presentation_controller: &UIPresentationController,
        );

        #[optional]
        #[method(presentationControllerDidDismiss:)]
        unsafe fn presentationControllerDidDismiss(
            &self,
            presentation_controller: &UIPresentationController,
        );

        #[optional]
        #[method(presentationControllerDidAttemptToDismiss:)]
        unsafe fn presentationControllerDidAttemptToDismiss(
            &self,
            presentation_controller: &UIPresentationController,
        );
    }

    unsafe impl ProtocolType for dyn UIAdaptivePresentationControllerDelegate {}
);

extern_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIPresentationController;
);

unsafe impl NSObjectProtocol for UIPresentationController {}

#[cfg(feature = "UIAppearance")]
unsafe impl UIAppearanceContainer for UIPresentationController {}

#[cfg(feature = "UIViewController")]
unsafe impl UIContentContainer for UIPresentationController {}

#[cfg(feature = "UIFocus")]
unsafe impl UIFocusEnvironment for UIPresentationController {}

#[cfg(feature = "UITraitCollection")]
unsafe impl UITraitEnvironment for UIPresentationController {}

extern_methods!(
    unsafe impl UIPresentationController {
        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[method_id(@__retain_semantics Other presentingViewController)]
        pub unsafe fn presentingViewController(&self) -> Retained<UIViewController>;

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[method_id(@__retain_semantics Other presentedViewController)]
        pub unsafe fn presentedViewController(&self) -> Retained<UIViewController>;

        #[cfg(feature = "UIViewController")]
        #[method(presentationStyle)]
        pub unsafe fn presentationStyle(&self) -> UIModalPresentationStyle;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Other containerView)]
        pub unsafe fn containerView(&self) -> Option<Retained<UIView>>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UIAdaptivePresentationControllerDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn UIAdaptivePresentationControllerDelegate>>,
        );

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[method_id(@__retain_semantics Init initWithPresentedViewController:presentingViewController:)]
        pub unsafe fn initWithPresentedViewController_presentingViewController(
            this: Allocated<Self>,
            presented_view_controller: &UIViewController,
            presenting_view_controller: Option<&UIViewController>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "UIViewController")]
        #[method(adaptivePresentationStyle)]
        pub unsafe fn adaptivePresentationStyle(&self) -> UIModalPresentationStyle;

        #[cfg(all(feature = "UITraitCollection", feature = "UIViewController"))]
        #[method(adaptivePresentationStyleForTraitCollection:)]
        pub unsafe fn adaptivePresentationStyleForTraitCollection(
            &self,
            trait_collection: &UITraitCollection,
        ) -> UIModalPresentationStyle;

        #[method(containerViewWillLayoutSubviews)]
        pub unsafe fn containerViewWillLayoutSubviews(&self);

        #[method(containerViewDidLayoutSubviews)]
        pub unsafe fn containerViewDidLayoutSubviews(&self);

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Other presentedView)]
        pub unsafe fn presentedView(&self) -> Option<Retained<UIView>>;

        #[method(frameOfPresentedViewInContainerView)]
        pub unsafe fn frameOfPresentedViewInContainerView(&self) -> CGRect;

        #[method(shouldPresentInFullscreen)]
        pub unsafe fn shouldPresentInFullscreen(&self) -> bool;

        #[method(shouldRemovePresentersView)]
        pub unsafe fn shouldRemovePresentersView(&self) -> bool;

        #[method(presentationTransitionWillBegin)]
        pub unsafe fn presentationTransitionWillBegin(&self);

        #[method(presentationTransitionDidEnd:)]
        pub unsafe fn presentationTransitionDidEnd(&self, completed: bool);

        #[method(dismissalTransitionWillBegin)]
        pub unsafe fn dismissalTransitionWillBegin(&self);

        #[method(dismissalTransitionDidEnd:)]
        pub unsafe fn dismissalTransitionDidEnd(&self, completed: bool);

        #[cfg(feature = "UITraitCollection")]
        #[deprecated = "Use the traitOverrides property instead"]
        #[method_id(@__retain_semantics Other overrideTraitCollection)]
        pub unsafe fn overrideTraitCollection(&self) -> Option<Retained<UITraitCollection>>;

        #[cfg(feature = "UITraitCollection")]
        #[deprecated = "Use the traitOverrides property instead"]
        #[method(setOverrideTraitCollection:)]
        pub unsafe fn setOverrideTraitCollection(
            &self,
            override_trait_collection: Option<&UITraitCollection>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIPresentationController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    unsafe impl UIPresentationController {
        #[cfg(feature = "UITraitCollection")]
        #[method_id(@__retain_semantics Other traitOverrides)]
        pub unsafe fn traitOverrides(&self) -> Retained<ProtocolObject<dyn UITraitOverrides>>;
    }
);

#[cfg(feature = "UITraitCollection")]
unsafe impl UITraitChangeObservable for UIPresentationController {}
