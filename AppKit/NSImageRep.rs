//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_TYPED_ENUM
pub type NSImageHintKey = NSString;

pub const NSImageRepMatchesDevice: c_uint = 0;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSImageLayoutDirection(pub NSInteger);
impl NSImageLayoutDirection {
    #[doc(alias = "NSImageLayoutDirectionUnspecified")]
    pub const Unspecified: Self = Self(-1);
    #[doc(alias = "NSImageLayoutDirectionLeftToRight")]
    pub const LeftToRight: Self = Self(2);
    #[doc(alias = "NSImageLayoutDirectionRightToLeft")]
    pub const RightToLeft: Self = Self(3);
}

unsafe impl Encode for NSImageLayoutDirection {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSImageLayoutDirection {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSImageRep;

    unsafe impl ClassType for NSImageRep {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for NSImageRep {}

unsafe impl NSCopying for NSImageRep {}

unsafe impl CopyingHelper for NSImageRep {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSImageRep {}

extern_methods!(
    unsafe impl NSImageRep {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method(draw)]
        pub unsafe fn draw(&self) -> bool;

        #[method(drawAtPoint:)]
        pub unsafe fn drawAtPoint(&self, point: NSPoint) -> bool;

        #[method(drawInRect:)]
        pub unsafe fn drawInRect(&self, rect: NSRect) -> bool;

        #[cfg(feature = "NSGraphics")]
        #[method(drawInRect:fromRect:operation:fraction:respectFlipped:hints:)]
        pub unsafe fn drawInRect_fromRect_operation_fraction_respectFlipped_hints(
            &self,
            dst_space_portion_rect: NSRect,
            src_space_portion_rect: NSRect,
            op: NSCompositingOperation,
            requested_alpha: CGFloat,
            respect_context_is_flipped: bool,
            hints: Option<&NSDictionary<NSImageHintKey, AnyObject>>,
        ) -> bool;

        #[method(size)]
        pub unsafe fn size(&self) -> NSSize;

        #[method(setSize:)]
        pub unsafe fn setSize(&self, size: NSSize);

        #[method(hasAlpha)]
        pub unsafe fn hasAlpha(&self) -> bool;

        #[method(setAlpha:)]
        pub unsafe fn setAlpha(&self, alpha: bool);

        #[method(isOpaque)]
        pub unsafe fn isOpaque(&self) -> bool;

        #[method(setOpaque:)]
        pub unsafe fn setOpaque(&self, opaque: bool);

        #[cfg(feature = "NSGraphics")]
        #[method_id(@__retain_semantics Other colorSpaceName)]
        pub unsafe fn colorSpaceName(&self) -> Retained<NSColorSpaceName>;

        #[cfg(feature = "NSGraphics")]
        #[method(setColorSpaceName:)]
        pub unsafe fn setColorSpaceName(&self, color_space_name: &NSColorSpaceName);

        #[method(bitsPerSample)]
        pub unsafe fn bitsPerSample(&self) -> NSInteger;

        #[method(setBitsPerSample:)]
        pub unsafe fn setBitsPerSample(&self, bits_per_sample: NSInteger);

        #[method(pixelsWide)]
        pub unsafe fn pixelsWide(&self) -> NSInteger;

        #[method(setPixelsWide:)]
        pub unsafe fn setPixelsWide(&self, pixels_wide: NSInteger);

        #[method(pixelsHigh)]
        pub unsafe fn pixelsHigh(&self) -> NSInteger;

        #[method(setPixelsHigh:)]
        pub unsafe fn setPixelsHigh(&self, pixels_high: NSInteger);

        #[method(layoutDirection)]
        pub unsafe fn layoutDirection(&self) -> NSImageLayoutDirection;

        #[method(setLayoutDirection:)]
        pub unsafe fn setLayoutDirection(&self, layout_direction: NSImageLayoutDirection);

        #[method(registerImageRepClass:)]
        pub unsafe fn registerImageRepClass(image_rep_class: &AnyClass);

        #[method(unregisterImageRepClass:)]
        pub unsafe fn unregisterImageRepClass(image_rep_class: &AnyClass);

        #[method_id(@__retain_semantics Other registeredImageRepClasses)]
        pub unsafe fn registeredImageRepClasses() -> Retained<NSArray<TodoClass>>;

        #[deprecated = "Use +imageRepClassForType: instead"]
        #[method(imageRepClassForFileType:)]
        pub unsafe fn imageRepClassForFileType(r#type: &NSString) -> Option<&'static AnyClass>;

        #[cfg(feature = "NSPasteboard")]
        #[deprecated = "Use +imageRepClassForType: instead"]
        #[method(imageRepClassForPasteboardType:)]
        pub unsafe fn imageRepClassForPasteboardType(
            r#type: &NSPasteboardType,
        ) -> Option<&'static AnyClass>;

        #[method(imageRepClassForType:)]
        pub unsafe fn imageRepClassForType(r#type: &NSString) -> Option<&'static AnyClass>;

        #[method(imageRepClassForData:)]
        pub unsafe fn imageRepClassForData(data: &NSData) -> Option<&'static AnyClass>;

        #[method(canInitWithData:)]
        pub unsafe fn canInitWithData(data: &NSData) -> bool;

        #[deprecated = "Use +imageUnfilteredTypes instead"]
        #[method_id(@__retain_semantics Other imageUnfilteredFileTypes)]
        pub unsafe fn imageUnfilteredFileTypes() -> Retained<NSArray<NSString>>;

        #[cfg(feature = "NSPasteboard")]
        #[deprecated = "Use +imageUnfilteredTypes instead"]
        #[method_id(@__retain_semantics Other imageUnfilteredPasteboardTypes)]
        pub unsafe fn imageUnfilteredPasteboardTypes() -> Retained<NSArray<NSPasteboardType>>;

        #[deprecated = "Use +imageTypes instead"]
        #[method_id(@__retain_semantics Other imageFileTypes)]
        pub unsafe fn imageFileTypes() -> Retained<NSArray<NSString>>;

        #[cfg(feature = "NSPasteboard")]
        #[deprecated = "Use +imageTypes instead"]
        #[method_id(@__retain_semantics Other imagePasteboardTypes)]
        pub unsafe fn imagePasteboardTypes() -> Retained<NSArray<NSPasteboardType>>;

        #[method_id(@__retain_semantics Other imageUnfilteredTypes)]
        pub unsafe fn imageUnfilteredTypes() -> Retained<NSArray<NSString>>;

        #[method_id(@__retain_semantics Other imageTypes)]
        pub unsafe fn imageTypes() -> Retained<NSArray<NSString>>;

        #[cfg(feature = "NSPasteboard")]
        #[method(canInitWithPasteboard:)]
        pub unsafe fn canInitWithPasteboard(pasteboard: &NSPasteboard) -> bool;

        #[method_id(@__retain_semantics Other imageRepsWithContentsOfFile:)]
        pub unsafe fn imageRepsWithContentsOfFile(
            filename: &NSString,
        ) -> Option<Retained<NSArray<NSImageRep>>>;

        #[method_id(@__retain_semantics Other imageRepWithContentsOfFile:)]
        pub unsafe fn imageRepWithContentsOfFile(
            filename: &NSString,
        ) -> Option<Retained<NSImageRep>>;

        #[method_id(@__retain_semantics Other imageRepsWithContentsOfURL:)]
        pub unsafe fn imageRepsWithContentsOfURL(
            url: &NSURL,
        ) -> Option<Retained<NSArray<NSImageRep>>>;

        #[method_id(@__retain_semantics Other imageRepWithContentsOfURL:)]
        pub unsafe fn imageRepWithContentsOfURL(url: &NSURL) -> Option<Retained<NSImageRep>>;

        #[cfg(feature = "NSPasteboard")]
        #[method_id(@__retain_semantics Other imageRepsWithPasteboard:)]
        pub unsafe fn imageRepsWithPasteboard(
            pasteboard: &NSPasteboard,
        ) -> Option<Retained<NSArray<NSImageRep>>>;

        #[cfg(feature = "NSPasteboard")]
        #[method_id(@__retain_semantics Other imageRepWithPasteboard:)]
        pub unsafe fn imageRepWithPasteboard(
            pasteboard: &NSPasteboard,
        ) -> Option<Retained<NSImageRep>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSImageRep {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    pub static NSImageRepRegistryDidChangeNotification: &'static NSNotificationName;
}
