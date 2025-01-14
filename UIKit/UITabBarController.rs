//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitabbarcontrollermode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UITabBarControllerMode(pub NSInteger);
impl UITabBarControllerMode {
    #[doc(alias = "UITabBarControllerModeAutomatic")]
    pub const Automatic: Self = Self(0);
    #[doc(alias = "UITabBarControllerModeTabBar")]
    pub const TabBar: Self = Self(1);
    #[doc(alias = "UITabBarControllerModeTabSidebar")]
    pub const TabSidebar: Self = Self(2);
}

unsafe impl Encode for UITabBarControllerMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UITabBarControllerMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitabbarcontroller?language=objc)
    #[unsafe(super(UIViewController, UIResponder, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    pub struct UITabBarController;
);

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl NSCoding for UITabBarController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl NSObjectProtocol for UITabBarController {}

#[cfg(all(
    feature = "UIAppearance",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl UIAppearanceContainer for UITabBarController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl UIContentContainer for UITabBarController {}

#[cfg(all(
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl UIFocusEnvironment for UITabBarController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl UIResponderStandardEditActions for UITabBarController {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UITabBar",
    feature = "UIViewController"
))]
unsafe impl UITabBarDelegate for UITabBarController {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIViewController"
))]
unsafe impl UITraitEnvironment for UITabBarController {}

