//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSControl;

    unsafe impl ClassType for NSControl {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSControl")]
    unsafe impl NSControl {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(
            this: Option<Allocated<Self>>,
            frameRect: NSRect,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Option<Id<Object, Shared>>;

        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&Object>);

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
        pub unsafe fn setIgnoresMultiClick(&self, ignoresMultiClick: bool);

        #[method(isContinuous)]
        pub unsafe fn isContinuous(&self) -> bool;

        #[method(setContinuous:)]
        pub unsafe fn setContinuous(&self, continuous: bool);

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[method(refusesFirstResponder)]
        pub unsafe fn refusesFirstResponder(&self) -> bool;

        #[method(setRefusesFirstResponder:)]
        pub unsafe fn setRefusesFirstResponder(&self, refusesFirstResponder: bool);

        #[method(isHighlighted)]
        pub unsafe fn isHighlighted(&self) -> bool;

        #[method(setHighlighted:)]
        pub unsafe fn setHighlighted(&self, highlighted: bool);

        #[method(controlSize)]
        pub unsafe fn controlSize(&self) -> NSControlSize;

        #[method(setControlSize:)]
        pub unsafe fn setControlSize(&self, controlSize: NSControlSize);

        #[method_id(@__retain_semantics Other formatter)]
        pub unsafe fn formatter(&self) -> Option<Id<NSFormatter, Shared>>;

        #[method(setFormatter:)]
        pub unsafe fn setFormatter(&self, formatter: Option<&NSFormatter>);

        #[method_id(@__retain_semantics Other objectValue)]
        pub unsafe fn objectValue(&self) -> Option<Id<Object, Shared>>;

        #[method(setObjectValue:)]
        pub unsafe fn setObjectValue(&self, objectValue: Option<&Object>);

        #[method_id(@__retain_semantics Other stringValue)]
        pub unsafe fn stringValue(&self) -> Id<NSString, Shared>;

        #[method(setStringValue:)]
        pub unsafe fn setStringValue(&self, stringValue: &NSString);

        #[method_id(@__retain_semantics Other attributedStringValue)]
        pub unsafe fn attributedStringValue(&self) -> Id<NSAttributedString, Shared>;

        #[method(setAttributedStringValue:)]
        pub unsafe fn setAttributedStringValue(&self, attributedStringValue: &NSAttributedString);

        #[method(intValue)]
        pub unsafe fn intValue(&self) -> c_int;

        #[method(setIntValue:)]
        pub unsafe fn setIntValue(&self, intValue: c_int);

        #[method(integerValue)]
        pub unsafe fn integerValue(&self) -> NSInteger;

        #[method(setIntegerValue:)]
        pub unsafe fn setIntegerValue(&self, integerValue: NSInteger);

        #[method(floatValue)]
        pub unsafe fn floatValue(&self) -> c_float;

        #[method(setFloatValue:)]
        pub unsafe fn setFloatValue(&self, floatValue: c_float);

        #[method(doubleValue)]
        pub unsafe fn doubleValue(&self) -> c_double;

        #[method(setDoubleValue:)]
        pub unsafe fn setDoubleValue(&self, doubleValue: c_double);

        #[method(sizeThatFits:)]
        pub unsafe fn sizeThatFits(&self, size: NSSize) -> NSSize;

        #[method(sizeToFit)]
        pub unsafe fn sizeToFit(&self);

        #[method(sendActionOn:)]
        pub unsafe fn sendActionOn(&self, mask: NSEventMask) -> NSInteger;

        #[method(sendAction:to:)]
        pub unsafe fn sendAction_to(&self, action: Option<Sel>, target: Option<&Object>) -> bool;

        #[method(takeIntValueFrom:)]
        pub unsafe fn takeIntValueFrom(&self, sender: Option<&Object>);

        #[method(takeFloatValueFrom:)]
        pub unsafe fn takeFloatValueFrom(&self, sender: Option<&Object>);

        #[method(takeDoubleValueFrom:)]
        pub unsafe fn takeDoubleValueFrom(&self, sender: Option<&Object>);

        #[method(takeStringValueFrom:)]
        pub unsafe fn takeStringValueFrom(&self, sender: Option<&Object>);

        #[method(takeObjectValueFrom:)]
        pub unsafe fn takeObjectValueFrom(&self, sender: Option<&Object>);

        #[method(takeIntegerValueFrom:)]
        pub unsafe fn takeIntegerValueFrom(&self, sender: Option<&Object>);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(mouseDown:)]
        pub unsafe fn mouseDown(&self, event: &NSEvent);

        #[method(performClick:)]
        pub unsafe fn performClick(&self, sender: Option<&Object>);

        #[cfg(feature = "AppKit_NSFont")]
        #[method_id(@__retain_semantics Other font)]
        pub unsafe fn font(&self) -> Option<Id<NSFont, Shared>>;

        #[cfg(feature = "AppKit_NSFont")]
        #[method(setFont:)]
        pub unsafe fn setFont(&self, font: Option<&NSFont>);

        #[method(usesSingleLineMode)]
        pub unsafe fn usesSingleLineMode(&self) -> bool;

        #[method(setUsesSingleLineMode:)]
        pub unsafe fn setUsesSingleLineMode(&self, usesSingleLineMode: bool);

        #[method(lineBreakMode)]
        pub unsafe fn lineBreakMode(&self) -> NSLineBreakMode;

        #[method(setLineBreakMode:)]
        pub unsafe fn setLineBreakMode(&self, lineBreakMode: NSLineBreakMode);

        #[method(alignment)]
        pub unsafe fn alignment(&self) -> NSTextAlignment;

        #[method(setAlignment:)]
        pub unsafe fn setAlignment(&self, alignment: NSTextAlignment);

        #[method(baseWritingDirection)]
        pub unsafe fn baseWritingDirection(&self) -> NSWritingDirection;

        #[method(setBaseWritingDirection:)]
        pub unsafe fn setBaseWritingDirection(&self, baseWritingDirection: NSWritingDirection);

        #[method(allowsExpansionToolTips)]
        pub unsafe fn allowsExpansionToolTips(&self) -> bool;

        #[method(setAllowsExpansionToolTips:)]
        pub unsafe fn setAllowsExpansionToolTips(&self, allowsExpansionToolTips: bool);

        #[method(expansionFrameWithFrame:)]
        pub unsafe fn expansionFrameWithFrame(&self, contentFrame: NSRect) -> NSRect;

        #[method(drawWithExpansionFrame:inView:)]
        pub unsafe fn drawWithExpansionFrame_inView(&self, contentFrame: NSRect, view: &NSView);
    }
);

