//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSCellType(pub NSUInteger);
impl NSCellType {
    pub const NSNullCellType: Self = Self(0);
    pub const NSTextCellType: Self = Self(1);
    pub const NSImageCellType: Self = Self(2);
}

unsafe impl Encode for NSCellType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSCellType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSCellAttribute(pub NSUInteger);
impl NSCellAttribute {
    pub const NSCellDisabled: Self = Self(0);
    pub const NSCellState: Self = Self(1);
    pub const NSPushInCell: Self = Self(2);
    pub const NSCellEditable: Self = Self(3);
    pub const NSChangeGrayCell: Self = Self(4);
    pub const NSCellHighlighted: Self = Self(5);
    pub const NSCellLightsByContents: Self = Self(6);
    pub const NSCellLightsByGray: Self = Self(7);
    pub const NSChangeBackgroundCell: Self = Self(8);
    pub const NSCellLightsByBackground: Self = Self(9);
    pub const NSCellIsBordered: Self = Self(10);
    pub const NSCellHasOverlappingImage: Self = Self(11);
    pub const NSCellHasImageHorizontal: Self = Self(12);
    pub const NSCellHasImageOnLeftOrBottom: Self = Self(13);
    pub const NSCellChangesContents: Self = Self(14);
    pub const NSCellIsInsetButton: Self = Self(15);
    pub const NSCellAllowsMixedState: Self = Self(16);
}

unsafe impl Encode for NSCellAttribute {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSCellAttribute {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSCellImagePosition(pub NSUInteger);
impl NSCellImagePosition {
    pub const NSNoImage: Self = Self(0);
    pub const NSImageOnly: Self = Self(1);
    pub const NSImageLeft: Self = Self(2);
    pub const NSImageRight: Self = Self(3);
    pub const NSImageBelow: Self = Self(4);
    pub const NSImageAbove: Self = Self(5);
    pub const NSImageOverlaps: Self = Self(6);
    pub const NSImageLeading: Self = Self(7);
    pub const NSImageTrailing: Self = Self(8);
}

unsafe impl Encode for NSCellImagePosition {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSCellImagePosition {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSImageScaling(pub NSUInteger);
impl NSImageScaling {
    pub const NSImageScaleProportionallyDown: Self = Self(0);
    pub const NSImageScaleAxesIndependently: Self = Self(1);
    pub const NSImageScaleNone: Self = Self(2);
    pub const NSImageScaleProportionallyUpOrDown: Self = Self(3);
    #[deprecated = "Use NSImageScaleProportionallyDown instead"]
    pub const NSScaleProportionally: Self = Self(0);
    #[deprecated = "Use NSImageScaleAxesIndependently instead"]
    pub const NSScaleToFit: Self = Self(1);
    #[deprecated = "Use NSImageScaleNone instead"]
    pub const NSScaleNone: Self = Self(2);
}

unsafe impl Encode for NSImageScaling {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSImageScaling {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_TYPED_EXTENSIBLE_ENUM
pub type NSControlStateValue = NSInteger;

pub static NSControlStateValueMixed: NSControlStateValue = -1;

pub static NSControlStateValueOff: NSControlStateValue = 0;

pub static NSControlStateValueOn: NSControlStateValue = 1;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSCellStyleMask(pub NSUInteger);
bitflags::bitflags! {
    impl NSCellStyleMask: NSUInteger {
        const NSNoCellMask = 0;
        const NSContentsCellMask = 1;
        const NSPushInCellMask = 2;
        const NSChangeGrayCellMask = 4;
        const NSChangeBackgroundCellMask = 8;
    }
}

unsafe impl Encode for NSCellStyleMask {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSCellStyleMask {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSControlTint(pub NSUInteger);
impl NSControlTint {
    pub const NSDefaultControlTint: Self = Self(0);
    pub const NSBlueControlTint: Self = Self(1);
    pub const NSGraphiteControlTint: Self = Self(6);
    pub const NSClearControlTint: Self = Self(7);
}

unsafe impl Encode for NSControlTint {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSControlTint {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSControlSize(pub NSUInteger);
impl NSControlSize {
    #[doc(alias = "NSControlSizeRegular")]
    pub const Regular: Self = Self(0);
    #[doc(alias = "NSControlSizeSmall")]
    pub const Small: Self = Self(1);
    #[doc(alias = "NSControlSizeMini")]
    pub const Mini: Self = Self(2);
    #[doc(alias = "NSControlSizeLarge")]
    pub const Large: Self = Self(3);
}

unsafe impl Encode for NSControlSize {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSControlSize {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCell;

