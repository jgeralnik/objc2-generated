//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
use objc2_quartz_core::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiupdatelink?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIUpdateLink;
);

unsafe impl NSObjectProtocol for UIUpdateLink {}

extern_methods!(
    unsafe impl UIUpdateLink {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIScene",
            feature = "UIWindowScene"
        ))]
        #[method_id(@__retain_semantics Other updateLinkForWindowScene:)]
        pub unsafe fn updateLinkForWindowScene(
            window_scene: &UIWindowScene,
        ) -> Retained<UIUpdateLink>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Other updateLinkForView:)]
        pub unsafe fn updateLinkForView(view: &UIView) -> Retained<UIUpdateLink>;

        #[cfg(all(
            feature = "UIUpdateActionPhase",
            feature = "UIUpdateInfo",
            feature = "block2"
        ))]
        #[method(addActionToPhase:handler:)]
        pub unsafe fn addActionToPhase_handler(
            &self,
            phase: &UIUpdateActionPhase,
            handler: &block2::Block<dyn Fn(NonNull<UIUpdateLink>, NonNull<UIUpdateInfo>)>,
        );

        #[cfg(feature = "UIUpdateActionPhase")]
        #[method(addActionToPhase:target:selector:)]
        pub unsafe fn addActionToPhase_target_selector(
            &self,
            phase: &UIUpdateActionPhase,
            target: &AnyObject,
            selector: Sel,
        );

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[method(requiresContinuousUpdates)]
        pub unsafe fn requiresContinuousUpdates(&self) -> bool;

        #[method(setRequiresContinuousUpdates:)]
        pub unsafe fn setRequiresContinuousUpdates(&self, requires_continuous_updates: bool);

        #[method(wantsLowLatencyEventDispatch)]
        pub unsafe fn wantsLowLatencyEventDispatch(&self) -> bool;

        #[method(setWantsLowLatencyEventDispatch:)]
        pub unsafe fn setWantsLowLatencyEventDispatch(
            &self,
            wants_low_latency_event_dispatch: bool,
        );

        #[method(wantsImmediatePresentation)]
        pub unsafe fn wantsImmediatePresentation(&self) -> bool;

        #[method(setWantsImmediatePresentation:)]
        pub unsafe fn setWantsImmediatePresentation(&self, wants_immediate_presentation: bool);

        #[cfg(feature = "objc2-quartz-core")]
        #[cfg(not(target_os = "watchos"))]
        #[method(preferredFrameRateRange)]
        pub unsafe fn preferredFrameRateRange(&self) -> CAFrameRateRange;

        #[cfg(feature = "objc2-quartz-core")]
        #[cfg(not(target_os = "watchos"))]
        #[method(setPreferredFrameRateRange:)]
        pub unsafe fn setPreferredFrameRateRange(
            &self,
            preferred_frame_rate_range: CAFrameRateRange,
        );

        #[cfg(feature = "UIUpdateInfo")]
        #[method_id(@__retain_semantics Other currentUpdateInfo)]
        pub unsafe fn currentUpdateInfo(&self) -> Option<Retained<UIUpdateInfo>>;
    }
);

extern_methods!(
    /// Convenience
    unsafe impl UIUpdateLink {
        #[cfg(all(feature = "UIUpdateInfo", feature = "block2"))]
        #[method(addActionWithHandler:)]
        pub unsafe fn addActionWithHandler(
            &self,
            handler: &block2::Block<dyn Fn(NonNull<UIUpdateLink>, NonNull<UIUpdateInfo>)>,
        );

        #[method(addActionWithTarget:selector:)]
        pub unsafe fn addActionWithTarget_selector(&self, target: &AnyObject, selector: Sel);

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIScene",
            feature = "UIUpdateInfo",
            feature = "UIWindowScene",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Other updateLinkForWindowScene:actionHandler:)]
        pub unsafe fn updateLinkForWindowScene_actionHandler(
            window_scene: &UIWindowScene,
            handler: &block2::Block<dyn Fn(NonNull<UIUpdateLink>, NonNull<UIUpdateInfo>)>,
        ) -> Retained<UIUpdateLink>;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIScene",
            feature = "UIWindowScene"
        ))]
        #[method_id(@__retain_semantics Other updateLinkForWindowScene:actionTarget:selector:)]
        pub unsafe fn updateLinkForWindowScene_actionTarget_selector(
            window_scene: &UIWindowScene,
            target: &AnyObject,
            selector: Sel,
        ) -> Retained<UIUpdateLink>;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIUpdateInfo",
            feature = "UIView",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Other updateLinkForView:actionHandler:)]
        pub unsafe fn updateLinkForView_actionHandler(
            view: &UIView,
            handler: &block2::Block<dyn Fn(NonNull<UIUpdateLink>, NonNull<UIUpdateInfo>)>,
        ) -> Retained<UIUpdateLink>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Other updateLinkForView:actionTarget:selector:)]
        pub unsafe fn updateLinkForView_actionTarget_selector(
            view: &UIView,
            target: &AnyObject,
            selector: Sel,
        ) -> Retained<UIUpdateLink>;
    }
);
