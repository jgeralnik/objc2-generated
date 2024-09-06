//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

pub static NSSearchFieldRecentsTitleMenuItemTag: NSInteger = 1000;

pub static NSSearchFieldRecentsMenuItemTag: NSInteger = 1001;

pub static NSSearchFieldClearRecentsMenuItemTag: NSInteger = 1002;

pub static NSSearchFieldNoRecentsMenuItemTag: NSInteger = 1003;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "NSActionCell",
        feature = "NSCell",
        feature = "NSTextFieldCell"
    ))]
    pub struct NSSearchFieldCell;

    #[cfg(all(
        feature = "NSActionCell",
        feature = "NSCell",
        feature = "NSTextFieldCell"
    ))]
    unsafe impl ClassType for NSSearchFieldCell {
        #[inherits(NSActionCell, NSCell, NSObject)]
        type Super = NSTextFieldCell;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSActionCell",
    feature = "NSCell",
    feature = "NSTextFieldCell"
))]
unsafe impl NSAccessibility for NSSearchFieldCell {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSActionCell",
    feature = "NSCell",
    feature = "NSTextFieldCell"
))]
unsafe impl NSAccessibilityElementProtocol for NSSearchFieldCell {}

#[cfg(all(
    feature = "NSActionCell",
    feature = "NSCell",
    feature = "NSTextFieldCell"
))]
unsafe impl NSCoding for NSSearchFieldCell {}

#[cfg(all(
    feature = "NSActionCell",
    feature = "NSCell",
    feature = "NSTextFieldCell"
))]
unsafe impl NSCopying for NSSearchFieldCell {}

#[cfg(all(
    feature = "NSActionCell",
    feature = "NSCell",
    feature = "NSTextFieldCell"
))]
unsafe impl CopyingHelper for NSSearchFieldCell {
    type Result = Self;
}

#[cfg(all(
    feature = "NSActionCell",
    feature = "NSCell",
    feature = "NSTextFieldCell"
))]
unsafe impl NSObjectProtocol for NSSearchFieldCell {}

#[cfg(all(
    feature = "NSActionCell",
    feature = "NSCell",
    feature = "NSTextFieldCell",
    feature = "NSUserInterfaceItemIdentification"
))]
unsafe impl NSUserInterfaceItemIdentification for NSSearchFieldCell {}

extern_methods!(
    #[cfg(all(
        feature = "NSActionCell",
        feature = "NSCell",
        feature = "NSTextFieldCell"
    ))]
    unsafe impl NSSearchFieldCell {
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

        #[cfg(feature = "NSButtonCell")]
        #[method_id(@__retain_semantics Other searchButtonCell)]
        pub unsafe fn searchButtonCell(&self) -> Option<Retained<NSButtonCell>>;

        #[cfg(feature = "NSButtonCell")]
        #[method(setSearchButtonCell:)]
        pub unsafe fn setSearchButtonCell(&self, search_button_cell: Option<&NSButtonCell>);

        #[cfg(feature = "NSButtonCell")]
        #[method_id(@__retain_semantics Other cancelButtonCell)]
        pub unsafe fn cancelButtonCell(&self) -> Option<Retained<NSButtonCell>>;

        #[cfg(feature = "NSButtonCell")]
        #[method(setCancelButtonCell:)]
        pub unsafe fn setCancelButtonCell(&self, cancel_button_cell: Option<&NSButtonCell>);

        #[method(resetSearchButtonCell)]
        pub unsafe fn resetSearchButtonCell(&self);

        #[method(resetCancelButtonCell)]
        pub unsafe fn resetCancelButtonCell(&self);

        #[method(searchTextRectForBounds:)]
        pub unsafe fn searchTextRectForBounds(&self, rect: NSRect) -> NSRect;

        #[method(searchButtonRectForBounds:)]
        pub unsafe fn searchButtonRectForBounds(&self, rect: NSRect) -> NSRect;

        #[method(cancelButtonRectForBounds:)]
        pub unsafe fn cancelButtonRectForBounds(&self, rect: NSRect) -> NSRect;

        #[cfg(feature = "NSMenu")]
        #[method_id(@__retain_semantics Other searchMenuTemplate)]
        pub unsafe fn searchMenuTemplate(&self) -> Option<Retained<NSMenu>>;

        #[cfg(feature = "NSMenu")]
        #[method(setSearchMenuTemplate:)]
        pub unsafe fn setSearchMenuTemplate(&self, search_menu_template: Option<&NSMenu>);

        #[method(sendsWholeSearchString)]
        pub unsafe fn sendsWholeSearchString(&self) -> bool;

        #[method(setSendsWholeSearchString:)]
        pub unsafe fn setSendsWholeSearchString(&self, sends_whole_search_string: bool);

        #[method(maximumRecents)]
        pub unsafe fn maximumRecents(&self) -> NSInteger;

        #[method(setMaximumRecents:)]
        pub unsafe fn setMaximumRecents(&self, maximum_recents: NSInteger);

        #[method_id(@__retain_semantics Other recentSearches)]
        pub unsafe fn recentSearches(&self) -> Retained<NSArray<NSString>>;

        #[method(setRecentSearches:)]
        pub unsafe fn setRecentSearches(&self, recent_searches: Option<&NSArray<NSString>>);

        #[cfg(feature = "NSSearchField")]
        #[method_id(@__retain_semantics Other recentsAutosaveName)]
        pub unsafe fn recentsAutosaveName(
            &self,
        ) -> Option<Retained<NSSearchFieldRecentsAutosaveName>>;

        #[cfg(feature = "NSSearchField")]
        #[method(setRecentsAutosaveName:)]
        pub unsafe fn setRecentsAutosaveName(
            &self,
            recents_autosave_name: Option<&NSSearchFieldRecentsAutosaveName>,
        );

        #[method(sendsSearchStringImmediately)]
        pub unsafe fn sendsSearchStringImmediately(&self) -> bool;

        #[method(setSendsSearchStringImmediately:)]
        pub unsafe fn setSendsSearchStringImmediately(&self, sends_search_string_immediately: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSCell`
    #[cfg(all(
        feature = "NSActionCell",
        feature = "NSCell",
        feature = "NSTextFieldCell"
    ))]
    unsafe impl NSSearchFieldCell {
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
    unsafe impl NSSearchFieldCell {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
