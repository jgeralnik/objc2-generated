//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    pub static UICollectionViewFlowLayoutAutomaticSize: CGSize;
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UICollectionViewFlowLayoutSectionInsetReference(pub NSInteger);
impl UICollectionViewFlowLayoutSectionInsetReference {
    pub const UICollectionViewFlowLayoutSectionInsetFromContentInset: Self = Self(0);
    pub const UICollectionViewFlowLayoutSectionInsetFromSafeArea: Self = Self(1);
    pub const UICollectionViewFlowLayoutSectionInsetFromLayoutMargins: Self = Self(2);
}

unsafe impl Encode for UICollectionViewFlowLayoutSectionInsetReference {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UICollectionViewFlowLayoutSectionInsetReference {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UICollectionViewLayout")]
    pub struct UICollectionViewFlowLayoutInvalidationContext;

    #[cfg(feature = "UICollectionViewLayout")]
    unsafe impl ClassType for UICollectionViewFlowLayoutInvalidationContext {
        #[inherits(NSObject)]
        type Super = UICollectionViewLayoutInvalidationContext;
        type ThreadKind = dyn MainThreadOnly;
    }
);

#[cfg(feature = "UICollectionViewLayout")]
unsafe impl NSObjectProtocol for UICollectionViewFlowLayoutInvalidationContext {}

extern_methods!(
    #[cfg(feature = "UICollectionViewLayout")]
    unsafe impl UICollectionViewFlowLayoutInvalidationContext {
        #[method(invalidateFlowLayoutDelegateMetrics)]
        pub unsafe fn invalidateFlowLayoutDelegateMetrics(&self) -> bool;

        #[method(setInvalidateFlowLayoutDelegateMetrics:)]
        pub unsafe fn setInvalidateFlowLayoutDelegateMetrics(
            &self,
            invalidate_flow_layout_delegate_metrics: bool,
        );

        #[method(invalidateFlowLayoutAttributes)]
        pub unsafe fn invalidateFlowLayoutAttributes(&self) -> bool;

        #[method(setInvalidateFlowLayoutAttributes:)]
        pub unsafe fn setInvalidateFlowLayoutAttributes(
            &self,
            invalidate_flow_layout_attributes: bool,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UICollectionViewLayout")]
    unsafe impl UICollectionViewFlowLayoutInvalidationContext {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    #[cfg(all(feature = "UICollectionView", feature = "UIScrollView"))]
    pub unsafe trait UICollectionViewDelegateFlowLayout:
        UICollectionViewDelegate + MainThreadOnly
    {
        #[cfg(all(
            feature = "UICollectionViewLayout",
            feature = "UIResponder",
            feature = "UIView"
        ))]
        #[optional]
        #[method(collectionView:layout:sizeForItemAtIndexPath:)]
        unsafe fn collectionView_layout_sizeForItemAtIndexPath(
            &self,
            collection_view: &UICollectionView,
            collection_view_layout: &UICollectionViewLayout,
            index_path: &NSIndexPath,
        ) -> CGSize;

        #[cfg(all(
            feature = "UICollectionViewLayout",
            feature = "UIGeometry",
            feature = "UIResponder",
            feature = "UIView"
        ))]
        #[optional]
        #[method(collectionView:layout:insetForSectionAtIndex:)]
        unsafe fn collectionView_layout_insetForSectionAtIndex(
            &self,
            collection_view: &UICollectionView,
            collection_view_layout: &UICollectionViewLayout,
            section: NSInteger,
        ) -> UIEdgeInsets;

        #[cfg(all(
            feature = "UICollectionViewLayout",
            feature = "UIResponder",
            feature = "UIView"
        ))]
        #[optional]
        #[method(collectionView:layout:minimumLineSpacingForSectionAtIndex:)]
        unsafe fn collectionView_layout_minimumLineSpacingForSectionAtIndex(
            &self,
            collection_view: &UICollectionView,
            collection_view_layout: &UICollectionViewLayout,
            section: NSInteger,
        ) -> CGFloat;

        #[cfg(all(
            feature = "UICollectionViewLayout",
            feature = "UIResponder",
            feature = "UIView"
        ))]
        #[optional]
        #[method(collectionView:layout:minimumInteritemSpacingForSectionAtIndex:)]
        unsafe fn collectionView_layout_minimumInteritemSpacingForSectionAtIndex(
            &self,
            collection_view: &UICollectionView,
            collection_view_layout: &UICollectionViewLayout,
            section: NSInteger,
        ) -> CGFloat;

        #[cfg(all(
            feature = "UICollectionViewLayout",
            feature = "UIResponder",
            feature = "UIView"
        ))]
        #[optional]
        #[method(collectionView:layout:referenceSizeForHeaderInSection:)]
        unsafe fn collectionView_layout_referenceSizeForHeaderInSection(
            &self,
            collection_view: &UICollectionView,
            collection_view_layout: &UICollectionViewLayout,
            section: NSInteger,
        ) -> CGSize;

        #[cfg(all(
            feature = "UICollectionViewLayout",
            feature = "UIResponder",
            feature = "UIView"
        ))]
        #[optional]
        #[method(collectionView:layout:referenceSizeForFooterInSection:)]
        unsafe fn collectionView_layout_referenceSizeForFooterInSection(
            &self,
            collection_view: &UICollectionView,
            collection_view_layout: &UICollectionViewLayout,
            section: NSInteger,
        ) -> CGSize;
    }

    #[cfg(all(feature = "UICollectionView", feature = "UIScrollView"))]
    unsafe impl ProtocolType for dyn UICollectionViewDelegateFlowLayout {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UICollectionViewLayout")]
    pub struct UICollectionViewFlowLayout;

    #[cfg(feature = "UICollectionViewLayout")]
    unsafe impl ClassType for UICollectionViewFlowLayout {
        #[inherits(NSObject)]
        type Super = UICollectionViewLayout;
        type ThreadKind = dyn MainThreadOnly;
    }
);