    unsafe impl ClassType for NSCell {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

#[cfg(feature = "NSAccessibilityProtocols")]
unsafe impl NSAccessibility for NSCell {}

#[cfg(feature = "NSAccessibilityProtocols")]
unsafe impl NSAccessibilityElementProtocol for NSCell {}

unsafe impl NSCoding for NSCell {}

unsafe impl NSCopying for NSCell {}

unsafe impl CopyingHelper for NSCell {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSCell {}

#[cfg(feature = "NSUserInterfaceItemIdentification")]
unsafe impl NSUserInterfaceItemIdentification for NSCell {}

extern_methods!(
    unsafe impl NSCell {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initTextCell:)]
        pub unsafe fn initTextCell(this: Allocated<Self>, string: &NSString) -> Retained<Self>;

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Init initImageCell:)]
        pub unsafe fn initImageCell(
            this: Allocated<Self>,
            image: Option<&NSImage>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Retained<Self>;

        #[method(prefersTrackingUntilMouseUp)]
        pub unsafe fn prefersTrackingUntilMouseUp(mtm: MainThreadMarker) -> bool;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method_id(@__retain_semantics Other controlView)]
        pub unsafe fn controlView(&self) -> Option<Retained<NSView>>;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method(setControlView:)]
        pub unsafe fn setControlView(&self, control_view: Option<&NSView>);

        #[method(type)]
        pub unsafe fn r#type(&self) -> NSCellType;

        #[method(setType:)]
        pub unsafe fn setType(&self, r#type: NSCellType);

        #[method(state)]
        pub unsafe fn state(&self) -> NSControlStateValue;

        #[method(setState:)]
        pub unsafe fn setState(&self, state: NSControlStateValue);

        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Option<Retained<AnyObject>>;

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

        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Retained<NSString>;

        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[method(isOpaque)]
        pub unsafe fn isOpaque(&self) -> bool;

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[cfg(feature = "NSEvent")]
        #[method(sendActionOn:)]
        pub unsafe fn sendActionOn(&self, mask: NSEventMask) -> NSInteger;

        #[method(isContinuous)]
        pub unsafe fn isContinuous(&self) -> bool;

        #[method(setContinuous:)]
        pub unsafe fn setContinuous(&self, continuous: bool);

        #[method(isEditable)]
        pub unsafe fn isEditable(&self) -> bool;

        #[method(setEditable:)]
        pub unsafe fn setEditable(&self, editable: bool);

        #[method(isSelectable)]
        pub unsafe fn isSelectable(&self) -> bool;

        #[method(setSelectable:)]
        pub unsafe fn setSelectable(&self, selectable: bool);

        #[method(isBordered)]
        pub unsafe fn isBordered(&self) -> bool;

        #[method(setBordered:)]
        pub unsafe fn setBordered(&self, bordered: bool);

        #[method(isBezeled)]
        pub unsafe fn isBezeled(&self) -> bool;

        #[method(setBezeled:)]
        pub unsafe fn setBezeled(&self, bezeled: bool);

        #[method(isScrollable)]
        pub unsafe fn isScrollable(&self) -> bool;

        #[method(setScrollable:)]
        pub unsafe fn setScrollable(&self, scrollable: bool);

        #[method(isHighlighted)]
        pub unsafe fn isHighlighted(&self) -> bool;

        #[method(setHighlighted:)]
        pub unsafe fn setHighlighted(&self, highlighted: bool);

        #[cfg(feature = "NSText")]
        #[method(alignment)]
        pub unsafe fn alignment(&self) -> NSTextAlignment;

        #[cfg(feature = "NSText")]
        #[method(setAlignment:)]
        pub unsafe fn setAlignment(&self, alignment: NSTextAlignment);

        #[method(wraps)]
        pub unsafe fn wraps(&self) -> bool;

        #[method(setWraps:)]
        pub unsafe fn setWraps(&self, wraps: bool);

        #[cfg(feature = "NSFont")]
        #[method_id(@__retain_semantics Other font)]
        pub unsafe fn font(&self) -> Option<Retained<NSFont>>;

        #[cfg(feature = "NSFont")]
        #[method(setFont:)]
        pub unsafe fn setFont(&self, font: Option<&NSFont>);

        #[method_id(@__retain_semantics Other keyEquivalent)]
        pub unsafe fn keyEquivalent(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other formatter)]
        pub unsafe fn formatter(&self) -> Option<Retained<NSFormatter>>;

