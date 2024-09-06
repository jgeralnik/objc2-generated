//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSDirectionalRectEdge(pub NSUInteger);
bitflags::bitflags! {
    impl NSDirectionalRectEdge: NSUInteger {
        #[doc(alias = "NSDirectionalRectEdgeNone")]
        const None = 0;
        #[doc(alias = "NSDirectionalRectEdgeTop")]
        const Top = 1<<0;
        #[doc(alias = "NSDirectionalRectEdgeLeading")]
        const Leading = 1<<1;
        #[doc(alias = "NSDirectionalRectEdgeBottom")]
        const Bottom = 1<<2;
        #[doc(alias = "NSDirectionalRectEdgeTrailing")]
        const Trailing = 1<<3;
        #[doc(alias = "NSDirectionalRectEdgeAll")]
        const All = NSDirectionalRectEdge::Top.0|NSDirectionalRectEdge::Leading.0|NSDirectionalRectEdge::Bottom.0|NSDirectionalRectEdge::Trailing.0;
    }
}

unsafe impl Encode for NSDirectionalRectEdge {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSDirectionalRectEdge {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NSDirectionalEdgeInsets {
    pub top: CGFloat,
    pub leading: CGFloat,
    pub bottom: CGFloat,
    pub trailing: CGFloat,
}

unsafe impl Encode for NSDirectionalEdgeInsets {
    const ENCODING: Encoding = Encoding::Struct(
        "NSDirectionalEdgeInsets",
        &[
            <CGFloat>::ENCODING,
            <CGFloat>::ENCODING,
            <CGFloat>::ENCODING,
            <CGFloat>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for NSDirectionalEdgeInsets {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

unsafe impl Send for NSDirectionalEdgeInsets {}

unsafe impl Sync for NSDirectionalEdgeInsets {}

extern "C" {
    pub static NSDirectionalEdgeInsetsZero: NSDirectionalEdgeInsets;
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSRectAlignment(pub NSInteger);
impl NSRectAlignment {
    #[doc(alias = "NSRectAlignmentNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "NSRectAlignmentTop")]
    pub const Top: Self = Self(1);
    #[doc(alias = "NSRectAlignmentTopLeading")]
    pub const TopLeading: Self = Self(2);
    #[doc(alias = "NSRectAlignmentLeading")]
    pub const Leading: Self = Self(3);
    #[doc(alias = "NSRectAlignmentBottomLeading")]
    pub const BottomLeading: Self = Self(4);
    #[doc(alias = "NSRectAlignmentBottom")]
    pub const Bottom: Self = Self(5);
    #[doc(alias = "NSRectAlignmentBottomTrailing")]
    pub const BottomTrailing: Self = Self(6);
    #[doc(alias = "NSRectAlignmentTrailing")]
    pub const Trailing: Self = Self(7);
    #[doc(alias = "NSRectAlignmentTopTrailing")]
    pub const TopTrailing: Self = Self(8);
}

unsafe impl Encode for NSRectAlignment {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSRectAlignment {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// TODO: pub fn NSDirectionalEdgeInsetsMake(top: CGFloat,leading: CGFloat,bottom: CGFloat,trailing: CGFloat,) -> NSDirectionalEdgeInsets;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCollectionViewCompositionalLayoutConfiguration;

    unsafe impl ClassType for NSCollectionViewCompositionalLayoutConfiguration {
        type Super = NSObject;
    }
);

unsafe impl NSCopying for NSCollectionViewCompositionalLayoutConfiguration {}

unsafe impl CopyingHelper for NSCollectionViewCompositionalLayoutConfiguration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSCollectionViewCompositionalLayoutConfiguration {}

extern_methods!(
    unsafe impl NSCollectionViewCompositionalLayoutConfiguration {
        #[cfg(feature = "NSCollectionViewFlowLayout")]
        #[method(scrollDirection)]
        pub unsafe fn scrollDirection(&self) -> NSCollectionViewScrollDirection;

        #[cfg(feature = "NSCollectionViewFlowLayout")]
        #[method(setScrollDirection:)]
        pub unsafe fn setScrollDirection(&self, scroll_direction: NSCollectionViewScrollDirection);

        #[method(interSectionSpacing)]
        pub unsafe fn interSectionSpacing(&self) -> CGFloat;

        #[method(setInterSectionSpacing:)]
        pub unsafe fn setInterSectionSpacing(&self, inter_section_spacing: CGFloat);

        #[method_id(@__retain_semantics Other boundarySupplementaryItems)]
        pub unsafe fn boundarySupplementaryItems(
            &self,
        ) -> Retained<NSArray<NSCollectionLayoutBoundarySupplementaryItem>>;

        #[method(setBoundarySupplementaryItems:)]
        pub unsafe fn setBoundarySupplementaryItems(
            &self,
            boundary_supplementary_items: &NSArray<NSCollectionLayoutBoundarySupplementaryItem>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSCollectionViewCompositionalLayoutConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

#[cfg(feature = "block2")]
pub type NSCollectionViewCompositionalLayoutSectionProvider = *mut block2::Block<
    dyn Fn(
        NSInteger,
        NonNull<ProtocolObject<dyn NSCollectionLayoutEnvironment>>,
    ) -> *mut NSCollectionLayoutSection,
>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSCollectionViewLayout")]
    pub struct NSCollectionViewCompositionalLayout;

    #[cfg(feature = "NSCollectionViewLayout")]
    unsafe impl ClassType for NSCollectionViewCompositionalLayout {
        #[inherits(NSObject)]
        type Super = NSCollectionViewLayout;
    }
);

#[cfg(feature = "NSCollectionViewLayout")]
unsafe impl NSCoding for NSCollectionViewCompositionalLayout {}

#[cfg(feature = "NSCollectionViewLayout")]
unsafe impl NSObjectProtocol for NSCollectionViewCompositionalLayout {}

extern_methods!(
    #[cfg(feature = "NSCollectionViewLayout")]
    unsafe impl NSCollectionViewCompositionalLayout {
        #[method_id(@__retain_semantics Init initWithSection:)]
        pub unsafe fn initWithSection(
            this: Allocated<Self>,
            section: &NSCollectionLayoutSection,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithSection:configuration:)]
        pub unsafe fn initWithSection_configuration(
            this: Allocated<Self>,
            section: &NSCollectionLayoutSection,
            configuration: &NSCollectionViewCompositionalLayoutConfiguration,
        ) -> Retained<Self>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Init initWithSectionProvider:)]
        pub unsafe fn initWithSectionProvider(
            this: Allocated<Self>,
            section_provider: NSCollectionViewCompositionalLayoutSectionProvider,
        ) -> Retained<Self>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Init initWithSectionProvider:configuration:)]
        pub unsafe fn initWithSectionProvider_configuration(
            this: Allocated<Self>,
            section_provider: NSCollectionViewCompositionalLayoutSectionProvider,
            configuration: &NSCollectionViewCompositionalLayoutConfiguration,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Other configuration)]
        pub unsafe fn configuration(
            &self,
        ) -> Retained<NSCollectionViewCompositionalLayoutConfiguration>;

        #[method(setConfiguration:)]
        pub unsafe fn setConfiguration(
            &self,
            configuration: &NSCollectionViewCompositionalLayoutConfiguration,
        );
    }
);

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSCollectionLayoutSectionOrthogonalScrollingBehavior(pub NSInteger);
impl NSCollectionLayoutSectionOrthogonalScrollingBehavior {
    #[doc(alias = "NSCollectionLayoutSectionOrthogonalScrollingBehaviorNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "NSCollectionLayoutSectionOrthogonalScrollingBehaviorContinuous")]
    pub const Continuous: Self = Self(1);
    #[doc(
        alias = "NSCollectionLayoutSectionOrthogonalScrollingBehaviorContinuousGroupLeadingBoundary"
    )]
    pub const ContinuousGroupLeadingBoundary: Self = Self(2);
    #[doc(alias = "NSCollectionLayoutSectionOrthogonalScrollingBehaviorPaging")]
    pub const Paging: Self = Self(3);
    #[doc(alias = "NSCollectionLayoutSectionOrthogonalScrollingBehaviorGroupPaging")]
    pub const GroupPaging: Self = Self(4);
    #[doc(alias = "NSCollectionLayoutSectionOrthogonalScrollingBehaviorGroupPagingCentered")]
    pub const GroupPagingCentered: Self = Self(5);
}

unsafe impl Encode for NSCollectionLayoutSectionOrthogonalScrollingBehavior {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSCollectionLayoutSectionOrthogonalScrollingBehavior {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(feature = "block2")]
pub type NSCollectionLayoutSectionVisibleItemsInvalidationHandler = *mut block2::Block<
    dyn Fn(
        NonNull<NSArray<ProtocolObject<dyn NSCollectionLayoutVisibleItem>>>,
        NSPoint,
        NonNull<ProtocolObject<dyn NSCollectionLayoutEnvironment>>,
    ),
>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCollectionLayoutSection;

