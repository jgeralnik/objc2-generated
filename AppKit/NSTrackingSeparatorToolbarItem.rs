//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSToolbarItem")]
    pub struct NSTrackingSeparatorToolbarItem;

    #[cfg(feature = "NSToolbarItem")]
    unsafe impl ClassType for NSTrackingSeparatorToolbarItem {
        #[inherits(NSObject)]
        type Super = NSToolbarItem;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "NSToolbarItem")]
unsafe impl NSCopying for NSTrackingSeparatorToolbarItem {}

#[cfg(feature = "NSToolbarItem")]
unsafe impl CopyingHelper for NSTrackingSeparatorToolbarItem {
    type Result = Self;
}

#[cfg(feature = "NSToolbarItem")]
unsafe impl NSObjectProtocol for NSTrackingSeparatorToolbarItem {}

extern_methods!(
    #[cfg(feature = "NSToolbarItem")]
    unsafe impl NSTrackingSeparatorToolbarItem {
        #[cfg(all(
            feature = "NSResponder",
            feature = "NSSplitView",
            feature = "NSToolbar",
            feature = "NSView"
        ))]
        #[method_id(@__retain_semantics Other trackingSeparatorToolbarItemWithIdentifier:splitView:dividerIndex:)]
        pub unsafe fn trackingSeparatorToolbarItemWithIdentifier_splitView_dividerIndex(
            identifier: &NSToolbarItemIdentifier,
            split_view: &NSSplitView,
            divider_index: NSInteger,
        ) -> Retained<Self>;

        #[cfg(all(feature = "NSResponder", feature = "NSSplitView", feature = "NSView"))]
        #[method_id(@__retain_semantics Other splitView)]
        pub unsafe fn splitView(&self) -> Retained<NSSplitView>;

        #[cfg(all(feature = "NSResponder", feature = "NSSplitView", feature = "NSView"))]
        #[method(setSplitView:)]
        pub unsafe fn setSplitView(&self, split_view: &NSSplitView);

        #[method(dividerIndex)]
        pub unsafe fn dividerIndex(&self) -> NSInteger;

        #[method(setDividerIndex:)]
        pub unsafe fn setDividerIndex(&self, divider_index: NSInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSToolbarItem`
    #[cfg(feature = "NSToolbarItem")]
    unsafe impl NSTrackingSeparatorToolbarItem {
        #[cfg(feature = "NSToolbar")]
        #[method_id(@__retain_semantics Init initWithItemIdentifier:)]
        pub unsafe fn initWithItemIdentifier(
            this: Allocated<Self>,
            item_identifier: &NSToolbarItemIdentifier,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSToolbarItem")]
    unsafe impl NSTrackingSeparatorToolbarItem {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