        #[method(setFormatter:)]
        pub unsafe fn setFormatter(&self, formatter: Option<&NSFormatter>);

        #[method_id(@__retain_semantics Other objectValue)]
        pub unsafe fn objectValue(&self) -> Option<Retained<AnyObject>>;

        #[method(setObjectValue:)]
        pub unsafe fn setObjectValue(&self, object_value: Option<&AnyObject>);

        #[method(hasValidObjectValue)]
        pub unsafe fn hasValidObjectValue(&self) -> bool;

        #[method_id(@__retain_semantics Other stringValue)]
        pub unsafe fn stringValue(&self) -> Retained<NSString>;

        #[method(setStringValue:)]
        pub unsafe fn setStringValue(&self, string_value: &NSString);

        #[method(compare:)]
        pub unsafe fn compare(&self, other_cell: &AnyObject) -> NSComparisonResult;

        #[method(intValue)]
        pub unsafe fn intValue(&self) -> c_int;

        #[method(setIntValue:)]
        pub unsafe fn setIntValue(&self, int_value: c_int);

        #[method(floatValue)]
        pub unsafe fn floatValue(&self) -> c_float;

        #[method(setFloatValue:)]
        pub unsafe fn setFloatValue(&self, float_value: c_float);

        #[method(doubleValue)]
        pub unsafe fn doubleValue(&self) -> c_double;

        #[method(setDoubleValue:)]
        pub unsafe fn setDoubleValue(&self, double_value: c_double);

        #[method(integerValue)]
        pub unsafe fn integerValue(&self) -> NSInteger;

        #[method(setIntegerValue:)]
        pub unsafe fn setIntegerValue(&self, integer_value: NSInteger);

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

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Retained<NSImage>>;

        #[cfg(feature = "NSImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);

        #[method(controlSize)]
        pub unsafe fn controlSize(&self) -> NSControlSize;

        #[method(setControlSize:)]
        pub unsafe fn setControlSize(&self, control_size: NSControlSize);

        #[method_id(@__retain_semantics Other representedObject)]
        pub unsafe fn representedObject(&self) -> Option<Retained<AnyObject>>;

        #[method(setRepresentedObject:)]
        pub unsafe fn setRepresentedObject(&self, represented_object: Option<&AnyObject>);

        #[method(cellAttribute:)]
        pub unsafe fn cellAttribute(&self, parameter: NSCellAttribute) -> NSInteger;

        #[method(setCellAttribute:to:)]
        pub unsafe fn setCellAttribute_to(&self, parameter: NSCellAttribute, value: NSInteger);

