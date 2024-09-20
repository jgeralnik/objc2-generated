//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UITabGroupSidebarAppearance(pub NSUInteger);
impl UITabGroupSidebarAppearance {
    #[doc(alias = "UITabGroupSidebarAppearanceAutomatic")]
    pub const Automatic: Self = Self(0);
    #[doc(alias = "UITabGroupSidebarAppearanceInline")]
    pub const Inline: Self = Self(1);
    #[doc(alias = "UITabGroupSidebarAppearanceRootSection")]
    pub const RootSection: Self = Self(2);
}

unsafe impl Encode for UITabGroupSidebarAppearance {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UITabGroupSidebarAppearance {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UITab")]
    pub struct UITabGroup;

    #[cfg(feature = "UITab")]
    unsafe impl ClassType for UITabGroup {
        #[inherits(NSObject)]
        type Super = UITab;
        type ThreadKind = dyn MainThreadOnly;
    }
);

#[cfg(feature = "UITab")]
unsafe impl NSObjectProtocol for UITabGroup {}

extern_methods!(
    #[cfg(feature = "UITab")]
    unsafe impl UITabGroup {
        #[method_id(@__retain_semantics Other selectedChild)]
        pub unsafe fn selectedChild(&self) -> Option<Retained<UITab>>;

        #[method(setSelectedChild:)]
        pub unsafe fn setSelectedChild(&self, selected_child: Option<&UITab>);

        #[method_id(@__retain_semantics Other defaultChildIdentifier)]
        pub unsafe fn defaultChildIdentifier(&self) -> Option<Retained<NSString>>;

        #[method(setDefaultChildIdentifier:)]
        pub unsafe fn setDefaultChildIdentifier(&self, default_child_identifier: Option<&NSString>);

        #[method_id(@__retain_semantics Other children)]
        pub unsafe fn children(&self) -> Retained<NSArray<UITab>>;

        #[method(setChildren:)]
        pub unsafe fn setChildren(&self, children: &NSArray<UITab>);

        #[method_id(@__retain_semantics Other displayOrderIdentifiers)]
        pub unsafe fn displayOrderIdentifiers(&self) -> Retained<NSArray<NSString>>;

        #[method(setDisplayOrderIdentifiers:)]
        pub unsafe fn setDisplayOrderIdentifiers(
            &self,
            display_order_identifiers: &NSArray<NSString>,
        );

        #[method(allowsReordering)]
        pub unsafe fn allowsReordering(&self) -> bool;

        #[method(setAllowsReordering:)]
        pub unsafe fn setAllowsReordering(&self, allows_reordering: bool);

        #[method_id(@__retain_semantics Other displayOrder)]
        pub unsafe fn displayOrder(&self) -> Retained<NSArray<UITab>>;

        #[method_id(@__retain_semantics Other tabForIdentifier:)]
        pub unsafe fn tabForIdentifier(&self, identifier: &NSString) -> Option<Retained<UITab>>;

        #[cfg(all(
            feature = "UINavigationController",
            feature = "UIResponder",
            feature = "UIViewController"
        ))]
        #[method_id(@__retain_semantics Other managingNavigationController)]
        pub unsafe fn managingNavigationController(
            &self,
        ) -> Option<Retained<UINavigationController>>;

        #[cfg(all(
            feature = "UINavigationController",
            feature = "UIResponder",
            feature = "UIViewController"
        ))]
        #[method(setManagingNavigationController:)]
        pub unsafe fn setManagingNavigationController(
            &self,
            managing_navigation_controller: Option<&UINavigationController>,
        );

        #[cfg(all(feature = "UIAction", feature = "UIMenuElement"))]
        #[method_id(@__retain_semantics Other sidebarActions)]
        pub unsafe fn sidebarActions(&self) -> Retained<NSArray<UIAction>>;

        #[cfg(all(feature = "UIAction", feature = "UIMenuElement"))]
        #[method(setSidebarActions:)]
        pub unsafe fn setSidebarActions(&self, sidebar_actions: &NSArray<UIAction>);

        #[method(sidebarAppearance)]
        pub unsafe fn sidebarAppearance(&self) -> UITabGroupSidebarAppearance;

        #[method(setSidebarAppearance:)]
        pub unsafe fn setSidebarAppearance(&self, sidebar_appearance: UITabGroupSidebarAppearance);

        #[cfg(all(
            feature = "UIImage",
            feature = "UIResponder",
            feature = "UIViewController",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Init initWithTitle:image:identifier:children:viewControllerProvider:)]
        pub unsafe fn initWithTitle_image_identifier_children_viewControllerProvider(
            this: Allocated<Self>,
            title: &NSString,
            image: Option<&UIImage>,
            identifier: &NSString,
            children: &NSArray<UITab>,
            view_controller_provider: Option<
                &block2::Block<dyn Fn(NonNull<UITab>) -> NonNull<UIViewController>>,
            >,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UITab`
    #[cfg(feature = "UITab")]
    unsafe impl UITabGroup {
        #[cfg(all(
            feature = "UIImage",
            feature = "UIResponder",
            feature = "UIViewController",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Init initWithTitle:image:identifier:viewControllerProvider:)]
        pub unsafe fn initWithTitle_image_identifier_viewControllerProvider(
            this: Allocated<Self>,
            title: &NSString,
            image: Option<&UIImage>,
            identifier: &NSString,
            view_controller_provider: Option<
                &block2::Block<dyn Fn(NonNull<UITab>) -> NonNull<UIViewController>>,
            >,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
