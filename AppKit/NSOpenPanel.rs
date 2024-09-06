//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "NSPanel",
        feature = "NSResponder",
        feature = "NSSavePanel",
        feature = "NSWindow"
    ))]
    pub struct NSOpenPanel;

    #[cfg(all(
        feature = "NSPanel",
        feature = "NSResponder",
        feature = "NSSavePanel",
        feature = "NSWindow"
    ))]
    unsafe impl ClassType for NSOpenPanel {
        #[inherits(NSPanel, NSWindow, NSResponder, NSObject)]
        type Super = NSSavePanel;
    }
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSPanel",
    feature = "NSResponder",
    feature = "NSSavePanel",
    feature = "NSWindow"
))]
unsafe impl NSAccessibility for NSOpenPanel {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSPanel",
    feature = "NSResponder",
    feature = "NSSavePanel",
    feature = "NSWindow"
))]
unsafe impl NSAccessibilityElementProtocol for NSOpenPanel {}

#[cfg(all(
    feature = "NSAnimation",
    feature = "NSPanel",
    feature = "NSResponder",
    feature = "NSSavePanel",
    feature = "NSWindow"
))]
unsafe impl NSAnimatablePropertyContainer for NSOpenPanel {}

#[cfg(all(
    feature = "NSAppearance",
    feature = "NSPanel",
    feature = "NSResponder",
    feature = "NSSavePanel",
    feature = "NSWindow"
))]
unsafe impl NSAppearanceCustomization for NSOpenPanel {}

#[cfg(all(
    feature = "NSPanel",
    feature = "NSResponder",
    feature = "NSSavePanel",
    feature = "NSWindow"
))]
unsafe impl NSCoding for NSOpenPanel {}

#[cfg(all(
    feature = "NSMenu",
    feature = "NSPanel",
    feature = "NSResponder",
    feature = "NSSavePanel",
    feature = "NSWindow"
))]
unsafe impl NSMenuItemValidation for NSOpenPanel {}

#[cfg(all(
    feature = "NSPanel",
    feature = "NSResponder",
    feature = "NSSavePanel",
    feature = "NSWindow"
))]
unsafe impl NSObjectProtocol for NSOpenPanel {}

#[cfg(all(
    feature = "NSPanel",
    feature = "NSResponder",
    feature = "NSSavePanel",
    feature = "NSUserInterfaceItemIdentification",
    feature = "NSWindow"
))]
unsafe impl NSUserInterfaceItemIdentification for NSOpenPanel {}

#[cfg(all(
    feature = "NSPanel",
    feature = "NSResponder",
    feature = "NSSavePanel",
    feature = "NSUserInterfaceValidation",
    feature = "NSWindow"
))]
unsafe impl NSUserInterfaceValidations for NSOpenPanel {}

