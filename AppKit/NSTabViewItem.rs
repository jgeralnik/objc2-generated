//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTabState(pub NSUInteger);
impl NSTabState {
    pub const NSSelectedTab: Self = Self(0);
    pub const NSBackgroundTab: Self = Self(1);
    pub const NSPressedTab: Self = Self(2);
}

unsafe impl Encode for NSTabState {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSTabState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTabViewItem;
);

unsafe impl NSCoding for NSTabViewItem {}

unsafe impl NSObjectProtocol for NSTabViewItem {}

extern_methods!(
    unsafe impl NSTabViewItem {
        #[cfg(all(feature = "NSResponder", feature = "NSViewController"))]
        #[method_id(@__retain_semantics Other tabViewItemWithViewController:)]
        pub unsafe fn tabViewItemWithViewController(
            view_controller: &NSViewController,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Allocated<Self>,
            identifier: Option<&AnyObject>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Option<Retained<AnyObject>>;

        #[method(setIdentifier:)]
        pub unsafe fn setIdentifier(&self, identifier: Option<&AnyObject>);

        #[cfg(feature = "NSColor")]
        #[method_id(@__retain_semantics Other color)]
        pub unsafe fn color(&self) -> Retained<NSColor>;

        #[cfg(feature = "NSColor")]
        #[method(setColor:)]
        pub unsafe fn setColor(&self, color: &NSColor);

        #[method_id(@__retain_semantics Other label)]
        pub unsafe fn label(&self) -> Retained<NSString>;

        #[method(setLabel:)]
        pub unsafe fn setLabel(&self, label: &NSString);

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Retained<NSImage>>;

        #[cfg(feature = "NSImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method_id(@__retain_semantics Other view)]
        pub unsafe fn view(&self, mtm: MainThreadMarker) -> Option<Retained<NSView>>;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method(setView:)]
        pub unsafe fn setView(&self, view: Option<&NSView>);

        #[cfg(all(feature = "NSResponder", feature = "NSViewController"))]
        #[method_id(@__retain_semantics Other viewController)]
        pub unsafe fn viewController(
            &self,
            mtm: MainThreadMarker,
        ) -> Option<Retained<NSViewController>>;

        #[cfg(all(feature = "NSResponder", feature = "NSViewController"))]
        #[method(setViewController:)]
        pub unsafe fn setViewController(&self, view_controller: Option<&NSViewController>);

        #[method(tabState)]
        pub unsafe fn tabState(&self) -> NSTabState;

        #[cfg(all(feature = "NSResponder", feature = "NSTabView", feature = "NSView"))]
        #[method_id(@__retain_semantics Other tabView)]
        pub unsafe fn tabView(&self, mtm: MainThreadMarker) -> Option<Retained<NSTabView>>;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method_id(@__retain_semantics Other initialFirstResponder)]
        pub unsafe fn initialFirstResponder(
            &self,
            mtm: MainThreadMarker,
        ) -> Option<Retained<NSView>>;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setInitialFirstResponder:)]
        pub unsafe fn setInitialFirstResponder(&self, initial_first_responder: Option<&NSView>);

        #[method_id(@__retain_semantics Other toolTip)]
        pub unsafe fn toolTip(&self) -> Option<Retained<NSString>>;

        #[method(setToolTip:)]
        pub unsafe fn setToolTip(&self, tool_tip: Option<&NSString>);

        #[method(drawLabel:inRect:)]
        pub unsafe fn drawLabel_inRect(&self, should_truncate_label: bool, label_rect: NSRect);

        #[method(sizeOfLabel:)]
        pub unsafe fn sizeOfLabel(&self, compute_min: bool) -> NSSize;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSTabViewItem {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
