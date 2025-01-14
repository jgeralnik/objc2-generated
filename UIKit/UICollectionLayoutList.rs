//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionlayoutlistappearance?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UICollectionLayoutListAppearance(pub NSInteger);
impl UICollectionLayoutListAppearance {
    #[doc(alias = "UICollectionLayoutListAppearancePlain")]
    pub const Plain: Self = Self(0);
    #[doc(alias = "UICollectionLayoutListAppearanceGrouped")]
    pub const Grouped: Self = Self(1);
    #[doc(alias = "UICollectionLayoutListAppearanceInsetGrouped")]
    pub const InsetGrouped: Self = Self(2);
    #[doc(alias = "UICollectionLayoutListAppearanceSidebar")]
    pub const Sidebar: Self = Self(3);
    #[doc(alias = "UICollectionLayoutListAppearanceSidebarPlain")]
    pub const SidebarPlain: Self = Self(4);
}

unsafe impl Encode for UICollectionLayoutListAppearance {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UICollectionLayoutListAppearance {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionlayoutlistheadermode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UICollectionLayoutListHeaderMode(pub NSInteger);
impl UICollectionLayoutListHeaderMode {
    #[doc(alias = "UICollectionLayoutListHeaderModeNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "UICollectionLayoutListHeaderModeSupplementary")]
    pub const Supplementary: Self = Self(1);
    #[doc(alias = "UICollectionLayoutListHeaderModeFirstItemInSection")]
    pub const FirstItemInSection: Self = Self(2);
}

unsafe impl Encode for UICollectionLayoutListHeaderMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UICollectionLayoutListHeaderMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionlayoutlistfootermode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UICollectionLayoutListFooterMode(pub NSInteger);
impl UICollectionLayoutListFooterMode {
    #[doc(alias = "UICollectionLayoutListFooterModeNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "UICollectionLayoutListFooterModeSupplementary")]
    pub const Supplementary: Self = Self(1);
}

unsafe impl Encode for UICollectionLayoutListFooterMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UICollectionLayoutListFooterMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionlayoutlistswipeactionsconfigurationprovider?language=objc)
#[cfg(all(feature = "UISwipeActionsConfiguration", feature = "block2"))]
pub type UICollectionLayoutListSwipeActionsConfigurationProvider =
    *mut block2::Block<dyn Fn(NonNull<NSIndexPath>) -> *mut UISwipeActionsConfiguration>;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionlayoutlistitemseparatorhandler?language=objc)
#[cfg(all(feature = "UIListSeparatorConfiguration", feature = "block2"))]
pub type UICollectionLayoutListItemSeparatorHandler = *mut block2::Block<
    dyn Fn(
        NonNull<NSIndexPath>,
        NonNull<UIListSeparatorConfiguration>,
    ) -> NonNull<UIListSeparatorConfiguration>,
>;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionlayoutlistcontenthuggingelements?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UICollectionLayoutListContentHuggingElements(pub NSUInteger);
bitflags::bitflags! {
    impl UICollectionLayoutListContentHuggingElements: NSUInteger {
        #[doc(alias = "UICollectionLayoutListContentHuggingElementsNone")]
        const None = 0;
        #[doc(alias = "UICollectionLayoutListContentHuggingElementsSupplementaryHeader")]
        const SupplementaryHeader = 1<<0;
    }
}

