//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstabviewcontrollertabstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTabViewControllerTabStyle(pub NSInteger);
impl NSTabViewControllerTabStyle {
    #[doc(alias = "NSTabViewControllerTabStyleSegmentedControlOnTop")]
    pub const SegmentedControlOnTop: Self = Self(0);
    #[doc(alias = "NSTabViewControllerTabStyleSegmentedControlOnBottom")]
    pub const SegmentedControlOnBottom: Self = Self(1);
    #[doc(alias = "NSTabViewControllerTabStyleToolbar")]
    pub const Toolbar: Self = Self(2);
    #[doc(alias = "NSTabViewControllerTabStyleUnspecified")]
    pub const Unspecified: Self = Self(-1);
}

unsafe impl Encode for NSTabViewControllerTabStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSTabViewControllerTabStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstabviewcontroller?language=objc)
    #[unsafe(super(NSViewController, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NSResponder", feature = "NSViewController"))]
    pub struct NSTabViewController;
);

#[cfg(all(feature = "NSResponder", feature = "NSViewController"))]
unsafe impl NSCoding for NSTabViewController {}

#[cfg(all(
    feature = "NSKeyValueBinding",
    feature = "NSResponder",
    feature = "NSViewController"
))]
unsafe impl NSEditor for NSTabViewController {}

#[cfg(all(feature = "NSResponder", feature = "NSViewController"))]
unsafe impl NSObjectProtocol for NSTabViewController {}

#[cfg(all(
    feature = "NSResponder",
    feature = "NSStoryboardSegue",
    feature = "NSViewController"
))]
unsafe impl NSSeguePerforming for NSTabViewController {}

#[cfg(all(
    feature = "NSResponder",
    feature = "NSTabView",
    feature = "NSViewController"
))]
unsafe impl NSTabViewDelegate for NSTabViewController {}

#[cfg(all(
    feature = "NSResponder",
    feature = "NSToolbar",
    feature = "NSViewController"
))]
unsafe impl NSToolbarDelegate for NSTabViewController {}

#[cfg(all(
    feature = "NSResponder",
    feature = "NSUserInterfaceItemIdentification",
    feature = "NSViewController"
))]
unsafe impl NSUserInterfaceItemIdentification for NSTabViewController {}

