//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uidragpreviewtarget?language=objc)
    #[unsafe(super(UIPreviewTarget, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UITargetedPreview")]
    pub struct UIDragPreviewTarget;
);

#[cfg(feature = "UITargetedPreview")]
unsafe impl NSCopying for UIDragPreviewTarget {}

#[cfg(feature = "UITargetedPreview")]
unsafe impl CopyingHelper for UIDragPreviewTarget {
    type Result = Self;
}

#[cfg(feature = "UITargetedPreview")]
unsafe impl NSObjectProtocol for UIDragPreviewTarget {}

extern_methods!(
    #[cfg(feature = "UITargetedPreview")]
    unsafe impl UIDragPreviewTarget {}
);

extern_methods!(
    /// Methods declared on superclass `UIPreviewTarget`
    #[cfg(feature = "UITargetedPreview")]
    unsafe impl UIDragPreviewTarget {
        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Init initWithContainer:center:transform:)]
        pub unsafe fn initWithContainer_center_transform(
            this: Allocated<Self>,
            container: &UIView,
            center: CGPoint,
            transform: CGAffineTransform,
        ) -> Retained<Self>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Init initWithContainer:center:)]
        pub unsafe fn initWithContainer_center(
            this: Allocated<Self>,
            container: &UIView,
            center: CGPoint,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitargeteddragpreview?language=objc)
    #[unsafe(super(UITargetedPreview, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UITargetedPreview")]
    pub struct UITargetedDragPreview;
);

#[cfg(feature = "UITargetedPreview")]
unsafe impl NSCopying for UITargetedDragPreview {}

#[cfg(feature = "UITargetedPreview")]
unsafe impl CopyingHelper for UITargetedDragPreview {
    type Result = Self;
}

#[cfg(feature = "UITargetedPreview")]
unsafe impl NSObjectProtocol for UITargetedDragPreview {}

extern_methods!(
    #[cfg(feature = "UITargetedPreview")]
    unsafe impl UITargetedDragPreview {
        #[method_id(@__retain_semantics Other retargetedPreviewWithTarget:)]
        pub unsafe fn retargetedPreviewWithTarget(
            &self,
            new_target: &UIDragPreviewTarget,
        ) -> Retained<UITargetedDragPreview>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UITargetedPreview`
    #[cfg(feature = "UITargetedPreview")]
    unsafe impl UITargetedDragPreview {
        #[cfg(all(
            feature = "UIPreviewParameters",
            feature = "UIResponder",
            feature = "UIView"
        ))]
        #[method_id(@__retain_semantics Init initWithView:parameters:target:)]
        pub unsafe fn initWithView_parameters_target(
            this: Allocated<Self>,
            view: &UIView,
            parameters: &UIPreviewParameters,
            target: &UIPreviewTarget,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "UIPreviewParameters",
            feature = "UIResponder",
            feature = "UIView"
        ))]
        #[method_id(@__retain_semantics Init initWithView:parameters:)]
        pub unsafe fn initWithView_parameters(
            this: Allocated<Self>,
            view: &UIView,
            parameters: &UIPreviewParameters,
        ) -> Retained<Self>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Init initWithView:)]
        pub unsafe fn initWithView(this: Allocated<Self>, view: &UIView) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
