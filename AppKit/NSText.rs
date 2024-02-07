//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSWritingDirection {
        NSWritingDirectionNatural = -1,
        NSWritingDirectionLeftToRight = 0,
        NSWritingDirectionRightToLeft = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSText")]
    pub struct NSText;

    #[cfg(feature = "AppKit_NSText")]
    unsafe impl ClassType for NSText {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "AppKit_NSText")]
unsafe impl NSAccessibility for NSText {}

#[cfg(feature = "AppKit_NSText")]
unsafe impl NSAccessibilityElementProtocol for NSText {}

#[cfg(feature = "AppKit_NSText")]
unsafe impl NSAnimatablePropertyContainer for NSText {}

#[cfg(feature = "AppKit_NSText")]
unsafe impl NSAppearanceCustomization for NSText {}

#[cfg(feature = "AppKit_NSText")]
unsafe impl NSChangeSpelling for NSText {}

#[cfg(feature = "AppKit_NSText")]
unsafe impl NSCoding for NSText {}

#[cfg(feature = "AppKit_NSText")]
unsafe impl NSDraggingDestination for NSText {}

#[cfg(feature = "AppKit_NSText")]
unsafe impl NSIgnoreMisspelledWords for NSText {}

#[cfg(feature = "AppKit_NSText")]
unsafe impl NSObjectProtocol for NSText {}

#[cfg(feature = "AppKit_NSText")]
unsafe impl NSUserInterfaceItemIdentification for NSText {}