extern_methods!(
    /// NSControlEditableTextMethods
    #[cfg(feature = "AppKit_NSControl")]
    unsafe impl NSControl {
        #[cfg(feature = "AppKit_NSText")]
        #[method_id(@__retain_semantics Other currentEditor)]
        pub unsafe fn currentEditor(&self) -> Option<Id<NSText, Shared>>;

        #[method(abortEditing)]
        pub unsafe fn abortEditing(&self) -> bool;

        #[method(validateEditing)]
        pub unsafe fn validateEditing(&self);

        #[cfg(all(feature = "AppKit_NSEvent", feature = "AppKit_NSText"))]
        #[method(editWithFrame:editor:delegate:event:)]
        pub unsafe fn editWithFrame_editor_delegate_event(
            &self,
            rect: NSRect,
            textObj: &NSText,
            delegate: Option<&Object>,
            event: &NSEvent,
        );

        #[cfg(feature = "AppKit_NSText")]
        #[method(selectWithFrame:editor:delegate:start:length:)]
        pub unsafe fn selectWithFrame_editor_delegate_start_length(
            &self,
            rect: NSRect,
            textObj: &NSText,
            delegate: Option<&Object>,
            selStart: NSInteger,
            selLength: NSInteger,
        );

        #[cfg(feature = "AppKit_NSText")]
        #[method(endEditing:)]
        pub unsafe fn endEditing(&self, textObj: &NSText);
    }
);

