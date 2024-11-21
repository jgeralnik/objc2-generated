//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSGradientDrawingOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSGradientDrawingOptions: NSUInteger {
        const NSGradientDrawsBeforeStartingLocation = 1<<0;
        const NSGradientDrawsAfterEndingLocation = 1<<1;
    }
}

unsafe impl Encode for NSGradientDrawingOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSGradientDrawingOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSGradient;
);

unsafe impl Send for NSGradient {}

unsafe impl Sync for NSGradient {}

unsafe impl NSCoding for NSGradient {}

unsafe impl NSCopying for NSGradient {}

unsafe impl CopyingHelper for NSGradient {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSGradient {}

unsafe impl NSSecureCoding for NSGradient {}

extern_methods!(
    unsafe impl NSGradient {
        #[cfg(feature = "NSColor")]
        #[method_id(@__retain_semantics Init initWithStartingColor:endingColor:)]
        pub unsafe fn initWithStartingColor_endingColor(
            this: Allocated<Self>,
            starting_color: &NSColor,
            ending_color: &NSColor,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "NSColor")]
        #[method_id(@__retain_semantics Init initWithColors:)]
        pub unsafe fn initWithColors(
            this: Allocated<Self>,
            color_array: &NSArray<NSColor>,
        ) -> Option<Retained<Self>>;

        #[cfg(all(feature = "NSColor", feature = "NSColorSpace"))]
        #[method_id(@__retain_semantics Init initWithColors:atLocations:colorSpace:)]
        pub unsafe fn initWithColors_atLocations_colorSpace(
            this: Allocated<Self>,
            color_array: &NSArray<NSColor>,
            locations: *mut CGFloat,
            color_space: &NSColorSpace,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method(drawFromPoint:toPoint:options:)]
        pub unsafe fn drawFromPoint_toPoint_options(
            &self,
            starting_point: NSPoint,
            ending_point: NSPoint,
            options: NSGradientDrawingOptions,
        );

        #[method(drawInRect:angle:)]
        pub unsafe fn drawInRect_angle(&self, rect: NSRect, angle: CGFloat);

        #[cfg(feature = "NSBezierPath")]
        #[method(drawInBezierPath:angle:)]
        pub unsafe fn drawInBezierPath_angle(&self, path: &NSBezierPath, angle: CGFloat);

        #[method(drawFromCenter:radius:toCenter:radius:options:)]
        pub unsafe fn drawFromCenter_radius_toCenter_radius_options(
            &self,
            start_center: NSPoint,
            start_radius: CGFloat,
            end_center: NSPoint,
            end_radius: CGFloat,
            options: NSGradientDrawingOptions,
        );

        #[method(drawInRect:relativeCenterPosition:)]
        pub unsafe fn drawInRect_relativeCenterPosition(
            &self,
            rect: NSRect,
            relative_center_position: NSPoint,
        );

        #[cfg(feature = "NSBezierPath")]
        #[method(drawInBezierPath:relativeCenterPosition:)]
        pub unsafe fn drawInBezierPath_relativeCenterPosition(
            &self,
            path: &NSBezierPath,
            relative_center_position: NSPoint,
        );

        #[cfg(feature = "NSColorSpace")]
        #[method_id(@__retain_semantics Other colorSpace)]
        pub unsafe fn colorSpace(&self) -> Retained<NSColorSpace>;

        #[method(numberOfColorStops)]
        pub unsafe fn numberOfColorStops(&self) -> NSInteger;

        #[cfg(feature = "NSColor")]
        #[method(getColor:location:atIndex:)]
        pub unsafe fn getColor_location_atIndex(
            &self,
            color: Option<&mut Retained<NSColor>>,
            location: *mut CGFloat,
            index: NSInteger,
        );

        #[cfg(feature = "NSColor")]
        #[method_id(@__retain_semantics Other interpolatedColorAtLocation:)]
        pub unsafe fn interpolatedColorAtLocation(&self, location: CGFloat) -> Retained<NSColor>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSGradient {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
