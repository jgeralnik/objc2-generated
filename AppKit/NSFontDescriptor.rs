//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

pub type NSFontSymbolicTraits = u32;

ns_options!(
    #[underlying(u32)]
    pub enum NSFontDescriptorSymbolicTraits {
        NSFontDescriptorTraitItalic = 1 << 0,
        NSFontDescriptorTraitBold = 1 << 1,
        NSFontDescriptorTraitExpanded = 1 << 5,
        NSFontDescriptorTraitCondensed = 1 << 6,
        NSFontDescriptorTraitMonoSpace = 1 << 10,
        NSFontDescriptorTraitVertical = 1 << 11,
        NSFontDescriptorTraitUIOptimized = 1 << 12,
        NSFontDescriptorTraitTightLeading = 1 << 15,
        NSFontDescriptorTraitLooseLeading = 1 << 16,
        NSFontDescriptorTraitEmphasized = NSFontDescriptorTraitBold,
        NSFontDescriptorClassMask = 0xF0000000,
        NSFontDescriptorClassUnknown = 0 << 28,
        NSFontDescriptorClassOldStyleSerifs = 1 << 28,
        NSFontDescriptorClassTransitionalSerifs = 2 << 28,
        NSFontDescriptorClassModernSerifs = 3 << 28,
        NSFontDescriptorClassClarendonSerifs = 4 << 28,
        NSFontDescriptorClassSlabSerifs = 5 << 28,
        NSFontDescriptorClassFreeformSerifs = 7 << 28,
        NSFontDescriptorClassSansSerif = 8 << 28,
        NSFontDescriptorClassOrnamentals = 9 << 28,
        NSFontDescriptorClassScripts = 10 << 28,
        NSFontDescriptorClassSymbolic = 12 << 28,
    }
);

typed_extensible_enum!(
    pub type NSFontDescriptorAttributeName = NSString;
);

typed_enum!(
    pub type NSFontDescriptorTraitKey = NSString;
);

typed_enum!(
    pub type NSFontDescriptorVariationKey = NSString;
);

typed_extensible_enum!(
    pub type NSFontDescriptorFeatureKey = NSString;
);

typed_extensible_enum!(
    pub type NSFontWeight = CGFloat;
);

typed_enum!(
    pub type NSFontDescriptorSystemDesign = NSString;
);

typed_enum!(
    pub type NSFontTextStyle = NSString;
);