    unsafe impl ClassType for NSCollectionLayoutSection {
        type Super = NSObject;
    }
);

unsafe impl NSCopying for NSCollectionLayoutSection {}

unsafe impl CopyingHelper for NSCollectionLayoutSection {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSCollectionLayoutSection {}

extern_methods!(
    unsafe impl NSCollectionLayoutSection {
        #[method_id(@__retain_semantics Other sectionWithGroup:)]
        pub unsafe fn sectionWithGroup(group: &NSCollectionLayoutGroup) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method(contentInsets)]
        pub unsafe fn contentInsets(&self) -> NSDirectionalEdgeInsets;

        #[method(setContentInsets:)]
        pub unsafe fn setContentInsets(&self, content_insets: NSDirectionalEdgeInsets);

        #[method(interGroupSpacing)]
        pub unsafe fn interGroupSpacing(&self) -> CGFloat;

        #[method(setInterGroupSpacing:)]
        pub unsafe fn setInterGroupSpacing(&self, inter_group_spacing: CGFloat);

        #[method(orthogonalScrollingBehavior)]
        pub unsafe fn orthogonalScrollingBehavior(
            &self,
        ) -> NSCollectionLayoutSectionOrthogonalScrollingBehavior;

        #[method(setOrthogonalScrollingBehavior:)]
        pub unsafe fn setOrthogonalScrollingBehavior(
            &self,
            orthogonal_scrolling_behavior: NSCollectionLayoutSectionOrthogonalScrollingBehavior,
        );

        #[method_id(@__retain_semantics Other boundarySupplementaryItems)]
        pub unsafe fn boundarySupplementaryItems(
            &self,
        ) -> Retained<NSArray<NSCollectionLayoutBoundarySupplementaryItem>>;

        #[method(setBoundarySupplementaryItems:)]
        pub unsafe fn setBoundarySupplementaryItems(
            &self,
            boundary_supplementary_items: &NSArray<NSCollectionLayoutBoundarySupplementaryItem>,
        );

        #[method(supplementariesFollowContentInsets)]
        pub unsafe fn supplementariesFollowContentInsets(&self) -> bool;

        #[method(setSupplementariesFollowContentInsets:)]
        pub unsafe fn setSupplementariesFollowContentInsets(
            &self,
            supplementaries_follow_content_insets: bool,
        );

        #[cfg(feature = "block2")]
        #[method(visibleItemsInvalidationHandler)]
        pub unsafe fn visibleItemsInvalidationHandler(
            &self,
        ) -> NSCollectionLayoutSectionVisibleItemsInvalidationHandler;

        #[cfg(feature = "block2")]
        #[method(setVisibleItemsInvalidationHandler:)]
        pub unsafe fn setVisibleItemsInvalidationHandler(
            &self,
            visible_items_invalidation_handler: NSCollectionLayoutSectionVisibleItemsInvalidationHandler,
        );

        #[method_id(@__retain_semantics Other decorationItems)]
        pub unsafe fn decorationItems(&self)
            -> Retained<NSArray<NSCollectionLayoutDecorationItem>>;

        #[method(setDecorationItems:)]
        pub unsafe fn setDecorationItems(
            &self,
            decoration_items: &NSArray<NSCollectionLayoutDecorationItem>,
        );
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCollectionLayoutItem;

    unsafe impl ClassType for NSCollectionLayoutItem {
        type Super = NSObject;
    }
);

unsafe impl NSCopying for NSCollectionLayoutItem {}

unsafe impl CopyingHelper for NSCollectionLayoutItem {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSCollectionLayoutItem {}

extern_methods!(
    unsafe impl NSCollectionLayoutItem {
        #[method_id(@__retain_semantics Other itemWithLayoutSize:)]
        pub unsafe fn itemWithLayoutSize(layout_size: &NSCollectionLayoutSize) -> Retained<Self>;

        #[method_id(@__retain_semantics Other itemWithLayoutSize:supplementaryItems:)]
        pub unsafe fn itemWithLayoutSize_supplementaryItems(
            layout_size: &NSCollectionLayoutSize,
            supplementary_items: &NSArray<NSCollectionLayoutSupplementaryItem>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method(contentInsets)]
        pub unsafe fn contentInsets(&self) -> NSDirectionalEdgeInsets;

        #[method(setContentInsets:)]
        pub unsafe fn setContentInsets(&self, content_insets: NSDirectionalEdgeInsets);

        #[method_id(@__retain_semantics Other edgeSpacing)]
        pub unsafe fn edgeSpacing(&self) -> Option<Retained<NSCollectionLayoutEdgeSpacing>>;

        #[method(setEdgeSpacing:)]
        pub unsafe fn setEdgeSpacing(&self, edge_spacing: Option<&NSCollectionLayoutEdgeSpacing>);

        #[method_id(@__retain_semantics Other layoutSize)]
        pub unsafe fn layoutSize(&self) -> Retained<NSCollectionLayoutSize>;

        #[method_id(@__retain_semantics Other supplementaryItems)]
        pub unsafe fn supplementaryItems(
            &self,
        ) -> Retained<NSArray<NSCollectionLayoutSupplementaryItem>>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCollectionLayoutGroupCustomItem;

    unsafe impl ClassType for NSCollectionLayoutGroupCustomItem {
        type Super = NSObject;
    }
);

unsafe impl NSCopying for NSCollectionLayoutGroupCustomItem {}

unsafe impl CopyingHelper for NSCollectionLayoutGroupCustomItem {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSCollectionLayoutGroupCustomItem {}

extern_methods!(
    unsafe impl NSCollectionLayoutGroupCustomItem {
        #[method_id(@__retain_semantics Other customItemWithFrame:)]
        pub unsafe fn customItemWithFrame(frame: NSRect) -> Retained<Self>;

        #[method_id(@__retain_semantics Other customItemWithFrame:zIndex:)]
        pub unsafe fn customItemWithFrame_zIndex(
            frame: NSRect,
            z_index: NSInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method(frame)]
        pub unsafe fn frame(&self) -> NSRect;

        #[method(zIndex)]
        pub unsafe fn zIndex(&self) -> NSInteger;
    }
);

#[cfg(feature = "block2")]
pub type NSCollectionLayoutGroupCustomItemProvider = *mut block2::Block<
    dyn Fn(
        NonNull<ProtocolObject<dyn NSCollectionLayoutEnvironment>>,
    ) -> NonNull<NSArray<NSCollectionLayoutGroupCustomItem>>,
>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCollectionLayoutGroup;

    unsafe impl ClassType for NSCollectionLayoutGroup {
        #[inherits(NSObject)]
        type Super = NSCollectionLayoutItem;
    }
);

unsafe impl NSCopying for NSCollectionLayoutGroup {}

unsafe impl CopyingHelper for NSCollectionLayoutGroup {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSCollectionLayoutGroup {}

extern_methods!(
    unsafe impl NSCollectionLayoutGroup {
        #[method_id(@__retain_semantics Other horizontalGroupWithLayoutSize:subitem:count:)]
        pub unsafe fn horizontalGroupWithLayoutSize_subitem_count(
            layout_size: &NSCollectionLayoutSize,
            subitem: &NSCollectionLayoutItem,
            count: NSInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other horizontalGroupWithLayoutSize:subitems:)]
        pub unsafe fn horizontalGroupWithLayoutSize_subitems(
            layout_size: &NSCollectionLayoutSize,
            subitems: &NSArray<NSCollectionLayoutItem>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other verticalGroupWithLayoutSize:subitem:count:)]
        pub unsafe fn verticalGroupWithLayoutSize_subitem_count(
            layout_size: &NSCollectionLayoutSize,
            subitem: &NSCollectionLayoutItem,
            count: NSInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other verticalGroupWithLayoutSize:subitems:)]
        pub unsafe fn verticalGroupWithLayoutSize_subitems(
            layout_size: &NSCollectionLayoutSize,
            subitems: &NSArray<NSCollectionLayoutItem>,
        ) -> Retained<Self>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Other customGroupWithLayoutSize:itemProvider:)]
        pub unsafe fn customGroupWithLayoutSize_itemProvider(
            layout_size: &NSCollectionLayoutSize,
            item_provider: NSCollectionLayoutGroupCustomItemProvider,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Other supplementaryItems)]
        pub unsafe fn supplementaryItems(
            &self,
        ) -> Retained<NSArray<NSCollectionLayoutSupplementaryItem>>;

