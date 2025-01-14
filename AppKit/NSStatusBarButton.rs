//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsstatusbarbutton?language=objc)
    #[unsafe(super(NSButton, NSControl, NSView, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "NSButton",
        feature = "NSControl",
        feature = "NSResponder",
        feature = "NSView"
    ))]
    pub struct NSStatusBarButton;
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSButton",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibility for NSStatusBarButton {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSButton",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibilityButton for NSStatusBarButton {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSButton",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibilityElementProtocol for NSStatusBarButton {}

#[cfg(all(
    feature = "NSAnimation",
    feature = "NSButton",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAnimatablePropertyContainer for NSStatusBarButton {}

#[cfg(all(
    feature = "NSAppearance",
    feature = "NSButton",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAppearanceCustomization for NSStatusBarButton {}

#[cfg(all(
    feature = "NSButton",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSCoding for NSStatusBarButton {}

#[cfg(all(
    feature = "NSButton",
    feature = "NSControl",
    feature = "NSDragging",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSDraggingDestination for NSStatusBarButton {}

#[cfg(all(
    feature = "NSButton",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSObjectProtocol for NSStatusBarButton {}

#[cfg(all(
    feature = "NSButton",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSUserInterfaceCompression",
    feature = "NSView"
))]
unsafe impl NSUserInterfaceCompression for NSStatusBarButton {}

#[cfg(all(
    feature = "NSButton",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSUserInterfaceItemIdentification",
    feature = "NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for NSStatusBarButton {}

#[cfg(all(
    feature = "NSButton",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSUserInterfaceValidation",
    feature = "NSView"
))]
unsafe impl NSUserInterfaceValidations for NSStatusBarButton {}

extern_methods!(
    #[cfg(all(
        feature = "NSButton",
        feature = "NSControl",
        feature = "NSResponder",
        feature = "NSView"
    ))]
    unsafe impl NSStatusBarButton {
        #[method(appearsDisabled)]
        pub unsafe fn appearsDisabled(&self) -> bool;

        #[method(setAppearsDisabled:)]
        pub unsafe fn setAppearsDisabled(&self, appears_disabled: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSButton`
    #[cfg(all(
        feature = "NSButton",
        feature = "NSControl",
        feature = "NSResponder",
        feature = "NSView"
    ))]
    unsafe impl NSStatusBarButton {
        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other buttonWithTitle:image:target:action:)]
        pub unsafe fn buttonWithTitle_image_target_action(
            title: &NSString,
            image: &NSImage,
            target: Option<&AnyObject>,
            action: Option<Sel>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other buttonWithTitle:target:action:)]
        pub unsafe fn buttonWithTitle_target_action(
            title: &NSString,
            target: Option<&AnyObject>,
            action: Option<Sel>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other buttonWithImage:target:action:)]
        pub unsafe fn buttonWithImage_target_action(
            image: &NSImage,
            target: Option<&AnyObject>,
            action: Option<Sel>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other checkboxWithTitle:target:action:)]
        pub unsafe fn checkboxWithTitle_target_action(
            title: &NSString,
            target: Option<&AnyObject>,
            action: Option<Sel>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other radioButtonWithTitle:target:action:)]
        pub unsafe fn radioButtonWithTitle_target_action(
            title: &NSString,
            target: Option<&AnyObject>,
            action: Option<Sel>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(all(
        feature = "NSButton",
        feature = "NSControl",
        feature = "NSResponder",
        feature = "NSView"
    ))]
    unsafe impl NSStatusBarButton {
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
    #[cfg(all(
        feature = "NSButton",
        feature = "NSControl",
        feature = "NSResponder",
        feature = "NSView"
    ))]
    unsafe impl NSStatusBarButton {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "NSButton",
        feature = "NSControl",
        feature = "NSResponder",
        feature = "NSView"
    ))]
    unsafe impl NSStatusBarButton {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
