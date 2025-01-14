//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uidynamicanimatordelegate?language=objc)
    pub unsafe trait UIDynamicAnimatorDelegate: NSObjectProtocol + MainThreadOnly {
        #[optional]
        #[method(dynamicAnimatorWillResume:)]
        unsafe fn dynamicAnimatorWillResume(&self, animator: &UIDynamicAnimator);

        #[optional]
        #[method(dynamicAnimatorDidPause:)]
        unsafe fn dynamicAnimatorDidPause(&self, animator: &UIDynamicAnimator);
    }

    unsafe impl ProtocolType for dyn UIDynamicAnimatorDelegate {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uidynamicanimator?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIDynamicAnimator;
);

unsafe impl NSObjectProtocol for UIDynamicAnimator {}

extern_methods!(
    unsafe impl UIDynamicAnimator {
        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Init initWithReferenceView:)]
        pub unsafe fn initWithReferenceView(this: Allocated<Self>, view: &UIView)
            -> Retained<Self>;

        #[cfg(feature = "UIDynamicBehavior")]
        #[method(addBehavior:)]
        pub unsafe fn addBehavior(&self, behavior: &UIDynamicBehavior);

        #[cfg(feature = "UIDynamicBehavior")]
        #[method(removeBehavior:)]
        pub unsafe fn removeBehavior(&self, behavior: &UIDynamicBehavior);

        #[method(removeAllBehaviors)]
        pub unsafe fn removeAllBehaviors(&self);

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Other referenceView)]
        pub unsafe fn referenceView(&self, mtm: MainThreadMarker) -> Option<Retained<UIView>>;

        #[cfg(feature = "UIDynamicBehavior")]
        #[method_id(@__retain_semantics Other behaviors)]
        pub unsafe fn behaviors(
            &self,
            mtm: MainThreadMarker,
        ) -> Retained<NSArray<UIDynamicBehavior>>;

        #[cfg(feature = "UIDynamicBehavior")]
        #[method_id(@__retain_semantics Other itemsInRect:)]
        pub unsafe fn itemsInRect(
            &self,
            rect: CGRect,
            mtm: MainThreadMarker,
        ) -> Retained<NSArray<ProtocolObject<dyn UIDynamicItem>>>;

        #[cfg(feature = "UIDynamicBehavior")]
        #[method(updateItemUsingCurrentState:)]
        pub unsafe fn updateItemUsingCurrentState(&self, item: &ProtocolObject<dyn UIDynamicItem>);

        #[method(isRunning)]
        pub unsafe fn isRunning(&self) -> bool;

        #[method(elapsedTime)]
        pub unsafe fn elapsedTime(&self) -> NSTimeInterval;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
            mtm: MainThreadMarker,
        ) -> Option<Retained<ProtocolObject<dyn UIDynamicAnimatorDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn UIDynamicAnimatorDelegate>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIDynamicAnimator {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// UICollectionViewAdditions
    unsafe impl UIDynamicAnimator {
        #[cfg(feature = "UICollectionViewLayout")]
        #[method_id(@__retain_semantics Init initWithCollectionViewLayout:)]
        pub unsafe fn initWithCollectionViewLayout(
            this: Allocated<Self>,
            layout: &UICollectionViewLayout,
        ) -> Retained<Self>;

        #[cfg(feature = "UICollectionViewLayout")]
        #[method_id(@__retain_semantics Other layoutAttributesForCellAtIndexPath:)]
        pub unsafe fn layoutAttributesForCellAtIndexPath(
            &self,
            index_path: &NSIndexPath,
            mtm: MainThreadMarker,
        ) -> Option<Retained<UICollectionViewLayoutAttributes>>;

        #[cfg(feature = "UICollectionViewLayout")]
        #[method_id(@__retain_semantics Other layoutAttributesForSupplementaryViewOfKind:atIndexPath:)]
        pub unsafe fn layoutAttributesForSupplementaryViewOfKind_atIndexPath(
            &self,
            kind: &NSString,
            index_path: &NSIndexPath,
            mtm: MainThreadMarker,
        ) -> Option<Retained<UICollectionViewLayoutAttributes>>;

        #[cfg(feature = "UICollectionViewLayout")]
        #[method_id(@__retain_semantics Other layoutAttributesForDecorationViewOfKind:atIndexPath:)]
        pub unsafe fn layoutAttributesForDecorationViewOfKind_atIndexPath(
            &self,
            decoration_view_kind: &NSString,
            index_path: &NSIndexPath,
            mtm: MainThreadMarker,
        ) -> Option<Retained<UICollectionViewLayoutAttributes>>;
    }
);