typed_enum!(
    pub type NSFontTextStyleOptionKey = NSString;
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSFontDescriptor;

    unsafe impl ClassType for NSFontDescriptor {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSFontDescriptor")]
    unsafe impl NSFontDescriptor {
        #[method_id(@__retain_semantics Other postscriptName)]
        pub unsafe fn postscriptName(&self) -> Option<Id<NSString, Shared>>;

        #[method(pointSize)]
        pub unsafe fn pointSize(&self) -> CGFloat;

        #[method_id(@__retain_semantics Other matrix)]
        pub unsafe fn matrix(&self) -> Option<Id<NSAffineTransform, Shared>>;

        #[method(symbolicTraits)]
        pub unsafe fn symbolicTraits(&self) -> NSFontDescriptorSymbolicTraits;

        #[method(requiresFontAssetRequest)]
        pub unsafe fn requiresFontAssetRequest(&self) -> bool;

        #[cfg(feature = "AppKit_NSFontDescriptorAttributeName")]
        #[method_id(@__retain_semantics Other objectForKey:)]
        pub unsafe fn objectForKey(
            &self,
            attribute: &NSFontDescriptorAttributeName,
        ) -> Option<Id<Object, Shared>>;

        #[cfg(feature = "AppKit_NSFontDescriptorAttributeName")]
        #[method_id(@__retain_semantics Other fontAttributes)]
        pub unsafe fn fontAttributes(
            &self,
        ) -> Id<NSDictionary<NSFontDescriptorAttributeName, Object>, Shared>;

        #[cfg(feature = "AppKit_NSFontDescriptorAttributeName")]
        #[method_id(@__retain_semantics Other fontDescriptorWithFontAttributes:)]
        pub unsafe fn fontDescriptorWithFontAttributes(
            attributes: Option<&NSDictionary<NSFontDescriptorAttributeName, Object>>,
        ) -> Id<NSFontDescriptor, Shared>;

        #[method_id(@__retain_semantics Other fontDescriptorWithName:size:)]
        pub unsafe fn fontDescriptorWithName_size(
            fontName: &NSString,
            size: CGFloat,
        ) -> Id<NSFontDescriptor, Shared>;

        #[method_id(@__retain_semantics Other fontDescriptorWithName:matrix:)]
        pub unsafe fn fontDescriptorWithName_matrix(
            fontName: &NSString,
            matrix: &NSAffineTransform,
        ) -> Id<NSFontDescriptor, Shared>;

        #[cfg(feature = "AppKit_NSFontDescriptorAttributeName")]
        #[method_id(@__retain_semantics Init initWithFontAttributes:)]
        pub unsafe fn initWithFontAttributes(
            this: Option<Allocated<Self>>,
            attributes: Option<&NSDictionary<NSFontDescriptorAttributeName, Object>>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSFontDescriptorAttributeName")]
        #[method_id(@__retain_semantics Other matchingFontDescriptorsWithMandatoryKeys:)]
        pub unsafe fn matchingFontDescriptorsWithMandatoryKeys(
            &self,
            mandatoryKeys: Option<&NSSet<NSFontDescriptorAttributeName>>,
        ) -> Id<NSArray<NSFontDescriptor>, Shared>;

        #[cfg(feature = "AppKit_NSFontDescriptorAttributeName")]
        #[method_id(@__retain_semantics Other matchingFontDescriptorWithMandatoryKeys:)]
        pub unsafe fn matchingFontDescriptorWithMandatoryKeys(
            &self,
            mandatoryKeys: Option<&NSSet<NSFontDescriptorAttributeName>>,
        ) -> Option<Id<NSFontDescriptor, Shared>>;

        #[cfg(feature = "AppKit_NSFontDescriptorAttributeName")]
        #[method_id(@__retain_semantics Other fontDescriptorByAddingAttributes:)]
        pub unsafe fn fontDescriptorByAddingAttributes(
            &self,
            attributes: &NSDictionary<NSFontDescriptorAttributeName, Object>,
        ) -> Id<NSFontDescriptor, Shared>;

        #[method_id(@__retain_semantics Other fontDescriptorWithSymbolicTraits:)]
        pub unsafe fn fontDescriptorWithSymbolicTraits(
            &self,
            symbolicTraits: NSFontDescriptorSymbolicTraits,
        ) -> Id<NSFontDescriptor, Shared>;

        #[method_id(@__retain_semantics Other fontDescriptorWithSize:)]
        pub unsafe fn fontDescriptorWithSize(
            &self,
            newPointSize: CGFloat,
        ) -> Id<NSFontDescriptor, Shared>;

        #[method_id(@__retain_semantics Other fontDescriptorWithMatrix:)]
        pub unsafe fn fontDescriptorWithMatrix(
            &self,
            matrix: &NSAffineTransform,
        ) -> Id<NSFontDescriptor, Shared>;

        #[method_id(@__retain_semantics Other fontDescriptorWithFace:)]
        pub unsafe fn fontDescriptorWithFace(
            &self,
            newFace: &NSString,
        ) -> Id<NSFontDescriptor, Shared>;

        #[method_id(@__retain_semantics Other fontDescriptorWithFamily:)]
        pub unsafe fn fontDescriptorWithFamily(
            &self,
            newFamily: &NSString,
        ) -> Id<NSFontDescriptor, Shared>;

        #[cfg(feature = "AppKit_NSFontDescriptorSystemDesign")]
        #[method_id(@__retain_semantics Other fontDescriptorWithDesign:)]
        pub unsafe fn fontDescriptorWithDesign(
            &self,
            design: &NSFontDescriptorSystemDesign,
        ) -> Option<Id<Self, Shared>>;
    }
);

extern_static!(NSFontFamilyAttribute: &'static NSFontDescriptorAttributeName);

extern_static!(NSFontNameAttribute: &'static NSFontDescriptorAttributeName);

extern_static!(NSFontFaceAttribute: &'static NSFontDescriptorAttributeName);

extern_static!(NSFontSizeAttribute: &'static NSFontDescriptorAttributeName);

extern_static!(NSFontVisibleNameAttribute: &'static NSFontDescriptorAttributeName);

extern_static!(NSFontMatrixAttribute: &'static NSFontDescriptorAttributeName);

extern_static!(NSFontVariationAttribute: &'static NSFontDescriptorAttributeName);

extern_static!(NSFontCharacterSetAttribute: &'static NSFontDescriptorAttributeName);

extern_static!(NSFontCascadeListAttribute: &'static NSFontDescriptorAttributeName);

extern_static!(NSFontTraitsAttribute: &'static NSFontDescriptorAttributeName);

extern_static!(NSFontFixedAdvanceAttribute: &'static NSFontDescriptorAttributeName);

extern_static!(NSFontFeatureSettingsAttribute: &'static NSFontDescriptorAttributeName);

extern_static!(NSFontSymbolicTrait: &'static NSFontDescriptorTraitKey);

extern_static!(NSFontWeightTrait: &'static NSFontDescriptorTraitKey);

extern_static!(NSFontWidthTrait: &'static NSFontDescriptorTraitKey);

extern_static!(NSFontSlantTrait: &'static NSFontDescriptorTraitKey);

extern_static!(NSFontVariationAxisIdentifierKey: &'static NSFontDescriptorVariationKey);

extern_static!(NSFontVariationAxisMinimumValueKey: &'static NSFontDescriptorVariationKey);

extern_static!(NSFontVariationAxisMaximumValueKey: &'static NSFontDescriptorVariationKey);

extern_static!(NSFontVariationAxisDefaultValueKey: &'static NSFontDescriptorVariationKey);

extern_static!(NSFontVariationAxisNameKey: &'static NSFontDescriptorVariationKey);

extern_static!(NSFontFeatureTypeIdentifierKey: &'static NSFontDescriptorFeatureKey);

extern_static!(NSFontFeatureSelectorIdentifierKey: &'static NSFontDescriptorFeatureKey);

extern_static!(NSFontWeightUltraLight: NSFontWeight);

extern_static!(NSFontWeightThin: NSFontWeight);

extern_static!(NSFontWeightLight: NSFontWeight);

extern_static!(NSFontWeightRegular: NSFontWeight);

extern_static!(NSFontWeightMedium: NSFontWeight);

extern_static!(NSFontWeightSemibold: NSFontWeight);

extern_static!(NSFontWeightBold: NSFontWeight);

extern_static!(NSFontWeightHeavy: NSFontWeight);

extern_static!(NSFontWeightBlack: NSFontWeight);

extern_static!(NSFontDescriptorSystemDesignDefault: &'static NSFontDescriptorSystemDesign);

extern_static!(NSFontDescriptorSystemDesignSerif: &'static NSFontDescriptorSystemDesign);

extern_static!(NSFontDescriptorSystemDesignMonospaced: &'static NSFontDescriptorSystemDesign);

extern_static!(NSFontDescriptorSystemDesignRounded: &'static NSFontDescriptorSystemDesign);

extern_static!(NSFontTextStyleLargeTitle: &'static NSFontTextStyle);

extern_static!(NSFontTextStyleTitle1: &'static NSFontTextStyle);

extern_static!(NSFontTextStyleTitle2: &'static NSFontTextStyle);

extern_static!(NSFontTextStyleTitle3: &'static NSFontTextStyle);

extern_static!(NSFontTextStyleHeadline: &'static NSFontTextStyle);

extern_static!(NSFontTextStyleSubheadline: &'static NSFontTextStyle);

extern_static!(NSFontTextStyleBody: &'static NSFontTextStyle);

extern_static!(NSFontTextStyleCallout: &'static NSFontTextStyle);

extern_static!(NSFontTextStyleFootnote: &'static NSFontTextStyle);

extern_static!(NSFontTextStyleCaption1: &'static NSFontTextStyle);

extern_static!(NSFontTextStyleCaption2: &'static NSFontTextStyle);

pub type NSFontFamilyClass = u32;

extern_enum!(
    #[underlying(c_int)]
    pub enum {
        NSFontUnknownClass = 0<<28,
        NSFontOldStyleSerifsClass = 1<<28,
        NSFontTransitionalSerifsClass = 2<<28,
        NSFontModernSerifsClass = 3<<28,
        NSFontClarendonSerifsClass = 4<<28,
        NSFontSlabSerifsClass = 5<<28,
        NSFontFreeformSerifsClass = 7<<28,
        NSFontSansSerifClass = 8<<28,
        NSFontOrnamentalsClass = 9<<28,
        NSFontScriptsClass = 10<<28,
        NSFontSymbolicClass = 12<<28,
    }
);

extern_enum!(
    #[underlying(c_uint)]
    pub enum {
        NSFontFamilyClassMask = 0xF0000000,
    }
);

extern_enum!(
    #[underlying(c_uint)]
    pub enum {
        NSFontItalicTrait = 1<<0,
        NSFontBoldTrait = 1<<1,
        NSFontExpandedTrait = 1<<5,
        NSFontCondensedTrait = 1<<6,
        NSFontMonoSpaceTrait = 1<<10,
        NSFontVerticalTrait = 1<<11,
        NSFontUIOptimizedTrait = 1<<12,
    }
);

extern_static!(NSFontColorAttribute: &'static NSString);

extern_methods!(
    /// NSFontDescriptor_TextStyles
    #[cfg(feature = "AppKit_NSFontDescriptor")]
    unsafe impl NSFontDescriptor {
        #[cfg(all(
            feature = "AppKit_NSFontTextStyle",
            feature = "AppKit_NSFontTextStyleOptionKey"
        ))]
        #[method_id(@__retain_semantics Other preferredFontDescriptorForTextStyle:options:)]
        pub unsafe fn preferredFontDescriptorForTextStyle_options(
            style: &NSFontTextStyle,
            options: &NSDictionary<NSFontTextStyleOptionKey, Object>,
        ) -> Id<NSFontDescriptor, Shared>;
    }
);