extern_protocol!(
    pub struct NSControlTextEditingDelegate;

    unsafe impl ProtocolType for NSControlTextEditingDelegate {
        #[optional]
        #[method(controlTextDidBeginEditing:)]
        pub unsafe fn controlTextDidBeginEditing(&self, obj: &NSNotification);

        #[optional]
        #[method(controlTextDidEndEditing:)]
        pub unsafe fn controlTextDidEndEditing(&self, obj: &NSNotification);

        #[optional]
        #[method(controlTextDidChange:)]
        pub unsafe fn controlTextDidChange(&self, obj: &NSNotification);

        #[optional]
        #[method(control:textShouldBeginEditing:)]
        pub unsafe fn control_textShouldBeginEditing(
            &self,
            control: &NSControl,
            fieldEditor: &NSText,
        ) -> bool;

        #[optional]
        #[method(control:textShouldEndEditing:)]
        pub unsafe fn control_textShouldEndEditing(
            &self,
            control: &NSControl,
            fieldEditor: &NSText,
        ) -> bool;

        #[optional]
        #[method(control:didFailToFormatString:errorDescription:)]
        pub unsafe fn control_didFailToFormatString_errorDescription(
            &self,
            control: &NSControl,
            string: &NSString,
            error: Option<&NSString>,
        ) -> bool;

        #[optional]
        #[method(control:didFailToValidatePartialString:errorDescription:)]
        pub unsafe fn control_didFailToValidatePartialString_errorDescription(
            &self,
            control: &NSControl,
            string: &NSString,
            error: Option<&NSString>,
        );

        #[optional]
        #[method(control:isValidObject:)]
        pub unsafe fn control_isValidObject(
            &self,
            control: &NSControl,
            obj: Option<&Object>,
        ) -> bool;

        #[optional]
        #[method(control:textView:doCommandBySelector:)]
        pub unsafe fn control_textView_doCommandBySelector(
            &self,
            control: &NSControl,
            textView: &NSTextView,
            commandSelector: Sel,
        ) -> bool;

        #[optional]
        #[method_id(@__retain_semantics Other control:textView:completions:forPartialWordRange:indexOfSelectedItem:)]
        pub unsafe fn control_textView_completions_forPartialWordRange_indexOfSelectedItem(
            &self,
            control: &NSControl,
            textView: &NSTextView,
            words: &NSArray<NSString>,
            charRange: NSRange,
            index: NonNull<NSInteger>,
        ) -> Id<NSArray<NSString>, Shared>;
    }
);

extern_static!(NSControlTextDidBeginEditingNotification: &'static NSNotificationName);

extern_static!(NSControlTextDidEndEditingNotification: &'static NSNotificationName);

extern_static!(NSControlTextDidChangeNotification: &'static NSNotificationName);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSControl")]
    unsafe impl NSControl {
        #[method(setFloatingPointFormat:left:right:)]
        pub unsafe fn setFloatingPointFormat_left_right(
            &self,
            autoRange: bool,
            leftDigits: NSUInteger,
            rightDigits: NSUInteger,
        );

        #[method(cellClass)]
        pub unsafe fn cellClass() -> Option<&'static Class>;

        #[method(setCellClass:)]
        pub unsafe fn setCellClass(cellClass: Option<&Class>);

        #[cfg(feature = "AppKit_NSCell")]
        #[method_id(@__retain_semantics Other cell)]
        pub unsafe fn cell(&self) -> Option<Id<NSCell, Shared>>;

        #[cfg(feature = "AppKit_NSCell")]
        #[method(setCell:)]
        pub unsafe fn setCell(&self, cell: Option<&NSCell>);

        #[cfg(feature = "AppKit_NSCell")]
        #[method_id(@__retain_semantics Other selectedCell)]
        pub unsafe fn selectedCell(&self) -> Option<Id<NSCell, Shared>>;

        #[method(selectedTag)]
        pub unsafe fn selectedTag(&self) -> NSInteger;

        #[method(setNeedsDisplay)]
        pub unsafe fn setNeedsDisplay(&self);

        #[method(calcSize)]
        pub unsafe fn calcSize(&self);

        #[cfg(feature = "AppKit_NSCell")]
        #[method(updateCell:)]
        pub unsafe fn updateCell(&self, cell: &NSCell);

        #[cfg(feature = "AppKit_NSCell")]
        #[method(updateCellInside:)]
        pub unsafe fn updateCellInside(&self, cell: &NSCell);

        #[cfg(feature = "AppKit_NSCell")]
        #[method(drawCellInside:)]
        pub unsafe fn drawCellInside(&self, cell: &NSCell);

        #[cfg(feature = "AppKit_NSCell")]
        #[method(drawCell:)]
        pub unsafe fn drawCell(&self, cell: &NSCell);

        #[cfg(feature = "AppKit_NSCell")]
        #[method(selectCell:)]
        pub unsafe fn selectCell(&self, cell: &NSCell);
    }
);