unsafe impl Encode for UICollectionLayoutListContentHuggingElements {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UICollectionLayoutListContentHuggingElements {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionlayoutlistconfiguration?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UICollectionLayoutListConfiguration;
);

unsafe impl NSCopying for UICollectionLayoutListConfiguration {}

unsafe impl CopyingHelper for UICollectionLayoutListConfiguration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UICollectionLayoutListConfiguration {}

extern_methods!(
    unsafe impl UICollectionLayoutListConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithAppearance:)]
        pub unsafe fn initWithAppearance(
            this: Allocated<Self>,
            appearance: UICollectionLayoutListAppearance,
        ) -> Retained<Self>;

        #[method(appearance)]
        pub unsafe fn appearance(&self) -> UICollectionLayoutListAppearance;

        #[method(showsSeparators)]
        pub unsafe fn showsSeparators(&self) -> bool;

        #[method(setShowsSeparators:)]
        pub unsafe fn setShowsSeparators(&self, shows_separators: bool);

        #[cfg(feature = "UIListSeparatorConfiguration")]
        #[method_id(@__retain_semantics Other separatorConfiguration)]
        pub unsafe fn separatorConfiguration(&self) -> Retained<UIListSeparatorConfiguration>;

        #[cfg(feature = "UIListSeparatorConfiguration")]
        #[method(setSeparatorConfiguration:)]
        pub unsafe fn setSeparatorConfiguration(
            &self,
            separator_configuration: &UIListSeparatorConfiguration,
        );

        #[cfg(all(feature = "UIListSeparatorConfiguration", feature = "block2"))]
        #[method(itemSeparatorHandler)]
        pub unsafe fn itemSeparatorHandler(&self) -> UICollectionLayoutListItemSeparatorHandler;

        #[cfg(all(feature = "UIListSeparatorConfiguration", feature = "block2"))]
        #[method(setItemSeparatorHandler:)]
        pub unsafe fn setItemSeparatorHandler(
            &self,
            item_separator_handler: UICollectionLayoutListItemSeparatorHandler,
        );

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, background_color: Option<&UIColor>);

        #[cfg(all(feature = "UISwipeActionsConfiguration", feature = "block2"))]
        #[method(leadingSwipeActionsConfigurationProvider)]
        pub unsafe fn leadingSwipeActionsConfigurationProvider(
            &self,
        ) -> UICollectionLayoutListSwipeActionsConfigurationProvider;

        #[cfg(all(feature = "UISwipeActionsConfiguration", feature = "block2"))]
        #[method(setLeadingSwipeActionsConfigurationProvider:)]
        pub unsafe fn setLeadingSwipeActionsConfigurationProvider(
            &self,
            leading_swipe_actions_configuration_provider: UICollectionLayoutListSwipeActionsConfigurationProvider,
        );

        #[cfg(all(feature = "UISwipeActionsConfiguration", feature = "block2"))]
        #[method(trailingSwipeActionsConfigurationProvider)]
        pub unsafe fn trailingSwipeActionsConfigurationProvider(
            &self,
        ) -> UICollectionLayoutListSwipeActionsConfigurationProvider;

        #[cfg(all(feature = "UISwipeActionsConfiguration", feature = "block2"))]
        #[method(setTrailingSwipeActionsConfigurationProvider:)]
        pub unsafe fn setTrailingSwipeActionsConfigurationProvider(
            &self,
            trailing_swipe_actions_configuration_provider: UICollectionLayoutListSwipeActionsConfigurationProvider,
        );

        #[method(headerMode)]
        pub unsafe fn headerMode(&self) -> UICollectionLayoutListHeaderMode;

        #[method(setHeaderMode:)]
        pub unsafe fn setHeaderMode(&self, header_mode: UICollectionLayoutListHeaderMode);

        #[method(footerMode)]
        pub unsafe fn footerMode(&self) -> UICollectionLayoutListFooterMode;

        #[method(setFooterMode:)]
        pub unsafe fn setFooterMode(&self, footer_mode: UICollectionLayoutListFooterMode);

        #[method(headerTopPadding)]
        pub unsafe fn headerTopPadding(&self) -> CGFloat;

        #[method(setHeaderTopPadding:)]
        pub unsafe fn setHeaderTopPadding(&self, header_top_padding: CGFloat);

        #[method(contentHuggingElements)]
        pub unsafe fn contentHuggingElements(&self)
            -> UICollectionLayoutListContentHuggingElements;

        #[method(setContentHuggingElements:)]
        pub unsafe fn setContentHuggingElements(
            &self,
            content_hugging_elements: UICollectionLayoutListContentHuggingElements,
        );
    }
);

extern_methods!(
    /// UICollectionLayoutListSection
    #[cfg(feature = "UICollectionViewCompositionalLayout")]
    unsafe impl NSCollectionLayoutSection {
        #[method_id(@__retain_semantics Other sectionWithListConfiguration:layoutEnvironment:)]
        pub unsafe fn sectionWithListConfiguration_layoutEnvironment(
            configuration: &UICollectionLayoutListConfiguration,
            layout_environment: &ProtocolObject<dyn NSCollectionLayoutEnvironment>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// UICollectionLayoutListSection
    #[cfg(all(
        feature = "UICollectionViewCompositionalLayout",
        feature = "UICollectionViewLayout"
    ))]
    unsafe impl UICollectionViewCompositionalLayout {
        #[method_id(@__retain_semantics Other layoutWithListConfiguration:)]
        pub unsafe fn layoutWithListConfiguration(
            configuration: &UICollectionLayoutListConfiguration,
        ) -> Retained<Self>;
    }
);
