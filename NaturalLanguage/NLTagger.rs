//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NLTaggerOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NLTaggerOptions: NSUInteger {
        const NLTaggerOmitWords = 1<<0;
        const NLTaggerOmitPunctuation = 1<<1;
        const NLTaggerOmitWhitespace = 1<<2;
        const NLTaggerOmitOther = 1<<3;
        const NLTaggerJoinNames = 1<<4;
        const NLTaggerJoinContractions = 1<<5;
    }
}

unsafe impl Encode for NLTaggerOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NLTaggerOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NLTaggerAssetsResult(pub NSInteger);
impl NLTaggerAssetsResult {
    #[doc(alias = "NLTaggerAssetsResultAvailable")]
    pub const Available: Self = Self(0);
    #[doc(alias = "NLTaggerAssetsResultNotAvailable")]
    pub const NotAvailable: Self = Self(1);
    #[doc(alias = "NLTaggerAssetsResultError")]
    pub const Error: Self = Self(2);
}

unsafe impl Encode for NLTaggerAssetsResult {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NLTaggerAssetsResult {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NLTagger;

    unsafe impl ClassType for NLTagger {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for NLTagger {}

extern_methods!(
    unsafe impl NLTagger {
        #[cfg(feature = "NLTagScheme")]
        #[method_id(@__retain_semantics Init initWithTagSchemes:)]
        pub unsafe fn initWithTagSchemes(
            this: Allocated<Self>,
            tag_schemes: &NSArray<NLTagScheme>,
        ) -> Retained<Self>;

        #[cfg(feature = "NLTagScheme")]
        #[method_id(@__retain_semantics Other tagSchemes)]
        pub unsafe fn tagSchemes(&self) -> Retained<NSArray<NLTagScheme>>;

        #[method_id(@__retain_semantics Other string)]
        pub unsafe fn string(&self) -> Option<Retained<NSString>>;

        #[method(setString:)]
        pub unsafe fn setString(&self, string: Option<&NSString>);

        #[cfg(all(
            feature = "NLLanguage",
            feature = "NLTagScheme",
            feature = "NLTokenizer"
        ))]
        #[method_id(@__retain_semantics Other availableTagSchemesForUnit:language:)]
        pub unsafe fn availableTagSchemesForUnit_language(
            unit: NLTokenUnit,
            language: &NLLanguage,
        ) -> Retained<NSArray<NLTagScheme>>;

        #[cfg(feature = "NLTokenizer")]
        #[method(tokenRangeAtIndex:unit:)]
        pub unsafe fn tokenRangeAtIndex_unit(
            &self,
            character_index: NSUInteger,
            unit: NLTokenUnit,
        ) -> NSRange;

        #[cfg(feature = "NLTokenizer")]
        #[method(tokenRangeForRange:unit:)]
        pub unsafe fn tokenRangeForRange_unit(&self, range: NSRange, unit: NLTokenUnit) -> NSRange;

        #[cfg(feature = "NLLanguage")]
        #[method_id(@__retain_semantics Other dominantLanguage)]
        pub unsafe fn dominantLanguage(&self) -> Option<Retained<NLLanguage>>;

        #[cfg(all(feature = "NLTagScheme", feature = "NLTokenizer", feature = "block2"))]
        #[method(enumerateTagsInRange:unit:scheme:options:usingBlock:)]
        pub unsafe fn enumerateTagsInRange_unit_scheme_options_usingBlock(
            &self,
            range: NSRange,
            unit: NLTokenUnit,
            scheme: &NLTagScheme,
            options: NLTaggerOptions,
            block: &block2::Block<dyn Fn(*mut NLTag, NSRange, NonNull<Bool>) + '_>,
        );

        #[cfg(all(feature = "NLTagScheme", feature = "NLTokenizer"))]
        #[method_id(@__retain_semantics Other tagAtIndex:unit:scheme:tokenRange:)]
        pub unsafe fn tagAtIndex_unit_scheme_tokenRange(
            &self,
            character_index: NSUInteger,
            unit: NLTokenUnit,
            scheme: &NLTagScheme,
            token_range: NSRangePointer,
        ) -> Option<Retained<NLTag>>;

        #[cfg(all(feature = "NLTagScheme", feature = "NLTokenizer"))]
        #[method_id(@__retain_semantics Other tagsInRange:unit:scheme:options:tokenRanges:)]
        pub unsafe fn tagsInRange_unit_scheme_options_tokenRanges(
            &self,
            range: NSRange,
            unit: NLTokenUnit,
            scheme: &NLTagScheme,
            options: NLTaggerOptions,
            token_ranges: Option<&mut Option<Retained<NSArray<NSValue>>>>,
        ) -> Retained<NSArray<NLTag>>;

        #[cfg(all(feature = "NLTagScheme", feature = "NLTokenizer"))]
        #[method_id(@__retain_semantics Other tagHypothesesAtIndex:unit:scheme:maximumCount:tokenRange:)]
        pub unsafe fn tagHypothesesAtIndex_unit_scheme_maximumCount_tokenRange(
            &self,
            character_index: NSUInteger,
            unit: NLTokenUnit,
            scheme: &NLTagScheme,
            maximum_count: NSUInteger,
            token_range: NSRangePointer,
        ) -> Retained<NSDictionary<NLTag, NSNumber>>;

        #[cfg(feature = "NLLanguage")]
        #[method(setLanguage:range:)]
        pub unsafe fn setLanguage_range(&self, language: &NLLanguage, range: NSRange);

        #[method(setOrthography:range:)]
        pub unsafe fn setOrthography_range(&self, orthography: &NSOrthography, range: NSRange);

        #[cfg(all(feature = "NLModel", feature = "NLTagScheme"))]
        #[method(setModels:forTagScheme:)]
        pub unsafe fn setModels_forTagScheme(
            &self,
            models: &NSArray<NLModel>,
            tag_scheme: &NLTagScheme,
        );

        #[cfg(all(feature = "NLModel", feature = "NLTagScheme"))]
        #[method_id(@__retain_semantics Other modelsForTagScheme:)]
        pub unsafe fn modelsForTagScheme(
            &self,
            tag_scheme: &NLTagScheme,
        ) -> Retained<NSArray<NLModel>>;

        #[cfg(all(feature = "NLGazetteer", feature = "NLTagScheme"))]
        #[method(setGazetteers:forTagScheme:)]
        pub unsafe fn setGazetteers_forTagScheme(
            &self,
            gazetteers: &NSArray<NLGazetteer>,
            tag_scheme: &NLTagScheme,
        );

        #[cfg(all(feature = "NLGazetteer", feature = "NLTagScheme"))]
        #[method_id(@__retain_semantics Other gazetteersForTagScheme:)]
        pub unsafe fn gazetteersForTagScheme(
            &self,
            tag_scheme: &NLTagScheme,
        ) -> Retained<NSArray<NLGazetteer>>;

        #[cfg(all(feature = "NLLanguage", feature = "NLTagScheme", feature = "block2"))]
        #[method(requestAssetsForLanguage:tagScheme:completionHandler:)]
        pub unsafe fn requestAssetsForLanguage_tagScheme_completionHandler(
            language: &NLLanguage,
            tag_scheme: &NLTagScheme,
            completion_handler: &block2::Block<dyn Fn(NLTaggerAssetsResult, *mut NSError)>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NLTagger {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
