//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nswindowtabgroup?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSWindowTabGroup;
);

unsafe impl NSObjectProtocol for NSWindowTabGroup {}

extern_methods!(
    unsafe impl NSWindowTabGroup {
        #[cfg(feature = "NSWindow")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSWindowTabbingIdentifier>;

        #[cfg(all(feature = "NSResponder", feature = "NSWindow"))]
        #[method_id(@__retain_semantics Other windows)]
        pub fn windows(&self) -> Retained<NSArray<NSWindow>>;

        #[method(isOverviewVisible)]
        pub unsafe fn isOverviewVisible(&self) -> bool;

        #[method(setOverviewVisible:)]
        pub unsafe fn setOverviewVisible(&self, overview_visible: bool);

        #[method(isTabBarVisible)]
        pub unsafe fn isTabBarVisible(&self) -> bool;

        #[cfg(all(feature = "NSResponder", feature = "NSWindow"))]
        #[method_id(@__retain_semantics Other selectedWindow)]
        pub unsafe fn selectedWindow(&self) -> Option<Retained<NSWindow>>;

        #[cfg(all(feature = "NSResponder", feature = "NSWindow"))]
        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setSelectedWindow:)]
        pub fn setSelectedWindow(&self, selected_window: Option<&NSWindow>);

        #[cfg(all(feature = "NSResponder", feature = "NSWindow"))]
        #[method(addWindow:)]
        pub unsafe fn addWindow(&self, window: &NSWindow);

        #[cfg(all(feature = "NSResponder", feature = "NSWindow"))]
        #[method(insertWindow:atIndex:)]
        pub unsafe fn insertWindow_atIndex(&self, window: &NSWindow, index: NSInteger);

        #[cfg(all(feature = "NSResponder", feature = "NSWindow"))]
        #[method(removeWindow:)]
        pub unsafe fn removeWindow(&self, window: &NSWindow);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSWindowTabGroup {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
