//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

#[cfg(feature = "Foundation_NSGeometry")]
extern_static!(NSFontIdentityMatrix: NonNull<CGFloat>);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSFont;

    unsafe impl ClassType for NSFont {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for NSFont {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for NSFont {}

unsafe impl NSObjectProtocol for NSFont {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for NSFont {}

extern_methods!(
    unsafe impl NSFont {
        #[cfg(all(feature = "Foundation_NSGeometry", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other fontWithName:size:)]
        pub unsafe fn fontWithName_size(
            font_name: &NSString,
            font_size: CGFloat,
        ) -> Option<Id<NSFont>>;

        #[cfg(all(feature = "Foundation_NSGeometry", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other fontWithName:matrix:)]
        pub unsafe fn fontWithName_matrix(
            font_name: &NSString,
            font_matrix: NonNull<CGFloat>,
        ) -> Option<Id<NSFont>>;

        #[cfg(all(feature = "AppKit_NSFontDescriptor", feature = "Foundation_NSGeometry"))]
        #[method_id(@__retain_semantics Other fontWithDescriptor:size:)]
        pub unsafe fn fontWithDescriptor_size(
            font_descriptor: &NSFontDescriptor,
            font_size: CGFloat,
        ) -> Option<Id<NSFont>>;

        #[cfg(all(
            feature = "AppKit_NSFontDescriptor",
            feature = "Foundation_NSAffineTransform"
        ))]
        #[method_id(@__retain_semantics Other fontWithDescriptor:textTransform:)]
        pub unsafe fn fontWithDescriptor_textTransform(
            font_descriptor: &NSFontDescriptor,
            text_transform: Option<&NSAffineTransform>,
        ) -> Option<Id<NSFont>>;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method_id(@__retain_semantics Other userFontOfSize:)]
        pub unsafe fn userFontOfSize(font_size: CGFloat) -> Option<Id<NSFont>>;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method_id(@__retain_semantics Other userFixedPitchFontOfSize:)]
        pub unsafe fn userFixedPitchFontOfSize(font_size: CGFloat) -> Option<Id<NSFont>>;

        #[method(setUserFont:)]
        pub unsafe fn setUserFont(font: Option<&NSFont>);

        #[method(setUserFixedPitchFont:)]
        pub unsafe fn setUserFixedPitchFont(font: Option<&NSFont>);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method_id(@__retain_semantics Other systemFontOfSize:)]
        pub unsafe fn systemFontOfSize(font_size: CGFloat) -> Id<NSFont>;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method_id(@__retain_semantics Other boldSystemFontOfSize:)]
        pub unsafe fn boldSystemFontOfSize(font_size: CGFloat) -> Id<NSFont>;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method_id(@__retain_semantics Other labelFontOfSize:)]
        pub unsafe fn labelFontOfSize(font_size: CGFloat) -> Id<NSFont>;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method_id(@__retain_semantics Other titleBarFontOfSize:)]
        pub unsafe fn titleBarFontOfSize(font_size: CGFloat) -> Id<NSFont>;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method_id(@__retain_semantics Other menuFontOfSize:)]
        pub unsafe fn menuFontOfSize(font_size: CGFloat) -> Id<NSFont>;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method_id(@__retain_semantics Other menuBarFontOfSize:)]
        pub unsafe fn menuBarFontOfSize(font_size: CGFloat) -> Id<NSFont>;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method_id(@__retain_semantics Other messageFontOfSize:)]
        pub unsafe fn messageFontOfSize(font_size: CGFloat) -> Id<NSFont>;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method_id(@__retain_semantics Other paletteFontOfSize:)]
        pub unsafe fn paletteFontOfSize(font_size: CGFloat) -> Id<NSFont>;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method_id(@__retain_semantics Other toolTipsFontOfSize:)]
        pub unsafe fn toolTipsFontOfSize(font_size: CGFloat) -> Id<NSFont>;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method_id(@__retain_semantics Other controlContentFontOfSize:)]
        pub unsafe fn controlContentFontOfSize(font_size: CGFloat) -> Id<NSFont>;

        #[cfg(all(feature = "AppKit_NSFontDescriptor", feature = "Foundation_NSGeometry"))]
        #[method_id(@__retain_semantics Other systemFontOfSize:weight:)]
        pub unsafe fn systemFontOfSize_weight(
            font_size: CGFloat,
            weight: NSFontWeight,
        ) -> Id<NSFont>;

        #[cfg(all(feature = "AppKit_NSFontDescriptor", feature = "Foundation_NSGeometry"))]
        #[method_id(@__retain_semantics Other monospacedDigitSystemFontOfSize:weight:)]
        pub unsafe fn monospacedDigitSystemFontOfSize_weight(
            font_size: CGFloat,
            weight: NSFontWeight,
        ) -> Id<NSFont>;

        #[cfg(all(feature = "AppKit_NSFontDescriptor", feature = "Foundation_NSGeometry"))]
        #[method_id(@__retain_semantics Other systemFontOfSize:weight:width:)]
        pub unsafe fn systemFontOfSize_weight_width(
            font_size: CGFloat,
            weight: NSFontWeight,
            width: NSFontWidth,
        ) -> Id<NSFont>;

        #[cfg(all(feature = "AppKit_NSFontDescriptor", feature = "Foundation_NSGeometry"))]
        #[method_id(@__retain_semantics Other monospacedSystemFontOfSize:weight:)]
        pub unsafe fn monospacedSystemFontOfSize_weight(
            font_size: CGFloat,
            weight: NSFontWeight,
        ) -> Id<NSFont>;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method_id(@__retain_semantics Other fontWithSize:)]
        pub unsafe fn fontWithSize(&self, font_size: CGFloat) -> Id<NSFont>;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(systemFontSize)]
        pub unsafe fn systemFontSize() -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(smallSystemFontSize)]
        pub unsafe fn smallSystemFontSize() -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(labelFontSize)]
        pub unsafe fn labelFontSize() -> CGFloat;

        #[cfg(all(feature = "AppKit_NSCell", feature = "Foundation_NSGeometry"))]
        #[method(systemFontSizeForControlSize:)]
        pub unsafe fn systemFontSizeForControlSize(control_size: NSControlSize) -> CGFloat;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other fontName)]
        pub unsafe fn fontName(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(pointSize)]
        pub unsafe fn pointSize(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(matrix)]
        pub unsafe fn matrix(&self) -> NonNull<CGFloat>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other familyName)]
        pub unsafe fn familyName(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other displayName)]
        pub unsafe fn displayName(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "AppKit_NSFontDescriptor")]
        #[method_id(@__retain_semantics Other fontDescriptor)]
        pub unsafe fn fontDescriptor(&self) -> Id<NSFontDescriptor>;

        #[cfg(feature = "Foundation_NSAffineTransform")]
        #[method_id(@__retain_semantics Other textTransform)]
        pub unsafe fn textTransform(&self) -> Id<NSAffineTransform>;

        #[method(numberOfGlyphs)]
        pub unsafe fn numberOfGlyphs(&self) -> NSUInteger;

        #[cfg(feature = "Foundation_NSString")]
        #[method(mostCompatibleStringEncoding)]
        pub unsafe fn mostCompatibleStringEncoding(&self) -> NSStringEncoding;

        #[cfg(feature = "Foundation_NSCharacterSet")]
        #[method_id(@__retain_semantics Other coveredCharacterSet)]
        pub unsafe fn coveredCharacterSet(&self) -> Id<NSCharacterSet>;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(boundingRectForFont)]
        pub unsafe fn boundingRectForFont(&self) -> NSRect;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(maximumAdvancement)]
        pub unsafe fn maximumAdvancement(&self) -> NSSize;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(ascender)]
        pub unsafe fn ascender(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(descender)]
        pub unsafe fn descender(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(leading)]
        pub unsafe fn leading(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(underlinePosition)]
        pub unsafe fn underlinePosition(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(underlineThickness)]
        pub unsafe fn underlineThickness(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(italicAngle)]
        pub unsafe fn italicAngle(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(capHeight)]
        pub unsafe fn capHeight(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(xHeight)]
        pub unsafe fn xHeight(&self) -> CGFloat;

        #[method(isFixedPitch)]
        pub unsafe fn isFixedPitch(&self) -> bool;

        #[method(set)]
        pub unsafe fn set(&self);

        #[cfg(feature = "AppKit_NSGraphicsContext")]
        #[method(setInContext:)]
        pub unsafe fn setInContext(&self, graphics_context: &NSGraphicsContext);

        #[method_id(@__retain_semantics Other verticalFont)]
        pub unsafe fn verticalFont(&self) -> Id<NSFont>;

        #[method(isVertical)]
        pub unsafe fn isVertical(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSFont {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

#[cfg(all(feature = "Foundation_NSNotification", feature = "Foundation_NSString"))]
extern_static!(NSAntialiasThresholdChangedNotification: &'static NSNotificationName);

#[cfg(all(feature = "Foundation_NSNotification", feature = "Foundation_NSString"))]
extern_static!(NSFontSetChangedNotification: &'static NSNotificationName);

pub type NSGlyph = c_uint;

pub const NSControlGlyph: c_uint = 0x00FFFFFF;
pub const NSNullGlyph: c_uint = 0x0;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSFontRenderingMode {
        NSFontDefaultRenderingMode = 0,
        NSFontAntialiasedRenderingMode = 1,
        NSFontIntegerAdvancementsRenderingMode = 2,
        NSFontAntialiasedIntegerAdvancementsRenderingMode = 3,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    #[deprecated]
    pub enum NSMultibyteGlyphPacking {
        #[deprecated]
        NSNativeShortGlyphPacking = 5,
    }
);

extern "C" {
    #[deprecated]
    pub fn NSConvertGlyphsToPackedGlyphs(
        gl_buf: NonNull<NSGlyph>,
        count: NSInteger,
        packing: NSMultibyteGlyphPacking,
        packed_glyphs: NonNull<c_char>,
    ) -> NSInteger;
}

extern_methods!(
    /// NSFont_Deprecated
    unsafe impl NSFont {
        #[cfg(feature = "Foundation_NSString")]
        #[method(glyphWithName:)]
        pub unsafe fn glyphWithName(&self, name: &NSString) -> NSGlyph;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(boundingRectForGlyph:)]
        pub unsafe fn boundingRectForGlyph(&self, glyph: NSGlyph) -> NSRect;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(advancementForGlyph:)]
        pub unsafe fn advancementForGlyph(&self, glyph: NSGlyph) -> NSSize;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(getBoundingRects:forGlyphs:count:)]
        pub unsafe fn getBoundingRects_forGlyphs_count(
            &self,
            bounds: NSRectArray,
            glyphs: NonNull<NSGlyph>,
            glyph_count: NSUInteger,
        );

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(getAdvancements:forGlyphs:count:)]
        pub unsafe fn getAdvancements_forGlyphs_count(
            &self,
            advancements: NSSizeArray,
            glyphs: NonNull<NSGlyph>,
            glyph_count: NSUInteger,
        );

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(getAdvancements:forPackedGlyphs:length:)]
        pub unsafe fn getAdvancements_forPackedGlyphs_length(
            &self,
            advancements: NSSizeArray,
            packed_glyphs: NonNull<c_void>,
            length: NSUInteger,
        );

        #[method_id(@__retain_semantics Other printerFont)]
        pub unsafe fn printerFont(&self) -> Id<NSFont>;

        #[method_id(@__retain_semantics Other screenFont)]
        pub unsafe fn screenFont(&self) -> Id<NSFont>;

        #[method_id(@__retain_semantics Other screenFontWithRenderingMode:)]
        pub unsafe fn screenFontWithRenderingMode(
            &self,
            rendering_mode: NSFontRenderingMode,
        ) -> Id<NSFont>;

        #[method(renderingMode)]
        pub unsafe fn renderingMode(&self) -> NSFontRenderingMode;
    }
);

extern_methods!(
    /// NSFont_TextStyles
    unsafe impl NSFont {
        #[cfg(all(
            feature = "AppKit_NSFontDescriptor",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other preferredFontForTextStyle:options:)]
        pub unsafe fn preferredFontForTextStyle_options(
            style: &NSFontTextStyle,
            options: &NSDictionary<NSFontTextStyleOptionKey, AnyObject>,
        ) -> Id<NSFont>;
    }
);