#[cfg(feature = "UICollectionViewLayout")]
unsafe impl NSCoding for UICollectionViewFlowLayout {}

#[cfg(feature = "UICollectionViewLayout")]
unsafe impl NSObjectProtocol for UICollectionViewFlowLayout {}

extern_methods!(
    #[cfg(feature = "UICollectionViewLayout")]
    unsafe impl UICollectionViewFlowLayout {
        #[method(minimumLineSpacing)]
        pub unsafe fn minimumLineSpacing(&self) -> CGFloat;

        #[method(setMinimumLineSpacing:)]
        pub unsafe fn setMinimumLineSpacing(&self, minimum_line_spacing: CGFloat);

        #[method(minimumInteritemSpacing)]
        pub unsafe fn minimumInteritemSpacing(&self) -> CGFloat;

        #[method(setMinimumInteritemSpacing:)]
        pub unsafe fn setMinimumInteritemSpacing(&self, minimum_interitem_spacing: CGFloat);

        #[method(itemSize)]
        pub unsafe fn itemSize(&self) -> CGSize;

        #[method(setItemSize:)]
        pub unsafe fn setItemSize(&self, item_size: CGSize);

        #[method(estimatedItemSize)]
        pub unsafe fn estimatedItemSize(&self) -> CGSize;

        #[method(setEstimatedItemSize:)]
        pub unsafe fn setEstimatedItemSize(&self, estimated_item_size: CGSize);

        #[method(scrollDirection)]
        pub unsafe fn scrollDirection(&self) -> UICollectionViewScrollDirection;

        #[method(setScrollDirection:)]
        pub unsafe fn setScrollDirection(&self, scroll_direction: UICollectionViewScrollDirection);

        #[method(headerReferenceSize)]
        pub unsafe fn headerReferenceSize(&self) -> CGSize;

        #[method(setHeaderReferenceSize:)]
        pub unsafe fn setHeaderReferenceSize(&self, header_reference_size: CGSize);

        #[method(footerReferenceSize)]
        pub unsafe fn footerReferenceSize(&self) -> CGSize;

        #[method(setFooterReferenceSize:)]
        pub unsafe fn setFooterReferenceSize(&self, footer_reference_size: CGSize);

        #[cfg(feature = "UIGeometry")]
        #[method(sectionInset)]
        pub unsafe fn sectionInset(&self) -> UIEdgeInsets;

        #[cfg(feature = "UIGeometry")]
        #[method(setSectionInset:)]
        pub unsafe fn setSectionInset(&self, section_inset: UIEdgeInsets);

        #[method(sectionInsetReference)]
        pub unsafe fn sectionInsetReference(
            &self,
        ) -> UICollectionViewFlowLayoutSectionInsetReference;

        #[method(setSectionInsetReference:)]
        pub unsafe fn setSectionInsetReference(
            &self,
            section_inset_reference: UICollectionViewFlowLayoutSectionInsetReference,
        );

        #[method(sectionHeadersPinToVisibleBounds)]
        pub unsafe fn sectionHeadersPinToVisibleBounds(&self) -> bool;

        #[method(setSectionHeadersPinToVisibleBounds:)]
        pub unsafe fn setSectionHeadersPinToVisibleBounds(
            &self,
            section_headers_pin_to_visible_bounds: bool,
        );

        #[method(sectionFootersPinToVisibleBounds)]
        pub unsafe fn sectionFootersPinToVisibleBounds(&self) -> bool;

        #[method(setSectionFootersPinToVisibleBounds:)]
        pub unsafe fn setSectionFootersPinToVisibleBounds(
            &self,
            section_footers_pin_to_visible_bounds: bool,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `UICollectionViewLayout`
    #[cfg(feature = "UICollectionViewLayout")]
    unsafe impl UICollectionViewFlowLayout {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UICollectionViewLayout")]
    unsafe impl UICollectionViewFlowLayout {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