        #[method(setSupplementaryItems:)]
        pub unsafe fn setSupplementaryItems(
            &self,
            supplementary_items: &NSArray<NSCollectionLayoutSupplementaryItem>,
        );

        #[method_id(@__retain_semantics Other interItemSpacing)]
        pub unsafe fn interItemSpacing(&self) -> Option<Retained<NSCollectionLayoutSpacing>>;

        #[method(setInterItemSpacing:)]
        pub unsafe fn setInterItemSpacing(
            &self,
            inter_item_spacing: Option<&NSCollectionLayoutSpacing>,
        );

        #[method_id(@__retain_semantics Other subitems)]
        pub unsafe fn subitems(&self) -> Retained<NSArray<NSCollectionLayoutItem>>;

        #[method_id(@__retain_semantics Other visualDescription)]
        pub unsafe fn visualDescription(&self) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSCollectionLayoutItem`
    unsafe impl NSCollectionLayoutGroup {
        #[method_id(@__retain_semantics Other itemWithLayoutSize:)]
        pub unsafe fn itemWithLayoutSize(layout_size: &NSCollectionLayoutSize) -> Retained<Self>;

        #[method_id(@__retain_semantics Other itemWithLayoutSize:supplementaryItems:)]
        pub unsafe fn itemWithLayoutSize_supplementaryItems(
            layout_size: &NSCollectionLayoutSize,
            supplementary_items: &NSArray<NSCollectionLayoutSupplementaryItem>,
        ) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCollectionLayoutDimension;

    unsafe impl ClassType for NSCollectionLayoutDimension {
        type Super = NSObject;
    }
);

unsafe impl NSCopying for NSCollectionLayoutDimension {}

unsafe impl CopyingHelper for NSCollectionLayoutDimension {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSCollectionLayoutDimension {}

extern_methods!(
    unsafe impl NSCollectionLayoutDimension {
        #[method_id(@__retain_semantics Other fractionalWidthDimension:)]
        pub unsafe fn fractionalWidthDimension(fractional_width: CGFloat) -> Retained<Self>;

        #[method_id(@__retain_semantics Other fractionalHeightDimension:)]
        pub unsafe fn fractionalHeightDimension(fractional_height: CGFloat) -> Retained<Self>;

        #[method_id(@__retain_semantics Other absoluteDimension:)]
        pub unsafe fn absoluteDimension(absolute_dimension: CGFloat) -> Retained<Self>;

        #[method_id(@__retain_semantics Other estimatedDimension:)]
        pub unsafe fn estimatedDimension(estimated_dimension: CGFloat) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method(isFractionalWidth)]
        pub unsafe fn isFractionalWidth(&self) -> bool;

        #[method(isFractionalHeight)]
        pub unsafe fn isFractionalHeight(&self) -> bool;

        #[method(isAbsolute)]
        pub unsafe fn isAbsolute(&self) -> bool;

        #[method(isEstimated)]
        pub unsafe fn isEstimated(&self) -> bool;

        #[method(dimension)]
        pub unsafe fn dimension(&self) -> CGFloat;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCollectionLayoutSize;

    unsafe impl ClassType for NSCollectionLayoutSize {
        type Super = NSObject;
    }
);

unsafe impl NSCopying for NSCollectionLayoutSize {}

unsafe impl CopyingHelper for NSCollectionLayoutSize {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSCollectionLayoutSize {}

extern_methods!(
    unsafe impl NSCollectionLayoutSize {
        #[method_id(@__retain_semantics Other sizeWithWidthDimension:heightDimension:)]
        pub unsafe fn sizeWithWidthDimension_heightDimension(
            width: &NSCollectionLayoutDimension,
            height: &NSCollectionLayoutDimension,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Other widthDimension)]
        pub unsafe fn widthDimension(&self) -> Retained<NSCollectionLayoutDimension>;

        #[method_id(@__retain_semantics Other heightDimension)]
        pub unsafe fn heightDimension(&self) -> Retained<NSCollectionLayoutDimension>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCollectionLayoutSpacing;

    unsafe impl ClassType for NSCollectionLayoutSpacing {
        type Super = NSObject;
    }
);

unsafe impl NSCopying for NSCollectionLayoutSpacing {}

unsafe impl CopyingHelper for NSCollectionLayoutSpacing {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSCollectionLayoutSpacing {}

extern_methods!(
    unsafe impl NSCollectionLayoutSpacing {
        #[method_id(@__retain_semantics Other flexibleSpacing:)]
        pub unsafe fn flexibleSpacing(flexible_spacing: CGFloat) -> Retained<Self>;

        #[method_id(@__retain_semantics Other fixedSpacing:)]
        pub unsafe fn fixedSpacing(fixed_spacing: CGFloat) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method(spacing)]
        pub unsafe fn spacing(&self) -> CGFloat;

        #[method(isFlexibleSpacing)]
        pub unsafe fn isFlexibleSpacing(&self) -> bool;

        #[method(isFixedSpacing)]
        pub unsafe fn isFixedSpacing(&self) -> bool;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCollectionLayoutEdgeSpacing;

    unsafe impl ClassType for NSCollectionLayoutEdgeSpacing {
        type Super = NSObject;
    }
);

unsafe impl NSCopying for NSCollectionLayoutEdgeSpacing {}

unsafe impl CopyingHelper for NSCollectionLayoutEdgeSpacing {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSCollectionLayoutEdgeSpacing {}

extern_methods!(
    unsafe impl NSCollectionLayoutEdgeSpacing {
        #[method_id(@__retain_semantics Other spacingForLeading:top:trailing:bottom:)]
        pub unsafe fn spacingForLeading_top_trailing_bottom(
            leading: Option<&NSCollectionLayoutSpacing>,
            top: Option<&NSCollectionLayoutSpacing>,
            trailing: Option<&NSCollectionLayoutSpacing>,
            bottom: Option<&NSCollectionLayoutSpacing>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Other leading)]
        pub unsafe fn leading(&self) -> Option<Retained<NSCollectionLayoutSpacing>>;

        #[method_id(@__retain_semantics Other top)]
        pub unsafe fn top(&self) -> Option<Retained<NSCollectionLayoutSpacing>>;

        #[method_id(@__retain_semantics Other trailing)]
        pub unsafe fn trailing(&self) -> Option<Retained<NSCollectionLayoutSpacing>>;

        #[method_id(@__retain_semantics Other bottom)]
        pub unsafe fn bottom(&self) -> Option<Retained<NSCollectionLayoutSpacing>>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCollectionLayoutSupplementaryItem;

    unsafe impl ClassType for NSCollectionLayoutSupplementaryItem {
        #[inherits(NSObject)]
        type Super = NSCollectionLayoutItem;
    }
);

unsafe impl NSCopying for NSCollectionLayoutSupplementaryItem {}

unsafe impl CopyingHelper for NSCollectionLayoutSupplementaryItem {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSCollectionLayoutSupplementaryItem {}

extern_methods!(
    unsafe impl NSCollectionLayoutSupplementaryItem {
        #[method_id(@__retain_semantics Other supplementaryItemWithLayoutSize:elementKind:containerAnchor:)]
        pub unsafe fn supplementaryItemWithLayoutSize_elementKind_containerAnchor(
            layout_size: &NSCollectionLayoutSize,
            element_kind: &NSString,
            container_anchor: &NSCollectionLayoutAnchor,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other supplementaryItemWithLayoutSize:elementKind:containerAnchor:itemAnchor:)]
        pub unsafe fn supplementaryItemWithLayoutSize_elementKind_containerAnchor_itemAnchor(
            layout_size: &NSCollectionLayoutSize,
            element_kind: &NSString,
            container_anchor: &NSCollectionLayoutAnchor,
            item_anchor: &NSCollectionLayoutAnchor,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method(zIndex)]
        pub unsafe fn zIndex(&self) -> NSInteger;

        #[method(setZIndex:)]
        pub unsafe fn setZIndex(&self, z_index: NSInteger);

        #[method_id(@__retain_semantics Other elementKind)]
        pub unsafe fn elementKind(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other containerAnchor)]
        pub unsafe fn containerAnchor(&self) -> Retained<NSCollectionLayoutAnchor>;

        #[method_id(@__retain_semantics Other itemAnchor)]
        pub unsafe fn itemAnchor(&self) -> Option<Retained<NSCollectionLayoutAnchor>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSCollectionLayoutItem`
    unsafe impl NSCollectionLayoutSupplementaryItem {
        #[method_id(@__retain_semantics Other itemWithLayoutSize:)]
        pub unsafe fn itemWithLayoutSize(layout_size: &NSCollectionLayoutSize) -> Retained<Self>;

