//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "AppKit_NSButton",
        feature = "AppKit_NSControl",
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSView"
    ))]
    pub struct NSPopUpButton;

    #[cfg(all(
        feature = "AppKit_NSButton",
        feature = "AppKit_NSControl",
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSView"
    ))]
    unsafe impl ClassType for NSPopUpButton {
        #[inherits(NSControl, NSView, NSResponder, NSObject)]
        type Super = NSButton;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSButton",
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAccessibility for NSPopUpButton {}

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSButton",
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAccessibilityButton for NSPopUpButton {}

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSButton",
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAccessibilityElementProtocol for NSPopUpButton {}

#[cfg(all(
    feature = "AppKit_NSAnimation",
    feature = "AppKit_NSButton",
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAnimatablePropertyContainer for NSPopUpButton {}

#[cfg(all(
    feature = "AppKit_NSAppearance",
    feature = "AppKit_NSButton",
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAppearanceCustomization for NSPopUpButton {}

#[cfg(all(
    feature = "AppKit_NSButton",
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView",
    feature = "Foundation_NSObject"
))]
unsafe impl NSCoding for NSPopUpButton {}

#[cfg(all(
    feature = "AppKit_NSButton",
    feature = "AppKit_NSControl",
    feature = "AppKit_NSDragging",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSDraggingDestination for NSPopUpButton {}

#[cfg(all(
    feature = "AppKit_NSButton",
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSObjectProtocol for NSPopUpButton {}

#[cfg(all(
    feature = "AppKit_NSButton",
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSUserInterfaceCompression",
    feature = "AppKit_NSView"
))]
unsafe impl NSUserInterfaceCompression for NSPopUpButton {}

#[cfg(all(
    feature = "AppKit_NSButton",
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSUserInterfaceItemIdentification",
    feature = "AppKit_NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for NSPopUpButton {}

#[cfg(all(
    feature = "AppKit_NSButton",
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSUserInterfaceValidation",
    feature = "AppKit_NSView"
))]
unsafe impl NSUserInterfaceValidations for NSPopUpButton {}

