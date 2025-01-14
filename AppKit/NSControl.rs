//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscontrol?language=objc)
    #[unsafe(super(NSView, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    pub struct NSControl;
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibility for NSControl {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibilityElementProtocol for NSControl {}

#[cfg(all(feature = "NSAnimation", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSAnimatablePropertyContainer for NSControl {}

#[cfg(all(feature = "NSAppearance", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSAppearanceCustomization for NSControl {}

#[cfg(all(feature = "NSResponder", feature = "NSView"))]
unsafe impl NSCoding for NSControl {}

#[cfg(all(feature = "NSDragging", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSDraggingDestination for NSControl {}

#[cfg(all(feature = "NSResponder", feature = "NSView"))]
unsafe impl NSObjectProtocol for NSControl {}

#[cfg(all(
    feature = "NSResponder",
    feature = "NSUserInterfaceItemIdentification",
    feature = "NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for NSControl {}

extern_methods!(
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSControl {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Option<Retained<AnyObject>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&AnyObject>);

        #[method(action)]
        pub unsafe fn action(&self) -> Option<Sel>;

        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: Option<Sel>);

        #[method(tag)]
        pub unsafe fn tag(&self) -> NSInteger;

        #[method(setTag:)]
        pub unsafe fn setTag(&self, tag: NSInteger);

        #[method(ignoresMultiClick)]
        pub unsafe fn ignoresMultiClick(&self) -> bool;

        #[method(setIgnoresMultiClick:)]
        pub unsafe fn setIgnoresMultiClick(&self, ignores_multi_click: bool);

        #[method(isContinuous)]
        pub unsafe fn isContinuous(&self) -> bool;

        #[method(setContinuous:)]
        pub unsafe fn setContinuous(&self, continuous: bool);

        #[method(isEnabled)]
        pub fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub fn setEnabled(&self, enabled: bool);

        #[method(refusesFirstResponder)]
        pub unsafe fn refusesFirstResponder(&self) -> bool;

        #[method(setRefusesFirstResponder:)]
        pub unsafe fn setRefusesFirstResponder(&self, refuses_first_responder: bool);

        #[method(isHighlighted)]
        pub unsafe fn isHighlighted(&self) -> bool;

        #[method(setHighlighted:)]
        pub unsafe fn setHighlighted(&self, highlighted: bool);

        #[cfg(feature = "NSCell")]
        #[method(controlSize)]
        pub unsafe fn controlSize(&self) -> NSControlSize;

        #[cfg(feature = "NSCell")]
        #[method(setControlSize:)]
        pub unsafe fn setControlSize(&self, control_size: NSControlSize);

        #[method_id(@__retain_semantics Other formatter)]
        pub unsafe fn formatter(&self) -> Option<Retained<NSFormatter>>;

        #[method(setFormatter:)]
        pub unsafe fn setFormatter(&self, formatter: Option<&NSFormatter>);

        #[method_id(@__retain_semantics Other objectValue)]
        pub unsafe fn objectValue(&self) -> Option<Retained<AnyObject>>;

        #[method(setObjectValue:)]
        pub unsafe fn setObjectValue(&self, object_value: Option<&AnyObject>);

        #[method_id(@__retain_semantics Other stringValue)]
        pub unsafe fn stringValue(&self) -> Retained<NSString>;

        #[method(setStringValue:)]
        pub unsafe fn setStringValue(&self, string_value: &NSString);

        #[method_id(@__retain_semantics Other attributedStringValue)]
        pub unsafe fn attributedStringValue(&self) -> Retained<NSAttributedString>;

        #[method(setAttributedStringValue:)]
        pub unsafe fn setAttributedStringValue(&self, attributed_string_value: &NSAttributedString);

        #[method(intValue)]
        pub unsafe fn intValue(&self) -> c_int;

        #[method(setIntValue:)]
        pub unsafe fn setIntValue(&self, int_value: c_int);

        #[method(integerValue)]
        pub unsafe fn integerValue(&self) -> NSInteger;

        #[method(setIntegerValue:)]
        pub unsafe fn setIntegerValue(&self, integer_value: NSInteger);

        #[method(floatValue)]
        pub unsafe fn floatValue(&self) -> c_float;

        #[method(setFloatValue:)]
        pub unsafe fn setFloatValue(&self, float_value: c_float);

        #[method(doubleValue)]
        pub unsafe fn doubleValue(&self) -> c_double;

        #[method(setDoubleValue:)]
        pub unsafe fn setDoubleValue(&self, double_value: c_double);

        #[method(sizeThatFits:)]
        pub unsafe fn sizeThatFits(&self, size: NSSize) -> NSSize;

        #[method(sizeToFit)]
        pub unsafe fn sizeToFit(&self);

        #[cfg(feature = "NSEvent")]
        #[method(sendActionOn:)]
        pub unsafe fn sendActionOn(&self, mask: NSEventMask) -> NSInteger;

        #[method(sendAction:to:)]
        pub unsafe fn sendAction_to(&self, action: Option<Sel>, target: Option<&AnyObject>)
            -> bool;

        #[method(takeIntValueFrom:)]
        pub unsafe fn takeIntValueFrom(&self, sender: Option<&AnyObject>);

        #[method(takeFloatValueFrom:)]
        pub unsafe fn takeFloatValueFrom(&self, sender: Option<&AnyObject>);

        #[method(takeDoubleValueFrom:)]
        pub unsafe fn takeDoubleValueFrom(&self, sender: Option<&AnyObject>);

        #[method(takeStringValueFrom:)]
        pub unsafe fn takeStringValueFrom(&self, sender: Option<&AnyObject>);

        #[method(takeObjectValueFrom:)]
        pub unsafe fn takeObjectValueFrom(&self, sender: Option<&AnyObject>);

        #[method(takeIntegerValueFrom:)]
        pub unsafe fn takeIntegerValueFrom(&self, sender: Option<&AnyObject>);

        #[cfg(feature = "NSEvent")]
        #[method(mouseDown:)]
        pub unsafe fn mouseDown(&self, event: &NSEvent);

        #[method(performClick:)]
        pub unsafe fn performClick(&self, sender: Option<&AnyObject>);

        #[cfg(feature = "NSFont")]
        #[method_id(@__retain_semantics Other font)]
        pub unsafe fn font(&self) -> Option<Retained<NSFont>>;

        #[cfg(feature = "NSFont")]
        #[method(setFont:)]
        pub unsafe fn setFont(&self, font: Option<&NSFont>);

        #[method(usesSingleLineMode)]
        pub unsafe fn usesSingleLineMode(&self) -> bool;

        #[method(setUsesSingleLineMode:)]
        pub unsafe fn setUsesSingleLineMode(&self, uses_single_line_mode: bool);

        #[cfg(feature = "NSParagraphStyle")]
        #[method(lineBreakMode)]
        pub unsafe fn lineBreakMode(&self) -> NSLineBreakMode;

        #[cfg(feature = "NSParagraphStyle")]
        #[method(setLineBreakMode:)]
        pub unsafe fn setLineBreakMode(&self, line_break_mode: NSLineBreakMode);

        #[cfg(feature = "NSText")]
        #[method(alignment)]
        pub unsafe fn alignment(&self) -> NSTextAlignment;

        #[cfg(feature = "NSText")]
        #[method(setAlignment:)]
        pub unsafe fn setAlignment(&self, alignment: NSTextAlignment);

        #[cfg(feature = "NSText")]
        #[method(baseWritingDirection)]
        pub unsafe fn baseWritingDirection(&self) -> NSWritingDirection;

        #[cfg(feature = "NSText")]
        #[method(setBaseWritingDirection:)]
        pub unsafe fn setBaseWritingDirection(&self, base_writing_direction: NSWritingDirection);

        #[method(allowsExpansionToolTips)]
        pub unsafe fn allowsExpansionToolTips(&self) -> bool;

        #[method(setAllowsExpansionToolTips:)]
        pub unsafe fn setAllowsExpansionToolTips(&self, allows_expansion_tool_tips: bool);

        #[method(expansionFrameWithFrame:)]
        pub unsafe fn expansionFrameWithFrame(&self, content_frame: NSRect) -> NSRect;

        #[method(drawWithExpansionFrame:inView:)]
        pub unsafe fn drawWithExpansionFrame_inView(&self, content_frame: NSRect, view: &NSView);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSControl {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSControl {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    /// NSControlEditableTextMethods
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSControl {
        #[cfg(feature = "NSText")]
        #[method_id(@__retain_semantics Other currentEditor)]
        pub unsafe fn currentEditor(&self) -> Option<Retained<NSText>>;

        #[method(abortEditing)]
        pub unsafe fn abortEditing(&self) -> bool;

        #[method(validateEditing)]
        pub unsafe fn validateEditing(&self);

        #[cfg(all(feature = "NSEvent", feature = "NSText"))]
        #[method(editWithFrame:editor:delegate:event:)]
        pub unsafe fn editWithFrame_editor_delegate_event(
            &self,
            rect: NSRect,
            text_obj: &NSText,
            delegate: Option<&AnyObject>,
            event: &NSEvent,
        );

        #[cfg(feature = "NSText")]
        #[method(selectWithFrame:editor:delegate:start:length:)]
        pub unsafe fn selectWithFrame_editor_delegate_start_length(
            &self,
            rect: NSRect,
            text_obj: &NSText,
            delegate: Option<&AnyObject>,
            sel_start: NSInteger,
            sel_length: NSInteger,
        );

        #[cfg(feature = "NSText")]
        #[method(endEditing:)]
        pub unsafe fn endEditing(&self, text_obj: &NSText);
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscontroltexteditingdelegate?language=objc)
    pub unsafe trait NSControlTextEditingDelegate:
        NSObjectProtocol + MainThreadOnly
    {
        #[optional]
        #[method(controlTextDidBeginEditing:)]
        unsafe fn controlTextDidBeginEditing(&self, obj: &NSNotification);

        #[optional]
        #[method(controlTextDidEndEditing:)]
        unsafe fn controlTextDidEndEditing(&self, obj: &NSNotification);

        #[optional]
        #[method(controlTextDidChange:)]
        unsafe fn controlTextDidChange(&self, obj: &NSNotification);

        #[cfg(all(feature = "NSResponder", feature = "NSText", feature = "NSView"))]
        #[optional]
        #[method(control:textShouldBeginEditing:)]
        unsafe fn control_textShouldBeginEditing(
            &self,
            control: &NSControl,
            field_editor: &NSText,
        ) -> bool;

        #[cfg(all(feature = "NSResponder", feature = "NSText", feature = "NSView"))]
        #[optional]
        #[method(control:textShouldEndEditing:)]
        unsafe fn control_textShouldEndEditing(
            &self,
            control: &NSControl,
            field_editor: &NSText,
        ) -> bool;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[optional]
        #[method(control:didFailToFormatString:errorDescription:)]
        unsafe fn control_didFailToFormatString_errorDescription(
            &self,
            control: &NSControl,
            string: &NSString,
            error: Option<&NSString>,
        ) -> bool;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[optional]
        #[method(control:didFailToValidatePartialString:errorDescription:)]
        unsafe fn control_didFailToValidatePartialString_errorDescription(
            &self,
            control: &NSControl,
            string: &NSString,
            error: Option<&NSString>,
        );

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[optional]
        #[method(control:isValidObject:)]
        unsafe fn control_isValidObject(
            &self,
            control: &NSControl,
            obj: Option<&AnyObject>,
        ) -> bool;

        #[cfg(all(
            feature = "NSResponder",
            feature = "NSText",
            feature = "NSTextView",
            feature = "NSView"
        ))]
        #[optional]
        #[method(control:textView:doCommandBySelector:)]
        unsafe fn control_textView_doCommandBySelector(
            &self,
            control: &NSControl,
            text_view: &NSTextView,
            command_selector: Sel,
        ) -> bool;

        #[cfg(all(
            feature = "NSResponder",
            feature = "NSText",
            feature = "NSTextView",
            feature = "NSView"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other control:textView:completions:forPartialWordRange:indexOfSelectedItem:)]
        unsafe fn control_textView_completions_forPartialWordRange_indexOfSelectedItem(
            &self,
            control: &NSControl,
            text_view: &NSTextView,
            words: &NSArray<NSString>,
            char_range: NSRange,
            index: NonNull<NSInteger>,
        ) -> Retained<NSArray<NSString>>;
    }

    unsafe impl ProtocolType for dyn NSControlTextEditingDelegate {}
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscontroltextdidbegineditingnotification?language=objc)
    pub static NSControlTextDidBeginEditingNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscontroltextdidendeditingnotification?language=objc)
    pub static NSControlTextDidEndEditingNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscontroltextdidchangenotification?language=objc)
    pub static NSControlTextDidChangeNotification: &'static NSNotificationName;
}

