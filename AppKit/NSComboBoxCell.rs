//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "NSActionCell",
        feature = "NSCell",
        feature = "NSTextFieldCell"
    ))]
    pub struct NSComboBoxCell;

    #[cfg(all(
        feature = "NSActionCell",
        feature = "NSCell",
        feature = "NSTextFieldCell"
    ))]
    unsafe impl ClassType for NSComboBoxCell {
        #[inherits(NSActionCell, NSCell, NSObject)]
        type Super = NSTextFieldCell;
    }
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSActionCell",
    feature = "NSCell",
    feature = "NSTextFieldCell"
))]
unsafe impl NSAccessibility for NSComboBoxCell {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSActionCell",
    feature = "NSCell",
    feature = "NSTextFieldCell"
))]
unsafe impl NSAccessibilityElementProtocol for NSComboBoxCell {}

#[cfg(all(
    feature = "NSActionCell",
    feature = "NSCell",
    feature = "NSTextFieldCell"
))]
unsafe impl NSCoding for NSComboBoxCell {}

#[cfg(all(
    feature = "NSActionCell",
    feature = "NSCell",
    feature = "NSTextFieldCell"
))]
unsafe impl NSCopying for NSComboBoxCell {}

#[cfg(all(
    feature = "NSActionCell",
    feature = "NSCell",
    feature = "NSTextFieldCell"
))]
unsafe impl CopyingHelper for NSComboBoxCell {
    type Result = Self;
}

#[cfg(all(
    feature = "NSActionCell",
    feature = "NSCell",
    feature = "NSTextFieldCell"
))]
unsafe impl NSObjectProtocol for NSComboBoxCell {}

#[cfg(all(
    feature = "NSActionCell",
    feature = "NSCell",
    feature = "NSTextFieldCell",
    feature = "NSUserInterfaceItemIdentification"
))]
unsafe impl NSUserInterfaceItemIdentification for NSComboBoxCell {}