extern_methods!(
    #[cfg(feature = "AppKit_NSText")]
    unsafe impl NSText {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other string)]
        pub unsafe fn string(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setString:)]
        pub unsafe fn setString(&self, string: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method(replaceCharactersInRange:withString:)]
        pub unsafe fn replaceCharactersInRange_withString(&self, range: NSRange, string: &NSString);

        #[cfg(feature = "Foundation_NSData")]
        #[method(replaceCharactersInRange:withRTF:)]
        pub unsafe fn replaceCharactersInRange_withRTF(&self, range: NSRange, rtf_data: &NSData);

        #[cfg(feature = "Foundation_NSData")]
        #[method(replaceCharactersInRange:withRTFD:)]
        pub unsafe fn replaceCharactersInRange_withRTFD(&self, range: NSRange, rtfd_data: &NSData);

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other RTFFromRange:)]
        pub unsafe fn RTFFromRange(&self, range: NSRange) -> Option<Id<NSData>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other RTFDFromRange:)]
        pub unsafe fn RTFDFromRange(&self, range: NSRange) -> Option<Id<NSData>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(writeRTFDToFile:atomically:)]
        pub unsafe fn writeRTFDToFile_atomically(&self, path: &NSString, flag: bool) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(readRTFDFromFile:)]
        pub unsafe fn readRTFDFromFile(&self, path: &NSString) -> bool;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSTextDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn NSTextDelegate>>);

        #[method(isEditable)]
        pub unsafe fn isEditable(&self) -> bool;

        #[method(setEditable:)]
        pub unsafe fn setEditable(&self, editable: bool);

        #[method(isSelectable)]
        pub unsafe fn isSelectable(&self) -> bool;

        #[method(setSelectable:)]
        pub unsafe fn setSelectable(&self, selectable: bool);

        #[method(isRichText)]
        pub unsafe fn isRichText(&self) -> bool;

        #[method(setRichText:)]
        pub unsafe fn setRichText(&self, rich_text: bool);

        #[method(importsGraphics)]
        pub unsafe fn importsGraphics(&self) -> bool;

        #[method(setImportsGraphics:)]
        pub unsafe fn setImportsGraphics(&self, imports_graphics: bool);

        #[method(isFieldEditor)]
        pub unsafe fn isFieldEditor(&self) -> bool;

        #[method(setFieldEditor:)]
        pub unsafe fn setFieldEditor(&self, field_editor: bool);

        #[method(usesFontPanel)]
        pub unsafe fn usesFontPanel(&self) -> bool;

        #[method(setUsesFontPanel:)]
        pub unsafe fn setUsesFontPanel(&self, uses_font_panel: bool);

        #[method(drawsBackground)]
        pub unsafe fn drawsBackground(&self) -> bool;

        #[method(setDrawsBackground:)]
        pub unsafe fn setDrawsBackground(&self, draws_background: bool);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Option<Id<NSColor>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, background_color: Option<&NSColor>);

        #[method(isRulerVisible)]
        pub unsafe fn isRulerVisible(&self) -> bool;

        #[method(selectedRange)]
        pub unsafe fn selectedRange(&self) -> NSRange;

        #[method(setSelectedRange:)]
        pub unsafe fn setSelectedRange(&self, selected_range: NSRange);

        #[method(scrollRangeToVisible:)]
        pub unsafe fn scrollRangeToVisible(&self, range: NSRange);

        #[cfg(feature = "AppKit_NSFont")]
        #[method_id(@__retain_semantics Other font)]
        pub unsafe fn font(&self) -> Option<Id<NSFont>>;

        #[cfg(feature = "AppKit_NSFont")]
        #[method(setFont:)]
        pub unsafe fn setFont(&self, font: Option<&NSFont>);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other textColor)]
        pub unsafe fn textColor(&self) -> Option<Id<NSColor>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setTextColor:)]
        pub unsafe fn setTextColor(&self, text_color: Option<&NSColor>);

        #[method(alignment)]
        pub unsafe fn alignment(&self) -> NSTextAlignment;

        #[method(setAlignment:)]
        pub unsafe fn setAlignment(&self, alignment: NSTextAlignment);

        #[method(baseWritingDirection)]
        pub unsafe fn baseWritingDirection(&self) -> NSWritingDirection;

        #[method(setBaseWritingDirection:)]
        pub unsafe fn setBaseWritingDirection(&self, base_writing_direction: NSWritingDirection);

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setTextColor:range:)]
        pub unsafe fn setTextColor_range(&self, color: Option<&NSColor>, range: NSRange);

        #[cfg(feature = "AppKit_NSFont")]
        #[method(setFont:range:)]
        pub unsafe fn setFont_range(&self, font: &NSFont, range: NSRange);

        #[method(maxSize)]
        pub unsafe fn maxSize(&self) -> NSSize;

        #[method(setMaxSize:)]
        pub unsafe fn setMaxSize(&self, max_size: NSSize);

        #[method(minSize)]
        pub unsafe fn minSize(&self) -> NSSize;

        #[method(setMinSize:)]
        pub unsafe fn setMinSize(&self, min_size: NSSize);

        #[method(isHorizontallyResizable)]
        pub unsafe fn isHorizontallyResizable(&self) -> bool;

        #[method(setHorizontallyResizable:)]
        pub unsafe fn setHorizontallyResizable(&self, horizontally_resizable: bool);

        #[method(isVerticallyResizable)]
        pub unsafe fn isVerticallyResizable(&self) -> bool;

        #[method(setVerticallyResizable:)]
        pub unsafe fn setVerticallyResizable(&self, vertically_resizable: bool);

        #[method(sizeToFit)]
        pub unsafe fn sizeToFit(&self);

        #[method(copy:)]
        pub unsafe fn copy(&self, sender: Option<&AnyObject>);

        #[method(copyFont:)]
        pub unsafe fn copyFont(&self, sender: Option<&AnyObject>);

        #[method(copyRuler:)]
        pub unsafe fn copyRuler(&self, sender: Option<&AnyObject>);

        #[method(cut:)]
        pub unsafe fn cut(&self, sender: Option<&AnyObject>);

        #[method(delete:)]
        pub unsafe fn delete(&self, sender: Option<&AnyObject>);

        #[method(paste:)]
        pub unsafe fn paste(&self, sender: Option<&AnyObject>);

        #[method(pasteFont:)]
        pub unsafe fn pasteFont(&self, sender: Option<&AnyObject>);

        #[method(pasteRuler:)]
        pub unsafe fn pasteRuler(&self, sender: Option<&AnyObject>);

        #[method(selectAll:)]
        pub unsafe fn selectAll(&self, sender: Option<&AnyObject>);

        #[method(changeFont:)]
        pub unsafe fn changeFont(&self, sender: Option<&AnyObject>);

        #[method(alignLeft:)]
        pub unsafe fn alignLeft(&self, sender: Option<&AnyObject>);

        #[method(alignRight:)]
        pub unsafe fn alignRight(&self, sender: Option<&AnyObject>);

        #[method(alignCenter:)]
        pub unsafe fn alignCenter(&self, sender: Option<&AnyObject>);

        #[method(subscript:)]
        pub unsafe fn subscript(&self, sender: Option<&AnyObject>);

        #[method(superscript:)]
        pub unsafe fn superscript(&self, sender: Option<&AnyObject>);

        #[method(underline:)]
        pub unsafe fn underline(&self, sender: Option<&AnyObject>);

        #[method(unscript:)]
        pub unsafe fn unscript(&self, sender: Option<&AnyObject>);

        #[method(showGuessPanel:)]
        pub unsafe fn showGuessPanel(&self, sender: Option<&AnyObject>);

        #[method(checkSpelling:)]
        pub unsafe fn checkSpelling(&self, sender: Option<&AnyObject>);

        #[method(toggleRuler:)]
        pub unsafe fn toggleRuler(&self, sender: Option<&AnyObject>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "AppKit_NSText")]
    unsafe impl NSText {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSText")]
    unsafe impl NSText {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

pub const NSEnterCharacter: c_uint = 0x0003;
pub const NSBackspaceCharacter: c_uint = 0x0008;
pub const NSTabCharacter: c_uint = 0x0009;
pub const NSNewlineCharacter: c_uint = 0x000a;
pub const NSFormFeedCharacter: c_uint = 0x000c;
pub const NSCarriageReturnCharacter: c_uint = 0x000d;
pub const NSBackTabCharacter: c_uint = 0x0019;
pub const NSDeleteCharacter: c_uint = 0x007f;
pub const NSLineSeparatorCharacter: c_uint = 0x2028;
pub const NSParagraphSeparatorCharacter: c_uint = 0x2029;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSTextMovement {
        NSTextMovementReturn = 0x10,
        NSTextMovementTab = 0x11,
        NSTextMovementBacktab = 0x12,
        NSTextMovementLeft = 0x13,
        NSTextMovementRight = 0x14,
        NSTextMovementUp = 0x15,
        NSTextMovementDown = 0x16,
        NSTextMovementCancel = 0x17,
        NSTextMovementOther = 0,
    }
);

extern_static!(NSTextDidBeginEditingNotification: &'static NSNotificationName);

extern_static!(NSTextDidEndEditingNotification: &'static NSNotificationName);

extern_static!(NSTextDidChangeNotification: &'static NSNotificationName);

extern_static!(NSTextMovementUserInfoKey: &'static NSString);

pub const NSIllegalTextMovement: c_uint = 0;
pub const NSReturnTextMovement: c_uint = 0x10;
pub const NSTabTextMovement: c_uint = 0x11;
pub const NSBacktabTextMovement: c_uint = 0x12;
pub const NSLeftTextMovement: c_uint = 0x13;
pub const NSRightTextMovement: c_uint = 0x14;
pub const NSUpTextMovement: c_uint = 0x15;
pub const NSDownTextMovement: c_uint = 0x16;
pub const NSCancelTextMovement: c_uint = 0x17;
pub const NSOtherTextMovement: c_uint = 0;

extern_protocol!(
    pub unsafe trait NSTextDelegate: NSObjectProtocol + IsMainThreadOnly {
        #[cfg(feature = "AppKit_NSText")]
        #[optional]
        #[method(textShouldBeginEditing:)]
        unsafe fn textShouldBeginEditing(&self, text_object: &NSText) -> bool;

        #[cfg(feature = "AppKit_NSText")]
        #[optional]
        #[method(textShouldEndEditing:)]
        unsafe fn textShouldEndEditing(&self, text_object: &NSText) -> bool;

        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(textDidBeginEditing:)]
        unsafe fn textDidBeginEditing(&self, notification: &NSNotification);

        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(textDidEndEditing:)]
        unsafe fn textDidEndEditing(&self, notification: &NSNotification);

        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(textDidChange:)]
        unsafe fn textDidChange(&self, notification: &NSNotification);
    }

    unsafe impl ProtocolType for dyn NSTextDelegate {}
);

#[deprecated = "Use NSWritingDirectionEmbedding instead"]
pub const NSTextWritingDirectionEmbedding: c_uint = 0 << 1;
#[deprecated = "Use NSWritingDirectionOverride instead"]
pub const NSTextWritingDirectionOverride: c_uint = 1 << 1;

extern_static!(NSLeftTextAlignment: NSTextAlignment = NSTextAlignment(NSTextAlignment::NSTextAlignmentLeft.0));

extern_static!(NSRightTextAlignment: NSTextAlignment = NSTextAlignment(NSTextAlignment::NSTextAlignmentRight.0));

extern_static!(NSCenterTextAlignment: NSTextAlignment = NSTextAlignment(NSTextAlignment::NSTextAlignmentCenter.0));

extern_static!(NSJustifiedTextAlignment: NSTextAlignment = NSTextAlignment(NSTextAlignment::NSTextAlignmentJustified.0));

extern_static!(NSNaturalTextAlignment: NSTextAlignment = NSTextAlignment(NSTextAlignment::NSTextAlignmentNatural.0));
