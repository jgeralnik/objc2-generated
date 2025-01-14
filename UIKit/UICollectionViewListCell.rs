//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
use objc2_quartz_core::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionviewlistcell?language=objc)
    #[unsafe(super(
        UICollectionViewCell,
        UICollectionReusableView,
        UIView,
        UIResponder,
        NSObject
    ))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "UICollectionViewCell",
        feature = "UIResponder",
        feature = "UIView"
    ))]
    pub struct UICollectionViewListCell;
);

#[cfg(all(
    feature = "UICollectionViewCell",
    feature = "UIResponder",
    feature = "UIView",
    feature = "objc2-quartz-core"
))]
#[cfg(not(target_os = "watchos"))]
unsafe impl CALayerDelegate for UICollectionViewListCell {}

#[cfg(all(
    feature = "UICollectionViewCell",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl NSCoding for UICollectionViewListCell {}

#[cfg(all(
    feature = "UICollectionViewCell",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl NSObjectProtocol for UICollectionViewListCell {}

#[cfg(all(
    feature = "UIAppearance",
    feature = "UICollectionViewCell",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIAppearance for UICollectionViewListCell {}

#[cfg(all(
    feature = "UIAppearance",
    feature = "UICollectionViewCell",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIAppearanceContainer for UICollectionViewListCell {}

#[cfg(all(
    feature = "UICollectionViewCell",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UICoordinateSpace for UICollectionViewListCell {}

#[cfg(all(
    feature = "UICollectionViewCell",
    feature = "UIDynamicBehavior",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIDynamicItem for UICollectionViewListCell {}

#[cfg(all(
    feature = "UICollectionViewCell",
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIFocusEnvironment for UICollectionViewListCell {}

#[cfg(all(
    feature = "UICollectionViewCell",
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIFocusItem for UICollectionViewListCell {}

#[cfg(all(
    feature = "UICollectionViewCell",
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIFocusItemContainer for UICollectionViewListCell {}

#[cfg(all(
    feature = "UICollectionViewCell",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIResponderStandardEditActions for UICollectionViewListCell {}

#[cfg(all(
    feature = "UICollectionViewCell",
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIView"
))]
unsafe impl UITraitEnvironment for UICollectionViewListCell {}

extern_methods!(
    #[cfg(all(
        feature = "UICollectionViewCell",
        feature = "UIResponder",
        feature = "UIView"
    ))]
    unsafe impl UICollectionViewListCell {
        #[cfg(feature = "UIListContentConfiguration")]
        #[method_id(@__retain_semantics Other defaultContentConfiguration)]
        pub unsafe fn defaultContentConfiguration(&self) -> Retained<UIListContentConfiguration>;

        #[method(indentationLevel)]
        pub unsafe fn indentationLevel(&self) -> NSInteger;

        #[method(setIndentationLevel:)]
        pub unsafe fn setIndentationLevel(&self, indentation_level: NSInteger);

        #[method(indentationWidth)]
        pub unsafe fn indentationWidth(&self) -> CGFloat;

        #[method(setIndentationWidth:)]
        pub unsafe fn setIndentationWidth(&self, indentation_width: CGFloat);

        #[method(indentsAccessories)]
        pub unsafe fn indentsAccessories(&self) -> bool;

        #[method(setIndentsAccessories:)]
        pub unsafe fn setIndentsAccessories(&self, indents_accessories: bool);

        #[cfg(feature = "UICellAccessory")]
        #[method_id(@__retain_semantics Other accessories)]
        pub unsafe fn accessories(&self) -> Retained<NSArray<UICellAccessory>>;

        #[cfg(feature = "UICellAccessory")]
        #[method(setAccessories:)]
        pub unsafe fn setAccessories(&self, accessories: &NSArray<UICellAccessory>);

        #[cfg(feature = "UILayoutGuide")]
        #[method_id(@__retain_semantics Other separatorLayoutGuide)]
        pub unsafe fn separatorLayoutGuide(&self) -> Retained<UILayoutGuide>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UIView`
    #[cfg(all(
        feature = "UICollectionViewCell",
        feature = "UIResponder",
        feature = "UIView"
    ))]
    unsafe impl UICollectionViewListCell {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame: CGRect) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "UICollectionViewCell",
        feature = "UIResponder",
        feature = "UIView"
    ))]
    unsafe impl UICollectionViewListCell {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
