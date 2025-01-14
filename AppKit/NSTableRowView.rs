//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstablerowview?language=objc)
    #[unsafe(super(NSView, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    pub struct NSTableRowView;
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibility for NSTableRowView {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibilityElementProtocol for NSTableRowView {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibilityGroup for NSTableRowView {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibilityRow for NSTableRowView {}

#[cfg(all(feature = "NSAnimation", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSAnimatablePropertyContainer for NSTableRowView {}

#[cfg(all(feature = "NSAppearance", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSAppearanceCustomization for NSTableRowView {}

#[cfg(all(feature = "NSResponder", feature = "NSView"))]
unsafe impl NSCoding for NSTableRowView {}

#[cfg(all(feature = "NSDragging", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSDraggingDestination for NSTableRowView {}

#[cfg(all(feature = "NSResponder", feature = "NSView"))]
unsafe impl NSObjectProtocol for NSTableRowView {}

#[cfg(all(
    feature = "NSResponder",
    feature = "NSUserInterfaceItemIdentification",
    feature = "NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for NSTableRowView {}

extern_methods!(
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSTableRowView {
        #[cfg(feature = "NSTableView")]
        #[method(selectionHighlightStyle)]
        pub unsafe fn selectionHighlightStyle(&self) -> NSTableViewSelectionHighlightStyle;

        #[cfg(feature = "NSTableView")]
        #[method(setSelectionHighlightStyle:)]
        pub unsafe fn setSelectionHighlightStyle(
            &self,
            selection_highlight_style: NSTableViewSelectionHighlightStyle,
        );

        #[method(isEmphasized)]
        pub unsafe fn isEmphasized(&self) -> bool;

        #[method(setEmphasized:)]
        pub unsafe fn setEmphasized(&self, emphasized: bool);

        #[method(isGroupRowStyle)]
        pub unsafe fn isGroupRowStyle(&self) -> bool;

        #[method(setGroupRowStyle:)]
        pub unsafe fn setGroupRowStyle(&self, group_row_style: bool);

        #[method(isSelected)]
        pub unsafe fn isSelected(&self) -> bool;

        #[method(setSelected:)]
        pub unsafe fn setSelected(&self, selected: bool);

        #[method(isPreviousRowSelected)]
        pub unsafe fn isPreviousRowSelected(&self) -> bool;

        #[method(setPreviousRowSelected:)]
        pub unsafe fn setPreviousRowSelected(&self, previous_row_selected: bool);

        #[method(isNextRowSelected)]
        pub unsafe fn isNextRowSelected(&self) -> bool;

        #[method(setNextRowSelected:)]
        pub unsafe fn setNextRowSelected(&self, next_row_selected: bool);

        #[method(isFloating)]
        pub unsafe fn isFloating(&self) -> bool;

        #[method(setFloating:)]
        pub unsafe fn setFloating(&self, floating: bool);

        #[method(isTargetForDropOperation)]
        pub unsafe fn isTargetForDropOperation(&self) -> bool;

        #[method(setTargetForDropOperation:)]
        pub unsafe fn setTargetForDropOperation(&self, target_for_drop_operation: bool);

        #[cfg(feature = "NSTableView")]
        #[method(draggingDestinationFeedbackStyle)]
        pub unsafe fn draggingDestinationFeedbackStyle(
            &self,
        ) -> NSTableViewDraggingDestinationFeedbackStyle;

        #[cfg(feature = "NSTableView")]
        #[method(setDraggingDestinationFeedbackStyle:)]
        pub unsafe fn setDraggingDestinationFeedbackStyle(
            &self,
            dragging_destination_feedback_style: NSTableViewDraggingDestinationFeedbackStyle,
        );

        #[method(indentationForDropOperation)]
        pub unsafe fn indentationForDropOperation(&self) -> CGFloat;

        #[method(setIndentationForDropOperation:)]
        pub unsafe fn setIndentationForDropOperation(
            &self,
            indentation_for_drop_operation: CGFloat,
        );

        #[cfg(feature = "NSCell")]
        #[method(interiorBackgroundStyle)]
        pub unsafe fn interiorBackgroundStyle(&self) -> NSBackgroundStyle;

        #[cfg(feature = "NSColor")]
        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Retained<NSColor>;

        #[cfg(feature = "NSColor")]
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, background_color: &NSColor);

        #[method(drawBackgroundInRect:)]
        pub unsafe fn drawBackgroundInRect(&self, dirty_rect: NSRect);

        #[method(drawSelectionInRect:)]
        pub unsafe fn drawSelectionInRect(&self, dirty_rect: NSRect);

        #[method(drawSeparatorInRect:)]
        pub unsafe fn drawSeparatorInRect(&self, dirty_rect: NSRect);

        #[method(drawDraggingDestinationFeedbackInRect:)]
        pub unsafe fn drawDraggingDestinationFeedbackInRect(&self, dirty_rect: NSRect);

        #[method_id(@__retain_semantics Other viewAtColumn:)]
        pub unsafe fn viewAtColumn(&self, column: NSInteger) -> Option<Retained<AnyObject>>;

        #[method(numberOfColumns)]
        pub unsafe fn numberOfColumns(&self) -> NSInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSTableRowView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSTableRowView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSTableRowView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
