//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingviewcontrollertextalignment?language=objc)
// NS_TYPED_ENUM
pub type UITextFormattingViewControllerTextAlignment = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingviewcontrollertextalignmentleft?language=objc)
    pub static UITextFormattingViewControllerTextAlignmentLeft:
        &'static UITextFormattingViewControllerTextAlignment;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingviewcontrollertextalignmentcenter?language=objc)
    pub static UITextFormattingViewControllerTextAlignmentCenter:
        &'static UITextFormattingViewControllerTextAlignment;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingviewcontrollertextalignmentright?language=objc)
    pub static UITextFormattingViewControllerTextAlignmentRight:
        &'static UITextFormattingViewControllerTextAlignment;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingviewcontrollertextalignmentjustified?language=objc)
    pub static UITextFormattingViewControllerTextAlignmentJustified:
        &'static UITextFormattingViewControllerTextAlignment;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingviewcontrollertextalignmentnatural?language=objc)
    pub static UITextFormattingViewControllerTextAlignmentNatural:
        &'static UITextFormattingViewControllerTextAlignment;
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingviewcontrollertextlist?language=objc)
// NS_TYPED_ENUM
pub type UITextFormattingViewControllerTextList = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingviewcontrollertextlistdisc?language=objc)
    pub static UITextFormattingViewControllerTextListDisc:
        &'static UITextFormattingViewControllerTextList;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingviewcontrollertextlisthyphen?language=objc)
    pub static UITextFormattingViewControllerTextListHyphen:
        &'static UITextFormattingViewControllerTextList;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingviewcontrollertextlistdecimal?language=objc)
    pub static UITextFormattingViewControllerTextListDecimal:
        &'static UITextFormattingViewControllerTextList;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingviewcontrollertextlistother?language=objc)
    pub static UITextFormattingViewControllerTextListOther:
        &'static UITextFormattingViewControllerTextList;
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingviewcontrollerhighlight?language=objc)
// NS_TYPED_ENUM
pub type UITextFormattingViewControllerHighlight = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingviewcontrollerhighlightdefault?language=objc)
    pub static UITextFormattingViewControllerHighlightDefault:
        &'static UITextFormattingViewControllerHighlight;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingviewcontrollerhighlightpurple?language=objc)
    pub static UITextFormattingViewControllerHighlightPurple:
        &'static UITextFormattingViewControllerHighlight;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingviewcontrollerhighlightpink?language=objc)
    pub static UITextFormattingViewControllerHighlightPink:
        &'static UITextFormattingViewControllerHighlight;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingviewcontrollerhighlightorange?language=objc)
    pub static UITextFormattingViewControllerHighlightOrange:
        &'static UITextFormattingViewControllerHighlight;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingviewcontrollerhighlightmint?language=objc)
    pub static UITextFormattingViewControllerHighlightMint:
        &'static UITextFormattingViewControllerHighlight;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingviewcontrollerhighlightblue?language=objc)
    pub static UITextFormattingViewControllerHighlightBlue:
        &'static UITextFormattingViewControllerHighlight;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingviewcontrollerformattingdescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITextFormattingViewControllerFormattingDescriptor;
);

unsafe impl NSCoding for UITextFormattingViewControllerFormattingDescriptor {}

unsafe impl NSCopying for UITextFormattingViewControllerFormattingDescriptor {}

unsafe impl CopyingHelper for UITextFormattingViewControllerFormattingDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UITextFormattingViewControllerFormattingDescriptor {}

unsafe impl NSSecureCoding for UITextFormattingViewControllerFormattingDescriptor {}

extern_methods!(
    unsafe impl UITextFormattingViewControllerFormattingDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithString:range:)]
        pub unsafe fn initWithString_range(
            this: Allocated<Self>,
            string: &NSAttributedString,
            range: NSRange,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithAttributes:)]
        pub unsafe fn initWithAttributes(
            this: Allocated<Self>,
            attributes: &NSDictionary<NSAttributedStringKey, AnyObject>,
        ) -> Retained<Self>;

        #[cfg(feature = "UIFont")]
        #[method_id(@__retain_semantics Other fonts)]
        pub unsafe fn fonts(&self) -> Option<Retained<NSArray<UIFont>>>;

        #[cfg(feature = "UIFont")]
        #[method(setFonts:)]
        pub unsafe fn setFonts(&self, fonts: Option<&NSArray<UIFont>>);

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other textColors)]
        pub unsafe fn textColors(&self) -> Option<Retained<NSArray<UIColor>>>;

        #[cfg(feature = "UIColor")]
        #[method(setTextColors:)]
        pub unsafe fn setTextColors(&self, text_colors: Option<&NSArray<UIColor>>);

        #[method(lineHeight)]
        pub unsafe fn lineHeight(&self) -> CGFloat;

        #[method(setLineHeight:)]
        pub unsafe fn setLineHeight(&self, line_height: CGFloat);

        #[method(underlinePresent)]
        pub unsafe fn underlinePresent(&self) -> bool;

        #[method(setUnderlinePresent:)]
        pub unsafe fn setUnderlinePresent(&self, underline_present: bool);

        #[method(strikethroughPresent)]
        pub unsafe fn strikethroughPresent(&self) -> bool;

        #[method(setStrikethroughPresent:)]
        pub unsafe fn setStrikethroughPresent(&self, strikethrough_present: bool);

        #[method_id(@__retain_semantics Other textAlignments)]
        pub unsafe fn textAlignments(
            &self,
        ) -> Retained<NSSet<UITextFormattingViewControllerTextAlignment>>;

        #[method(setTextAlignments:)]
        pub unsafe fn setTextAlignments(
            &self,
            text_alignments: &NSSet<UITextFormattingViewControllerTextAlignment>,
        );

        #[method_id(@__retain_semantics Other textLists)]
        pub unsafe fn textLists(&self) -> Retained<NSSet<UITextFormattingViewControllerTextList>>;

        #[method(setTextLists:)]
        pub unsafe fn setTextLists(
            &self,
            text_lists: &NSSet<UITextFormattingViewControllerTextList>,
        );

        #[method_id(@__retain_semantics Other highlights)]
        pub unsafe fn highlights(&self)
            -> Retained<NSSet<UITextFormattingViewControllerHighlight>>;

        #[method(setHighlights:)]
        pub unsafe fn setHighlights(
            &self,
            highlights: &NSSet<UITextFormattingViewControllerHighlight>,
        );

        #[method_id(@__retain_semantics Other formattingStyleKey)]
        pub unsafe fn formattingStyleKey(&self) -> Option<Retained<NSString>>;

        #[method(setFormattingStyleKey:)]
        pub unsafe fn setFormattingStyleKey(&self, formatting_style_key: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITextFormattingViewControllerFormattingDescriptor {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