        #[method(imageRectForBounds:)]
        pub unsafe fn imageRectForBounds(&self, rect: NSRect) -> NSRect;

        #[method(titleRectForBounds:)]
        pub unsafe fn titleRectForBounds(&self, rect: NSRect) -> NSRect;

        #[method(drawingRectForBounds:)]
        pub unsafe fn drawingRectForBounds(&self, rect: NSRect) -> NSRect;

        #[method(cellSize)]
        pub unsafe fn cellSize(&self) -> NSSize;

        #[method(cellSizeForBounds:)]
        pub unsafe fn cellSizeForBounds(&self, rect: NSRect) -> NSSize;

        #[cfg(all(feature = "NSColor", feature = "NSResponder", feature = "NSView"))]
        #[method_id(@__retain_semantics Other highlightColorWithFrame:inView:)]
        pub unsafe fn highlightColorWithFrame_inView(
            &self,
            cell_frame: NSRect,
            control_view: &NSView,
        ) -> Option<Retained<NSColor>>;

        #[method(calcDrawInfo:)]
        pub unsafe fn calcDrawInfo(&self, rect: NSRect);

        #[cfg(all(feature = "NSResponder", feature = "NSText", feature = "NSView"))]
        #[method_id(@__retain_semantics Other setUpFieldEditorAttributes:)]
        pub unsafe fn setUpFieldEditorAttributes(&self, text_obj: &NSText) -> Retained<NSText>;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method(drawInteriorWithFrame:inView:)]
        pub unsafe fn drawInteriorWithFrame_inView(
            &self,
            cell_frame: NSRect,
            control_view: &NSView,
        );

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method(drawWithFrame:inView:)]
        pub unsafe fn drawWithFrame_inView(&self, cell_frame: NSRect, control_view: &NSView);

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method(highlight:withFrame:inView:)]
        pub unsafe fn highlight_withFrame_inView(
            &self,
            flag: bool,
            cell_frame: NSRect,
            control_view: &NSView,
        );

        #[method(mouseDownFlags)]
        pub unsafe fn mouseDownFlags(&self) -> NSInteger;

        #[method(getPeriodicDelay:interval:)]
        pub unsafe fn getPeriodicDelay_interval(
            &self,
            delay: NonNull<c_float>,
            interval: NonNull<c_float>,
        );

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method(startTrackingAt:inView:)]
        pub unsafe fn startTrackingAt_inView(
            &self,
            start_point: NSPoint,
            control_view: &NSView,
        ) -> bool;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method(continueTracking:at:inView:)]
        pub unsafe fn continueTracking_at_inView(
            &self,
            last_point: NSPoint,
            current_point: NSPoint,
            control_view: &NSView,
        ) -> bool;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method(stopTracking:at:inView:mouseIsUp:)]
        pub unsafe fn stopTracking_at_inView_mouseIsUp(
            &self,
            last_point: NSPoint,
            stop_point: NSPoint,
            control_view: &NSView,
            flag: bool,
        );

        #[cfg(all(feature = "NSEvent", feature = "NSResponder", feature = "NSView"))]
        #[method(trackMouse:inRect:ofView:untilMouseUp:)]
        pub unsafe fn trackMouse_inRect_ofView_untilMouseUp(
            &self,
            event: &NSEvent,
            cell_frame: NSRect,
            control_view: &NSView,
            flag: bool,
        ) -> bool;

        #[cfg(all(
            feature = "NSEvent",
            feature = "NSResponder",
            feature = "NSText",
            feature = "NSView"
        ))]
        #[method(editWithFrame:inView:editor:delegate:event:)]
        pub unsafe fn editWithFrame_inView_editor_delegate_event(
            &self,
            rect: NSRect,
            control_view: &NSView,
            text_obj: &NSText,
            delegate: Option<&AnyObject>,
            event: Option<&NSEvent>,
        );

