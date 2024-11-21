//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_TYPED_EXTENSIBLE_ENUM
pub type NSLayoutPriority = c_float;

pub static NSLayoutPriorityRequired: NSLayoutPriority = 1000 as _;

pub static NSLayoutPriorityDefaultHigh: NSLayoutPriority = 750 as _;

pub static NSLayoutPriorityDragThatCanResizeWindow: NSLayoutPriority = 510 as _;

pub static NSLayoutPriorityWindowSizeStayPut: NSLayoutPriority = 500 as _;

pub static NSLayoutPriorityDragThatCannotResizeWindow: NSLayoutPriority = 490 as _;

pub static NSLayoutPriorityDefaultLow: NSLayoutPriority = 250 as _;

pub static NSLayoutPriorityFittingSizeCompression: NSLayoutPriority = 50 as _;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSLayoutConstraintOrientation(pub NSInteger);
impl NSLayoutConstraintOrientation {
    #[doc(alias = "NSLayoutConstraintOrientationHorizontal")]
    pub const Horizontal: Self = Self(0);
    #[doc(alias = "NSLayoutConstraintOrientationVertical")]
    pub const Vertical: Self = Self(1);
}

unsafe impl Encode for NSLayoutConstraintOrientation {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSLayoutConstraintOrientation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSLayoutRelation(pub NSInteger);
impl NSLayoutRelation {
    #[doc(alias = "NSLayoutRelationLessThanOrEqual")]
    pub const LessThanOrEqual: Self = Self(-1);
    #[doc(alias = "NSLayoutRelationEqual")]
    pub const Equal: Self = Self(0);
    #[doc(alias = "NSLayoutRelationGreaterThanOrEqual")]
    pub const GreaterThanOrEqual: Self = Self(1);
}

unsafe impl Encode for NSLayoutRelation {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSLayoutRelation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSLayoutAttribute(pub NSInteger);
impl NSLayoutAttribute {
    #[doc(alias = "NSLayoutAttributeLeft")]
    pub const Left: Self = Self(1);
    #[doc(alias = "NSLayoutAttributeRight")]
    pub const Right: Self = Self(2);
    #[doc(alias = "NSLayoutAttributeTop")]
    pub const Top: Self = Self(3);
    #[doc(alias = "NSLayoutAttributeBottom")]
    pub const Bottom: Self = Self(4);
    #[doc(alias = "NSLayoutAttributeLeading")]
    pub const Leading: Self = Self(5);
    #[doc(alias = "NSLayoutAttributeTrailing")]
    pub const Trailing: Self = Self(6);
    #[doc(alias = "NSLayoutAttributeWidth")]
    pub const Width: Self = Self(7);
    #[doc(alias = "NSLayoutAttributeHeight")]
    pub const Height: Self = Self(8);
    #[doc(alias = "NSLayoutAttributeCenterX")]
    pub const CenterX: Self = Self(9);
    #[doc(alias = "NSLayoutAttributeCenterY")]
    pub const CenterY: Self = Self(10);
    #[doc(alias = "NSLayoutAttributeLastBaseline")]
    pub const LastBaseline: Self = Self(11);
    #[doc(alias = "NSLayoutAttributeBaseline")]
    pub const Baseline: Self = Self(NSLayoutAttribute::LastBaseline.0);
    #[doc(alias = "NSLayoutAttributeFirstBaseline")]
    pub const FirstBaseline: Self = Self(12);
    #[doc(alias = "NSLayoutAttributeNotAnAttribute")]
    pub const NotAnAttribute: Self = Self(0);
}

unsafe impl Encode for NSLayoutAttribute {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSLayoutAttribute {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSLayoutFormatOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSLayoutFormatOptions: NSUInteger {
        const NSLayoutFormatAlignAllLeft = 1<<NSLayoutAttribute::Left.0;
        const NSLayoutFormatAlignAllRight = 1<<NSLayoutAttribute::Right.0;
        const NSLayoutFormatAlignAllTop = 1<<NSLayoutAttribute::Top.0;
        const NSLayoutFormatAlignAllBottom = 1<<NSLayoutAttribute::Bottom.0;
        const NSLayoutFormatAlignAllLeading = 1<<NSLayoutAttribute::Leading.0;
        const NSLayoutFormatAlignAllTrailing = 1<<NSLayoutAttribute::Trailing.0;
        const NSLayoutFormatAlignAllCenterX = 1<<NSLayoutAttribute::CenterX.0;
        const NSLayoutFormatAlignAllCenterY = 1<<NSLayoutAttribute::CenterY.0;
        const NSLayoutFormatAlignAllLastBaseline = 1<<NSLayoutAttribute::LastBaseline.0;
        const NSLayoutFormatAlignAllFirstBaseline = 1<<NSLayoutAttribute::FirstBaseline.0;
        const NSLayoutFormatAlignAllBaseline = NSLayoutFormatOptions::NSLayoutFormatAlignAllLastBaseline.0;
        const NSLayoutFormatAlignmentMask = 0xFFFF;
        const NSLayoutFormatDirectionLeadingToTrailing = 0<<16;
        const NSLayoutFormatDirectionLeftToRight = 1<<16;
        const NSLayoutFormatDirectionRightToLeft = 2<<16;
        const NSLayoutFormatDirectionMask = 0x3<<16;
    }
}

unsafe impl Encode for NSLayoutFormatOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSLayoutFormatOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSLayoutConstraint;
);

unsafe impl NSObjectProtocol for NSLayoutConstraint {}

extern_methods!(
    unsafe impl NSLayoutConstraint {
        #[method_id(@__retain_semantics Other constraintsWithVisualFormat:options:metrics:views:)]
        pub unsafe fn constraintsWithVisualFormat_options_metrics_views(
            format: &NSString,
            opts: NSLayoutFormatOptions,
            metrics: Option<&NSDictionary<NSString, AnyObject>>,
            views: &NSDictionary<NSString, AnyObject>,
        ) -> Retained<NSArray<NSLayoutConstraint>>;

        #[method_id(@__retain_semantics Other constraintWithItem:attribute:relatedBy:toItem:attribute:multiplier:constant:)]
        pub unsafe fn constraintWithItem_attribute_relatedBy_toItem_attribute_multiplier_constant(
            view1: &AnyObject,
            attr1: NSLayoutAttribute,
            relation: NSLayoutRelation,
            view2: Option<&AnyObject>,
            attr2: NSLayoutAttribute,
            multiplier: CGFloat,
            c: CGFloat,
        ) -> Retained<Self>;

        #[method(priority)]
        pub unsafe fn priority(&self) -> NSLayoutPriority;

        #[method(setPriority:)]
        pub unsafe fn setPriority(&self, priority: NSLayoutPriority);

        #[method(shouldBeArchived)]
        pub unsafe fn shouldBeArchived(&self) -> bool;

        #[method(setShouldBeArchived:)]
        pub unsafe fn setShouldBeArchived(&self, should_be_archived: bool);

        #[method_id(@__retain_semantics Other firstItem)]
        pub unsafe fn firstItem(&self) -> Option<Retained<AnyObject>>;

        #[method_id(@__retain_semantics Other secondItem)]
        pub unsafe fn secondItem(&self) -> Option<Retained<AnyObject>>;

        #[method(firstAttribute)]
        pub unsafe fn firstAttribute(&self) -> NSLayoutAttribute;

        #[method(secondAttribute)]
        pub unsafe fn secondAttribute(&self) -> NSLayoutAttribute;

        #[cfg(feature = "NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other firstAnchor)]
        pub unsafe fn firstAnchor(&self) -> Retained<NSLayoutAnchor>;

        #[cfg(feature = "NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other secondAnchor)]
        pub unsafe fn secondAnchor(&self) -> Option<Retained<NSLayoutAnchor>>;

        #[method(relation)]
        pub unsafe fn relation(&self) -> NSLayoutRelation;

        #[method(multiplier)]
        pub unsafe fn multiplier(&self) -> CGFloat;

        #[method(constant)]
        pub unsafe fn constant(&self) -> CGFloat;

        #[method(setConstant:)]
        pub unsafe fn setConstant(&self, constant: CGFloat);

        #[method(isActive)]
        pub unsafe fn isActive(&self) -> bool;

        #[method(setActive:)]
        pub unsafe fn setActive(&self, active: bool);

        #[method(activateConstraints:)]
        pub unsafe fn activateConstraints(constraints: &NSArray<NSLayoutConstraint>);

        #[method(deactivateConstraints:)]
        pub unsafe fn deactivateConstraints(constraints: &NSArray<NSLayoutConstraint>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSLayoutConstraint {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// NSIdentifier
    unsafe impl NSLayoutConstraint {
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Option<Retained<NSString>>;

        #[method(setIdentifier:)]
        pub unsafe fn setIdentifier(&self, identifier: Option<&NSString>);
    }
);

extern_methods!(
    unsafe impl NSLayoutConstraint {}
);

#[cfg(feature = "NSAnimation")]
unsafe impl NSAnimatablePropertyContainer for NSLayoutConstraint {}

extern_methods!(
    /// NSConstraintBasedLayoutInstallingConstraints
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSView {
        #[cfg(feature = "NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other leadingAnchor)]
        pub unsafe fn leadingAnchor(&self) -> Retained<NSLayoutXAxisAnchor>;

        #[cfg(feature = "NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other trailingAnchor)]
        pub unsafe fn trailingAnchor(&self) -> Retained<NSLayoutXAxisAnchor>;

        #[cfg(feature = "NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other leftAnchor)]
        pub unsafe fn leftAnchor(&self) -> Retained<NSLayoutXAxisAnchor>;

        #[cfg(feature = "NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other rightAnchor)]
        pub unsafe fn rightAnchor(&self) -> Retained<NSLayoutXAxisAnchor>;

        #[cfg(feature = "NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other topAnchor)]
        pub unsafe fn topAnchor(&self) -> Retained<NSLayoutYAxisAnchor>;

        #[cfg(feature = "NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other bottomAnchor)]
        pub unsafe fn bottomAnchor(&self) -> Retained<NSLayoutYAxisAnchor>;

        #[cfg(feature = "NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other widthAnchor)]
        pub unsafe fn widthAnchor(&self) -> Retained<NSLayoutDimension>;

        #[cfg(feature = "NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other heightAnchor)]
        pub unsafe fn heightAnchor(&self) -> Retained<NSLayoutDimension>;

        #[cfg(feature = "NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other centerXAnchor)]
        pub unsafe fn centerXAnchor(&self) -> Retained<NSLayoutXAxisAnchor>;

        #[cfg(feature = "NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other centerYAnchor)]
        pub unsafe fn centerYAnchor(&self) -> Retained<NSLayoutYAxisAnchor>;

        #[cfg(feature = "NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other firstBaselineAnchor)]
        pub unsafe fn firstBaselineAnchor(&self) -> Retained<NSLayoutYAxisAnchor>;

        #[cfg(feature = "NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other lastBaselineAnchor)]
        pub unsafe fn lastBaselineAnchor(&self) -> Retained<NSLayoutYAxisAnchor>;

        #[method_id(@__retain_semantics Other constraints)]
        pub unsafe fn constraints(&self) -> Retained<NSArray<NSLayoutConstraint>>;

        #[method(addConstraint:)]
        pub unsafe fn addConstraint(&self, constraint: &NSLayoutConstraint);

        #[method(addConstraints:)]
        pub unsafe fn addConstraints(&self, constraints: &NSArray<NSLayoutConstraint>);

        #[method(removeConstraint:)]
        pub unsafe fn removeConstraint(&self, constraint: &NSLayoutConstraint);

        #[method(removeConstraints:)]
        pub unsafe fn removeConstraints(&self, constraints: &NSArray<NSLayoutConstraint>);
    }
);

extern_methods!(
    /// NSConstraintBasedLayoutCoreMethods
    #[cfg(all(feature = "NSResponder", feature = "NSWindow"))]
    unsafe impl NSWindow {
        #[method(updateConstraintsIfNeeded)]
        pub unsafe fn updateConstraintsIfNeeded(&self);

        #[method(layoutIfNeeded)]
        pub unsafe fn layoutIfNeeded(&self);
    }
);

extern_methods!(
    /// NSConstraintBasedLayoutCoreMethods
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSView {
        #[method(updateConstraintsForSubtreeIfNeeded)]
        pub unsafe fn updateConstraintsForSubtreeIfNeeded(&self);

        #[method(updateConstraints)]
        pub unsafe fn updateConstraints(&self);

        #[method(needsUpdateConstraints)]
        pub unsafe fn needsUpdateConstraints(&self) -> bool;

        #[method(setNeedsUpdateConstraints:)]
        pub unsafe fn setNeedsUpdateConstraints(&self, needs_update_constraints: bool);
    }
);

extern_methods!(
    /// NSConstraintBasedCompatibility
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSView {
        #[method(translatesAutoresizingMaskIntoConstraints)]
        pub unsafe fn translatesAutoresizingMaskIntoConstraints(&self) -> bool;

        #[method(setTranslatesAutoresizingMaskIntoConstraints:)]
        pub unsafe fn setTranslatesAutoresizingMaskIntoConstraints(
            &self,
            translates_autoresizing_mask_into_constraints: bool,
        );

        #[method(requiresConstraintBasedLayout)]
        pub unsafe fn requiresConstraintBasedLayout(mtm: MainThreadMarker) -> bool;
    }
);

extern "C" {
    pub static NSViewNoInstrinsicMetric: CGFloat;
}

extern "C" {
    pub static NSViewNoIntrinsicMetric: CGFloat;
}

extern_methods!(
    /// NSConstraintBasedLayoutLayering
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSView {
        #[method(alignmentRectForFrame:)]
        pub unsafe fn alignmentRectForFrame(&self, frame: NSRect) -> NSRect;

        #[method(frameForAlignmentRect:)]
        pub unsafe fn frameForAlignmentRect(&self, alignment_rect: NSRect) -> NSRect;

        #[method(alignmentRectInsets)]
        pub unsafe fn alignmentRectInsets(&self) -> NSEdgeInsets;

        #[method(firstBaselineOffsetFromTop)]
        pub unsafe fn firstBaselineOffsetFromTop(&self) -> CGFloat;

        #[method(lastBaselineOffsetFromBottom)]
        pub unsafe fn lastBaselineOffsetFromBottom(&self) -> CGFloat;

        #[method(baselineOffsetFromBottom)]
        pub unsafe fn baselineOffsetFromBottom(&self) -> CGFloat;

        #[method(intrinsicContentSize)]
        pub unsafe fn intrinsicContentSize(&self) -> NSSize;

        #[method(invalidateIntrinsicContentSize)]
        pub unsafe fn invalidateIntrinsicContentSize(&self);

        #[method(contentHuggingPriorityForOrientation:)]
        pub unsafe fn contentHuggingPriorityForOrientation(
            &self,
            orientation: NSLayoutConstraintOrientation,
        ) -> NSLayoutPriority;

        #[method(setContentHuggingPriority:forOrientation:)]
        pub unsafe fn setContentHuggingPriority_forOrientation(
            &self,
            priority: NSLayoutPriority,
            orientation: NSLayoutConstraintOrientation,
        );

        #[method(contentCompressionResistancePriorityForOrientation:)]
        pub unsafe fn contentCompressionResistancePriorityForOrientation(
            &self,
            orientation: NSLayoutConstraintOrientation,
        ) -> NSLayoutPriority;

        #[method(setContentCompressionResistancePriority:forOrientation:)]
        pub unsafe fn setContentCompressionResistancePriority_forOrientation(
            &self,
            priority: NSLayoutPriority,
            orientation: NSLayoutConstraintOrientation,
        );

        #[method(isHorizontalContentSizeConstraintActive)]
        pub unsafe fn isHorizontalContentSizeConstraintActive(&self) -> bool;

        #[method(setHorizontalContentSizeConstraintActive:)]
        pub unsafe fn setHorizontalContentSizeConstraintActive(
            &self,
            horizontal_content_size_constraint_active: bool,
        );

        #[method(isVerticalContentSizeConstraintActive)]
        pub unsafe fn isVerticalContentSizeConstraintActive(&self) -> bool;

        #[method(setVerticalContentSizeConstraintActive:)]
        pub unsafe fn setVerticalContentSizeConstraintActive(
            &self,
            vertical_content_size_constraint_active: bool,
        );
    }
);

extern_methods!(
    /// NSConstraintBasedLayoutLayering
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSControl {
        #[cfg(feature = "NSCell")]
        #[method(invalidateIntrinsicContentSizeForCell:)]
        pub unsafe fn invalidateIntrinsicContentSizeForCell(&self, cell: &NSCell);
    }
);

extern_methods!(
    /// NSConstraintBasedLayoutAnchoring
    #[cfg(all(feature = "NSResponder", feature = "NSWindow"))]
    unsafe impl NSWindow {
        #[method(anchorAttributeForOrientation:)]
        pub unsafe fn anchorAttributeForOrientation(
            &self,
            orientation: NSLayoutConstraintOrientation,
        ) -> NSLayoutAttribute;

        #[method(setAnchorAttribute:forOrientation:)]
        pub unsafe fn setAnchorAttribute_forOrientation(
            &self,
            attr: NSLayoutAttribute,
            orientation: NSLayoutConstraintOrientation,
        );
    }
);

extern_methods!(
    /// NSConstraintBasedLayoutFittingSize
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSView {
        #[method(fittingSize)]
        pub unsafe fn fittingSize(&self) -> NSSize;
    }
);

extern_methods!(
    /// NSConstraintBasedLayoutDebugging
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSView {
        #[method_id(@__retain_semantics Other constraintsAffectingLayoutForOrientation:)]
        pub unsafe fn constraintsAffectingLayoutForOrientation(
            &self,
            orientation: NSLayoutConstraintOrientation,
        ) -> Retained<NSArray<NSLayoutConstraint>>;

        #[method(hasAmbiguousLayout)]
        pub unsafe fn hasAmbiguousLayout(&self) -> bool;

        #[method(exerciseAmbiguityInLayout)]
        pub unsafe fn exerciseAmbiguityInLayout(&self);
    }
);

extern_methods!(
    /// NSConstraintBasedLayoutDebugging
    #[cfg(all(feature = "NSResponder", feature = "NSWindow"))]
    unsafe impl NSWindow {
        #[method(visualizeConstraints:)]
        pub unsafe fn visualizeConstraints(
            &self,
            constraints: Option<&NSArray<NSLayoutConstraint>>,
        );
    }
);
