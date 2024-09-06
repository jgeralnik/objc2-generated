//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSCollectionElementCategory(pub NSInteger);
impl NSCollectionElementCategory {
    #[doc(alias = "NSCollectionElementCategoryItem")]
    pub const Item: Self = Self(0);
    #[doc(alias = "NSCollectionElementCategorySupplementaryView")]
    pub const SupplementaryView: Self = Self(1);
    #[doc(alias = "NSCollectionElementCategoryDecorationView")]
    pub const DecorationView: Self = Self(2);
    #[doc(alias = "NSCollectionElementCategoryInterItemGap")]
    pub const InterItemGap: Self = Self(3);
}

unsafe impl Encode for NSCollectionElementCategory {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSCollectionElementCategory {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

pub type NSCollectionViewDecorationElementKind = NSString;

extern "C" {
    #[cfg(feature = "NSCollectionView")]
    pub static NSCollectionElementKindInterItemGapIndicator:
        &'static NSCollectionViewSupplementaryElementKind;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCollectionViewLayoutAttributes;

    unsafe impl ClassType for NSCollectionViewLayoutAttributes {
        type Super = NSObject;
    }
);

unsafe impl NSCopying for NSCollectionViewLayoutAttributes {}

unsafe impl CopyingHelper for NSCollectionViewLayoutAttributes {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSCollectionViewLayoutAttributes {}

extern_methods!(
    unsafe impl NSCollectionViewLayoutAttributes {
        #[method(frame)]
        pub unsafe fn frame(&self) -> NSRect;

        #[method(setFrame:)]
        pub unsafe fn setFrame(&self, frame: NSRect);

        #[method(size)]
        pub unsafe fn size(&self) -> NSSize;

        #[method(setSize:)]
        pub unsafe fn setSize(&self, size: NSSize);

        #[method(alpha)]
        pub unsafe fn alpha(&self) -> CGFloat;

        #[method(setAlpha:)]
        pub unsafe fn setAlpha(&self, alpha: CGFloat);

        #[method(zIndex)]
        pub unsafe fn zIndex(&self) -> NSInteger;

        #[method(setZIndex:)]
        pub unsafe fn setZIndex(&self, z_index: NSInteger);

        #[method(isHidden)]
        pub unsafe fn isHidden(&self) -> bool;

        #[method(setHidden:)]
        pub unsafe fn setHidden(&self, hidden: bool);

        #[method_id(@__retain_semantics Other indexPath)]
        pub unsafe fn indexPath(&self) -> Option<Retained<NSIndexPath>>;

        #[method(setIndexPath:)]
        pub unsafe fn setIndexPath(&self, index_path: Option<&NSIndexPath>);

        #[method(representedElementCategory)]
        pub unsafe fn representedElementCategory(&self) -> NSCollectionElementCategory;

        #[method_id(@__retain_semantics Other representedElementKind)]
        pub unsafe fn representedElementKind(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other layoutAttributesForItemWithIndexPath:)]
        pub unsafe fn layoutAttributesForItemWithIndexPath(
            index_path: &NSIndexPath,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other layoutAttributesForInterItemGapBeforeIndexPath:)]
        pub unsafe fn layoutAttributesForInterItemGapBeforeIndexPath(
            index_path: &NSIndexPath,
        ) -> Retained<Self>;

