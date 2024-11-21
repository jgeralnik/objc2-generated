//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTextLayoutFragmentEnumerationOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSTextLayoutFragmentEnumerationOptions: NSUInteger {
        #[doc(alias = "NSTextLayoutFragmentEnumerationOptionsNone")]
        const None = 0;
        #[doc(alias = "NSTextLayoutFragmentEnumerationOptionsReverse")]
        const Reverse = 1<<0;
        #[doc(alias = "NSTextLayoutFragmentEnumerationOptionsEstimatesSize")]
        const EstimatesSize = 1<<1;
        #[doc(alias = "NSTextLayoutFragmentEnumerationOptionsEnsuresLayout")]
        const EnsuresLayout = 1<<2;
        #[doc(alias = "NSTextLayoutFragmentEnumerationOptionsEnsuresExtraLineFragment")]
        const EnsuresExtraLineFragment = 1<<3;
    }
}

unsafe impl Encode for NSTextLayoutFragmentEnumerationOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSTextLayoutFragmentEnumerationOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTextLayoutFragmentState(pub NSUInteger);
impl NSTextLayoutFragmentState {
    #[doc(alias = "NSTextLayoutFragmentStateNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "NSTextLayoutFragmentStateEstimatedUsageBounds")]
    pub const EstimatedUsageBounds: Self = Self(1);
    #[doc(alias = "NSTextLayoutFragmentStateCalculatedUsageBounds")]
    pub const CalculatedUsageBounds: Self = Self(2);
    #[doc(alias = "NSTextLayoutFragmentStateLayoutAvailable")]
    pub const LayoutAvailable: Self = Self(3);
}

unsafe impl Encode for NSTextLayoutFragmentState {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSTextLayoutFragmentState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextLayoutFragment;
);

unsafe impl NSCoding for NSTextLayoutFragment {}

unsafe impl NSObjectProtocol for NSTextLayoutFragment {}

unsafe impl NSSecureCoding for NSTextLayoutFragment {}

extern_methods!(
    unsafe impl NSTextLayoutFragment {
        #[cfg(all(feature = "NSTextElement", feature = "NSTextRange"))]
        #[method_id(@__retain_semantics Init initWithTextElement:range:)]
        pub unsafe fn initWithTextElement_range(
            this: Allocated<Self>,
            text_element: &NSTextElement,
            range_in_element: Option<&NSTextRange>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "NSTextLayoutManager")]
        #[method_id(@__retain_semantics Other textLayoutManager)]
        pub unsafe fn textLayoutManager(&self) -> Option<Retained<NSTextLayoutManager>>;

        #[cfg(feature = "NSTextElement")]
        #[method_id(@__retain_semantics Other textElement)]
        pub unsafe fn textElement(&self) -> Option<Retained<NSTextElement>>;

        #[cfg(feature = "NSTextRange")]
        #[method_id(@__retain_semantics Other rangeInElement)]
        pub unsafe fn rangeInElement(&self) -> Retained<NSTextRange>;

        #[cfg(feature = "NSTextLineFragment")]
        #[method_id(@__retain_semantics Other textLineFragments)]
        pub unsafe fn textLineFragments(&self) -> Retained<NSArray<NSTextLineFragment>>;

        #[cfg(feature = "NSTextLineFragment")]
        #[method_id(@__retain_semantics Other textLineFragmentForVerticalOffset:requiresExactMatch:)]
        pub unsafe fn textLineFragmentForVerticalOffset_requiresExactMatch(
            &self,
            vertical_offset: CGFloat,
            requires_exact_match: bool,
        ) -> Option<Retained<NSTextLineFragment>>;

        #[cfg(all(feature = "NSTextLineFragment", feature = "NSTextRange"))]
        #[method_id(@__retain_semantics Other textLineFragmentForTextLocation:isUpstreamAffinity:)]
        pub unsafe fn textLineFragmentForTextLocation_isUpstreamAffinity(
            &self,
            text_location: &ProtocolObject<dyn NSTextLocation>,
            is_upstream_affinity: bool,
        ) -> Option<Retained<NSTextLineFragment>>;

        #[method_id(@__retain_semantics Other layoutQueue)]
        pub unsafe fn layoutQueue(&self) -> Option<Retained<NSOperationQueue>>;

        #[method(setLayoutQueue:)]
        pub unsafe fn setLayoutQueue(&self, layout_queue: Option<&NSOperationQueue>);

        #[method(state)]
        pub unsafe fn state(&self) -> NSTextLayoutFragmentState;

        #[method(invalidateLayout)]
        pub unsafe fn invalidateLayout(&self);

        #[method(layoutFragmentFrame)]
        pub unsafe fn layoutFragmentFrame(&self) -> CGRect;

        #[method(renderingSurfaceBounds)]
        pub unsafe fn renderingSurfaceBounds(&self) -> CGRect;

        #[method(leadingPadding)]
        pub unsafe fn leadingPadding(&self) -> CGFloat;

        #[method(trailingPadding)]
        pub unsafe fn trailingPadding(&self) -> CGFloat;

        #[method(topMargin)]
        pub unsafe fn topMargin(&self) -> CGFloat;

        #[method(bottomMargin)]
        pub unsafe fn bottomMargin(&self) -> CGFloat;

        #[cfg(feature = "NSTextAttachment")]
        #[method_id(@__retain_semantics Other textAttachmentViewProviders)]
        pub unsafe fn textAttachmentViewProviders(
            &self,
        ) -> Retained<NSArray<NSTextAttachmentViewProvider>>;

        #[cfg(feature = "NSTextRange")]
        #[method(frameForTextAttachmentAtLocation:)]
        pub unsafe fn frameForTextAttachmentAtLocation(
            &self,
            location: &ProtocolObject<dyn NSTextLocation>,
        ) -> CGRect;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSTextLayoutFragment {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