extern_methods!(
    #[cfg(all(feature = "NSResponder", feature = "NSViewController"))]
    unsafe impl NSTabViewController {
        #[method(tabStyle)]
        pub unsafe fn tabStyle(&self) -> NSTabViewControllerTabStyle;

        #[method(setTabStyle:)]
        pub unsafe fn setTabStyle(&self, tab_style: NSTabViewControllerTabStyle);

        #[cfg(all(feature = "NSTabView", feature = "NSView"))]
        #[method_id(@__retain_semantics Other tabView)]
        pub unsafe fn tabView(&self) -> Retained<NSTabView>;

        #[cfg(all(feature = "NSTabView", feature = "NSView"))]
        #[method(setTabView:)]
        pub unsafe fn setTabView(&self, tab_view: &NSTabView);

        #[method(transitionOptions)]
        pub unsafe fn transitionOptions(&self) -> NSViewControllerTransitionOptions;

        #[method(setTransitionOptions:)]
        pub unsafe fn setTransitionOptions(
            &self,
            transition_options: NSViewControllerTransitionOptions,
        );

        #[method(canPropagateSelectedChildViewControllerTitle)]
        pub unsafe fn canPropagateSelectedChildViewControllerTitle(&self) -> bool;

        #[method(setCanPropagateSelectedChildViewControllerTitle:)]
        pub unsafe fn setCanPropagateSelectedChildViewControllerTitle(
            &self,
            can_propagate_selected_child_view_controller_title: bool,
        );

        #[cfg(feature = "NSTabViewItem")]
        #[method_id(@__retain_semantics Other tabViewItems)]
        pub unsafe fn tabViewItems(&self) -> Retained<NSArray<NSTabViewItem>>;

        #[cfg(feature = "NSTabViewItem")]
        #[method(setTabViewItems:)]
        pub unsafe fn setTabViewItems(&self, tab_view_items: &NSArray<NSTabViewItem>);

        #[method(selectedTabViewItemIndex)]
        pub unsafe fn selectedTabViewItemIndex(&self) -> NSInteger;

        #[method(setSelectedTabViewItemIndex:)]
        pub unsafe fn setSelectedTabViewItemIndex(&self, selected_tab_view_item_index: NSInteger);

        #[cfg(feature = "NSTabViewItem")]
        #[method(addTabViewItem:)]
        pub unsafe fn addTabViewItem(&self, tab_view_item: &NSTabViewItem);

        #[cfg(feature = "NSTabViewItem")]
        #[method(insertTabViewItem:atIndex:)]
        pub unsafe fn insertTabViewItem_atIndex(
            &self,
            tab_view_item: &NSTabViewItem,
            index: NSInteger,
        );

        #[cfg(feature = "NSTabViewItem")]
        #[method(removeTabViewItem:)]
        pub unsafe fn removeTabViewItem(&self, tab_view_item: &NSTabViewItem);

        #[cfg(feature = "NSTabViewItem")]
        #[method_id(@__retain_semantics Other tabViewItemForViewController:)]
        pub unsafe fn tabViewItemForViewController(
            &self,
            view_controller: &NSViewController,
        ) -> Option<Retained<NSTabViewItem>>;

        #[method(viewDidLoad)]
        pub unsafe fn viewDidLoad(&self);

        #[cfg(all(feature = "NSTabView", feature = "NSTabViewItem", feature = "NSView"))]
        #[method(tabView:willSelectTabViewItem:)]
        pub unsafe fn tabView_willSelectTabViewItem(
            &self,
            tab_view: &NSTabView,
            tab_view_item: Option<&NSTabViewItem>,
        );

        #[cfg(all(feature = "NSTabView", feature = "NSTabViewItem", feature = "NSView"))]
        #[method(tabView:didSelectTabViewItem:)]
        pub unsafe fn tabView_didSelectTabViewItem(
            &self,
            tab_view: &NSTabView,
            tab_view_item: Option<&NSTabViewItem>,
        );

        #[cfg(all(feature = "NSTabView", feature = "NSTabViewItem", feature = "NSView"))]
        #[method(tabView:shouldSelectTabViewItem:)]
        pub unsafe fn tabView_shouldSelectTabViewItem(
            &self,
            tab_view: &NSTabView,
            tab_view_item: Option<&NSTabViewItem>,
        ) -> bool;

        #[cfg(all(feature = "NSToolbar", feature = "NSToolbarItem"))]
        #[method_id(@__retain_semantics Other toolbar:itemForItemIdentifier:willBeInsertedIntoToolbar:)]
        pub unsafe fn toolbar_itemForItemIdentifier_willBeInsertedIntoToolbar(
            &self,
            toolbar: &NSToolbar,
            item_identifier: &NSToolbarItemIdentifier,
            flag: bool,
        ) -> Option<Retained<NSToolbarItem>>;

        #[cfg(feature = "NSToolbar")]
        #[method_id(@__retain_semantics Other toolbarDefaultItemIdentifiers:)]
        pub unsafe fn toolbarDefaultItemIdentifiers(
            &self,
            toolbar: &NSToolbar,
        ) -> Retained<NSArray<NSToolbarItemIdentifier>>;

        #[cfg(feature = "NSToolbar")]
        #[method_id(@__retain_semantics Other toolbarAllowedItemIdentifiers:)]
        pub unsafe fn toolbarAllowedItemIdentifiers(
            &self,
            toolbar: &NSToolbar,
        ) -> Retained<NSArray<NSToolbarItemIdentifier>>;

        #[cfg(feature = "NSToolbar")]
        #[method_id(@__retain_semantics Other toolbarSelectableItemIdentifiers:)]
        pub unsafe fn toolbarSelectableItemIdentifiers(
            &self,
            toolbar: &NSToolbar,
        ) -> Retained<NSArray<NSToolbarItemIdentifier>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(all(feature = "NSResponder", feature = "NSViewController"))]
    unsafe impl NSTabViewController {
        #[cfg(feature = "NSNib")]
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(feature = "NSResponder", feature = "NSViewController"))]
    unsafe impl NSTabViewController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "NSResponder", feature = "NSViewController"))]
    unsafe impl NSTabViewController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
