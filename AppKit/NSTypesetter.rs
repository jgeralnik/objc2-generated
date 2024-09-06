//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTypesetter;

    unsafe impl ClassType for NSTypesetter {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for NSTypesetter {}

extern_methods!(
    unsafe impl NSTypesetter {
        #[method(usesFontLeading)]
        pub unsafe fn usesFontLeading(&self) -> bool;

        #[method(setUsesFontLeading:)]
        pub unsafe fn setUsesFontLeading(&self, uses_font_leading: bool);

        #[cfg(feature = "NSLayoutManager")]
        #[method(typesetterBehavior)]
        pub unsafe fn typesetterBehavior(&self) -> NSTypesetterBehavior;

        #[cfg(feature = "NSLayoutManager")]
        #[method(setTypesetterBehavior:)]
        pub unsafe fn setTypesetterBehavior(&self, typesetter_behavior: NSTypesetterBehavior);

        #[method(hyphenationFactor)]
        pub unsafe fn hyphenationFactor(&self) -> c_float;

        #[method(setHyphenationFactor:)]
        pub unsafe fn setHyphenationFactor(&self, hyphenation_factor: c_float);

        #[method(lineFragmentPadding)]
        pub unsafe fn lineFragmentPadding(&self) -> CGFloat;

        #[method(setLineFragmentPadding:)]
        pub unsafe fn setLineFragmentPadding(&self, line_fragment_padding: CGFloat);

        #[cfg(feature = "NSFont")]
        #[method_id(@__retain_semantics Other substituteFontForFont:)]
        pub unsafe fn substituteFontForFont(&self, original_font: &NSFont) -> Retained<NSFont>;

        #[cfg(all(feature = "NSParagraphStyle", feature = "NSText"))]
        #[method_id(@__retain_semantics Other textTabForGlyphLocation:writingDirection:maxLocation:)]
        pub unsafe fn textTabForGlyphLocation_writingDirection_maxLocation(
            &self,
            glyph_location: CGFloat,
            direction: NSWritingDirection,
            max_location: CGFloat,
        ) -> Option<Retained<NSTextTab>>;

        #[method(bidiProcessingEnabled)]
        pub unsafe fn bidiProcessingEnabled(&self) -> bool;

        #[method(setBidiProcessingEnabled:)]
        pub unsafe fn setBidiProcessingEnabled(&self, bidi_processing_enabled: bool);

        #[method_id(@__retain_semantics Other attributedString)]
        pub unsafe fn attributedString(&self) -> Option<Retained<NSAttributedString>>;

        #[method(setAttributedString:)]
        pub unsafe fn setAttributedString(&self, attributed_string: Option<&NSAttributedString>);

        #[method(setParagraphGlyphRange:separatorGlyphRange:)]
        pub unsafe fn setParagraphGlyphRange_separatorGlyphRange(
            &self,
            paragraph_range: NSRange,
            paragraph_separator_range: NSRange,
        );

        #[method(paragraphGlyphRange)]
        pub unsafe fn paragraphGlyphRange(&self) -> NSRange;

        #[method(paragraphSeparatorGlyphRange)]
        pub unsafe fn paragraphSeparatorGlyphRange(&self) -> NSRange;

        #[method(paragraphCharacterRange)]
        pub unsafe fn paragraphCharacterRange(&self) -> NSRange;

        #[method(paragraphSeparatorCharacterRange)]
        pub unsafe fn paragraphSeparatorCharacterRange(&self) -> NSRange;

        #[method(layoutParagraphAtPoint:)]
        pub unsafe fn layoutParagraphAtPoint(
            &self,
            line_fragment_origin: NSPointPointer,
        ) -> NSUInteger;

        #[method(beginParagraph)]
        pub unsafe fn beginParagraph(&self);

        #[method(endParagraph)]
        pub unsafe fn endParagraph(&self);

        #[method(beginLineWithGlyphAtIndex:)]
        pub unsafe fn beginLineWithGlyphAtIndex(&self, glyph_index: NSUInteger);

        #[method(endLineWithGlyphRange:)]
        pub unsafe fn endLineWithGlyphRange(&self, line_glyph_range: NSRange);

        #[method(lineSpacingAfterGlyphAtIndex:withProposedLineFragmentRect:)]
        pub unsafe fn lineSpacingAfterGlyphAtIndex_withProposedLineFragmentRect(
            &self,
            glyph_index: NSUInteger,
            rect: NSRect,
        ) -> CGFloat;

        #[method(paragraphSpacingBeforeGlyphAtIndex:withProposedLineFragmentRect:)]
        pub unsafe fn paragraphSpacingBeforeGlyphAtIndex_withProposedLineFragmentRect(
            &self,
            glyph_index: NSUInteger,
            rect: NSRect,
        ) -> CGFloat;

        #[method(paragraphSpacingAfterGlyphAtIndex:withProposedLineFragmentRect:)]
        pub unsafe fn paragraphSpacingAfterGlyphAtIndex_withProposedLineFragmentRect(
            &self,
            glyph_index: NSUInteger,
            rect: NSRect,
        ) -> CGFloat;

        #[method(getLineFragmentRect:usedRect:forParagraphSeparatorGlyphRange:atProposedOrigin:)]
        pub unsafe fn getLineFragmentRect_usedRect_forParagraphSeparatorGlyphRange_atProposedOrigin(
            &self,
            line_fragment_rect: NSRectPointer,
            line_fragment_used_rect: NSRectPointer,
            paragraph_separator_glyph_range: NSRange,
            line_origin: NSPoint,
        );

        #[method_id(@__retain_semantics Other attributesForExtraLineFragment)]
        pub unsafe fn attributesForExtraLineFragment(
            &self,
        ) -> Retained<NSDictionary<NSAttributedStringKey, AnyObject>>;

        #[cfg(feature = "NSLayoutManager")]
        #[method_id(@__retain_semantics Other layoutManager)]
        pub unsafe fn layoutManager(&self) -> Option<Retained<NSLayoutManager>>;

        #[cfg(feature = "NSTextContainer")]
        #[method_id(@__retain_semantics Other textContainers)]
        pub unsafe fn textContainers(&self) -> Option<Retained<NSArray<NSTextContainer>>>;

        #[cfg(feature = "NSTextContainer")]
        #[method_id(@__retain_semantics Other currentTextContainer)]
        pub unsafe fn currentTextContainer(&self) -> Option<Retained<NSTextContainer>>;

        #[cfg(feature = "NSParagraphStyle")]
        #[method_id(@__retain_semantics Other currentParagraphStyle)]
        pub unsafe fn currentParagraphStyle(&self) -> Option<Retained<NSParagraphStyle>>;

        #[method(setHardInvalidation:forGlyphRange:)]
        pub unsafe fn setHardInvalidation_forGlyphRange(&self, flag: bool, glyph_range: NSRange);

        #[cfg(feature = "NSLayoutManager")]
        #[method(layoutGlyphsInLayoutManager:startingAtGlyphIndex:maxNumberOfLineFragments:nextGlyphIndex:)]
        pub unsafe fn layoutGlyphsInLayoutManager_startingAtGlyphIndex_maxNumberOfLineFragments_nextGlyphIndex(
            &self,
            layout_manager: &NSLayoutManager,
            start_glyph_index: NSUInteger,
            max_num_lines: NSUInteger,
            next_glyph: NonNull<NSUInteger>,
        );

        #[cfg(feature = "NSLayoutManager")]
        #[method(layoutCharactersInRange:forLayoutManager:maximumNumberOfLineFragments:)]
        pub unsafe fn layoutCharactersInRange_forLayoutManager_maximumNumberOfLineFragments(
            &self,
            character_range: NSRange,
            layout_manager: &NSLayoutManager,
            max_num_lines: NSUInteger,
        ) -> NSRange;

        #[cfg(feature = "NSLayoutManager")]
        #[method(printingAdjustmentInLayoutManager:forNominallySpacedGlyphRange:packedGlyphs:count:)]
        pub unsafe fn printingAdjustmentInLayoutManager_forNominallySpacedGlyphRange_packedGlyphs_count(
            layout_mgr: &NSLayoutManager,
            nominally_spaced_glyphs_range: NSRange,
            packed_glyphs: NonNull<c_uchar>,
            packed_glyphs_count: NSUInteger,
        ) -> NSSize;

        #[cfg(feature = "NSLayoutManager")]
        #[method(baselineOffsetInLayoutManager:glyphIndex:)]
        pub unsafe fn baselineOffsetInLayoutManager_glyphIndex(
            &self,
            layout_mgr: &NSLayoutManager,
            glyph_index: NSUInteger,
        ) -> CGFloat;

        #[method_id(@__retain_semantics Other sharedSystemTypesetter)]
        pub unsafe fn sharedSystemTypesetter() -> Retained<NSTypesetter>;

        #[cfg(feature = "NSLayoutManager")]
        #[method_id(@__retain_semantics Other sharedSystemTypesetterForBehavior:)]
        pub unsafe fn sharedSystemTypesetterForBehavior(
            behavior: NSTypesetterBehavior,
        ) -> Retained<AnyObject>;

        #[cfg(feature = "NSLayoutManager")]
        #[method(defaultTypesetterBehavior)]
        pub unsafe fn defaultTypesetterBehavior() -> NSTypesetterBehavior;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSTypesetter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// NSLayoutPhaseInterface
    unsafe impl NSTypesetter {
        #[method(willSetLineFragmentRect:forGlyphRange:usedRect:baselineOffset:)]
        pub unsafe fn willSetLineFragmentRect_forGlyphRange_usedRect_baselineOffset(
            &self,
            line_rect: NSRectPointer,
            glyph_range: NSRange,
            used_rect: NSRectPointer,
            baseline_offset: NonNull<CGFloat>,
        );

        #[method(shouldBreakLineByWordBeforeCharacterAtIndex:)]
        pub unsafe fn shouldBreakLineByWordBeforeCharacterAtIndex(
            &self,
            char_index: NSUInteger,
        ) -> bool;

        #[method(shouldBreakLineByHyphenatingBeforeCharacterAtIndex:)]
        pub unsafe fn shouldBreakLineByHyphenatingBeforeCharacterAtIndex(
            &self,
            char_index: NSUInteger,
        ) -> bool;

        #[method(hyphenationFactorForGlyphAtIndex:)]
        pub unsafe fn hyphenationFactorForGlyphAtIndex(&self, glyph_index: NSUInteger) -> c_float;

        #[method(hyphenCharacterForGlyphAtIndex:)]
        pub unsafe fn hyphenCharacterForGlyphAtIndex(&self, glyph_index: NSUInteger) -> UTF32Char;

        #[cfg(feature = "NSTextContainer")]
        #[method(boundingBoxForControlGlyphAtIndex:forTextContainer:proposedLineFragment:glyphPosition:characterIndex:)]
        pub unsafe fn boundingBoxForControlGlyphAtIndex_forTextContainer_proposedLineFragment_glyphPosition_characterIndex(
            &self,
            glyph_index: NSUInteger,
            text_container: &NSTextContainer,
            proposed_rect: NSRect,
            glyph_position: NSPoint,
            char_index: NSUInteger,
        ) -> NSRect;
    }
);

extern_methods!(
    /// NSGlyphStorageInterface
    unsafe impl NSTypesetter {
        #[method(characterRangeForGlyphRange:actualGlyphRange:)]
        pub unsafe fn characterRangeForGlyphRange_actualGlyphRange(
            &self,
            glyph_range: NSRange,
            actual_glyph_range: NSRangePointer,
        ) -> NSRange;

        #[method(glyphRangeForCharacterRange:actualCharacterRange:)]
        pub unsafe fn glyphRangeForCharacterRange_actualCharacterRange(
            &self,
            char_range: NSRange,
            actual_char_range: NSRangePointer,
        ) -> NSRange;

        #[method(getLineFragmentRect:usedRect:remainingRect:forStartingGlyphAtIndex:proposedRect:lineSpacing:paragraphSpacingBefore:paragraphSpacingAfter:)]
        pub unsafe fn getLineFragmentRect_usedRect_remainingRect_forStartingGlyphAtIndex_proposedRect_lineSpacing_paragraphSpacingBefore_paragraphSpacingAfter(
            &self,
            line_fragment_rect: NSRectPointer,
            line_fragment_used_rect: NSRectPointer,
            remaining_rect: NSRectPointer,
            starting_glyph_index: NSUInteger,
            proposed_rect: NSRect,
            line_spacing: CGFloat,
            paragraph_spacing_before: CGFloat,
            paragraph_spacing_after: CGFloat,
        );

        #[method(setLineFragmentRect:forGlyphRange:usedRect:baselineOffset:)]
        pub unsafe fn setLineFragmentRect_forGlyphRange_usedRect_baselineOffset(
            &self,
            fragment_rect: NSRect,
            glyph_range: NSRange,
            used_rect: NSRect,
            baseline_offset: CGFloat,
        );

        #[method(setNotShownAttribute:forGlyphRange:)]
        pub unsafe fn setNotShownAttribute_forGlyphRange(&self, flag: bool, glyph_range: NSRange);

        #[method(setDrawsOutsideLineFragment:forGlyphRange:)]
        pub unsafe fn setDrawsOutsideLineFragment_forGlyphRange(
            &self,
            flag: bool,
            glyph_range: NSRange,
        );

        #[method(setLocation:withAdvancements:forStartOfGlyphRange:)]
        pub unsafe fn setLocation_withAdvancements_forStartOfGlyphRange(
            &self,
            location: NSPoint,
            advancements: *mut CGFloat,
            glyph_range: NSRange,
        );

        #[method(setAttachmentSize:forGlyphRange:)]
        pub unsafe fn setAttachmentSize_forGlyphRange(
            &self,
            attachment_size: NSSize,
            glyph_range: NSRange,
        );

        #[method(setBidiLevels:forGlyphRange:)]
        pub unsafe fn setBidiLevels_forGlyphRange(&self, levels: *mut u8, glyph_range: NSRange);
    }
);

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTypesetterControlCharacterAction(pub NSUInteger);
bitflags::bitflags! {
    impl NSTypesetterControlCharacterAction: NSUInteger {
        const NSTypesetterZeroAdvancementAction = 1<<0;
        const NSTypesetterWhitespaceAction = 1<<1;
        const NSTypesetterHorizontalTabAction = 1<<2;
        const NSTypesetterLineBreakAction = 1<<3;
        const NSTypesetterParagraphBreakAction = 1<<4;
        const NSTypesetterContainerBreakAction = 1<<5;
    }
}

unsafe impl Encode for NSTypesetterControlCharacterAction {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSTypesetterControlCharacterAction {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// NSTypesetter_Deprecated
    unsafe impl NSTypesetter {
        #[method(actionForControlCharacterAtIndex:)]
        pub unsafe fn actionForControlCharacterAtIndex(
            &self,
            char_index: NSUInteger,
        ) -> NSTypesetterControlCharacterAction;

        #[cfg(all(feature = "NSFont", feature = "NSLayoutManager"))]
        #[deprecated]
        #[method(getGlyphsInRange:glyphs:characterIndexes:glyphInscriptions:elasticBits:bidiLevels:)]
        pub unsafe fn getGlyphsInRange_glyphs_characterIndexes_glyphInscriptions_elasticBits_bidiLevels(
            &self,
            glyphs_range: NSRange,
            glyph_buffer: *mut NSGlyph,
            char_index_buffer: *mut NSUInteger,
            inscribe_buffer: *mut NSGlyphInscription,
            elastic_buffer: *mut Bool,
            bidi_level_buffer: *mut c_uchar,
        ) -> NSUInteger;

        #[cfg(feature = "NSFont")]
        #[deprecated]
        #[method(substituteGlyphsInRange:withGlyphs:)]
        pub unsafe fn substituteGlyphsInRange_withGlyphs(
            &self,
            glyph_range: NSRange,
            glyphs: *mut NSGlyph,
        );

        #[cfg(feature = "NSFont")]
        #[deprecated]
        #[method(insertGlyph:atGlyphIndex:characterIndex:)]
        pub unsafe fn insertGlyph_atGlyphIndex_characterIndex(
            &self,
            glyph: NSGlyph,
            glyph_index: NSUInteger,
            character_index: NSUInteger,
        );

        #[deprecated]
        #[method(deleteGlyphsInRange:)]
        pub unsafe fn deleteGlyphsInRange(&self, glyph_range: NSRange);
    }
);
