//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspathcontrol?language=objc)
    #[unsafe(super(NSControl, NSView, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    pub struct NSPathControl;
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibility for NSPathControl {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibilityElementProtocol for NSPathControl {}

#[cfg(all(
    feature = "NSAnimation",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAnimatablePropertyContainer for NSPathControl {}

#[cfg(all(
    feature = "NSAppearance",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAppearanceCustomization for NSPathControl {}

#[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSCoding for NSPathControl {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSDragging",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSDraggingDestination for NSPathControl {}

#[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSObjectProtocol for NSPathControl {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSUserInterfaceItemIdentification",
    feature = "NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for NSPathControl {}

extern_methods!(
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSPathControl {
        #[method(isEditable)]
        pub unsafe fn isEditable(&self) -> bool;

        #[method(setEditable:)]
        pub unsafe fn setEditable(&self, editable: bool);

        #[method_id(@__retain_semantics Other allowedTypes)]
        pub unsafe fn allowedTypes(&self) -> Option<Retained<NSArray<NSString>>>;

        #[method(setAllowedTypes:)]
        pub unsafe fn setAllowedTypes(&self, allowed_types: Option<&NSArray<NSString>>);

        #[method_id(@__retain_semantics Other placeholderString)]
        pub unsafe fn placeholderString(&self) -> Option<Retained<NSString>>;

        #[method(setPlaceholderString:)]
        pub unsafe fn setPlaceholderString(&self, placeholder_string: Option<&NSString>);

        #[method_id(@__retain_semantics Other placeholderAttributedString)]
        pub unsafe fn placeholderAttributedString(&self) -> Option<Retained<NSAttributedString>>;

        #[method(setPlaceholderAttributedString:)]
        pub unsafe fn setPlaceholderAttributedString(
            &self,
            placeholder_attributed_string: Option<&NSAttributedString>,
        );

        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Option<Retained<NSURL>>;

        #[method(setURL:)]
        pub unsafe fn setURL(&self, url: Option<&NSURL>);

        #[method(doubleAction)]
        pub unsafe fn doubleAction(&self) -> Option<Sel>;

        #[method(setDoubleAction:)]
        pub unsafe fn setDoubleAction(&self, double_action: Option<Sel>);

        #[cfg(feature = "NSPathCell")]
        #[method(pathStyle)]
        pub unsafe fn pathStyle(&self) -> NSPathStyle;

        #[cfg(feature = "NSPathCell")]
        #[method(setPathStyle:)]
        pub unsafe fn setPathStyle(&self, path_style: NSPathStyle);

        #[cfg(feature = "NSPathControlItem")]
        #[method_id(@__retain_semantics Other clickedPathItem)]
        pub unsafe fn clickedPathItem(&self) -> Option<Retained<NSPathControlItem>>;

        #[cfg(feature = "NSPathControlItem")]
        #[method_id(@__retain_semantics Other pathItems)]
        pub unsafe fn pathItems(&self) -> Retained<NSArray<NSPathControlItem>>;

        #[cfg(feature = "NSPathControlItem")]
        #[method(setPathItems:)]
        pub unsafe fn setPathItems(&self, path_items: &NSArray<NSPathControlItem>);

        #[cfg(feature = "NSColor")]
        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Option<Retained<NSColor>>;

        #[cfg(feature = "NSColor")]
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, background_color: Option<&NSColor>);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn NSPathControlDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSPathControlDelegate>>,
        );

        #[cfg(feature = "NSDragging")]
        #[method(setDraggingSourceOperationMask:forLocal:)]
        pub unsafe fn setDraggingSourceOperationMask_forLocal(
            &self,
            mask: NSDragOperation,
            is_local: bool,
        );

        #[cfg(feature = "NSMenu")]
        #[method_id(@__retain_semantics Other menu)]
        pub unsafe fn menu(&self) -> Option<Retained<NSMenu>>;

        #[cfg(feature = "NSMenu")]
        #[method(setMenu:)]
        pub unsafe fn setMenu(&self, menu: Option<&NSMenu>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSPathControl {
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
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSPathControl {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSPathControl {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspathcontroldelegate?language=objc)
    pub unsafe trait NSPathControlDelegate: NSObjectProtocol + MainThreadOnly {
        #[cfg(all(
            feature = "NSControl",
            feature = "NSPasteboard",
            feature = "NSPathControlItem",
            feature = "NSResponder",
            feature = "NSView"
        ))]
        #[optional]
        #[method(pathControl:shouldDragItem:withPasteboard:)]
        unsafe fn pathControl_shouldDragItem_withPasteboard(
            &self,
            path_control: &NSPathControl,
            path_item: &NSPathControlItem,
            pasteboard: &NSPasteboard,
        ) -> bool;

        #[cfg(all(
            feature = "NSActionCell",
            feature = "NSCell",
            feature = "NSControl",
            feature = "NSPasteboard",
            feature = "NSPathComponentCell",
            feature = "NSResponder",
            feature = "NSTextFieldCell",
            feature = "NSView"
        ))]
        #[optional]
        #[method(pathControl:shouldDragPathComponentCell:withPasteboard:)]
        unsafe fn pathControl_shouldDragPathComponentCell_withPasteboard(
            &self,
            path_control: &NSPathControl,
            path_component_cell: &NSPathComponentCell,
            pasteboard: &NSPasteboard,
        ) -> bool;

        #[cfg(all(
            feature = "NSControl",
            feature = "NSDragging",
            feature = "NSResponder",
            feature = "NSView"
        ))]
        #[optional]
        #[method(pathControl:validateDrop:)]
        unsafe fn pathControl_validateDrop(
            &self,
            path_control: &NSPathControl,
            info: &ProtocolObject<dyn NSDraggingInfo>,
        ) -> NSDragOperation;

        #[cfg(all(
            feature = "NSControl",
            feature = "NSDragging",
            feature = "NSResponder",
            feature = "NSView"
        ))]
        #[optional]
        #[method(pathControl:acceptDrop:)]
        unsafe fn pathControl_acceptDrop(
            &self,
            path_control: &NSPathControl,
            info: &ProtocolObject<dyn NSDraggingInfo>,
        ) -> bool;

        #[cfg(all(
            feature = "NSControl",
            feature = "NSOpenPanel",
            feature = "NSPanel",
            feature = "NSResponder",
            feature = "NSSavePanel",
            feature = "NSView",
            feature = "NSWindow"
        ))]
        #[optional]
        #[method(pathControl:willDisplayOpenPanel:)]
        unsafe fn pathControl_willDisplayOpenPanel(
            &self,
            path_control: &NSPathControl,
            open_panel: &NSOpenPanel,
        );

        #[cfg(all(
            feature = "NSControl",
            feature = "NSMenu",
            feature = "NSResponder",
            feature = "NSView"
        ))]
        #[optional]
        #[method(pathControl:willPopUpMenu:)]
        unsafe fn pathControl_willPopUpMenu(&self, path_control: &NSPathControl, menu: &NSMenu);
    }

    unsafe impl ProtocolType for dyn NSPathControlDelegate {}
);

