//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern "C" {
    #[cfg(feature = "Foundation_NSGeometry")]
    pub static NSSplitViewControllerAutomaticDimension: CGFloat;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
    pub struct NSSplitViewController;

    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
    unsafe impl ClassType for NSSplitViewController {
        #[inherits(NSResponder, NSObject)]
        type Super = NSViewController;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSViewController",
    feature = "Foundation_NSObject"
))]
unsafe impl NSCoding for NSSplitViewController {}

#[cfg(all(
    feature = "AppKit_NSKeyValueBinding",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSViewController"
))]
unsafe impl NSEditor for NSSplitViewController {}

#[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
unsafe impl NSObjectProtocol for NSSplitViewController {}

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSStoryboardSegue",
    feature = "AppKit_NSViewController"
))]
unsafe impl NSSeguePerforming for NSSplitViewController {}

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSSplitView",
    feature = "AppKit_NSViewController"
))]
unsafe impl NSSplitViewDelegate for NSSplitViewController {}

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSUserInterfaceItemIdentification",
    feature = "AppKit_NSViewController"
))]
unsafe impl NSUserInterfaceItemIdentification for NSSplitViewController {}

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSUserInterfaceValidation",
    feature = "AppKit_NSViewController"
))]
unsafe impl NSUserInterfaceValidations for NSSplitViewController {}

extern_methods!(
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
    unsafe impl NSSplitViewController {
        #[cfg(all(feature = "AppKit_NSSplitView", feature = "AppKit_NSView"))]
        #[method_id(@__retain_semantics Other splitView)]
        pub unsafe fn splitView(&self) -> Id<NSSplitView>;

        #[cfg(all(feature = "AppKit_NSSplitView", feature = "AppKit_NSView"))]
        #[method(setSplitView:)]
        pub unsafe fn setSplitView(&self, split_view: &NSSplitView);

        #[cfg(all(feature = "AppKit_NSSplitViewItem", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other splitViewItems)]
        pub unsafe fn splitViewItems(&self) -> Id<NSArray<NSSplitViewItem>>;

        #[cfg(all(feature = "AppKit_NSSplitViewItem", feature = "Foundation_NSArray"))]
        #[method(setSplitViewItems:)]
        pub unsafe fn setSplitViewItems(&self, split_view_items: &NSArray<NSSplitViewItem>);

        #[cfg(feature = "AppKit_NSSplitViewItem")]
        #[method(addSplitViewItem:)]
        pub unsafe fn addSplitViewItem(&self, split_view_item: &NSSplitViewItem);

        #[cfg(feature = "AppKit_NSSplitViewItem")]
        #[method(insertSplitViewItem:atIndex:)]
        pub unsafe fn insertSplitViewItem_atIndex(
            &self,
            split_view_item: &NSSplitViewItem,
            index: NSInteger,
        );

        #[cfg(feature = "AppKit_NSSplitViewItem")]
        #[method(removeSplitViewItem:)]
        pub unsafe fn removeSplitViewItem(&self, split_view_item: &NSSplitViewItem);

        #[cfg(feature = "AppKit_NSSplitViewItem")]
        #[method_id(@__retain_semantics Other splitViewItemForViewController:)]
        pub unsafe fn splitViewItemForViewController(
            &self,
            view_controller: &NSViewController,
        ) -> Option<Id<NSSplitViewItem>>;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(minimumThicknessForInlineSidebars)]
        pub unsafe fn minimumThicknessForInlineSidebars(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setMinimumThicknessForInlineSidebars:)]
        pub unsafe fn setMinimumThicknessForInlineSidebars(
            &self,
            minimum_thickness_for_inline_sidebars: CGFloat,
        );

        #[cfg(feature = "AppKit_NSUserInterfaceValidation")]
        #[method(validateUserInterfaceItem:)]
        pub unsafe fn validateUserInterfaceItem(
            &self,
            item: &ProtocolObject<dyn NSValidatedUserInterfaceItem>,
        ) -> bool;

        #[method(viewDidLoad)]
        pub unsafe fn viewDidLoad(&self);

        #[cfg(all(feature = "AppKit_NSSplitView", feature = "AppKit_NSView"))]
        #[method(splitView:canCollapseSubview:)]
        pub unsafe fn splitView_canCollapseSubview(
            &self,
            split_view: &NSSplitView,
            subview: &NSView,
        ) -> bool;

        #[cfg(all(feature = "AppKit_NSSplitView", feature = "AppKit_NSView"))]
        #[deprecated = "NSSplitView no longer supports collapsing sections via double-click. This delegate method is never called, and NSSplitViewController's implementation always returns NO."]
        #[method(splitView:shouldCollapseSubview:forDoubleClickOnDividerAtIndex:)]
        pub unsafe fn splitView_shouldCollapseSubview_forDoubleClickOnDividerAtIndex(
            &self,
            split_view: &NSSplitView,
            subview: &NSView,
            divider_index: NSInteger,
        ) -> bool;

        #[cfg(all(feature = "AppKit_NSSplitView", feature = "AppKit_NSView"))]
        #[method(splitView:shouldHideDividerAtIndex:)]
        pub unsafe fn splitView_shouldHideDividerAtIndex(
            &self,
            split_view: &NSSplitView,
            divider_index: NSInteger,
        ) -> bool;

        #[cfg(all(
            feature = "AppKit_NSSplitView",
            feature = "AppKit_NSView",
            feature = "Foundation_NSGeometry"
        ))]
        #[method(splitView:effectiveRect:forDrawnRect:ofDividerAtIndex:)]
        pub unsafe fn splitView_effectiveRect_forDrawnRect_ofDividerAtIndex(
            &self,
            split_view: &NSSplitView,
            proposed_effective_rect: NSRect,
            drawn_rect: NSRect,
            divider_index: NSInteger,
        ) -> NSRect;

        #[cfg(all(
            feature = "AppKit_NSSplitView",
            feature = "AppKit_NSView",
            feature = "Foundation_NSGeometry"
        ))]
        #[method(splitView:additionalEffectiveRectOfDividerAtIndex:)]
        pub unsafe fn splitView_additionalEffectiveRectOfDividerAtIndex(
            &self,
            split_view: &NSSplitView,
            divider_index: NSInteger,
        ) -> NSRect;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
    unsafe impl NSSplitViewController {
        #[cfg(all(
            feature = "AppKit_NSNib",
            feature = "Foundation_NSBundle",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
    unsafe impl NSSplitViewController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
    unsafe impl NSSplitViewController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_methods!(
    /// NSSplitViewControllerToggleSidebarAction
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
    unsafe impl NSSplitViewController {
        #[method(toggleSidebar:)]
        pub unsafe fn toggleSidebar(&self, sender: Option<&AnyObject>);

        #[method(toggleInspector:)]
        pub unsafe fn toggleInspector(&self, sender: Option<&AnyObject>);
    }
);