        #[cfg(all(feature = "NSResponder", feature = "NSText", feature = "NSView"))]
        #[method(selectWithFrame:inView:editor:delegate:start:length:)]
        pub unsafe fn selectWithFrame_inView_editor_delegate_start_length(
            &self,
            rect: NSRect,
            control_view: &NSView,
            text_obj: &NSText,
            delegate: Option<&AnyObject>,
            sel_start: NSInteger,
            sel_length: NSInteger,
        );

        #[cfg(all(feature = "NSResponder", feature = "NSText", feature = "NSView"))]
        #[method(endEditing:)]
        pub unsafe fn endEditing(&self, text_obj: &NSText);

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method(resetCursorRect:inView:)]
        pub unsafe fn resetCursorRect_inView(&self, cell_frame: NSRect, control_view: &NSView);

        #[cfg(feature = "NSMenu")]
        #[method_id(@__retain_semantics Other menu)]
        pub unsafe fn menu(&self) -> Option<Retained<NSMenu>>;

        #[cfg(feature = "NSMenu")]
        #[method(setMenu:)]
        pub unsafe fn setMenu(&self, menu: Option<&NSMenu>);

        #[cfg(all(
            feature = "NSEvent",
            feature = "NSMenu",
            feature = "NSResponder",
            feature = "NSView"
        ))]
        #[method_id(@__retain_semantics Other menuForEvent:inRect:ofView:)]
        pub unsafe fn menuForEvent_inRect_ofView(
            &self,
            event: &NSEvent,
            cell_frame: NSRect,
            view: &NSView,
        ) -> Option<Retained<NSMenu>>;

        #[cfg(feature = "NSMenu")]
        #[method_id(@__retain_semantics Other defaultMenu)]
        pub unsafe fn defaultMenu(mtm: MainThreadMarker) -> Option<Retained<NSMenu>>;

        #[method(sendsActionOnEndEditing)]
        pub unsafe fn sendsActionOnEndEditing(&self) -> bool;

        #[method(setSendsActionOnEndEditing:)]
        pub unsafe fn setSendsActionOnEndEditing(&self, sends_action_on_end_editing: bool);

        #[cfg(feature = "NSText")]
        #[method(baseWritingDirection)]
        pub unsafe fn baseWritingDirection(&self) -> NSWritingDirection;

        #[cfg(feature = "NSText")]
        #[method(setBaseWritingDirection:)]
        pub unsafe fn setBaseWritingDirection(&self, base_writing_direction: NSWritingDirection);

        #[cfg(feature = "NSParagraphStyle")]
        #[method(lineBreakMode)]
        pub unsafe fn lineBreakMode(&self) -> NSLineBreakMode;

        #[cfg(feature = "NSParagraphStyle")]
        #[method(setLineBreakMode:)]
        pub unsafe fn setLineBreakMode(&self, line_break_mode: NSLineBreakMode);

        #[method(allowsUndo)]
        pub unsafe fn allowsUndo(&self) -> bool;

        #[method(setAllowsUndo:)]
        pub unsafe fn setAllowsUndo(&self, allows_undo: bool);

        #[method(truncatesLastVisibleLine)]
        pub unsafe fn truncatesLastVisibleLine(&self) -> bool;

        #[method(setTruncatesLastVisibleLine:)]
        pub unsafe fn setTruncatesLastVisibleLine(&self, truncates_last_visible_line: bool);

        #[cfg(feature = "NSUserInterfaceLayout")]
        #[method(userInterfaceLayoutDirection)]
        pub unsafe fn userInterfaceLayoutDirection(&self) -> NSUserInterfaceLayoutDirection;

        #[cfg(feature = "NSUserInterfaceLayout")]
        #[method(setUserInterfaceLayoutDirection:)]
        pub unsafe fn setUserInterfaceLayoutDirection(
            &self,
            user_interface_layout_direction: NSUserInterfaceLayoutDirection,
        );

        #[cfg(all(
            feature = "NSResponder",
            feature = "NSText",
            feature = "NSTextView",
            feature = "NSView"
        ))]
        #[method_id(@__retain_semantics Other fieldEditorForView:)]
        pub unsafe fn fieldEditorForView(
            &self,
            control_view: &NSView,
        ) -> Option<Retained<NSTextView>>;

        #[method(usesSingleLineMode)]
        pub unsafe fn usesSingleLineMode(&self) -> bool;

        #[method(setUsesSingleLineMode:)]
        pub unsafe fn setUsesSingleLineMode(&self, uses_single_line_mode: bool);

        #[cfg(all(
            feature = "NSDraggingItem",
            feature = "NSResponder",
            feature = "NSView"
        ))]
        #[method_id(@__retain_semantics Other draggingImageComponentsWithFrame:inView:)]
        pub unsafe fn draggingImageComponentsWithFrame_inView(
            &self,
            frame: NSRect,
            view: &NSView,
        ) -> Retained<NSArray<NSDraggingImageComponent>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSCell {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    /// NSKeyboardUI
    unsafe impl NSCell {
        #[method(refusesFirstResponder)]
        pub unsafe fn refusesFirstResponder(&self) -> bool;

        #[method(setRefusesFirstResponder:)]
        pub unsafe fn setRefusesFirstResponder(&self, refuses_first_responder: bool);

        #[method(acceptsFirstResponder)]
        pub unsafe fn acceptsFirstResponder(&self) -> bool;

        #[method(showsFirstResponder)]
        pub unsafe fn showsFirstResponder(&self) -> bool;

        #[method(setShowsFirstResponder:)]
        pub unsafe fn setShowsFirstResponder(&self, shows_first_responder: bool);

        #[method(performClick:)]
        pub unsafe fn performClick(&self, sender: Option<&AnyObject>);

        #[cfg(feature = "NSGraphics")]
        #[method(focusRingType)]
        pub unsafe fn focusRingType(&self) -> NSFocusRingType;

        #[cfg(feature = "NSGraphics")]
        #[method(setFocusRingType:)]
        pub unsafe fn setFocusRingType(&self, focus_ring_type: NSFocusRingType);

        #[cfg(feature = "NSGraphics")]
        #[method(defaultFocusRingType)]
        pub unsafe fn defaultFocusRingType(mtm: MainThreadMarker) -> NSFocusRingType;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method(drawFocusRingMaskWithFrame:inView:)]
        pub unsafe fn drawFocusRingMaskWithFrame_inView(
            &self,
            cell_frame: NSRect,
            control_view: &NSView,
        );

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method(focusRingMaskBoundsForFrame:inView:)]
        pub unsafe fn focusRingMaskBoundsForFrame_inView(
            &self,
            cell_frame: NSRect,
            control_view: &NSView,
        ) -> NSRect;

        #[method(wantsNotificationForMarkedText)]
        pub unsafe fn wantsNotificationForMarkedText(&self) -> bool;
    }
);

