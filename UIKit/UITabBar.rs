//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
use objc2_quartz_core::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UITabBarItemPositioning(pub NSInteger);
impl UITabBarItemPositioning {
    #[doc(alias = "UITabBarItemPositioningAutomatic")]
    pub const Automatic: Self = Self(0);
    #[doc(alias = "UITabBarItemPositioningFill")]
    pub const Fill: Self = Self(1);
    #[doc(alias = "UITabBarItemPositioningCentered")]
    pub const Centered: Self = Self(2);
}

unsafe impl Encode for UITabBarItemPositioning {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UITabBarItemPositioning {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    pub struct UITabBar;

    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl ClassType for UITabBar {
        #[inherits(UIResponder, NSObject)]
        type Super = UIView;
        type ThreadKind = dyn MainThreadOnly;
    }
);

#[cfg(all(
    feature = "UIResponder",
    feature = "UIView",
    feature = "objc2-quartz-core"
))]
#[cfg(not(target_os = "watchos"))]
unsafe impl CALayerDelegate for UITabBar {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl NSCoding for UITabBar {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl NSObjectProtocol for UITabBar {}

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIAppearance for UITabBar {}

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIAppearanceContainer for UITabBar {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl UICoordinateSpace for UITabBar {}

#[cfg(all(
    feature = "UIDynamicBehavior",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIDynamicItem for UITabBar {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusEnvironment for UITabBar {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusItem for UITabBar {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusItemContainer for UITabBar {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl UIResponderStandardEditActions for UITabBar {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIView"
))]
unsafe impl UITraitEnvironment for UITabBar {}

extern_methods!(
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UITabBar {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Retained<ProtocolObject<dyn UITabBarDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn UITabBarDelegate>>);

        #[cfg(all(feature = "UIBarItem", feature = "UITabBarItem"))]
        #[method_id(@__retain_semantics Other items)]
        pub unsafe fn items(&self) -> Option<Retained<NSArray<UITabBarItem>>>;

        #[cfg(all(feature = "UIBarItem", feature = "UITabBarItem"))]
        #[method(setItems:)]
        pub unsafe fn setItems(&self, items: Option<&NSArray<UITabBarItem>>);

        #[cfg(all(feature = "UIBarItem", feature = "UITabBarItem"))]
        #[method_id(@__retain_semantics Other selectedItem)]
        pub unsafe fn selectedItem(&self) -> Option<Retained<UITabBarItem>>;

        #[cfg(all(feature = "UIBarItem", feature = "UITabBarItem"))]
        #[method(setSelectedItem:)]
        pub unsafe fn setSelectedItem(&self, selected_item: Option<&UITabBarItem>);

        #[cfg(all(feature = "UIBarItem", feature = "UITabBarItem"))]
        #[method(setItems:animated:)]
        pub unsafe fn setItems_animated(
            &self,
            items: Option<&NSArray<UITabBarItem>>,
            animated: bool,
        );

        #[cfg(all(feature = "UIBarItem", feature = "UITabBarItem"))]
        #[method(beginCustomizingItems:)]
        pub unsafe fn beginCustomizingItems(&self, items: &NSArray<UITabBarItem>);

        #[method(endCustomizingAnimated:)]
        pub unsafe fn endCustomizingAnimated(&self, animated: bool) -> bool;

        #[method(isCustomizing)]
        pub unsafe fn isCustomizing(&self) -> bool;

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other tintColor)]
        pub unsafe fn tintColor(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        #[method(setTintColor:)]
        pub unsafe fn setTintColor(&self, tint_color: Option<&UIColor>);

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other barTintColor)]
        pub unsafe fn barTintColor(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        #[method(setBarTintColor:)]
        pub unsafe fn setBarTintColor(&self, bar_tint_color: Option<&UIColor>);

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other unselectedItemTintColor)]
        pub unsafe fn unselectedItemTintColor(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        #[method(setUnselectedItemTintColor:)]
        pub unsafe fn setUnselectedItemTintColor(
            &self,
            unselected_item_tint_color: Option<&UIColor>,
        );

        #[cfg(feature = "UIColor")]
        #[deprecated]
        #[method_id(@__retain_semantics Other selectedImageTintColor)]
        pub unsafe fn selectedImageTintColor(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        #[deprecated]
        #[method(setSelectedImageTintColor:)]
        pub unsafe fn setSelectedImageTintColor(&self, selected_image_tint_color: Option<&UIColor>);

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other backgroundImage)]
        pub unsafe fn backgroundImage(&self) -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIImage")]
        #[method(setBackgroundImage:)]
        pub unsafe fn setBackgroundImage(&self, background_image: Option<&UIImage>);

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other selectionIndicatorImage)]
        pub unsafe fn selectionIndicatorImage(&self) -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIImage")]
        #[method(setSelectionIndicatorImage:)]
        pub unsafe fn setSelectionIndicatorImage(
            &self,
            selection_indicator_image: Option<&UIImage>,
        );

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other shadowImage)]
        pub unsafe fn shadowImage(&self) -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIImage")]
        #[method(setShadowImage:)]
        pub unsafe fn setShadowImage(&self, shadow_image: Option<&UIImage>);

        #[method(itemPositioning)]
        pub unsafe fn itemPositioning(&self) -> UITabBarItemPositioning;

        #[method(setItemPositioning:)]
        pub unsafe fn setItemPositioning(&self, item_positioning: UITabBarItemPositioning);

        #[method(itemWidth)]
        pub unsafe fn itemWidth(&self) -> CGFloat;

        #[method(setItemWidth:)]
        pub unsafe fn setItemWidth(&self, item_width: CGFloat);

        #[method(itemSpacing)]
        pub unsafe fn itemSpacing(&self) -> CGFloat;

        #[method(setItemSpacing:)]
        pub unsafe fn setItemSpacing(&self, item_spacing: CGFloat);

        #[cfg(feature = "UIInterface")]
        #[method(barStyle)]
        pub unsafe fn barStyle(&self) -> UIBarStyle;

        #[cfg(feature = "UIInterface")]
        #[method(setBarStyle:)]
        pub unsafe fn setBarStyle(&self, bar_style: UIBarStyle);

        #[method(isTranslucent)]
        pub unsafe fn isTranslucent(&self) -> bool;

        #[method(setTranslucent:)]
        pub unsafe fn setTranslucent(&self, translucent: bool);

        #[cfg(all(feature = "UIBarAppearance", feature = "UITabBarAppearance"))]
        #[method_id(@__retain_semantics Other standardAppearance)]
        pub unsafe fn standardAppearance(&self) -> Retained<UITabBarAppearance>;

        #[cfg(all(feature = "UIBarAppearance", feature = "UITabBarAppearance"))]
        #[method(setStandardAppearance:)]
        pub unsafe fn setStandardAppearance(&self, standard_appearance: &UITabBarAppearance);

        #[cfg(all(feature = "UIBarAppearance", feature = "UITabBarAppearance"))]
        #[method_id(@__retain_semantics Other scrollEdgeAppearance)]
        pub unsafe fn scrollEdgeAppearance(&self) -> Option<Retained<UITabBarAppearance>>;

        #[cfg(all(feature = "UIBarAppearance", feature = "UITabBarAppearance"))]
        #[method(setScrollEdgeAppearance:)]
        pub unsafe fn setScrollEdgeAppearance(
            &self,
            scroll_edge_appearance: Option<&UITabBarAppearance>,
        );

        #[method_id(@__retain_semantics Other leadingAccessoryView)]
        pub unsafe fn leadingAccessoryView(&self) -> Retained<UIView>;

        #[method_id(@__retain_semantics Other trailingAccessoryView)]
        pub unsafe fn trailingAccessoryView(&self) -> Retained<UIView>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UIView`
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UITabBar {
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
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UITabBar {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    pub unsafe trait UITabBarDelegate: NSObjectProtocol + MainThreadOnly {
        #[cfg(all(
            feature = "UIBarItem",
            feature = "UIResponder",
            feature = "UITabBarItem",
            feature = "UIView"
        ))]
        #[optional]
        #[method(tabBar:didSelectItem:)]
        unsafe fn tabBar_didSelectItem(&self, tab_bar: &UITabBar, item: &UITabBarItem);

        #[cfg(all(
            feature = "UIBarItem",
            feature = "UIResponder",
            feature = "UITabBarItem",
            feature = "UIView"
        ))]
        #[optional]
        #[method(tabBar:willBeginCustomizingItems:)]
        unsafe fn tabBar_willBeginCustomizingItems(
            &self,
            tab_bar: &UITabBar,
            items: &NSArray<UITabBarItem>,
        );

        #[cfg(all(
            feature = "UIBarItem",
            feature = "UIResponder",
            feature = "UITabBarItem",
            feature = "UIView"
        ))]
        #[optional]
        #[method(tabBar:didBeginCustomizingItems:)]
        unsafe fn tabBar_didBeginCustomizingItems(
            &self,
            tab_bar: &UITabBar,
            items: &NSArray<UITabBarItem>,
        );

        #[cfg(all(
            feature = "UIBarItem",
            feature = "UIResponder",
            feature = "UITabBarItem",
            feature = "UIView"
        ))]
        #[optional]
        #[method(tabBar:willEndCustomizingItems:changed:)]
        unsafe fn tabBar_willEndCustomizingItems_changed(
            &self,
            tab_bar: &UITabBar,
            items: &NSArray<UITabBarItem>,
            changed: bool,
        );

        #[cfg(all(
            feature = "UIBarItem",
            feature = "UIResponder",
            feature = "UITabBarItem",
            feature = "UIView"
        ))]
        #[optional]
        #[method(tabBar:didEndCustomizingItems:changed:)]
        unsafe fn tabBar_didEndCustomizingItems_changed(
            &self,
            tab_bar: &UITabBar,
            items: &NSArray<UITabBarItem>,
            changed: bool,
        );
    }

    unsafe impl ProtocolType for dyn UITabBarDelegate {}
);

extern_methods!(
    /// SpringLoading
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UITabBar {}
);

#[cfg(all(
    feature = "UIResponder",
    feature = "UISpringLoadedInteractionSupporting",
    feature = "UIView"
))]
unsafe impl UISpringLoadedInteractionSupporting for UITabBar {}