extern_methods!(
    #[cfg(all(
        feature = "NSActionCell",
        feature = "NSCell",
        feature = "NSTextFieldCell"
    ))]
    unsafe impl NSComboBoxCell {
        #[method(hasVerticalScroller)]
        pub unsafe fn hasVerticalScroller(&self) -> bool;

        #[method(setHasVerticalScroller:)]
        pub unsafe fn setHasVerticalScroller(&self, has_vertical_scroller: bool);

        #[method(intercellSpacing)]
        pub unsafe fn intercellSpacing(&self) -> NSSize;

        #[method(setIntercellSpacing:)]
        pub unsafe fn setIntercellSpacing(&self, intercell_spacing: NSSize);

        #[method(itemHeight)]
        pub unsafe fn itemHeight(&self) -> CGFloat;

        #[method(setItemHeight:)]
        pub unsafe fn setItemHeight(&self, item_height: CGFloat);

        #[method(numberOfVisibleItems)]
        pub unsafe fn numberOfVisibleItems(&self) -> NSInteger;

        #[method(setNumberOfVisibleItems:)]
        pub unsafe fn setNumberOfVisibleItems(&self, number_of_visible_items: NSInteger);

        #[method(isButtonBordered)]
        pub unsafe fn isButtonBordered(&self) -> bool;

        #[method(setButtonBordered:)]
        pub unsafe fn setButtonBordered(&self, button_bordered: bool);

        #[method(reloadData)]
        pub unsafe fn reloadData(&self);

        #[method(noteNumberOfItemsChanged)]
        pub unsafe fn noteNumberOfItemsChanged(&self);

        #[method(usesDataSource)]
        pub unsafe fn usesDataSource(&self) -> bool;

        #[method(setUsesDataSource:)]
        pub unsafe fn setUsesDataSource(&self, uses_data_source: bool);

        #[method(scrollItemAtIndexToTop:)]
        pub unsafe fn scrollItemAtIndexToTop(&self, index: NSInteger);

        #[method(scrollItemAtIndexToVisible:)]
        pub unsafe fn scrollItemAtIndexToVisible(&self, index: NSInteger);

        #[method(selectItemAtIndex:)]
        pub unsafe fn selectItemAtIndex(&self, index: NSInteger);

        #[method(deselectItemAtIndex:)]
        pub unsafe fn deselectItemAtIndex(&self, index: NSInteger);

        #[method(indexOfSelectedItem)]
        pub unsafe fn indexOfSelectedItem(&self) -> NSInteger;

        #[method(numberOfItems)]
        pub unsafe fn numberOfItems(&self) -> NSInteger;

        #[method(completes)]
        pub unsafe fn completes(&self) -> bool;

        #[method(setCompletes:)]
        pub unsafe fn setCompletes(&self, completes: bool);

        #[method_id(@__retain_semantics Other completedString:)]
        pub unsafe fn completedString(&self, string: &NSString) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other dataSource)]
        pub unsafe fn dataSource(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn NSComboBoxCellDataSource>>>;

        #[method(setDataSource:)]
        pub unsafe fn setDataSource(
            &self,
            data_source: Option<&ProtocolObject<dyn NSComboBoxCellDataSource>>,
        );

        #[method(addItemWithObjectValue:)]
        pub unsafe fn addItemWithObjectValue(&self, object: &AnyObject);

        #[method(addItemsWithObjectValues:)]
        pub unsafe fn addItemsWithObjectValues(&self, objects: &NSArray);

        #[method(insertItemWithObjectValue:atIndex:)]
        pub unsafe fn insertItemWithObjectValue_atIndex(
            &self,
            object: &AnyObject,
            index: NSInteger,
        );

        #[method(removeItemWithObjectValue:)]
        pub unsafe fn removeItemWithObjectValue(&self, object: &AnyObject);

        #[method(removeItemAtIndex:)]
        pub unsafe fn removeItemAtIndex(&self, index: NSInteger);

        #[method(removeAllItems)]
        pub unsafe fn removeAllItems(&self);

        #[method(selectItemWithObjectValue:)]
        pub unsafe fn selectItemWithObjectValue(&self, object: Option<&AnyObject>);

        #[method_id(@__retain_semantics Other itemObjectValueAtIndex:)]
        pub unsafe fn itemObjectValueAtIndex(&self, index: NSInteger) -> Retained<AnyObject>;

        #[method_id(@__retain_semantics Other objectValueOfSelectedItem)]
        pub unsafe fn objectValueOfSelectedItem(&self) -> Option<Retained<AnyObject>>;

        #[method(indexOfItemWithObjectValue:)]
        pub unsafe fn indexOfItemWithObjectValue(&self, object: &AnyObject) -> NSInteger;

        #[method_id(@__retain_semantics Other objectValues)]
        pub unsafe fn objectValues(&self) -> Retained<NSArray>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTextFieldCell`
    #[cfg(all(
        feature = "NSActionCell",
        feature = "NSCell",
        feature = "NSTextFieldCell"
    ))]
    unsafe impl NSComboBoxCell {
        #[method_id(@__retain_semantics Init initTextCell:)]
        pub unsafe fn initTextCell(this: Allocated<Self>, string: &NSString) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Retained<Self>;

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Init initImageCell:)]
        pub unsafe fn initImageCell(
            this: Allocated<Self>,
            image: Option<&NSImage>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSCell`
    #[cfg(all(
        feature = "NSActionCell",
        feature = "NSCell",
        feature = "NSTextFieldCell"
    ))]
    unsafe impl NSComboBoxCell {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "NSActionCell",
        feature = "NSCell",
        feature = "NSTextFieldCell"
    ))]
    unsafe impl NSComboBoxCell {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    pub unsafe trait NSComboBoxCellDataSource: NSObjectProtocol {
        #[cfg(all(
            feature = "NSActionCell",
            feature = "NSCell",
            feature = "NSTextFieldCell"
        ))]
        #[optional]
        #[method(numberOfItemsInComboBoxCell:)]
        unsafe fn numberOfItemsInComboBoxCell(&self, combo_box_cell: &NSComboBoxCell) -> NSInteger;

        #[cfg(all(
            feature = "NSActionCell",
            feature = "NSCell",
            feature = "NSTextFieldCell"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other comboBoxCell:objectValueForItemAtIndex:)]
        unsafe fn comboBoxCell_objectValueForItemAtIndex(
            &self,
            combo_box_cell: &NSComboBoxCell,
            index: NSInteger,
        ) -> Retained<AnyObject>;

        #[cfg(all(
            feature = "NSActionCell",
            feature = "NSCell",
            feature = "NSTextFieldCell"
        ))]
        #[optional]
        #[method(comboBoxCell:indexOfItemWithStringValue:)]
        unsafe fn comboBoxCell_indexOfItemWithStringValue(
            &self,
            combo_box_cell: &NSComboBoxCell,
            string: &NSString,
        ) -> NSUInteger;

        #[cfg(all(
            feature = "NSActionCell",
            feature = "NSCell",
            feature = "NSTextFieldCell"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other comboBoxCell:completedString:)]
        unsafe fn comboBoxCell_completedString(
            &self,
            combo_box_cell: &NSComboBoxCell,
            uncompleted_string: &NSString,
        ) -> Option<Retained<NSString>>;
    }

    unsafe impl ProtocolType for dyn NSComboBoxCellDataSource {}
);
