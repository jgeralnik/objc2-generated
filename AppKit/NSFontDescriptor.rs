//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

pub type NSFontSymbolicTraits = u32;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSFontDescriptorSymbolicTraits(pub u32);
bitflags::bitflags! {
    impl NSFontDescriptorSymbolicTraits: u32 {
        const NSFontDescriptorTraitItalic = 1<<0;
        const NSFontDescriptorTraitBold = 1<<1;
        const NSFontDescriptorTraitExpanded = 1<<5;
        const NSFontDescriptorTraitCondensed = 1<<6;
        const NSFontDescriptorTraitMonoSpace = 1<<10;
        const NSFontDescriptorTraitVertical = 1<<11;
        const NSFontDescriptorTraitUIOptimized = 1<<12;
        const NSFontDescriptorTraitTightLeading = 1<<15;
        const NSFontDescriptorTraitLooseLeading = 1<<16;
        const NSFontDescriptorTraitEmphasized = NSFontDescriptorSymbolicTraits::NSFontDescriptorTraitBold.0;
        const NSFontDescriptorClassMask = 0xF0000000;
        const NSFontDescriptorClassUnknown = 0<<28;
        const NSFontDescriptorClassOldStyleSerifs = 1<<28;
        const NSFontDescriptorClassTransitionalSerifs = 2<<28;
        const NSFontDescriptorClassModernSerifs = 3<<28;
        const NSFontDescriptorClassClarendonSerifs = 4<<28;
        const NSFontDescriptorClassSlabSerifs = 5<<28;
        const NSFontDescriptorClassFreeformSerifs = 7<<28;
        const NSFontDescriptorClassSansSerif = 8<<28;
        const NSFontDescriptorClassOrnamentals = 9<<28;
        const NSFontDescriptorClassScripts = 10<<28;
        const NSFontDescriptorClassSymbolic = 12<<28;
    }
}

unsafe impl Encode for NSFontDescriptorSymbolicTraits {
    const ENCODING: Encoding = u32::ENCODING;
}

unsafe impl RefEncode for NSFontDescriptorSymbolicTraits {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_TYPED_EXTENSIBLE_ENUM
pub type NSFontDescriptorAttributeName = NSString;

// NS_TYPED_ENUM
pub type NSFontDescriptorTraitKey = NSString;

// NS_TYPED_ENUM
pub type NSFontDescriptorVariationKey = NSString;

// NS_TYPED_EXTENSIBLE_ENUM
pub type NSFontDescriptorFeatureKey = NSString;

// NS_TYPED_EXTENSIBLE_ENUM
pub type NSFontWeight = CGFloat;

// NS_TYPED_EXTENSIBLE_ENUM
pub type NSFontWidth = CGFloat;

// NS_TYPED_ENUM
pub type NSFontDescriptorSystemDesign = NSString;

// NS_TYPED_ENUM
pub type NSFontTextStyle = NSString;

// NS_TYPED_ENUM
pub type NSFontTextStyleOptionKey = NSString;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSFontDescriptor;