extern_methods!(
    /// NSCellAttributedStringMethods
    unsafe impl NSCell {
        #[method_id(@__retain_semantics Other attributedStringValue)]
        pub unsafe fn attributedStringValue(&self) -> Retained<NSAttributedString>;

        #[method(setAttributedStringValue:)]
        pub unsafe fn setAttributedStringValue(&self, attributed_string_value: &NSAttributedString);

        #[method(allowsEditingTextAttributes)]
        pub unsafe fn allowsEditingTextAttributes(&self) -> bool;

        #[method(setAllowsEditingTextAttributes:)]
        pub unsafe fn setAllowsEditingTextAttributes(&self, allows_editing_text_attributes: bool);

        #[method(importsGraphics)]
        pub unsafe fn importsGraphics(&self) -> bool;

        #[method(setImportsGraphics:)]
        pub unsafe fn setImportsGraphics(&self, imports_graphics: bool);
    }
);

extern_methods!(
    /// NSCellMixedState
    unsafe impl NSCell {
        #[method(allowsMixedState)]
        pub unsafe fn allowsMixedState(&self) -> bool;

        #[method(setAllowsMixedState:)]
        pub unsafe fn setAllowsMixedState(&self, allows_mixed_state: bool);

        #[method(nextState)]
        pub unsafe fn nextState(&self) -> NSInteger;

        #[method(setNextState)]
        pub unsafe fn setNextState(&self);
    }
);

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSCellHitResult(pub NSUInteger);
bitflags::bitflags! {
    impl NSCellHitResult: NSUInteger {
        const NSCellHitNone = 0;
        const NSCellHitContentArea = 1<<0;
        const NSCellHitEditableTextArea = 1<<1;
        const NSCellHitTrackableArea = 1<<2;
    }
}