        #[method_id(@__retain_semantics Other itemWithLayoutSize:supplementaryItems:)]
        pub unsafe fn itemWithLayoutSize_supplementaryItems(
            layout_size: &NSCollectionLayoutSize,
            supplementary_items: &NSArray<NSCollectionLayoutSupplementaryItem>,
        ) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCollectionLayoutBoundarySupplementaryItem;

    unsafe impl ClassType for NSCollectionLayoutBoundarySupplementaryItem {
        #[inherits(NSCollectionLayoutItem, NSObject)]
        type Super = NSCollectionLayoutSupplementaryItem;
    }
);

unsafe impl NSCopying for NSCollectionLayoutBoundarySupplementaryItem {}

unsafe impl CopyingHelper for NSCollectionLayoutBoundarySupplementaryItem {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSCollectionLayoutBoundarySupplementaryItem {}

extern_methods!(
    unsafe impl NSCollectionLayoutBoundarySupplementaryItem {
        #[method_id(@__retain_semantics Other boundarySupplementaryItemWithLayoutSize:elementKind:alignment:)]
        pub unsafe fn boundarySupplementaryItemWithLayoutSize_elementKind_alignment(
            layout_size: &NSCollectionLayoutSize,
            element_kind: &NSString,
            alignment: NSRectAlignment,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other boundarySupplementaryItemWithLayoutSize:elementKind:alignment:absoluteOffset:)]
        pub unsafe fn boundarySupplementaryItemWithLayoutSize_elementKind_alignment_absoluteOffset(
            layout_size: &NSCollectionLayoutSize,
            element_kind: &NSString,
            alignment: NSRectAlignment,
            absolute_offset: NSPoint,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method(extendsBoundary)]
        pub unsafe fn extendsBoundary(&self) -> bool;

        #[method(setExtendsBoundary:)]
        pub unsafe fn setExtendsBoundary(&self, extends_boundary: bool);

        #[method(pinToVisibleBounds)]
        pub unsafe fn pinToVisibleBounds(&self) -> bool;

        #[method(setPinToVisibleBounds:)]
        pub unsafe fn setPinToVisibleBounds(&self, pin_to_visible_bounds: bool);

        #[method(alignment)]
        pub unsafe fn alignment(&self) -> NSRectAlignment;

        #[method(offset)]
        pub unsafe fn offset(&self) -> NSPoint;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSCollectionLayoutSupplementaryItem`
    unsafe impl NSCollectionLayoutBoundarySupplementaryItem {
        #[method_id(@__retain_semantics Other supplementaryItemWithLayoutSize:elementKind:containerAnchor:)]
        pub unsafe fn supplementaryItemWithLayoutSize_elementKind_containerAnchor(
            layout_size: &NSCollectionLayoutSize,
            element_kind: &NSString,
            container_anchor: &NSCollectionLayoutAnchor,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other supplementaryItemWithLayoutSize:elementKind:containerAnchor:itemAnchor:)]
        pub unsafe fn supplementaryItemWithLayoutSize_elementKind_containerAnchor_itemAnchor(
            layout_size: &NSCollectionLayoutSize,
            element_kind: &NSString,
            container_anchor: &NSCollectionLayoutAnchor,
            item_anchor: &NSCollectionLayoutAnchor,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSCollectionLayoutItem`
    unsafe impl NSCollectionLayoutBoundarySupplementaryItem {
        #[method_id(@__retain_semantics Other itemWithLayoutSize:)]
        pub unsafe fn itemWithLayoutSize(layout_size: &NSCollectionLayoutSize) -> Retained<Self>;

        #[method_id(@__retain_semantics Other itemWithLayoutSize:supplementaryItems:)]
        pub unsafe fn itemWithLayoutSize_supplementaryItems(
            layout_size: &NSCollectionLayoutSize,
            supplementary_items: &NSArray<NSCollectionLayoutSupplementaryItem>,
        ) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCollectionLayoutDecorationItem;

    unsafe impl ClassType for NSCollectionLayoutDecorationItem {
        #[inherits(NSObject)]
        type Super = NSCollectionLayoutItem;
    }
);

unsafe impl NSCopying for NSCollectionLayoutDecorationItem {}

unsafe impl CopyingHelper for NSCollectionLayoutDecorationItem {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSCollectionLayoutDecorationItem {}

extern_methods!(
    unsafe impl NSCollectionLayoutDecorationItem {
        #[method_id(@__retain_semantics Other backgroundDecorationItemWithElementKind:)]
        pub unsafe fn backgroundDecorationItemWithElementKind(
            element_kind: &NSString,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method(zIndex)]
        pub unsafe fn zIndex(&self) -> NSInteger;

        #[method(setZIndex:)]
        pub unsafe fn setZIndex(&self, z_index: NSInteger);

        #[method_id(@__retain_semantics Other elementKind)]
        pub unsafe fn elementKind(&self) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSCollectionLayoutItem`
    unsafe impl NSCollectionLayoutDecorationItem {
        #[method_id(@__retain_semantics Other itemWithLayoutSize:)]
        pub unsafe fn itemWithLayoutSize(layout_size: &NSCollectionLayoutSize) -> Retained<Self>;

        #[method_id(@__retain_semantics Other itemWithLayoutSize:supplementaryItems:)]
        pub unsafe fn itemWithLayoutSize_supplementaryItems(
            layout_size: &NSCollectionLayoutSize,
            supplementary_items: &NSArray<NSCollectionLayoutSupplementaryItem>,
        ) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCollectionLayoutAnchor;

    unsafe impl ClassType for NSCollectionLayoutAnchor {
        type Super = NSObject;
    }
);

unsafe impl NSCopying for NSCollectionLayoutAnchor {}

unsafe impl CopyingHelper for NSCollectionLayoutAnchor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSCollectionLayoutAnchor {}

extern_methods!(
    unsafe impl NSCollectionLayoutAnchor {
        #[method_id(@__retain_semantics Other layoutAnchorWithEdges:)]
        pub unsafe fn layoutAnchorWithEdges(edges: NSDirectionalRectEdge) -> Retained<Self>;

        #[method_id(@__retain_semantics Other layoutAnchorWithEdges:absoluteOffset:)]
        pub unsafe fn layoutAnchorWithEdges_absoluteOffset(
            edges: NSDirectionalRectEdge,
            absolute_offset: NSPoint,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other layoutAnchorWithEdges:fractionalOffset:)]
        pub unsafe fn layoutAnchorWithEdges_fractionalOffset(
            edges: NSDirectionalRectEdge,
            fractional_offset: NSPoint,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method(edges)]
        pub unsafe fn edges(&self) -> NSDirectionalRectEdge;

        #[method(offset)]
        pub unsafe fn offset(&self) -> NSPoint;

        #[method(isAbsoluteOffset)]
        pub unsafe fn isAbsoluteOffset(&self) -> bool;

        #[method(isFractionalOffset)]
        pub unsafe fn isFractionalOffset(&self) -> bool;
    }
);

extern_protocol!(
    pub unsafe trait NSCollectionLayoutContainer: NSObjectProtocol {
        #[method(contentSize)]
        unsafe fn contentSize(&self) -> NSSize;

        #[method(effectiveContentSize)]
        unsafe fn effectiveContentSize(&self) -> NSSize;

        #[method(contentInsets)]
        unsafe fn contentInsets(&self) -> NSDirectionalEdgeInsets;

        #[method(effectiveContentInsets)]
        unsafe fn effectiveContentInsets(&self) -> NSDirectionalEdgeInsets;
    }

    unsafe impl ProtocolType for dyn NSCollectionLayoutContainer {}
);

extern_protocol!(
    pub unsafe trait NSCollectionLayoutEnvironment: NSObjectProtocol {
        #[method_id(@__retain_semantics Other container)]
        unsafe fn container(&self) -> Retained<ProtocolObject<dyn NSCollectionLayoutContainer>>;
    }

    unsafe impl ProtocolType for dyn NSCollectionLayoutEnvironment {}
);

extern_protocol!(
    pub unsafe trait NSCollectionLayoutVisibleItem: NSObjectProtocol {
        #[method(alpha)]
        unsafe fn alpha(&self) -> CGFloat;

        #[method(setAlpha:)]
        unsafe fn setAlpha(&self, alpha: CGFloat);

        #[method(zIndex)]
        unsafe fn zIndex(&self) -> NSInteger;

        #[method(setZIndex:)]
        unsafe fn setZIndex(&self, z_index: NSInteger);

        #[method(isHidden)]
        unsafe fn isHidden(&self) -> bool;

        #[method(setHidden:)]
        unsafe fn setHidden(&self, hidden: bool);

        #[method(center)]
        unsafe fn center(&self) -> NSPoint;

        #[method(setCenter:)]
        unsafe fn setCenter(&self, center: NSPoint);

        #[method_id(@__retain_semantics Other name)]
        unsafe fn name(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other indexPath)]
        unsafe fn indexPath(&self) -> Retained<NSIndexPath>;

        #[method(frame)]
        unsafe fn frame(&self) -> NSRect;

        #[method(bounds)]
        unsafe fn bounds(&self) -> NSRect;

        #[cfg(feature = "NSCollectionViewLayout")]
        #[method(representedElementCategory)]
        unsafe fn representedElementCategory(&self) -> NSCollectionElementCategory;

        #[method_id(@__retain_semantics Other representedElementKind)]
        unsafe fn representedElementKind(&self) -> Option<Retained<NSString>>;
    }

    unsafe impl ProtocolType for dyn NSCollectionLayoutVisibleItem {}
);