    unsafe impl ClassType for NSFontDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for NSFontDescriptor {}

unsafe impl NSCopying for NSFontDescriptor {}

unsafe impl CopyingHelper for NSFontDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSFontDescriptor {}

unsafe impl NSSecureCoding for NSFontDescriptor {}

extern_methods!(
    unsafe impl NSFontDescriptor {
        #[method_id(@__retain_semantics Other postscriptName)]
        pub unsafe fn postscriptName(&self) -> Option<Retained<NSString>>;

        #[method(pointSize)]
        pub unsafe fn pointSize(&self) -> CGFloat;

        #[method_id(@__retain_semantics Other matrix)]
        pub unsafe fn matrix(&self) -> Option<Retained<NSAffineTransform>>;

        #[method(symbolicTraits)]
        pub unsafe fn symbolicTraits(&self) -> NSFontDescriptorSymbolicTraits;

        #[method(requiresFontAssetRequest)]
        pub unsafe fn requiresFontAssetRequest(&self) -> bool;

        #[method_id(@__retain_semantics Other objectForKey:)]
        pub unsafe fn objectForKey(
            &self,
            attribute: &NSFontDescriptorAttributeName,
        ) -> Option<Retained<AnyObject>>;

        #[method_id(@__retain_semantics Other fontAttributes)]
        pub unsafe fn fontAttributes(
            &self,
        ) -> Retained<NSDictionary<NSFontDescriptorAttributeName, AnyObject>>;

        #[method_id(@__retain_semantics Other fontDescriptorWithFontAttributes:)]
        pub unsafe fn fontDescriptorWithFontAttributes(
            attributes: Option<&NSDictionary<NSFontDescriptorAttributeName, AnyObject>>,
        ) -> Retained<NSFontDescriptor>;

        #[method_id(@__retain_semantics Other fontDescriptorWithName:size:)]
        pub unsafe fn fontDescriptorWithName_size(
            font_name: &NSString,
            size: CGFloat,
        ) -> Retained<NSFontDescriptor>;

        #[method_id(@__retain_semantics Other fontDescriptorWithName:matrix:)]
        pub unsafe fn fontDescriptorWithName_matrix(
            font_name: &NSString,
            matrix: &NSAffineTransform,
        ) -> Retained<NSFontDescriptor>;

        #[method_id(@__retain_semantics Init initWithFontAttributes:)]
        pub unsafe fn initWithFontAttributes(
            this: Allocated<Self>,
            attributes: Option<&NSDictionary<NSFontDescriptorAttributeName, AnyObject>>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other matchingFontDescriptorsWithMandatoryKeys:)]
        pub unsafe fn matchingFontDescriptorsWithMandatoryKeys(
            &self,
            mandatory_keys: Option<&NSSet<NSFontDescriptorAttributeName>>,
        ) -> Retained<NSArray<NSFontDescriptor>>;

        #[method_id(@__retain_semantics Other matchingFontDescriptorWithMandatoryKeys:)]
        pub unsafe fn matchingFontDescriptorWithMandatoryKeys(
            &self,
            mandatory_keys: Option<&NSSet<NSFontDescriptorAttributeName>>,
        ) -> Option<Retained<NSFontDescriptor>>;

        #[method_id(@__retain_semantics Other fontDescriptorByAddingAttributes:)]
        pub unsafe fn fontDescriptorByAddingAttributes(
            &self,
            attributes: &NSDictionary<NSFontDescriptorAttributeName, AnyObject>,
        ) -> Retained<NSFontDescriptor>;

        #[method_id(@__retain_semantics Other fontDescriptorWithSymbolicTraits:)]
        pub unsafe fn fontDescriptorWithSymbolicTraits(
            &self,
            symbolic_traits: NSFontDescriptorSymbolicTraits,
        ) -> Retained<NSFontDescriptor>;

        #[method_id(@__retain_semantics Other fontDescriptorWithSize:)]
        pub unsafe fn fontDescriptorWithSize(
            &self,
            new_point_size: CGFloat,
        ) -> Retained<NSFontDescriptor>;

        #[method_id(@__retain_semantics Other fontDescriptorWithMatrix:)]
        pub unsafe fn fontDescriptorWithMatrix(
            &self,
            matrix: &NSAffineTransform,
        ) -> Retained<NSFontDescriptor>;

        #[method_id(@__retain_semantics Other fontDescriptorWithFace:)]
        pub unsafe fn fontDescriptorWithFace(
            &self,
            new_face: &NSString,
        ) -> Retained<NSFontDescriptor>;

        #[method_id(@__retain_semantics Other fontDescriptorWithFamily:)]
        pub unsafe fn fontDescriptorWithFamily(
            &self,
            new_family: &NSString,
        ) -> Retained<NSFontDescriptor>;

        #[method_id(@__retain_semantics Other fontDescriptorWithDesign:)]
        pub unsafe fn fontDescriptorWithDesign(
            &self,
            design: &NSFontDescriptorSystemDesign,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSFontDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    pub static NSFontFamilyAttribute: &'static NSFontDescriptorAttributeName;
}

extern "C" {
    pub static NSFontNameAttribute: &'static NSFontDescriptorAttributeName;
}

extern "C" {
    pub static NSFontFaceAttribute: &'static NSFontDescriptorAttributeName;
}

extern "C" {
    pub static NSFontSizeAttribute: &'static NSFontDescriptorAttributeName;
}

extern "C" {
    pub static NSFontVisibleNameAttribute: &'static NSFontDescriptorAttributeName;
}

extern "C" {
    pub static NSFontMatrixAttribute: &'static NSFontDescriptorAttributeName;
}

extern "C" {
    pub static NSFontVariationAttribute: &'static NSFontDescriptorAttributeName;
}

extern "C" {
    pub static NSFontCharacterSetAttribute: &'static NSFontDescriptorAttributeName;
}

extern "C" {
    pub static NSFontCascadeListAttribute: &'static NSFontDescriptorAttributeName;
}

extern "C" {
    pub static NSFontTraitsAttribute: &'static NSFontDescriptorAttributeName;
}

extern "C" {
    pub static NSFontFixedAdvanceAttribute: &'static NSFontDescriptorAttributeName;
}

extern "C" {
    pub static NSFontFeatureSettingsAttribute: &'static NSFontDescriptorAttributeName;
}

extern "C" {
    pub static NSFontSymbolicTrait: &'static NSFontDescriptorTraitKey;
}

extern "C" {
    pub static NSFontWeightTrait: &'static NSFontDescriptorTraitKey;
}

extern "C" {
    pub static NSFontWidthTrait: &'static NSFontDescriptorTraitKey;
}

extern "C" {
    pub static NSFontSlantTrait: &'static NSFontDescriptorTraitKey;
}

extern "C" {
    pub static NSFontVariationAxisIdentifierKey: &'static NSFontDescriptorVariationKey;
}

extern "C" {
    pub static NSFontVariationAxisMinimumValueKey: &'static NSFontDescriptorVariationKey;
}

extern "C" {
    pub static NSFontVariationAxisMaximumValueKey: &'static NSFontDescriptorVariationKey;
}

extern "C" {
    pub static NSFontVariationAxisDefaultValueKey: &'static NSFontDescriptorVariationKey;
}

extern "C" {
    pub static NSFontVariationAxisNameKey: &'static NSFontDescriptorVariationKey;
}

extern "C" {
    pub static NSFontFeatureTypeIdentifierKey: &'static NSFontDescriptorFeatureKey;
}

extern "C" {
    pub static NSFontFeatureSelectorIdentifierKey: &'static NSFontDescriptorFeatureKey;
}

extern "C" {
    pub static NSFontWeightUltraLight: NSFontWeight;
}

extern "C" {
    pub static NSFontWeightThin: NSFontWeight;
}

extern "C" {
    pub static NSFontWeightLight: NSFontWeight;
}

extern "C" {
    pub static NSFontWeightRegular: NSFontWeight;
}

extern "C" {
    pub static NSFontWeightMedium: NSFontWeight;
}

extern "C" {
    pub static NSFontWeightSemibold: NSFontWeight;
}

extern "C" {
    pub static NSFontWeightBold: NSFontWeight;
}

extern "C" {
    pub static NSFontWeightHeavy: NSFontWeight;
}

extern "C" {
    pub static NSFontWeightBlack: NSFontWeight;
}

extern "C" {
    pub static NSFontWidthCompressed: NSFontWidth;
}

extern "C" {
    pub static NSFontWidthCondensed: NSFontWidth;
}

extern "C" {
    pub static NSFontWidthStandard: NSFontWidth;
}

extern "C" {
    pub static NSFontWidthExpanded: NSFontWidth;
}

extern "C" {
    pub static NSFontDescriptorSystemDesignDefault: &'static NSFontDescriptorSystemDesign;
}

extern "C" {
    pub static NSFontDescriptorSystemDesignSerif: &'static NSFontDescriptorSystemDesign;
}

extern "C" {
    pub static NSFontDescriptorSystemDesignMonospaced: &'static NSFontDescriptorSystemDesign;
}

extern "C" {
    pub static NSFontDescriptorSystemDesignRounded: &'static NSFontDescriptorSystemDesign;
}

extern "C" {
    pub static NSFontTextStyleLargeTitle: &'static NSFontTextStyle;
}

extern "C" {
    pub static NSFontTextStyleTitle1: &'static NSFontTextStyle;
}

extern "C" {
    pub static NSFontTextStyleTitle2: &'static NSFontTextStyle;
}

extern "C" {
    pub static NSFontTextStyleTitle3: &'static NSFontTextStyle;
}

extern "C" {
    pub static NSFontTextStyleHeadline: &'static NSFontTextStyle;
}

extern "C" {
    pub static NSFontTextStyleSubheadline: &'static NSFontTextStyle;
}

extern "C" {
    pub static NSFontTextStyleBody: &'static NSFontTextStyle;
}

extern "C" {
    pub static NSFontTextStyleCallout: &'static NSFontTextStyle;
}

extern "C" {
    pub static NSFontTextStyleFootnote: &'static NSFontTextStyle;
}

extern "C" {
    pub static NSFontTextStyleCaption1: &'static NSFontTextStyle;
}

extern "C" {
    pub static NSFontTextStyleCaption2: &'static NSFontTextStyle;
}

pub type NSFontFamilyClass = u32;

pub const NSFontUnknownClass: c_int = 0 << 28;
pub const NSFontOldStyleSerifsClass: c_int = 1 << 28;
pub const NSFontTransitionalSerifsClass: c_int = 2 << 28;
pub const NSFontModernSerifsClass: c_int = 3 << 28;
pub const NSFontClarendonSerifsClass: c_int = 4 << 28;
pub const NSFontSlabSerifsClass: c_int = 5 << 28;
pub const NSFontFreeformSerifsClass: c_int = 7 << 28;
pub const NSFontSansSerifClass: c_int = 8 << 28;
pub const NSFontOrnamentalsClass: c_int = 9 << 28;
pub const NSFontScriptsClass: c_int = 10 << 28;
pub const NSFontSymbolicClass: c_int = 12 << 28;

pub const NSFontFamilyClassMask: c_uint = 0xF0000000;

pub const NSFontItalicTrait: c_uint = 1 << 0;
pub const NSFontBoldTrait: c_uint = 1 << 1;
pub const NSFontExpandedTrait: c_uint = 1 << 5;
pub const NSFontCondensedTrait: c_uint = 1 << 6;
pub const NSFontMonoSpaceTrait: c_uint = 1 << 10;
pub const NSFontVerticalTrait: c_uint = 1 << 11;
pub const NSFontUIOptimizedTrait: c_uint = 1 << 12;

extern "C" {
    pub static NSFontColorAttribute: &'static NSString;
}

extern_methods!(
    /// NSFontDescriptor_TextStyles
    unsafe impl NSFontDescriptor {
        #[method_id(@__retain_semantics Other preferredFontDescriptorForTextStyle:options:)]
        pub unsafe fn preferredFontDescriptorForTextStyle_options(
            style: &NSFontTextStyle,
            options: &NSDictionary<NSFontTextStyleOptionKey, AnyObject>,
        ) -> Retained<NSFontDescriptor>;
    }
);
