//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSFormattingContext(pub NSInteger);
impl NSFormattingContext {
    #[doc(alias = "NSFormattingContextUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "NSFormattingContextDynamic")]
    pub const Dynamic: Self = Self(1);
    #[doc(alias = "NSFormattingContextStandalone")]
    pub const Standalone: Self = Self(2);
    #[doc(alias = "NSFormattingContextListItem")]
    pub const ListItem: Self = Self(3);
    #[doc(alias = "NSFormattingContextBeginningOfSentence")]
    pub const BeginningOfSentence: Self = Self(4);
    #[doc(alias = "NSFormattingContextMiddleOfSentence")]
    pub const MiddleOfSentence: Self = Self(5);
}

unsafe impl Encode for NSFormattingContext {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSFormattingContext {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSFormattingUnitStyle(pub NSInteger);
impl NSFormattingUnitStyle {
    #[doc(alias = "NSFormattingUnitStyleShort")]
    pub const Short: Self = Self(1);
    #[doc(alias = "NSFormattingUnitStyleMedium")]
    pub const Medium: Self = Self(2);
    #[doc(alias = "NSFormattingUnitStyleLong")]
    pub const Long: Self = Self(3);
}

unsafe impl Encode for NSFormattingUnitStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSFormattingUnitStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSFormatter;

    unsafe impl ClassType for NSFormatter {
        type Super = NSObject;
    }
);

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSFormatter {}

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSFormatter {}

#[cfg(feature = "NSObject")]
unsafe impl CopyingHelper for NSFormatter {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSFormatter {}

extern_methods!(
    unsafe impl NSFormatter {
        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other stringForObjectValue:)]
        pub unsafe fn stringForObjectValue(
            &self,
            obj: Option<&AnyObject>,
        ) -> Option<Retained<NSString>>;

        #[cfg(all(
            feature = "NSAttributedString",
            feature = "NSDictionary",
            feature = "NSString"
        ))]
        #[method_id(@__retain_semantics Other attributedStringForObjectValue:withDefaultAttributes:)]
        pub unsafe fn attributedStringForObjectValue_withDefaultAttributes(
            &self,
            obj: &AnyObject,
            attrs: Option<&NSDictionary<NSAttributedStringKey, AnyObject>>,
        ) -> Option<Retained<NSAttributedString>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other editingStringForObjectValue:)]
        pub unsafe fn editingStringForObjectValue(
            &self,
            obj: &AnyObject,
        ) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method(getObjectValue:forString:errorDescription:)]
        pub unsafe fn getObjectValue_forString_errorDescription(
            &self,
            obj: Option<&mut Option<Retained<AnyObject>>>,
            string: &NSString,
            error: Option<&mut Option<Retained<NSString>>>,
        ) -> bool;

        #[cfg(feature = "NSString")]
        #[method(isPartialStringValid:newEditingString:errorDescription:)]
        pub unsafe fn isPartialStringValid_newEditingString_errorDescription(
            &self,
            partial_string: &NSString,
            new_string: Option<&mut Option<Retained<NSString>>>,
            error: Option<&mut Option<Retained<NSString>>>,
        ) -> bool;

        #[cfg(all(feature = "NSRange", feature = "NSString"))]
        #[method(isPartialStringValid:proposedSelectedRange:originalString:originalSelectedRange:errorDescription:)]
        pub unsafe fn isPartialStringValid_proposedSelectedRange_originalString_originalSelectedRange_errorDescription(
            &self,
            partial_string_ptr: &mut Retained<NSString>,
            proposed_sel_range_ptr: NSRangePointer,
            orig_string: &NSString,
            orig_sel_range: NSRange,
            error: Option<&mut Option<Retained<NSString>>>,
        ) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSFormatter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
