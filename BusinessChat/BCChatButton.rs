//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/businesschat/bcchatbuttonstyle?language=objc)
// NS_ENUM
#[deprecated]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BCChatButtonStyle(pub NSInteger);
impl BCChatButtonStyle {
    #[deprecated]
    #[doc(alias = "BCChatButtonStyleLight")]
    pub const Light: Self = Self(0);
    #[deprecated]
    #[doc(alias = "BCChatButtonStyleDark")]
    pub const Dark: Self = Self(1);
}

unsafe impl Encode for BCChatButtonStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for BCChatButtonStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/businesschat/bcchatbutton?language=objc)
    #[unsafe(super(NSControl, NSView, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(target_os = "macos")]
    #[deprecated]
    pub struct BCChatButton;
);

#[cfg(target_os = "macos")]
unsafe impl NSAccessibility for BCChatButton {}

#[cfg(target_os = "macos")]
unsafe impl NSAccessibilityElementProtocol for BCChatButton {}

#[cfg(target_os = "macos")]
unsafe impl NSAnimatablePropertyContainer for BCChatButton {}

#[cfg(target_os = "macos")]
unsafe impl NSAppearanceCustomization for BCChatButton {}

#[cfg(target_os = "macos")]
unsafe impl NSCoding for BCChatButton {}

#[cfg(target_os = "macos")]
unsafe impl NSDraggingDestination for BCChatButton {}

#[cfg(target_os = "macos")]
unsafe impl NSObjectProtocol for BCChatButton {}

#[cfg(target_os = "macos")]
unsafe impl NSUserInterfaceItemIdentification for BCChatButton {}

extern_methods!(
    #[cfg(target_os = "macos")]
    unsafe impl BCChatButton {
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithStyle:)]
        pub unsafe fn initWithStyle(
            this: Allocated<Self>,
            style: BCChatButtonStyle,
        ) -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(target_os = "macos")]
    unsafe impl BCChatButton {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(target_os = "macos")]
    unsafe impl BCChatButton {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(target_os = "macos")]
    unsafe impl BCChatButton {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