unsafe impl Encode for NSCellHitResult {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSCellHitResult {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// NSCellHitTest
    unsafe impl NSCell {
        #[cfg(all(feature = "NSEvent", feature = "NSResponder", feature = "NSView"))]
        #[method(hitTestForEvent:inRect:ofView:)]
        pub unsafe fn hitTestForEvent_inRect_ofView(
            &self,
            event: &NSEvent,
            cell_frame: NSRect,
            control_view: &NSView,
        ) -> NSCellHitResult;
    }
);

extern_methods!(
    /// NSCellExpansion
    unsafe impl NSCell {
        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method(expansionFrameWithFrame:inView:)]
        pub unsafe fn expansionFrameWithFrame_inView(
            &self,
            cell_frame: NSRect,
            view: &NSView,
        ) -> NSRect;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method(drawWithExpansionFrame:inView:)]
        pub unsafe fn drawWithExpansionFrame_inView(&self, cell_frame: NSRect, view: &NSView);
    }
);

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSBackgroundStyle(pub NSInteger);
impl NSBackgroundStyle {
    #[doc(alias = "NSBackgroundStyleNormal")]
    pub const Normal: Self = Self(0);
    #[doc(alias = "NSBackgroundStyleEmphasized")]
    pub const Emphasized: Self = Self(1);
    #[doc(alias = "NSBackgroundStyleRaised")]
    pub const Raised: Self = Self(2);
    #[doc(alias = "NSBackgroundStyleLowered")]
    pub const Lowered: Self = Self(3);
}

unsafe impl Encode for NSBackgroundStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSBackgroundStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// NSCellBackgroundStyle
    unsafe impl NSCell {
        #[method(backgroundStyle)]
        pub unsafe fn backgroundStyle(&self) -> NSBackgroundStyle;

        #[method(setBackgroundStyle:)]
        pub unsafe fn setBackgroundStyle(&self, background_style: NSBackgroundStyle);

        #[method(interiorBackgroundStyle)]
        pub unsafe fn interiorBackgroundStyle(&self) -> NSBackgroundStyle;
    }
);

extern "C" {
    #[cfg(all(feature = "NSGraphics", feature = "NSImage"))]
    pub fn NSDrawThreePartImage(
        frame: NSRect,
        start_cap: Option<&NSImage>,
        center_fill: Option<&NSImage>,
        end_cap: Option<&NSImage>,
        vertical: Bool,
        op: NSCompositingOperation,
        alpha_fraction: CGFloat,
        flipped: Bool,
    );
}

extern "C" {
    #[cfg(all(feature = "NSGraphics", feature = "NSImage"))]
    pub fn NSDrawNinePartImage(
        frame: NSRect,
        top_left_corner: Option<&NSImage>,
        top_edge_fill: Option<&NSImage>,
        top_right_corner: Option<&NSImage>,
        left_edge_fill: Option<&NSImage>,
        center_fill: Option<&NSImage>,
        right_edge_fill: Option<&NSImage>,
        bottom_left_corner: Option<&NSImage>,
        bottom_edge_fill: Option<&NSImage>,
        bottom_right_corner: Option<&NSImage>,
        op: NSCompositingOperation,
        alpha_fraction: CGFloat,
        flipped: Bool,
    );
}