extern_methods!(
    #[cfg(all(
        feature = "NSPanel",
        feature = "NSResponder",
        feature = "NSSavePanel",
        feature = "NSWindow"
    ))]
    unsafe impl NSOpenPanel {
        #[method_id(@__retain_semantics Other openPanel)]
        pub unsafe fn openPanel(mtm: MainThreadMarker) -> Retained<NSOpenPanel>;

        #[method_id(@__retain_semantics Other URLs)]
        pub unsafe fn URLs(&self) -> Retained<NSArray<NSURL>>;

        #[method(resolvesAliases)]
        pub unsafe fn resolvesAliases(&self) -> bool;

        #[method(setResolvesAliases:)]
        pub unsafe fn setResolvesAliases(&self, resolves_aliases: bool);

        #[method(canChooseDirectories)]
        pub unsafe fn canChooseDirectories(&self) -> bool;

        #[method(setCanChooseDirectories:)]
        pub unsafe fn setCanChooseDirectories(&self, can_choose_directories: bool);

        #[method(allowsMultipleSelection)]
        pub unsafe fn allowsMultipleSelection(&self) -> bool;

        #[method(setAllowsMultipleSelection:)]
        pub unsafe fn setAllowsMultipleSelection(&self, allows_multiple_selection: bool);

        #[method(canChooseFiles)]
        pub unsafe fn canChooseFiles(&self) -> bool;

        #[method(setCanChooseFiles:)]
        pub unsafe fn setCanChooseFiles(&self, can_choose_files: bool);

        #[method(canResolveUbiquitousConflicts)]
        pub unsafe fn canResolveUbiquitousConflicts(&self) -> bool;

        #[method(setCanResolveUbiquitousConflicts:)]
        pub unsafe fn setCanResolveUbiquitousConflicts(
            &self,
            can_resolve_ubiquitous_conflicts: bool,
        );

        #[method(canDownloadUbiquitousContents)]
        pub unsafe fn canDownloadUbiquitousContents(&self) -> bool;

        #[method(setCanDownloadUbiquitousContents:)]
        pub unsafe fn setCanDownloadUbiquitousContents(
            &self,
            can_download_ubiquitous_contents: bool,
        );

        #[method(isAccessoryViewDisclosed)]
        pub unsafe fn isAccessoryViewDisclosed(&self) -> bool;

        #[method(setAccessoryViewDisclosed:)]
        pub unsafe fn setAccessoryViewDisclosed(&self, accessory_view_disclosed: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSWindow`
    #[cfg(all(
        feature = "NSPanel",
        feature = "NSResponder",
        feature = "NSSavePanel",
        feature = "NSWindow"
    ))]
    unsafe impl NSOpenPanel {
        #[cfg(feature = "NSGraphics")]
        #[method_id(@__retain_semantics Init initWithContentRect:styleMask:backing:defer:)]
        pub unsafe fn initWithContentRect_styleMask_backing_defer(
            this: Allocated<Self>,
            content_rect: NSRect,
            style: NSWindowStyleMask,
            backing_store_type: NSBackingStoreType,
            flag: bool,
        ) -> Retained<Self>;

        #[cfg(all(feature = "NSGraphics", feature = "NSScreen"))]
        #[method_id(@__retain_semantics Init initWithContentRect:styleMask:backing:defer:screen:)]
        pub unsafe fn initWithContentRect_styleMask_backing_defer_screen(
            this: Allocated<Self>,
            content_rect: NSRect,
            style: NSWindowStyleMask,
            backing_store_type: NSBackingStoreType,
            flag: bool,
            screen: Option<&NSScreen>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Retained<Self>;

        #[cfg(feature = "NSViewController")]
        #[method_id(@__retain_semantics Other windowWithContentViewController:)]
        pub unsafe fn windowWithContentViewController(
            content_view_controller: &NSViewController,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(
        feature = "NSPanel",
        feature = "NSResponder",
        feature = "NSSavePanel",
        feature = "NSWindow"
    ))]
    unsafe impl NSOpenPanel {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "NSPanel",
        feature = "NSResponder",
        feature = "NSSavePanel",
        feature = "NSWindow"
    ))]
    unsafe impl NSOpenPanel {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(all(
        feature = "NSPanel",
        feature = "NSResponder",
        feature = "NSSavePanel",
        feature = "NSWindow"
    ))]
    unsafe impl NSOpenPanel {
        #[deprecated]
        #[method_id(@__retain_semantics Other filenames)]
        pub unsafe fn filenames(&self) -> Retained<NSArray>;

        #[deprecated]
        #[method(beginSheetForDirectory:file:types:modalForWindow:modalDelegate:didEndSelector:contextInfo:)]
        pub unsafe fn beginSheetForDirectory_file_types_modalForWindow_modalDelegate_didEndSelector_contextInfo(
            &self,
            path: Option<&NSString>,
            name: Option<&NSString>,
            file_types: Option<&NSArray>,
            doc_window: Option<&NSWindow>,
            delegate: Option<&AnyObject>,
            did_end_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[deprecated]
        #[method(beginForDirectory:file:types:modelessDelegate:didEndSelector:contextInfo:)]
        pub unsafe fn beginForDirectory_file_types_modelessDelegate_didEndSelector_contextInfo(
            &self,
            path: Option<&NSString>,
            name: Option<&NSString>,
            file_types: Option<&NSArray>,
            delegate: Option<&AnyObject>,
            did_end_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[deprecated]
        #[method(runModalForDirectory:file:types:)]
        pub unsafe fn runModalForDirectory_file_types(
            &self,
            path: Option<&NSString>,
            name: Option<&NSString>,
            file_types: Option<&NSArray>,
        ) -> NSInteger;

        #[deprecated]
        #[method(runModalForTypes:)]
        pub unsafe fn runModalForTypes(&self, file_types: Option<&NSArray>) -> NSInteger;
    }
);