extern_methods!(
    /// NSDeprecated
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSPathControl {
        #[cfg(all(
            feature = "NSActionCell",
            feature = "NSCell",
            feature = "NSPathComponentCell",
            feature = "NSTextFieldCell"
        ))]
        #[deprecated = "Use the clickedPathItem property instead"]
        #[method_id(@__retain_semantics Other clickedPathComponentCell)]
        pub unsafe fn clickedPathComponentCell(&self) -> Option<Retained<NSPathComponentCell>>;

        #[cfg(all(
            feature = "NSActionCell",
            feature = "NSCell",
            feature = "NSPathComponentCell",
            feature = "NSTextFieldCell"
        ))]
        #[deprecated = "Use the pathItems property instead"]
        #[method_id(@__retain_semantics Other pathComponentCells)]
        pub unsafe fn pathComponentCells(&self) -> Retained<NSArray<NSPathComponentCell>>;

        #[cfg(all(
            feature = "NSActionCell",
            feature = "NSCell",
            feature = "NSPathComponentCell",
            feature = "NSTextFieldCell"
        ))]
        #[deprecated = "Use the pathItems property instead"]
        #[method(setPathComponentCells:)]
        pub unsafe fn setPathComponentCells(&self, cells: &NSArray<NSPathComponentCell>);
    }
);
