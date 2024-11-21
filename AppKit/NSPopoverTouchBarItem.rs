//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSTouchBarItem, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSTouchBarItem")]
    pub struct NSPopoverTouchBarItem;
);

#[cfg(feature = "NSTouchBarItem")]
unsafe impl NSCoding for NSPopoverTouchBarItem {}

#[cfg(feature = "NSTouchBarItem")]
unsafe impl NSObjectProtocol for NSPopoverTouchBarItem {}

extern_methods!(
    #[cfg(feature = "NSTouchBarItem")]
    unsafe impl NSPopoverTouchBarItem {
        #[cfg(feature = "NSTouchBar")]
        #[method_id(@__retain_semantics Other popoverTouchBar)]
        pub unsafe fn popoverTouchBar(&self) -> Retained<NSTouchBar>;

        #[cfg(feature = "NSTouchBar")]
        #[method(setPopoverTouchBar:)]
        pub unsafe fn setPopoverTouchBar(&self, popover_touch_bar: &NSTouchBar);

        #[method_id(@__retain_semantics Other customizationLabel)]
        pub unsafe fn customizationLabel(&self) -> Retained<NSString>;

        #[method(setCustomizationLabel:)]
        pub unsafe fn setCustomizationLabel(&self, customization_label: Option<&NSString>);

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method_id(@__retain_semantics Other collapsedRepresentation)]
        pub unsafe fn collapsedRepresentation(&self) -> Retained<NSView>;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method(setCollapsedRepresentation:)]
        pub unsafe fn setCollapsedRepresentation(&self, collapsed_representation: &NSView);

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other collapsedRepresentationImage)]
        pub unsafe fn collapsedRepresentationImage(&self) -> Option<Retained<NSImage>>;

        #[cfg(feature = "NSImage")]
        #[method(setCollapsedRepresentationImage:)]
        pub unsafe fn setCollapsedRepresentationImage(
            &self,
            collapsed_representation_image: Option<&NSImage>,
        );

        #[method_id(@__retain_semantics Other collapsedRepresentationLabel)]
        pub unsafe fn collapsedRepresentationLabel(&self) -> Retained<NSString>;

        #[method(setCollapsedRepresentationLabel:)]
        pub unsafe fn setCollapsedRepresentationLabel(
            &self,
            collapsed_representation_label: &NSString,
        );

        #[cfg(feature = "NSTouchBar")]
        #[method_id(@__retain_semantics Other pressAndHoldTouchBar)]
        pub unsafe fn pressAndHoldTouchBar(&self) -> Option<Retained<NSTouchBar>>;

        #[cfg(feature = "NSTouchBar")]
        #[method(setPressAndHoldTouchBar:)]
        pub unsafe fn setPressAndHoldTouchBar(&self, press_and_hold_touch_bar: Option<&NSTouchBar>);

        #[method(showsCloseButton)]
        pub unsafe fn showsCloseButton(&self) -> bool;

        #[method(setShowsCloseButton:)]
        pub unsafe fn setShowsCloseButton(&self, shows_close_button: bool);

        #[method(showPopover:)]
        pub unsafe fn showPopover(&self, sender: Option<&AnyObject>);

        #[method(dismissPopover:)]
        pub unsafe fn dismissPopover(&self, sender: Option<&AnyObject>);

        #[cfg(feature = "NSGestureRecognizer")]
        #[method_id(@__retain_semantics Other makeStandardActivatePopoverGestureRecognizer)]
        pub unsafe fn makeStandardActivatePopoverGestureRecognizer(
            &self,
        ) -> Retained<NSGestureRecognizer>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTouchBarItem`
    #[cfg(feature = "NSTouchBarItem")]
    unsafe impl NSPopoverTouchBarItem {
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Allocated<Self>,
            identifier: &NSTouchBarItemIdentifier,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSTouchBarItem")]
    unsafe impl NSPopoverTouchBarItem {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
