//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSStringDrawingContext;
);

unsafe impl NSObjectProtocol for NSStringDrawingContext {}

extern_methods!(
    unsafe impl NSStringDrawingContext {
        #[method(minimumScaleFactor)]
        pub unsafe fn minimumScaleFactor(&self) -> CGFloat;

        #[method(setMinimumScaleFactor:)]
        pub unsafe fn setMinimumScaleFactor(&self, minimum_scale_factor: CGFloat);

        #[method(actualScaleFactor)]
        pub unsafe fn actualScaleFactor(&self) -> CGFloat;

        #[method(totalBounds)]
        pub unsafe fn totalBounds(&self) -> CGRect;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSStringDrawingContext {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_category!(
    /// Category on [`NSString`].
    pub unsafe trait NSStringDrawing {
        #[method(sizeWithAttributes:)]
        unsafe fn sizeWithAttributes(
            &self,
            attrs: Option<&NSDictionary<NSAttributedStringKey, AnyObject>>,
        ) -> CGSize;

        #[method(drawAtPoint:withAttributes:)]
        unsafe fn drawAtPoint_withAttributes(
            &self,
            point: CGPoint,
            attrs: Option<&NSDictionary<NSAttributedStringKey, AnyObject>>,
        );

        #[method(drawInRect:withAttributes:)]
        unsafe fn drawInRect_withAttributes(
            &self,
            rect: CGRect,
            attrs: Option<&NSDictionary<NSAttributedStringKey, AnyObject>>,
        );
    }

    unsafe impl NSStringDrawing for NSString {}
);

extern_category!(
    /// Category "NSStringDrawing" on [`NSAttributedString`].
    #[doc(alias = "NSStringDrawing")]
    pub unsafe trait NSAttributedStringNSStringDrawing {
        #[method(size)]
        unsafe fn size(&self) -> CGSize;

        #[method(drawAtPoint:)]
        unsafe fn drawAtPoint(&self, point: CGPoint);

        #[method(drawInRect:)]
        unsafe fn drawInRect(&self, rect: CGRect);
    }

    unsafe impl NSAttributedStringNSStringDrawing for NSAttributedString {}
);

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSStringDrawingOptions(pub NSInteger);
bitflags::bitflags! {
    impl NSStringDrawingOptions: NSInteger {
        const NSStringDrawingUsesLineFragmentOrigin = 1<<0;
        const NSStringDrawingUsesFontLeading = 1<<1;
        const NSStringDrawingUsesDeviceMetrics = 1<<3;
        const NSStringDrawingTruncatesLastVisibleLine = 1<<5;
    }
}

unsafe impl Encode for NSStringDrawingOptions {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSStringDrawingOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_category!(
    /// Category "NSExtendedStringDrawing" on [`NSString`].
    #[doc(alias = "NSExtendedStringDrawing")]
    pub unsafe trait NSStringNSExtendedStringDrawing {
        #[method(drawWithRect:options:attributes:context:)]
        unsafe fn drawWithRect_options_attributes_context(
            &self,
            rect: CGRect,
            options: NSStringDrawingOptions,
            attributes: Option<&NSDictionary<NSAttributedStringKey, AnyObject>>,
            context: Option<&NSStringDrawingContext>,
        );

        #[method(boundingRectWithSize:options:attributes:context:)]
        unsafe fn boundingRectWithSize_options_attributes_context(
            &self,
            size: CGSize,
            options: NSStringDrawingOptions,
            attributes: Option<&NSDictionary<NSAttributedStringKey, AnyObject>>,
            context: Option<&NSStringDrawingContext>,
        ) -> CGRect;
    }

    unsafe impl NSStringNSExtendedStringDrawing for NSString {}
);

extern_category!(
    /// Category "NSExtendedStringDrawing" on [`NSAttributedString`].
    #[doc(alias = "NSExtendedStringDrawing")]
    pub unsafe trait NSAttributedStringNSExtendedStringDrawing {
        #[method(drawWithRect:options:context:)]
        unsafe fn drawWithRect_options_context(
            &self,
            rect: CGRect,
            options: NSStringDrawingOptions,
            context: Option<&NSStringDrawingContext>,
        );

        #[method(boundingRectWithSize:options:context:)]
        unsafe fn boundingRectWithSize_options_context(
            &self,
            size: CGSize,
            options: NSStringDrawingOptions,
            context: Option<&NSStringDrawingContext>,
        ) -> CGRect;
    }

    unsafe impl NSAttributedStringNSExtendedStringDrawing for NSAttributedString {}
);

extern_methods!(
    /// NSStringDrawingContextDeprecated
    unsafe impl NSStringDrawingContext {
        #[deprecated]
        #[method(minimumTrackingAdjustment)]
        pub unsafe fn minimumTrackingAdjustment(&self) -> CGFloat;

        #[deprecated]
        #[method(setMinimumTrackingAdjustment:)]
        pub unsafe fn setMinimumTrackingAdjustment(&self, minimum_tracking_adjustment: CGFloat);

        #[deprecated]
        #[method(actualTrackingAdjustment)]
        pub unsafe fn actualTrackingAdjustment(&self) -> CGFloat;
    }
);
