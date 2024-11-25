//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTableColumnResizingOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSTableColumnResizingOptions: NSUInteger {
        const NSTableColumnNoResizing = 0;
        const NSTableColumnAutoresizingMask = 1<<0;
        const NSTableColumnUserResizingMask = 1<<1;
    }
}

unsafe impl Encode for NSTableColumnResizingOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSTableColumnResizingOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTableColumn;
);

unsafe impl NSCoding for NSTableColumn {}

unsafe impl NSObjectProtocol for NSTableColumn {}

#[cfg(feature = "NSUserInterfaceItemIdentification")]
unsafe impl NSUserInterfaceItemIdentification for NSTableColumn {}

extern_methods!(
    unsafe impl NSTableColumn {
        #[cfg(feature = "NSUserInterfaceItemIdentification")]
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Allocated<Self>,
            identifier: &NSUserInterfaceItemIdentifier,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Retained<Self>;

        #[cfg(feature = "NSUserInterfaceItemIdentification")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSUserInterfaceItemIdentifier>;

        #[cfg(feature = "NSUserInterfaceItemIdentification")]
        #[method(setIdentifier:)]
        pub unsafe fn setIdentifier(&self, identifier: &NSUserInterfaceItemIdentifier);

        #[cfg(all(
            feature = "NSControl",
            feature = "NSResponder",
            feature = "NSTableView",
            feature = "NSView"
        ))]
        #[method_id(@__retain_semantics Other tableView)]
        pub unsafe fn tableView(&self) -> Option<Retained<NSTableView>>;

        #[cfg(all(
            feature = "NSControl",
            feature = "NSResponder",
            feature = "NSTableView",
            feature = "NSView"
        ))]
        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setTableView:)]
        pub unsafe fn setTableView(&self, table_view: Option<&NSTableView>);

        #[method(width)]
        pub unsafe fn width(&self) -> CGFloat;

        #[method(setWidth:)]
        pub unsafe fn setWidth(&self, width: CGFloat);

        #[method(minWidth)]
        pub unsafe fn minWidth(&self) -> CGFloat;

        #[method(setMinWidth:)]
        pub unsafe fn setMinWidth(&self, min_width: CGFloat);

        #[method(maxWidth)]
        pub unsafe fn maxWidth(&self) -> CGFloat;

        #[method(setMaxWidth:)]
        pub unsafe fn setMaxWidth(&self, max_width: CGFloat);

        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Retained<NSString>;

        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[cfg(all(
            feature = "NSActionCell",
            feature = "NSCell",
            feature = "NSTableHeaderCell",
            feature = "NSTextFieldCell"
        ))]
        #[method_id(@__retain_semantics Other headerCell)]
        pub unsafe fn headerCell(&self) -> Retained<NSTableHeaderCell>;

        #[cfg(all(
            feature = "NSActionCell",
            feature = "NSCell",
            feature = "NSTableHeaderCell",
            feature = "NSTextFieldCell"
        ))]
        #[method(setHeaderCell:)]
        pub unsafe fn setHeaderCell(&self, header_cell: &NSTableHeaderCell);

        #[method(isEditable)]
        pub unsafe fn isEditable(&self) -> bool;

        #[method(setEditable:)]
        pub unsafe fn setEditable(&self, editable: bool);

        #[method(sizeToFit)]
        pub unsafe fn sizeToFit(&self);

        #[method_id(@__retain_semantics Other sortDescriptorPrototype)]
        pub unsafe fn sortDescriptorPrototype(&self) -> Option<Retained<NSSortDescriptor>>;

        #[method(setSortDescriptorPrototype:)]
        pub unsafe fn setSortDescriptorPrototype(
            &self,
            sort_descriptor_prototype: Option<&NSSortDescriptor>,
        );

        #[method(resizingMask)]
        pub unsafe fn resizingMask(&self) -> NSTableColumnResizingOptions;

        #[method(setResizingMask:)]
        pub unsafe fn setResizingMask(&self, resizing_mask: NSTableColumnResizingOptions);

        #[method_id(@__retain_semantics Other headerToolTip)]
        pub unsafe fn headerToolTip(&self) -> Option<Retained<NSString>>;

        #[method(setHeaderToolTip:)]
        pub unsafe fn setHeaderToolTip(&self, header_tool_tip: Option<&NSString>);

        #[method(isHidden)]
        pub unsafe fn isHidden(&self) -> bool;

        #[method(setHidden:)]
        pub unsafe fn setHidden(&self, hidden: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSTableColumn {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    /// NSDeprecated
    unsafe impl NSTableColumn {
        #[deprecated]
        #[method(setResizable:)]
        pub unsafe fn setResizable(&self, flag: bool);

        #[deprecated]
        #[method(isResizable)]
        pub unsafe fn isResizable(&self) -> bool;

        #[method_id(@__retain_semantics Other dataCell)]
        pub unsafe fn dataCell(&self) -> Retained<AnyObject>;

        #[method(setDataCell:)]
        pub unsafe fn setDataCell(&self, data_cell: &AnyObject);

        #[method_id(@__retain_semantics Other dataCellForRow:)]
        pub unsafe fn dataCellForRow(&self, row: NSInteger) -> Retained<AnyObject>;
    }
);