extern_methods!(
    /// NSDeprecated
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSControl {
        #[deprecated]
        #[method(setFloatingPointFormat:left:right:)]
        pub unsafe fn setFloatingPointFormat_left_right(
            &self,
            auto_range: bool,
            left_digits: NSUInteger,
            right_digits: NSUInteger,
        );

        #[method(cellClass)]
        pub unsafe fn cellClass(mtm: MainThreadMarker) -> Option<&'static AnyClass>;

        #[method(setCellClass:)]
        pub unsafe fn setCellClass(cell_class: Option<&AnyClass>, mtm: MainThreadMarker);

        #[cfg(feature = "NSCell")]
        #[method_id(@__retain_semantics Other cell)]
        pub unsafe fn cell(&self) -> Option<Retained<NSCell>>;

        #[cfg(feature = "NSCell")]
        #[method(setCell:)]
        pub unsafe fn setCell(&self, cell: Option<&NSCell>);

        #[cfg(feature = "NSCell")]
        #[method_id(@__retain_semantics Other selectedCell)]
        pub unsafe fn selectedCell(&self) -> Option<Retained<NSCell>>;

        #[method(selectedTag)]
        pub unsafe fn selectedTag(&self) -> NSInteger;

        #[deprecated = "Set the needsDisplay property to YES instead"]
        #[method(setNeedsDisplay)]
        pub unsafe fn setNeedsDisplay(&self);

        #[deprecated = "Override -layout instead. This method should never be called"]
        #[method(calcSize)]
        pub unsafe fn calcSize(&self);

        #[cfg(feature = "NSCell")]
        #[method(updateCell:)]
        pub unsafe fn updateCell(&self, cell: &NSCell);

        #[cfg(feature = "NSCell")]
        #[method(updateCellInside:)]
        pub unsafe fn updateCellInside(&self, cell: &NSCell);

        #[cfg(feature = "NSCell")]
        #[method(drawCellInside:)]
        pub unsafe fn drawCellInside(&self, cell: &NSCell);

        #[cfg(feature = "NSCell")]
        #[method(drawCell:)]
        pub unsafe fn drawCell(&self, cell: &NSCell);

        #[cfg(feature = "NSCell")]
        #[method(selectCell:)]
        pub unsafe fn selectCell(&self, cell: &NSCell);
    }
);
