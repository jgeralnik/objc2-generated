//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uigraphicsimagedrawingactions?language=objc)
#[cfg(all(feature = "UIGraphicsRenderer", feature = "block2"))]
pub type UIGraphicsImageDrawingActions =
    *mut block2::Block<dyn Fn(NonNull<UIGraphicsImageRendererContext>)>;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uigraphicsimagerendererformatrange?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIGraphicsImageRendererFormatRange(pub NSInteger);
impl UIGraphicsImageRendererFormatRange {
    #[doc(alias = "UIGraphicsImageRendererFormatRangeUnspecified")]
    pub const Unspecified: Self = Self(-1);
    #[doc(alias = "UIGraphicsImageRendererFormatRangeAutomatic")]
    pub const Automatic: Self = Self(0);
    #[doc(alias = "UIGraphicsImageRendererFormatRangeExtended")]
    pub const Extended: Self = Self(1);
    #[doc(alias = "UIGraphicsImageRendererFormatRangeStandard")]
    pub const Standard: Self = Self(2);
}

unsafe impl Encode for UIGraphicsImageRendererFormatRange {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIGraphicsImageRendererFormatRange {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uigraphicsimagerendererformat?language=objc)
    #[unsafe(super(UIGraphicsRendererFormat, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIGraphicsRenderer")]
    pub struct UIGraphicsImageRendererFormat;
);

#[cfg(feature = "UIGraphicsRenderer")]
unsafe impl NSCopying for UIGraphicsImageRendererFormat {}

#[cfg(feature = "UIGraphicsRenderer")]
unsafe impl CopyingHelper for UIGraphicsImageRendererFormat {
    type Result = Self;
}

#[cfg(feature = "UIGraphicsRenderer")]
unsafe impl NSObjectProtocol for UIGraphicsImageRendererFormat {}

extern_methods!(
    #[cfg(feature = "UIGraphicsRenderer")]
    unsafe impl UIGraphicsImageRendererFormat {
        #[method(scale)]
        pub unsafe fn scale(&self) -> CGFloat;

        #[method(setScale:)]
        pub unsafe fn setScale(&self, scale: CGFloat);

        #[method(opaque)]
        pub unsafe fn opaque(&self) -> bool;

        #[method(setOpaque:)]
        pub unsafe fn setOpaque(&self, opaque: bool);

        #[deprecated = "Use the preferredRange property instead"]
        #[method(prefersExtendedRange)]
        pub unsafe fn prefersExtendedRange(&self) -> bool;

        #[deprecated = "Use the preferredRange property instead"]
        #[method(setPrefersExtendedRange:)]
        pub unsafe fn setPrefersExtendedRange(&self, prefers_extended_range: bool);

        #[method(supportsHighDynamicRange)]
        pub unsafe fn supportsHighDynamicRange(&self) -> bool;

        #[method(preferredRange)]
        pub unsafe fn preferredRange(&self) -> UIGraphicsImageRendererFormatRange;

        #[method(setPreferredRange:)]
        pub unsafe fn setPreferredRange(&self, preferred_range: UIGraphicsImageRendererFormatRange);

        #[cfg(feature = "UITraitCollection")]
        #[method_id(@__retain_semantics Other formatForTraitCollection:)]
        pub unsafe fn formatForTraitCollection(
            trait_collection: &UITraitCollection,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UIGraphicsRendererFormat`
    #[cfg(feature = "UIGraphicsRenderer")]
    unsafe impl UIGraphicsImageRendererFormat {
        #[deprecated]
        #[method_id(@__retain_semantics Other defaultFormat)]
        pub unsafe fn defaultFormat() -> Retained<Self>;

        #[method_id(@__retain_semantics Other preferredFormat)]
        pub unsafe fn preferredFormat() -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UIGraphicsRenderer")]
    unsafe impl UIGraphicsImageRendererFormat {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uigraphicsimagerenderercontext?language=objc)
    #[unsafe(super(UIGraphicsRendererContext, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIGraphicsRenderer")]
    pub struct UIGraphicsImageRendererContext;
);

#[cfg(feature = "UIGraphicsRenderer")]
unsafe impl NSObjectProtocol for UIGraphicsImageRendererContext {}

extern_methods!(
    #[cfg(feature = "UIGraphicsRenderer")]
    unsafe impl UIGraphicsImageRendererContext {
        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other currentImage)]
        pub unsafe fn currentImage(&self) -> Retained<UIImage>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UIGraphicsRenderer")]
    unsafe impl UIGraphicsImageRendererContext {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uigraphicsimagerenderer?language=objc)
    #[unsafe(super(UIGraphicsRenderer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIGraphicsRenderer")]
    pub struct UIGraphicsImageRenderer;
);

#[cfg(feature = "UIGraphicsRenderer")]
unsafe impl NSObjectProtocol for UIGraphicsImageRenderer {}

extern_methods!(
    #[cfg(feature = "UIGraphicsRenderer")]
    unsafe impl UIGraphicsImageRenderer {
        #[method_id(@__retain_semantics Init initWithSize:)]
        pub unsafe fn initWithSize(this: Allocated<Self>, size: CGSize) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithSize:format:)]
        pub unsafe fn initWithSize_format(
            this: Allocated<Self>,
            size: CGSize,
            format: &UIGraphicsImageRendererFormat,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithBounds:format:)]
        pub unsafe fn initWithBounds_format(
            this: Allocated<Self>,
            bounds: CGRect,
            format: &UIGraphicsImageRendererFormat,
        ) -> Retained<Self>;

        #[cfg(all(feature = "UIImage", feature = "block2"))]
        #[method_id(@__retain_semantics Other imageWithActions:)]
        pub unsafe fn imageWithActions(
            &self,
            actions: UIGraphicsImageDrawingActions,
        ) -> Retained<UIImage>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Other PNGDataWithActions:)]
        pub unsafe fn PNGDataWithActions(
            &self,
            actions: UIGraphicsImageDrawingActions,
        ) -> Retained<NSData>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Other JPEGDataWithCompressionQuality:actions:)]
        pub unsafe fn JPEGDataWithCompressionQuality_actions(
            &self,
            compression_quality: CGFloat,
            actions: UIGraphicsImageDrawingActions,
        ) -> Retained<NSData>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UIGraphicsRenderer`
    #[cfg(feature = "UIGraphicsRenderer")]
    unsafe impl UIGraphicsImageRenderer {
        #[method_id(@__retain_semantics Init initWithBounds:)]
        pub unsafe fn initWithBounds(this: Allocated<Self>, bounds: CGRect) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UIGraphicsRenderer")]
    unsafe impl UIGraphicsImageRenderer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