        #[cfg(feature = "NSCollectionView")]
        #[method_id(@__retain_semantics Other layoutAttributesForSupplementaryViewOfKind:withIndexPath:)]
        pub unsafe fn layoutAttributesForSupplementaryViewOfKind_withIndexPath(
            element_kind: &NSCollectionViewSupplementaryElementKind,
            index_path: &NSIndexPath,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other layoutAttributesForDecorationViewOfKind:withIndexPath:)]
        pub unsafe fn layoutAttributesForDecorationViewOfKind_withIndexPath(
            decoration_view_kind: &NSCollectionViewDecorationElementKind,
            index_path: &NSIndexPath,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSCollectionViewLayoutAttributes {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSCollectionUpdateAction(pub NSInteger);
impl NSCollectionUpdateAction {
    #[doc(alias = "NSCollectionUpdateActionInsert")]
    pub const Insert: Self = Self(0);
    #[doc(alias = "NSCollectionUpdateActionDelete")]
    pub const Delete: Self = Self(1);
    #[doc(alias = "NSCollectionUpdateActionReload")]
    pub const Reload: Self = Self(2);
    #[doc(alias = "NSCollectionUpdateActionMove")]
    pub const Move: Self = Self(3);
    #[doc(alias = "NSCollectionUpdateActionNone")]
    pub const None: Self = Self(4);
}

unsafe impl Encode for NSCollectionUpdateAction {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSCollectionUpdateAction {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCollectionViewUpdateItem;

    unsafe impl ClassType for NSCollectionViewUpdateItem {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for NSCollectionViewUpdateItem {}

extern_methods!(
    unsafe impl NSCollectionViewUpdateItem {
        #[method_id(@__retain_semantics Other indexPathBeforeUpdate)]
        pub unsafe fn indexPathBeforeUpdate(&self) -> Option<Retained<NSIndexPath>>;

        #[method_id(@__retain_semantics Other indexPathAfterUpdate)]
        pub unsafe fn indexPathAfterUpdate(&self) -> Option<Retained<NSIndexPath>>;

        #[method(updateAction)]
        pub unsafe fn updateAction(&self) -> NSCollectionUpdateAction;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSCollectionViewUpdateItem {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCollectionViewLayoutInvalidationContext;

    unsafe impl ClassType for NSCollectionViewLayoutInvalidationContext {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for NSCollectionViewLayoutInvalidationContext {}

extern_methods!(
    unsafe impl NSCollectionViewLayoutInvalidationContext {
        #[method(invalidateEverything)]
        pub unsafe fn invalidateEverything(&self) -> bool;

        #[method(invalidateDataSourceCounts)]
        pub unsafe fn invalidateDataSourceCounts(&self) -> bool;

        #[method(invalidateItemsAtIndexPaths:)]
        pub unsafe fn invalidateItemsAtIndexPaths(&self, index_paths: &NSSet<NSIndexPath>);

        #[cfg(feature = "NSCollectionView")]
        #[method(invalidateSupplementaryElementsOfKind:atIndexPaths:)]
        pub unsafe fn invalidateSupplementaryElementsOfKind_atIndexPaths(
            &self,
            element_kind: &NSCollectionViewSupplementaryElementKind,
            index_paths: &NSSet<NSIndexPath>,
        );

        #[method(invalidateDecorationElementsOfKind:atIndexPaths:)]
        pub unsafe fn invalidateDecorationElementsOfKind_atIndexPaths(
            &self,
            element_kind: &NSCollectionViewDecorationElementKind,
            index_paths: &NSSet<NSIndexPath>,
        );

        #[method_id(@__retain_semantics Other invalidatedItemIndexPaths)]
        pub unsafe fn invalidatedItemIndexPaths(&self) -> Option<Retained<NSSet<NSIndexPath>>>;

        #[cfg(feature = "NSCollectionView")]
        #[method_id(@__retain_semantics Other invalidatedSupplementaryIndexPaths)]
        pub unsafe fn invalidatedSupplementaryIndexPaths(
            &self,
        ) -> Option<
            Retained<NSDictionary<NSCollectionViewSupplementaryElementKind, NSSet<NSIndexPath>>>,
        >;

        #[method_id(@__retain_semantics Other invalidatedDecorationIndexPaths)]
        pub unsafe fn invalidatedDecorationIndexPaths(
            &self,
        ) -> Option<Retained<NSDictionary<NSCollectionViewDecorationElementKind, NSSet<NSIndexPath>>>>;

        #[method(contentOffsetAdjustment)]
        pub unsafe fn contentOffsetAdjustment(&self) -> NSPoint;

        #[method(setContentOffsetAdjustment:)]
        pub unsafe fn setContentOffsetAdjustment(&self, content_offset_adjustment: NSPoint);

        #[method(contentSizeAdjustment)]
        pub unsafe fn contentSizeAdjustment(&self) -> NSSize;

        #[method(setContentSizeAdjustment:)]
        pub unsafe fn setContentSizeAdjustment(&self, content_size_adjustment: NSSize);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSCollectionViewLayoutInvalidationContext {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCollectionViewLayout;

    unsafe impl ClassType for NSCollectionViewLayout {
        type Super = NSObject;
    }
);

unsafe impl NSCoding for NSCollectionViewLayout {}

unsafe impl NSObjectProtocol for NSCollectionViewLayout {}

extern_methods!(
    unsafe impl NSCollectionViewLayout {
        #[cfg(all(
            feature = "NSCollectionView",
            feature = "NSResponder",
            feature = "NSView"
        ))]
        #[method_id(@__retain_semantics Other collectionView)]
        pub unsafe fn collectionView(
            &self,
            mtm: MainThreadMarker,
        ) -> Option<Retained<NSCollectionView>>;

        #[method(invalidateLayout)]
        pub unsafe fn invalidateLayout(&self);

        #[method(invalidateLayoutWithContext:)]
        pub unsafe fn invalidateLayoutWithContext(
            &self,
            context: &NSCollectionViewLayoutInvalidationContext,
        );

        #[method(registerClass:forDecorationViewOfKind:)]
        pub unsafe fn registerClass_forDecorationViewOfKind(
            &self,
            view_class: Option<&AnyClass>,
            element_kind: &NSCollectionViewDecorationElementKind,
        );

        #[cfg(feature = "NSNib")]
        #[method(registerNib:forDecorationViewOfKind:)]
        pub unsafe fn registerNib_forDecorationViewOfKind(
            &self,
            nib: Option<&NSNib>,
            element_kind: &NSCollectionViewDecorationElementKind,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSCollectionViewLayout {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// NSSubclassingHooks
    unsafe impl NSCollectionViewLayout {
        #[method(layoutAttributesClass)]
        pub unsafe fn layoutAttributesClass() -> &'static AnyClass;

        #[method(invalidationContextClass)]
        pub unsafe fn invalidationContextClass() -> &'static AnyClass;

        #[method(prepareLayout)]
        pub unsafe fn prepareLayout(&self);

        #[method_id(@__retain_semantics Other layoutAttributesForElementsInRect:)]
        pub unsafe fn layoutAttributesForElementsInRect(
            &self,
            rect: NSRect,
        ) -> Retained<NSArray<NSCollectionViewLayoutAttributes>>;

        #[method_id(@__retain_semantics Other layoutAttributesForItemAtIndexPath:)]
        pub unsafe fn layoutAttributesForItemAtIndexPath(
            &self,
            index_path: &NSIndexPath,
        ) -> Option<Retained<NSCollectionViewLayoutAttributes>>;

        #[cfg(feature = "NSCollectionView")]
        #[method_id(@__retain_semantics Other layoutAttributesForSupplementaryViewOfKind:atIndexPath:)]
        pub unsafe fn layoutAttributesForSupplementaryViewOfKind_atIndexPath(
            &self,
            element_kind: &NSCollectionViewSupplementaryElementKind,
            index_path: &NSIndexPath,
        ) -> Option<Retained<NSCollectionViewLayoutAttributes>>;

        #[method_id(@__retain_semantics Other layoutAttributesForDecorationViewOfKind:atIndexPath:)]
        pub unsafe fn layoutAttributesForDecorationViewOfKind_atIndexPath(
            &self,
            element_kind: &NSCollectionViewDecorationElementKind,
            index_path: &NSIndexPath,
        ) -> Option<Retained<NSCollectionViewLayoutAttributes>>;

        #[method_id(@__retain_semantics Other layoutAttributesForDropTargetAtPoint:)]
        pub unsafe fn layoutAttributesForDropTargetAtPoint(
            &self,
            point_in_collection_view: NSPoint,
        ) -> Option<Retained<NSCollectionViewLayoutAttributes>>;

        #[method_id(@__retain_semantics Other layoutAttributesForInterItemGapBeforeIndexPath:)]
        pub unsafe fn layoutAttributesForInterItemGapBeforeIndexPath(
            &self,
            index_path: &NSIndexPath,
        ) -> Option<Retained<NSCollectionViewLayoutAttributes>>;

        #[method(shouldInvalidateLayoutForBoundsChange:)]
        pub unsafe fn shouldInvalidateLayoutForBoundsChange(&self, new_bounds: NSRect) -> bool;

        #[method_id(@__retain_semantics Other invalidationContextForBoundsChange:)]
        pub unsafe fn invalidationContextForBoundsChange(
            &self,
            new_bounds: NSRect,
        ) -> Retained<NSCollectionViewLayoutInvalidationContext>;

        #[method(shouldInvalidateLayoutForPreferredLayoutAttributes:withOriginalAttributes:)]
        pub unsafe fn shouldInvalidateLayoutForPreferredLayoutAttributes_withOriginalAttributes(
            &self,
            preferred_attributes: &NSCollectionViewLayoutAttributes,
            original_attributes: &NSCollectionViewLayoutAttributes,
        ) -> bool;

        #[method_id(@__retain_semantics Other invalidationContextForPreferredLayoutAttributes:withOriginalAttributes:)]
        pub unsafe fn invalidationContextForPreferredLayoutAttributes_withOriginalAttributes(
            &self,
            preferred_attributes: &NSCollectionViewLayoutAttributes,
            original_attributes: &NSCollectionViewLayoutAttributes,
        ) -> Retained<NSCollectionViewLayoutInvalidationContext>;

        #[method(targetContentOffsetForProposedContentOffset:withScrollingVelocity:)]
        pub unsafe fn targetContentOffsetForProposedContentOffset_withScrollingVelocity(
            &self,
            proposed_content_offset: NSPoint,
            velocity: NSPoint,
        ) -> NSPoint;

        #[method(targetContentOffsetForProposedContentOffset:)]
        pub unsafe fn targetContentOffsetForProposedContentOffset(
            &self,
            proposed_content_offset: NSPoint,
        ) -> NSPoint;

        #[method(collectionViewContentSize)]
        pub unsafe fn collectionViewContentSize(&self) -> NSSize;
    }
);

extern_methods!(
    /// NSUpdateSupportHooks
    unsafe impl NSCollectionViewLayout {
        #[method(prepareForCollectionViewUpdates:)]
        pub unsafe fn prepareForCollectionViewUpdates(
            &self,
            update_items: &NSArray<NSCollectionViewUpdateItem>,
        );

        #[method(finalizeCollectionViewUpdates)]
        pub unsafe fn finalizeCollectionViewUpdates(&self);

        #[method(prepareForAnimatedBoundsChange:)]
        pub unsafe fn prepareForAnimatedBoundsChange(&self, old_bounds: NSRect);

        #[method(finalizeAnimatedBoundsChange)]
        pub unsafe fn finalizeAnimatedBoundsChange(&self);

        #[method(prepareForTransitionToLayout:)]
        pub unsafe fn prepareForTransitionToLayout(&self, new_layout: &NSCollectionViewLayout);

        #[method(prepareForTransitionFromLayout:)]
        pub unsafe fn prepareForTransitionFromLayout(&self, old_layout: &NSCollectionViewLayout);

        #[method(finalizeLayoutTransition)]
        pub unsafe fn finalizeLayoutTransition(&self);

        #[method_id(@__retain_semantics Other initialLayoutAttributesForAppearingItemAtIndexPath:)]
        pub unsafe fn initialLayoutAttributesForAppearingItemAtIndexPath(
            &self,
            item_index_path: &NSIndexPath,
        ) -> Option<Retained<NSCollectionViewLayoutAttributes>>;

        #[method_id(@__retain_semantics Other finalLayoutAttributesForDisappearingItemAtIndexPath:)]
        pub unsafe fn finalLayoutAttributesForDisappearingItemAtIndexPath(
            &self,
            item_index_path: &NSIndexPath,
        ) -> Option<Retained<NSCollectionViewLayoutAttributes>>;

        #[cfg(feature = "NSCollectionView")]
        #[method_id(@__retain_semantics Other initialLayoutAttributesForAppearingSupplementaryElementOfKind:atIndexPath:)]
        pub unsafe fn initialLayoutAttributesForAppearingSupplementaryElementOfKind_atIndexPath(
            &self,
            element_kind: &NSCollectionViewSupplementaryElementKind,
            element_index_path: &NSIndexPath,
        ) -> Option<Retained<NSCollectionViewLayoutAttributes>>;

        #[cfg(feature = "NSCollectionView")]
        #[method_id(@__retain_semantics Other finalLayoutAttributesForDisappearingSupplementaryElementOfKind:atIndexPath:)]
        pub unsafe fn finalLayoutAttributesForDisappearingSupplementaryElementOfKind_atIndexPath(
            &self,
            element_kind: &NSCollectionViewSupplementaryElementKind,
            element_index_path: &NSIndexPath,
        ) -> Option<Retained<NSCollectionViewLayoutAttributes>>;

        #[method_id(@__retain_semantics Other initialLayoutAttributesForAppearingDecorationElementOfKind:atIndexPath:)]
        pub unsafe fn initialLayoutAttributesForAppearingDecorationElementOfKind_atIndexPath(
            &self,
            element_kind: &NSCollectionViewDecorationElementKind,
            decoration_index_path: &NSIndexPath,
        ) -> Option<Retained<NSCollectionViewLayoutAttributes>>;

        #[method_id(@__retain_semantics Other finalLayoutAttributesForDisappearingDecorationElementOfKind:atIndexPath:)]
        pub unsafe fn finalLayoutAttributesForDisappearingDecorationElementOfKind_atIndexPath(
            &self,
            element_kind: &NSCollectionViewDecorationElementKind,
            decoration_index_path: &NSIndexPath,
        ) -> Option<Retained<NSCollectionViewLayoutAttributes>>;

        #[cfg(feature = "NSCollectionView")]
        #[method_id(@__retain_semantics Other indexPathsToDeleteForSupplementaryViewOfKind:)]
        pub unsafe fn indexPathsToDeleteForSupplementaryViewOfKind(
            &self,
            element_kind: &NSCollectionViewSupplementaryElementKind,
        ) -> Retained<NSSet<NSIndexPath>>;

        #[method_id(@__retain_semantics Other indexPathsToDeleteForDecorationViewOfKind:)]
        pub unsafe fn indexPathsToDeleteForDecorationViewOfKind(
            &self,
            element_kind: &NSCollectionViewDecorationElementKind,
        ) -> Retained<NSSet<NSIndexPath>>;

        #[cfg(feature = "NSCollectionView")]
        #[method_id(@__retain_semantics Other indexPathsToInsertForSupplementaryViewOfKind:)]
        pub unsafe fn indexPathsToInsertForSupplementaryViewOfKind(
            &self,
            element_kind: &NSCollectionViewSupplementaryElementKind,
        ) -> Retained<NSSet<NSIndexPath>>;

        #[method_id(@__retain_semantics Other indexPathsToInsertForDecorationViewOfKind:)]
        pub unsafe fn indexPathsToInsertForDecorationViewOfKind(
            &self,
            element_kind: &NSCollectionViewDecorationElementKind,
        ) -> Retained<NSSet<NSIndexPath>>;
    }
);
