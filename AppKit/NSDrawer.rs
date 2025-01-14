//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsdrawerstate?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSDrawerState(pub NSUInteger);
impl NSDrawerState {
    #[deprecated = "Drawers are deprecated; consider using NSSplitViewController"]
    pub const NSDrawerClosedState: Self = Self(0);
    #[deprecated = "Drawers are deprecated; consider using NSSplitViewController"]
    pub const NSDrawerOpeningState: Self = Self(1);
    #[deprecated = "Drawers are deprecated; consider using NSSplitViewController"]
    pub const NSDrawerOpenState: Self = Self(2);
    #[deprecated = "Drawers are deprecated; consider using NSSplitViewController"]
    pub const NSDrawerClosingState: Self = Self(3);
}

unsafe impl Encode for NSDrawerState {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSDrawerState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsdrawer?language=objc)
    #[unsafe(super(NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSResponder")]
    #[deprecated = "Drawers are deprecated; consider using NSSplitViewController"]
    pub struct NSDrawer;
);

#[cfg(all(feature = "NSAccessibilityProtocols", feature = "NSResponder"))]
unsafe impl NSAccessibility for NSDrawer {}

#[cfg(all(feature = "NSAccessibilityProtocols", feature = "NSResponder"))]
unsafe impl NSAccessibilityElementProtocol for NSDrawer {}

#[cfg(feature = "NSResponder")]
unsafe impl NSCoding for NSDrawer {}

#[cfg(feature = "NSResponder")]
unsafe impl NSObjectProtocol for NSDrawer {}

extern_methods!(
    #[cfg(feature = "NSResponder")]
    unsafe impl NSDrawer {
        #[method_id(@__retain_semantics Init initWithContentSize:preferredEdge:)]
        pub unsafe fn initWithContentSize_preferredEdge(
            this: Allocated<Self>,
            content_size: NSSize,
            edge: NSRectEdge,
        ) -> Retained<Self>;

        #[cfg(feature = "NSWindow")]
        #[method_id(@__retain_semantics Other parentWindow)]
        pub unsafe fn parentWindow(&self) -> Option<Retained<NSWindow>>;

        #[cfg(feature = "NSWindow")]
        #[method(setParentWindow:)]
        pub unsafe fn setParentWindow(&self, parent_window: Option<&NSWindow>);

        #[cfg(feature = "NSView")]
        #[method_id(@__retain_semantics Other contentView)]
        pub unsafe fn contentView(&self) -> Option<Retained<NSView>>;

        #[cfg(feature = "NSView")]
        #[method(setContentView:)]
        pub unsafe fn setContentView(&self, content_view: Option<&NSView>);

        #[method(preferredEdge)]
        pub unsafe fn preferredEdge(&self) -> NSRectEdge;

        #[method(setPreferredEdge:)]
        pub unsafe fn setPreferredEdge(&self, preferred_edge: NSRectEdge);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Retained<ProtocolObject<dyn NSDrawerDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn NSDrawerDelegate>>);

        #[method(open)]
        pub unsafe fn open(&self);

        #[method(openOnEdge:)]
        pub unsafe fn openOnEdge(&self, edge: NSRectEdge);

        #[method(close)]
        pub unsafe fn close(&self);

        #[method(open:)]
        pub unsafe fn open_(&self, sender: Option<&AnyObject>);

        #[method(close:)]
        pub unsafe fn close_(&self, sender: Option<&AnyObject>);

        #[method(toggle:)]
        pub unsafe fn toggle(&self, sender: Option<&AnyObject>);

        #[method(state)]
        pub unsafe fn state(&self) -> NSInteger;

        #[method(edge)]
        pub unsafe fn edge(&self) -> NSRectEdge;

        #[method(contentSize)]
        pub unsafe fn contentSize(&self) -> NSSize;

        #[method(setContentSize:)]
        pub unsafe fn setContentSize(&self, content_size: NSSize);

        #[method(minContentSize)]
        pub unsafe fn minContentSize(&self) -> NSSize;

        #[method(setMinContentSize:)]
        pub unsafe fn setMinContentSize(&self, min_content_size: NSSize);

        #[method(maxContentSize)]
        pub unsafe fn maxContentSize(&self) -> NSSize;

        #[method(setMaxContentSize:)]
        pub unsafe fn setMaxContentSize(&self, max_content_size: NSSize);

        #[method(leadingOffset)]
        pub unsafe fn leadingOffset(&self) -> CGFloat;

        #[method(setLeadingOffset:)]
        pub unsafe fn setLeadingOffset(&self, leading_offset: CGFloat);

        #[method(trailingOffset)]
        pub unsafe fn trailingOffset(&self) -> CGFloat;

        #[method(setTrailingOffset:)]
        pub unsafe fn setTrailingOffset(&self, trailing_offset: CGFloat);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "NSResponder")]
    unsafe impl NSDrawer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSResponder")]
    unsafe impl NSDrawer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    /// NSDrawers
    #[cfg(all(feature = "NSResponder", feature = "NSWindow"))]
    unsafe impl NSWindow {
        #[deprecated = "Drawers are deprecated; consider using NSSplitViewController"]
        #[method_id(@__retain_semantics Other drawers)]
        pub unsafe fn drawers(&self) -> Option<Retained<NSArray<NSDrawer>>>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsdrawerdelegate?language=objc)
    pub unsafe trait NSDrawerDelegate: NSObjectProtocol {
        #[cfg(feature = "NSResponder")]
        #[deprecated = "Drawers are deprecated; consider using NSSplitViewController"]
        #[optional]
        #[method(drawerShouldOpen:)]
        unsafe fn drawerShouldOpen(&self, sender: &NSDrawer) -> bool;

        #[cfg(feature = "NSResponder")]
        #[deprecated = "Drawers are deprecated; consider using NSSplitViewController"]
        #[optional]
        #[method(drawerShouldClose:)]
        unsafe fn drawerShouldClose(&self, sender: &NSDrawer) -> bool;

        #[cfg(feature = "NSResponder")]
        #[deprecated = "Drawers are deprecated; consider using NSSplitViewController"]
        #[optional]
        #[method(drawerWillResizeContents:toSize:)]
        unsafe fn drawerWillResizeContents_toSize(
            &self,
            sender: &NSDrawer,
            content_size: NSSize,
        ) -> NSSize;

        #[deprecated = "Drawers are deprecated; consider using NSSplitViewController"]
        #[optional]
        #[method(drawerWillOpen:)]
        unsafe fn drawerWillOpen(&self, notification: &NSNotification);

        #[deprecated = "Drawers are deprecated; consider using NSSplitViewController"]
        #[optional]
        #[method(drawerDidOpen:)]
        unsafe fn drawerDidOpen(&self, notification: &NSNotification);

        #[deprecated = "Drawers are deprecated; consider using NSSplitViewController"]
        #[optional]
        #[method(drawerWillClose:)]
        unsafe fn drawerWillClose(&self, notification: &NSNotification);

        #[deprecated = "Drawers are deprecated; consider using NSSplitViewController"]
        #[optional]
        #[method(drawerDidClose:)]
        unsafe fn drawerDidClose(&self, notification: &NSNotification);
    }

    unsafe impl ProtocolType for dyn NSDrawerDelegate {}
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsdrawerwillopennotification?language=objc)
    pub static NSDrawerWillOpenNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsdrawerdidopennotification?language=objc)
    pub static NSDrawerDidOpenNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsdrawerwillclosenotification?language=objc)
    pub static NSDrawerWillCloseNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsdrawerdidclosenotification?language=objc)
    pub static NSDrawerDidCloseNotification: &'static NSNotificationName;
}