extern_methods!(
    #[cfg(all(
        feature = "AppKit_NSButton",
        feature = "AppKit_NSControl",
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSView"
    ))]
    unsafe impl NSPopUpButton {
        #[cfg(feature = "Foundation_NSGeometry")]
        #[method_id(@__retain_semantics Init initWithFrame:pullsDown:)]
        pub unsafe fn initWithFrame_pullsDown(
            this: Allocated<Self>,
            button_frame: NSRect,
            flag: bool,
        ) -> Id<Self>;

        #[cfg(feature = "AppKit_NSMenu")]
        #[method_id(@__retain_semantics Other menu)]
        pub unsafe fn menu(&self) -> Option<Id<NSMenu>>;

        #[cfg(feature = "AppKit_NSMenu")]
        #[method(setMenu:)]
        pub unsafe fn setMenu(&self, menu: Option<&NSMenu>);

        #[method(pullsDown)]
        pub unsafe fn pullsDown(&self) -> bool;

        #[method(setPullsDown:)]
        pub unsafe fn setPullsDown(&self, pulls_down: bool);

        #[method(autoenablesItems)]
        pub unsafe fn autoenablesItems(&self) -> bool;

        #[method(setAutoenablesItems:)]
        pub unsafe fn setAutoenablesItems(&self, autoenables_items: bool);

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(preferredEdge)]
        pub unsafe fn preferredEdge(&self) -> NSRectEdge;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setPreferredEdge:)]
        pub unsafe fn setPreferredEdge(&self, preferred_edge: NSRectEdge);

        #[cfg(feature = "Foundation_NSString")]
        #[method(addItemWithTitle:)]
        pub unsafe fn addItemWithTitle(&self, title: &NSString);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(addItemsWithTitles:)]
        pub unsafe fn addItemsWithTitles(&self, item_titles: &NSArray<NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method(insertItemWithTitle:atIndex:)]
        pub unsafe fn insertItemWithTitle_atIndex(&self, title: &NSString, index: NSInteger);

        #[cfg(feature = "Foundation_NSString")]
        #[method(removeItemWithTitle:)]
        pub unsafe fn removeItemWithTitle(&self, title: &NSString);

        #[method(removeItemAtIndex:)]
        pub unsafe fn removeItemAtIndex(&self, index: NSInteger);

        #[method(removeAllItems)]
        pub unsafe fn removeAllItems(&self);

        #[cfg(all(feature = "AppKit_NSMenuItem", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other itemArray)]
        pub unsafe fn itemArray(&self) -> Id<NSArray<NSMenuItem>>;

        #[method(numberOfItems)]
        pub unsafe fn numberOfItems(&self) -> NSInteger;

        #[cfg(feature = "AppKit_NSMenuItem")]
        #[method(indexOfItem:)]
        pub unsafe fn indexOfItem(&self, item: &NSMenuItem) -> NSInteger;

        #[cfg(feature = "Foundation_NSString")]
        #[method(indexOfItemWithTitle:)]
        pub unsafe fn indexOfItemWithTitle(&self, title: &NSString) -> NSInteger;

        #[method(indexOfItemWithTag:)]
        pub unsafe fn indexOfItemWithTag(&self, tag: NSInteger) -> NSInteger;

        #[method(indexOfItemWithRepresentedObject:)]
        pub unsafe fn indexOfItemWithRepresentedObject(&self, obj: Option<&AnyObject>)
            -> NSInteger;

        #[method(indexOfItemWithTarget:andAction:)]
        pub unsafe fn indexOfItemWithTarget_andAction(
            &self,
            target: Option<&AnyObject>,
            action_selector: Option<Sel>,
        ) -> NSInteger;

        #[cfg(feature = "AppKit_NSMenuItem")]
        #[method_id(@__retain_semantics Other itemAtIndex:)]
        pub unsafe fn itemAtIndex(&self, index: NSInteger) -> Option<Id<NSMenuItem>>;

        #[cfg(all(feature = "AppKit_NSMenuItem", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other itemWithTitle:)]
        pub unsafe fn itemWithTitle(&self, title: &NSString) -> Option<Id<NSMenuItem>>;

        #[cfg(feature = "AppKit_NSMenuItem")]
        #[method_id(@__retain_semantics Other lastItem)]
        pub unsafe fn lastItem(&self) -> Option<Id<NSMenuItem>>;

        #[cfg(feature = "AppKit_NSMenuItem")]
        #[method(selectItem:)]
        pub unsafe fn selectItem(&self, item: Option<&NSMenuItem>);

        #[method(selectItemAtIndex:)]
        pub unsafe fn selectItemAtIndex(&self, index: NSInteger);

        #[cfg(feature = "Foundation_NSString")]
        #[method(selectItemWithTitle:)]
        pub unsafe fn selectItemWithTitle(&self, title: &NSString);

        #[method(selectItemWithTag:)]
        pub unsafe fn selectItemWithTag(&self, tag: NSInteger) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, string: &NSString);

        #[cfg(feature = "AppKit_NSMenuItem")]
        #[method_id(@__retain_semantics Other selectedItem)]
        pub unsafe fn selectedItem(&self) -> Option<Id<NSMenuItem>>;

        #[method(indexOfSelectedItem)]
        pub unsafe fn indexOfSelectedItem(&self) -> NSInteger;

        #[method(selectedTag)]
        pub unsafe fn selectedTag(&self) -> NSInteger;

        #[method(synchronizeTitleAndSelectedItem)]
        pub unsafe fn synchronizeTitleAndSelectedItem(&self);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other itemTitleAtIndex:)]
        pub unsafe fn itemTitleAtIndex(&self, index: NSInteger) -> Id<NSString>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other itemTitles)]
        pub unsafe fn itemTitles(&self) -> Id<NSArray<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other titleOfSelectedItem)]
        pub unsafe fn titleOfSelectedItem(&self) -> Option<Id<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSButton`
    #[cfg(all(
        feature = "AppKit_NSButton",
        feature = "AppKit_NSControl",
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSView"
    ))]
    unsafe impl NSPopUpButton {
        #[cfg(all(feature = "AppKit_NSImage", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other buttonWithTitle:image:target:action:)]
        pub unsafe fn buttonWithTitle_image_target_action(
            title: &NSString,
            image: &NSImage,
            target: Option<&AnyObject>,
            action: Option<Sel>,
            mtm: MainThreadMarker,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other buttonWithTitle:target:action:)]
        pub unsafe fn buttonWithTitle_target_action(
            title: &NSString,
            target: Option<&AnyObject>,
            action: Option<Sel>,
            mtm: MainThreadMarker,
        ) -> Id<Self>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other buttonWithImage:target:action:)]
        pub unsafe fn buttonWithImage_target_action(
            image: &NSImage,
            target: Option<&AnyObject>,
            action: Option<Sel>,
            mtm: MainThreadMarker,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other checkboxWithTitle:target:action:)]
        pub unsafe fn checkboxWithTitle_target_action(
            title: &NSString,
            target: Option<&AnyObject>,
            action: Option<Sel>,
            mtm: MainThreadMarker,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other radioButtonWithTitle:target:action:)]
        pub unsafe fn radioButtonWithTitle_target_action(
            title: &NSString,
            target: Option<&AnyObject>,
            action: Option<Sel>,
            mtm: MainThreadMarker,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(all(
        feature = "AppKit_NSButton",
        feature = "AppKit_NSControl",
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSView"
    ))]
    unsafe impl NSPopUpButton {
        #[cfg(feature = "Foundation_NSGeometry")]
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(
        feature = "AppKit_NSButton",
        feature = "AppKit_NSControl",
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSView"
    ))]
    unsafe impl NSPopUpButton {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "AppKit_NSButton",
        feature = "AppKit_NSControl",
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSView"
    ))]
    unsafe impl NSPopUpButton {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern "C" {
    #[cfg(all(feature = "Foundation_NSNotification", feature = "Foundation_NSString"))]
    pub static NSPopUpButtonWillPopUpNotification: &'static NSNotificationName;
}
