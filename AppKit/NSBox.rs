//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstitleposition?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTitlePosition(pub NSUInteger);
impl NSTitlePosition {
    pub const NSNoTitle: Self = Self(0);
    pub const NSAboveTop: Self = Self(1);
    pub const NSAtTop: Self = Self(2);
    pub const NSBelowTop: Self = Self(3);
    pub const NSAboveBottom: Self = Self(4);
    pub const NSAtBottom: Self = Self(5);
    pub const NSBelowBottom: Self = Self(6);
}

unsafe impl Encode for NSTitlePosition {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSTitlePosition {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsboxtype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSBoxType(pub NSUInteger);
impl NSBoxType {
    pub const NSBoxPrimary: Self = Self(0);
    pub const NSBoxSeparator: Self = Self(2);
    pub const NSBoxCustom: Self = Self(4);
}

unsafe impl Encode for NSBoxType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSBoxType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsbox?language=objc)
    #[unsafe(super(NSView, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    pub struct NSBox;
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibility for NSBox {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibilityElementProtocol for NSBox {}

#[cfg(all(feature = "NSAnimation", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSAnimatablePropertyContainer for NSBox {}

#[cfg(all(feature = "NSAppearance", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSAppearanceCustomization for NSBox {}

#[cfg(all(feature = "NSResponder", feature = "NSView"))]
unsafe impl NSCoding for NSBox {}

#[cfg(all(feature = "NSDragging", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSDraggingDestination for NSBox {}

#[cfg(all(feature = "NSResponder", feature = "NSView"))]
unsafe impl NSObjectProtocol for NSBox {}

#[cfg(all(
    feature = "NSResponder",
    feature = "NSUserInterfaceItemIdentification",
    feature = "NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for NSBox {}

extern_methods!(
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSBox {
        #[method(boxType)]
        pub unsafe fn boxType(&self) -> NSBoxType;

        #[method(setBoxType:)]
        pub unsafe fn setBoxType(&self, box_type: NSBoxType);

        #[method(titlePosition)]
        pub unsafe fn titlePosition(&self) -> NSTitlePosition;

        #[method(setTitlePosition:)]
        pub unsafe fn setTitlePosition(&self, title_position: NSTitlePosition);

        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Retained<NSString>;

        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[cfg(feature = "NSFont")]
        #[method_id(@__retain_semantics Other titleFont)]
        pub unsafe fn titleFont(&self) -> Retained<NSFont>;

        #[cfg(feature = "NSFont")]
        #[method(setTitleFont:)]
        pub unsafe fn setTitleFont(&self, title_font: &NSFont);

        #[method(borderRect)]
        pub unsafe fn borderRect(&self) -> NSRect;

        #[method(titleRect)]
        pub unsafe fn titleRect(&self) -> NSRect;

        #[method_id(@__retain_semantics Other titleCell)]
        pub unsafe fn titleCell(&self) -> Retained<AnyObject>;

        #[method(contentViewMargins)]
        pub unsafe fn contentViewMargins(&self) -> NSSize;

        #[method(setContentViewMargins:)]
        pub unsafe fn setContentViewMargins(&self, content_view_margins: NSSize);

        #[method(sizeToFit)]
        pub unsafe fn sizeToFit(&self);

        #[method(setFrameFromContentFrame:)]
        pub unsafe fn setFrameFromContentFrame(&self, content_frame: NSRect);

        #[method_id(@__retain_semantics Other contentView)]
        pub unsafe fn contentView(&self) -> Option<Retained<NSView>>;

        #[method(setContentView:)]
        pub unsafe fn setContentView(&self, content_view: Option<&NSView>);

        #[method(isTransparent)]
        pub unsafe fn isTransparent(&self) -> bool;

        #[method(setTransparent:)]
        pub unsafe fn setTransparent(&self, transparent: bool);

        #[method(borderWidth)]
        pub unsafe fn borderWidth(&self) -> CGFloat;

        #[method(setBorderWidth:)]
        pub unsafe fn setBorderWidth(&self, border_width: CGFloat);

        #[method(cornerRadius)]
        pub unsafe fn cornerRadius(&self) -> CGFloat;

        #[method(setCornerRadius:)]
        pub unsafe fn setCornerRadius(&self, corner_radius: CGFloat);

        #[cfg(feature = "NSColor")]
        #[method_id(@__retain_semantics Other borderColor)]
        pub unsafe fn borderColor(&self) -> Retained<NSColor>;

        #[cfg(feature = "NSColor")]
        #[method(setBorderColor:)]
        pub unsafe fn setBorderColor(&self, border_color: &NSColor);

        #[cfg(feature = "NSColor")]
        #[method_id(@__retain_semantics Other fillColor)]
        pub unsafe fn fillColor(&self) -> Retained<NSColor>;

        #[cfg(feature = "NSColor")]
        #[method(setFillColor:)]
        pub unsafe fn setFillColor(&self, fill_color: &NSColor);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSBox {
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
    unsafe impl NSBox {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSBox {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSBox {
        #[deprecated = "borderType is only applicable to NSBoxOldStyle, which is deprecated. To replace a borderType of NSNoBorder, use the `transparent` property."]
        #[method(borderType)]
        pub unsafe fn borderType(&self) -> NSBorderType;

        #[deprecated = "borderType is only applicable to NSBoxOldStyle, which is deprecated. To replace a borderType of NSNoBorder, use the `transparent` property."]
        #[method(setBorderType:)]
        pub unsafe fn setBorderType(&self, border_type: NSBorderType);

        #[deprecated]
        #[method(setTitleWithMnemonic:)]
        pub unsafe fn setTitleWithMnemonic(&self, string_with_ampersand: Option<&NSString>);
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsboxsecondary?language=objc)
pub static NSBoxSecondary: NSBoxType = NSBoxType(1);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsboxoldstyle?language=objc)
pub static NSBoxOldStyle: NSBoxType = NSBoxType(3);