extern_methods!(
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UITabBarController {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UITabBarControllerDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn UITabBarControllerDelegate>>,
        );

        #[method(mode)]
        pub unsafe fn mode(&self) -> UITabBarControllerMode;

        #[method(setMode:)]
        pub unsafe fn setMode(&self, mode: UITabBarControllerMode);

        #[cfg(feature = "UITabBarControllerSidebar")]
        #[method_id(@__retain_semantics Other sidebar)]
        pub unsafe fn sidebar(&self) -> Retained<UITabBarControllerSidebar>;

        #[method_id(@__retain_semantics Other customizationIdentifier)]
        pub unsafe fn customizationIdentifier(&self) -> Option<Retained<NSString>>;

        #[method(setCustomizationIdentifier:)]
        pub unsafe fn setCustomizationIdentifier(
            &self,
            customization_identifier: Option<&NSString>,
        );

        #[method_id(@__retain_semantics Other compactTabIdentifiers)]
        pub unsafe fn compactTabIdentifiers(&self) -> Option<Retained<NSArray<NSString>>>;

        #[method(setCompactTabIdentifiers:)]
        pub unsafe fn setCompactTabIdentifiers(
            &self,
            compact_tab_identifiers: Option<&NSArray<NSString>>,
        );

        #[cfg(feature = "UITab")]
        #[method_id(@__retain_semantics Other selectedTab)]
        pub unsafe fn selectedTab(&self) -> Option<Retained<UITab>>;

        #[cfg(feature = "UITab")]
        #[method(setSelectedTab:)]
        pub unsafe fn setSelectedTab(&self, selected_tab: Option<&UITab>);

        #[cfg(feature = "UITab")]
        #[method_id(@__retain_semantics Other tabs)]
        pub unsafe fn tabs(&self) -> Retained<NSArray<UITab>>;

        #[cfg(feature = "UITab")]
        #[method(setTabs:)]
        pub unsafe fn setTabs(&self, tabs: &NSArray<UITab>);

        #[cfg(feature = "UITab")]
        #[method(setTabs:animated:)]
        pub unsafe fn setTabs_animated(&self, tabs: &NSArray<UITab>, animated: bool);

        #[cfg(feature = "UITab")]
        #[method_id(@__retain_semantics Other tabForIdentifier:)]
        pub unsafe fn tabForIdentifier(&self, identifier: &NSString) -> Option<Retained<UITab>>;

        #[cfg(feature = "UITab")]
        #[method_id(@__retain_semantics Init initWithTabs:)]
        pub unsafe fn initWithTabs(this: Allocated<Self>, tabs: &NSArray<UITab>) -> Retained<Self>;

        #[method(isTabBarHidden)]
        pub unsafe fn isTabBarHidden(&self) -> bool;

        #[method(setTabBarHidden:)]
        pub unsafe fn setTabBarHidden(&self, tab_bar_hidden: bool);

        #[method(setTabBarHidden:animated:)]
        pub unsafe fn setTabBarHidden_animated(&self, hidden: bool, animated: bool);

        #[method_id(@__retain_semantics Other viewControllers)]
        pub unsafe fn viewControllers(&self) -> Option<Retained<NSArray<UIViewController>>>;

        #[method(setViewControllers:)]
        pub unsafe fn setViewControllers(
            &self,
            view_controllers: Option<&NSArray<UIViewController>>,
        );

        #[method(setViewControllers:animated:)]
        pub unsafe fn setViewControllers_animated(
            &self,
            view_controllers: Option<&NSArray<UIViewController>>,
            animated: bool,
        );

        #[method_id(@__retain_semantics Other selectedViewController)]
        pub unsafe fn selectedViewController(&self) -> Option<Retained<UIViewController>>;

        #[method(setSelectedViewController:)]
        pub unsafe fn setSelectedViewController(
            &self,
            selected_view_controller: Option<&UIViewController>,
        );

        #[method(selectedIndex)]
        pub unsafe fn selectedIndex(&self) -> NSUInteger;

        #[method(setSelectedIndex:)]
        pub unsafe fn setSelectedIndex(&self, selected_index: NSUInteger);

        #[cfg(feature = "UINavigationController")]
        #[method_id(@__retain_semantics Other moreNavigationController)]
        pub unsafe fn moreNavigationController(&self) -> Retained<UINavigationController>;

        #[method_id(@__retain_semantics Other customizableViewControllers)]
        pub unsafe fn customizableViewControllers(
            &self,
        ) -> Option<Retained<NSArray<UIViewController>>>;

        #[method(setCustomizableViewControllers:)]
        pub unsafe fn setCustomizableViewControllers(
            &self,
            customizable_view_controllers: Option<&NSArray<UIViewController>>,
        );

        #[cfg(all(feature = "UITabBar", feature = "UIView"))]
        #[method_id(@__retain_semantics Other tabBar)]
        pub unsafe fn tabBar(&self) -> Retained<UITabBar>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UIViewController`
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UITabBarController {
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name_or_nil: Option<&NSString>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UITabBarController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitabbarcontrollerdelegate?language=objc)
    pub unsafe trait UITabBarControllerDelegate: NSObjectProtocol + MainThreadOnly {
        #[cfg(all(
            feature = "UIResponder",
            feature = "UITab",
            feature = "UIViewController"
        ))]
        #[optional]
        #[method(tabBarController:shouldSelectTab:)]
        unsafe fn tabBarController_shouldSelectTab(
            &self,
            tab_bar_controller: &UITabBarController,
            tab: &UITab,
        ) -> bool;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UITab",
            feature = "UIViewController"
        ))]
        #[optional]
        #[method(tabBarController:didSelectTab:previousTab:)]
        unsafe fn tabBarController_didSelectTab_previousTab(
            &self,
            tab_bar_controller: &UITabBarController,
            selected_tab: &UITab,
            previous_tab: Option<&UITab>,
        );

        #[cfg(all(
            feature = "UIDragSession",
            feature = "UIDropInteraction",
            feature = "UIResponder",
            feature = "UITab",
            feature = "UIViewController"
        ))]
        #[optional]
        #[method(tabBarController:tab:operationForAcceptingItemsFromDropSession:)]
        unsafe fn tabBarController_tab_operationForAcceptingItemsFromDropSession(
            &self,
            tab_bar_controller: &UITabBarController,
            tab: &UITab,
            session: &ProtocolObject<dyn UIDropSession>,
        ) -> UIDropOperation;

        #[cfg(all(
            feature = "UIDragSession",
            feature = "UIResponder",
            feature = "UITab",
            feature = "UIViewController"
        ))]
        #[optional]
        #[method(tabBarController:tab:acceptItemsFromDropSession:)]
        unsafe fn tabBarController_tab_acceptItemsFromDropSession(
            &self,
            tab_bar_controller: &UITabBarController,
            tab: &UITab,
            session: &ProtocolObject<dyn UIDropSession>,
        );

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method(tabBarControllerWillBeginEditing:)]
        unsafe fn tabBarControllerWillBeginEditing(&self, tab_bar_controller: &UITabBarController);

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method(tabBarControllerDidEndEditing:)]
        unsafe fn tabBarControllerDidEndEditing(&self, tab_bar_controller: &UITabBarController);

        #[cfg(all(
            feature = "UIResponder",
            feature = "UITab",
            feature = "UIViewController"
        ))]
        #[optional]
        #[method(tabBarController:visibilityDidChangeForTabs:)]
        unsafe fn tabBarController_visibilityDidChangeForTabs(
            &self,
            tab_bar_controller: &UITabBarController,
            tabs: &NSArray<UITab>,
        );

        #[cfg(all(
            feature = "UIResponder",
            feature = "UITab",
            feature = "UITabGroup",
            feature = "UIViewController"
        ))]
        #[optional]
        #[method(tabBarController:displayOrderDidChangeForGroup:)]
        unsafe fn tabBarController_displayOrderDidChangeForGroup(
            &self,
            tab_bar_controller: &UITabBarController,
            group: &UITabGroup,
        );

        #[cfg(all(
            feature = "UIResponder",
            feature = "UITab",
            feature = "UIViewController"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other tabBarController:displayedViewControllersForTab:proposedViewControllers:)]
        unsafe fn tabBarController_displayedViewControllersForTab_proposedViewControllers(
            &self,
            tab_bar_controller: &UITabBarController,
            tab: &UITab,
            proposed_view_controllers: &NSArray<UIViewController>,
        ) -> Retained<NSArray<UIViewController>>;

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method(tabBarController:shouldSelectViewController:)]
        unsafe fn tabBarController_shouldSelectViewController(
            &self,
            tab_bar_controller: &UITabBarController,
            view_controller: &UIViewController,
        ) -> bool;

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method(tabBarController:didSelectViewController:)]
        unsafe fn tabBarController_didSelectViewController(
            &self,
            tab_bar_controller: &UITabBarController,
            view_controller: &UIViewController,
        );

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method(tabBarController:willBeginCustomizingViewControllers:)]
        unsafe fn tabBarController_willBeginCustomizingViewControllers(
            &self,
            tab_bar_controller: &UITabBarController,
            view_controllers: &NSArray<UIViewController>,
        );

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method(tabBarController:willEndCustomizingViewControllers:changed:)]
        unsafe fn tabBarController_willEndCustomizingViewControllers_changed(
            &self,
            tab_bar_controller: &UITabBarController,
            view_controllers: &NSArray<UIViewController>,
            changed: bool,
        );

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method(tabBarController:didEndCustomizingViewControllers:changed:)]
        unsafe fn tabBarController_didEndCustomizingViewControllers_changed(
            &self,
            tab_bar_controller: &UITabBarController,
            view_controllers: &NSArray<UIViewController>,
            changed: bool,
        );

        #[cfg(all(
            feature = "UIOrientation",
            feature = "UIResponder",
            feature = "UIViewController"
        ))]
        #[optional]
        #[method(tabBarControllerSupportedInterfaceOrientations:)]
        unsafe fn tabBarControllerSupportedInterfaceOrientations(
            &self,
            tab_bar_controller: &UITabBarController,
        ) -> UIInterfaceOrientationMask;

        #[cfg(all(
            feature = "UIOrientation",
            feature = "UIResponder",
            feature = "UIViewController"
        ))]
        #[optional]
        #[method(tabBarControllerPreferredInterfaceOrientationForPresentation:)]
        unsafe fn tabBarControllerPreferredInterfaceOrientationForPresentation(
            &self,
            tab_bar_controller: &UITabBarController,
        ) -> UIInterfaceOrientation;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIViewController",
            feature = "UIViewControllerTransitioning"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other tabBarController:interactionControllerForAnimationController:)]
        unsafe fn tabBarController_interactionControllerForAnimationController(
            &self,
            tab_bar_controller: &UITabBarController,
            animation_controller: &ProtocolObject<dyn UIViewControllerAnimatedTransitioning>,
        ) -> Option<Retained<ProtocolObject<dyn UIViewControllerInteractiveTransitioning>>>;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIViewController",
            feature = "UIViewControllerTransitioning"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other tabBarController:animationControllerForTransitionFromViewController:toViewController:)]
        unsafe fn tabBarController_animationControllerForTransitionFromViewController_toViewController(
            &self,
            tab_bar_controller: &UITabBarController,
            from_vc: &UIViewController,
            to_vc: &UIViewController,
        ) -> Option<Retained<ProtocolObject<dyn UIViewControllerAnimatedTransitioning>>>;
    }

    unsafe impl ProtocolType for dyn UITabBarControllerDelegate {}
);

extern_methods!(
    /// UITabBarControllerItem
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UIViewController {
        #[cfg(all(feature = "UIBarItem", feature = "UITabBarItem"))]
        #[method_id(@__retain_semantics Other tabBarItem)]
        pub unsafe fn tabBarItem(&self) -> Option<Retained<UITabBarItem>>;

        #[cfg(all(feature = "UIBarItem", feature = "UITabBarItem"))]
        #[method(setTabBarItem:)]
        pub unsafe fn setTabBarItem(&self, tab_bar_item: Option<&UITabBarItem>);

        #[method_id(@__retain_semantics Other tabBarController)]
        pub unsafe fn tabBarController(&self) -> Option<Retained<UITabBarController>>;

        #[cfg(all(feature = "UIScrollView", feature = "UIView"))]
        #[deprecated = "Use -setContentScrollView:forEdge: instead."]
        #[method_id(@__retain_semantics Other tabBarObservedScrollView)]
        pub unsafe fn tabBarObservedScrollView(&self) -> Option<Retained<UIScrollView>>;

        #[cfg(all(feature = "UIScrollView", feature = "UIView"))]
        #[deprecated = "Use -setContentScrollView:forEdge: instead."]
        #[method(setTabBarObservedScrollView:)]
        pub unsafe fn setTabBarObservedScrollView(
            &self,
            tab_bar_observed_scroll_view: Option<&UIScrollView>,
        );
    }
);