extern_methods!(
    /// NSDeprecated
    unsafe impl NSCell {
        #[deprecated = "The controlTint property is not respected on 10.14 and later. For custom cells, use +[NSColor controlAccentColor] to respect the user's preferred accent color when drawing."]
        #[method(controlTint)]
        pub unsafe fn controlTint(&self) -> NSControlTint;

        #[deprecated = "The controlTint property is not respected on 10.14 and later. For custom cells, use +[NSColor controlAccentColor] to respect the user's preferred accent color when drawing."]
        #[method(setControlTint:)]
        pub unsafe fn setControlTint(&self, control_tint: NSControlTint);

        #[deprecated]
        #[method(entryType)]
        pub unsafe fn entryType(&self) -> NSInteger;

        #[deprecated]
        #[method(setEntryType:)]
        pub unsafe fn setEntryType(&self, r#type: NSInteger);

        #[deprecated]
        #[method(isEntryAcceptable:)]
        pub unsafe fn isEntryAcceptable(&self, string: &NSString) -> bool;

        #[deprecated]
        #[method(setFloatingPointFormat:left:right:)]
        pub unsafe fn setFloatingPointFormat_left_right(
            &self,
            auto_range: bool,
            left_digits: NSUInteger,
            right_digits: NSUInteger,
        );

        #[deprecated]
        #[method(setMnemonicLocation:)]
        pub unsafe fn setMnemonicLocation(&self, location: NSUInteger);

        #[deprecated]
        #[method(mnemonicLocation)]
        pub unsafe fn mnemonicLocation(&self) -> NSUInteger;

        #[deprecated]
        #[method_id(@__retain_semantics Other mnemonic)]
        pub unsafe fn mnemonic(&self) -> Retained<NSString>;

        #[deprecated]
        #[method(setTitleWithMnemonic:)]
        pub unsafe fn setTitleWithMnemonic(&self, string_with_ampersand: &NSString);
    }
);

pub static NSBackgroundStyleLight: NSBackgroundStyle =
    NSBackgroundStyle(NSBackgroundStyle::Normal.0);

pub static NSBackgroundStyleDark: NSBackgroundStyle =
    NSBackgroundStyle(NSBackgroundStyle::Emphasized.0);

pub type NSCellStateValue = NSControlStateValue;

pub static NSMixedState: NSControlStateValue = NSControlStateValueMixed;

pub static NSOffState: NSControlStateValue = NSControlStateValueOff;

pub static NSOnState: NSControlStateValue = NSControlStateValueOn;

pub static NSRegularControlSize: NSControlSize = NSControlSize(NSControlSize::Regular.0);

pub static NSSmallControlSize: NSControlSize = NSControlSize(NSControlSize::Small.0);

pub static NSMiniControlSize: NSControlSize = NSControlSize(NSControlSize::Mini.0);

extern "C" {
    pub static NSControlTintDidChangeNotification: &'static NSNotificationName;
}

#[deprecated = "Use formatters instead"]
pub const NSAnyType: c_uint = 0;
#[deprecated = "Use formatters instead"]
pub const NSIntType: c_uint = 1;
#[deprecated = "Use formatters instead"]
pub const NSPositiveIntType: c_uint = 2;
#[deprecated = "Use formatters instead"]
pub const NSFloatType: c_uint = 3;
#[deprecated = "Use formatters instead"]
pub const NSPositiveFloatType: c_uint = 4;
#[deprecated = "Use formatters instead"]
pub const NSDoubleType: c_uint = 6;
#[deprecated = "Use formatters instead"]
pub const NSPositiveDoubleType: c_uint = 7;
