//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(UICollectionViewLayout, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UICollectionViewLayout")]
    pub struct UICollectionViewTransitionLayout;
);

#[cfg(feature = "UICollectionViewLayout")]
unsafe impl NSCoding for UICollectionViewTransitionLayout {}

#[cfg(feature = "UICollectionViewLayout")]
unsafe impl NSObjectProtocol for UICollectionViewTransitionLayout {}

extern_methods!(
    #[cfg(feature = "UICollectionViewLayout")]
    unsafe impl UICollectionViewTransitionLayout {
        #[method(transitionProgress)]
        pub unsafe fn transitionProgress(&self) -> CGFloat;

        #[method(setTransitionProgress:)]
        pub unsafe fn setTransitionProgress(&self, transition_progress: CGFloat);

        #[method_id(@__retain_semantics Other currentLayout)]
        pub unsafe fn currentLayout(&self) -> Retained<UICollectionViewLayout>;

        #[method_id(@__retain_semantics Other nextLayout)]
        pub unsafe fn nextLayout(&self) -> Retained<UICollectionViewLayout>;

        #[method_id(@__retain_semantics Init initWithCurrentLayout:nextLayout:)]
        pub unsafe fn initWithCurrentLayout_nextLayout(
            this: Allocated<Self>,
            current_layout: &UICollectionViewLayout,
            new_layout: &UICollectionViewLayout,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method(updateValue:forAnimatedKey:)]
        pub unsafe fn updateValue_forAnimatedKey(&self, value: CGFloat, key: &NSString);

        #[method(valueForAnimatedKey:)]
        pub unsafe fn valueForAnimatedKey(&self, key: &NSString) -> CGFloat;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UICollectionViewLayout")]
    unsafe impl UICollectionViewTransitionLayout {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
