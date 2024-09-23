//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTokenStyle(pub NSUInteger);
impl NSTokenStyle {
    #[doc(alias = "NSTokenStyleDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "NSTokenStyleNone")]
    pub const None: Self = Self(1);
    #[doc(alias = "NSTokenStyleRounded")]
    pub const Rounded: Self = Self(2);
    #[doc(alias = "NSTokenStyleSquared")]
    pub const Squared: Self = Self(3);
    #[doc(alias = "NSTokenStylePlainSquared")]
    pub const PlainSquared: Self = Self(4);
}

unsafe impl Encode for NSTokenStyle {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSTokenStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "NSActionCell",
        feature = "NSCell",
        feature = "NSTextFieldCell"
    ))]
    pub struct NSTokenFieldCell;

    #[cfg(all(
        feature = "NSActionCell",
        feature = "NSCell",
        feature = "NSTextFieldCell"
    ))]
    unsafe impl ClassType for NSTokenFieldCell {
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
unsafe impl NSAccessibility for NSTokenFieldCell {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSActionCell",
    feature = "NSCell",
    feature = "NSTextFieldCell"
))]
unsafe impl NSAccessibilityElementProtocol for NSTokenFieldCell {}

#[cfg(all(
    feature = "NSActionCell",
    feature = "NSCell",
    feature = "NSTextFieldCell"
))]
unsafe impl NSCoding for NSTokenFieldCell {}

#[cfg(all(
    feature = "NSActionCell",
    feature = "NSCell",
    feature = "NSTextFieldCell"
))]
unsafe impl NSCopying for NSTokenFieldCell {}

#[cfg(all(
    feature = "NSActionCell",
    feature = "NSCell",
    feature = "NSTextFieldCell"
))]
unsafe impl CopyingHelper for NSTokenFieldCell {
    type Result = Self;
}

#[cfg(all(
    feature = "NSActionCell",
    feature = "NSCell",
    feature = "NSTextFieldCell"
))]
unsafe impl NSObjectProtocol for NSTokenFieldCell {}

#[cfg(all(
    feature = "NSActionCell",
    feature = "NSCell",
    feature = "NSTextFieldCell",
    feature = "NSUserInterfaceItemIdentification"
))]
unsafe impl NSUserInterfaceItemIdentification for NSTokenFieldCell {}

extern_methods!(
    #[cfg(all(
        feature = "NSActionCell",
        feature = "NSCell",
        feature = "NSTextFieldCell"
    ))]
    unsafe impl NSTokenFieldCell {
        #[method(tokenStyle)]
        pub unsafe fn tokenStyle(&self) -> NSTokenStyle;

        #[method(setTokenStyle:)]
        pub unsafe fn setTokenStyle(&self, token_style: NSTokenStyle);

        #[method(completionDelay)]
        pub unsafe fn completionDelay(&self) -> NSTimeInterval;

        #[method(setCompletionDelay:)]
        pub unsafe fn setCompletionDelay(&self, completion_delay: NSTimeInterval);

        #[method(defaultCompletionDelay)]
        pub unsafe fn defaultCompletionDelay(mtm: MainThreadMarker) -> NSTimeInterval;

        #[method_id(@__retain_semantics Other tokenizingCharacterSet)]
        pub unsafe fn tokenizingCharacterSet(&self) -> Retained<NSCharacterSet>;

        #[method(setTokenizingCharacterSet:)]
        pub unsafe fn setTokenizingCharacterSet(
            &self,
            tokenizing_character_set: Option<&NSCharacterSet>,
        );

        #[method_id(@__retain_semantics Other defaultTokenizingCharacterSet)]
        pub unsafe fn defaultTokenizingCharacterSet(
            mtm: MainThreadMarker,
        ) -> Retained<NSCharacterSet>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn NSTokenFieldCellDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSTokenFieldCellDelegate>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTextFieldCell`
    #[cfg(all(
        feature = "NSActionCell",
        feature = "NSCell",
        feature = "NSTextFieldCell"
    ))]
    unsafe impl NSTokenFieldCell {
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
    unsafe impl NSTokenFieldCell {
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
    unsafe impl NSTokenFieldCell {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    pub unsafe trait NSTokenFieldCellDelegate: NSObjectProtocol + MainThreadOnly {
        #[cfg(all(
            feature = "NSActionCell",
            feature = "NSCell",
            feature = "NSTextFieldCell"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other tokenFieldCell:completionsForSubstring:indexOfToken:indexOfSelectedItem:)]
        unsafe fn tokenFieldCell_completionsForSubstring_indexOfToken_indexOfSelectedItem(
            &self,
            token_field_cell: &NSTokenFieldCell,
            substring: &NSString,
            token_index: NSInteger,
            selected_index: NonNull<NSInteger>,
        ) -> Retained<NSArray>;

        #[cfg(all(
            feature = "NSActionCell",
            feature = "NSCell",
            feature = "NSTextFieldCell"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other tokenFieldCell:shouldAddObjects:atIndex:)]
        unsafe fn tokenFieldCell_shouldAddObjects_atIndex(
            &self,
            token_field_cell: &NSTokenFieldCell,
            tokens: &NSArray,
            index: NSUInteger,
        ) -> Retained<NSArray>;

        #[cfg(all(
            feature = "NSActionCell",
            feature = "NSCell",
            feature = "NSTextFieldCell"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other tokenFieldCell:displayStringForRepresentedObject:)]
        unsafe fn tokenFieldCell_displayStringForRepresentedObject(
            &self,
            token_field_cell: &NSTokenFieldCell,
            represented_object: &AnyObject,
        ) -> Option<Retained<NSString>>;

        #[cfg(all(
            feature = "NSActionCell",
            feature = "NSCell",
            feature = "NSTextFieldCell"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other tokenFieldCell:editingStringForRepresentedObject:)]
        unsafe fn tokenFieldCell_editingStringForRepresentedObject(
            &self,
            token_field_cell: &NSTokenFieldCell,
            represented_object: &AnyObject,
        ) -> Option<Retained<NSString>>;

        #[cfg(all(
            feature = "NSActionCell",
            feature = "NSCell",
            feature = "NSTextFieldCell"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other tokenFieldCell:representedObjectForEditingString:)]
        unsafe fn tokenFieldCell_representedObjectForEditingString(
            &self,
            token_field_cell: &NSTokenFieldCell,
            editing_string: &NSString,
        ) -> Option<Retained<AnyObject>>;

        #[cfg(all(
            feature = "NSActionCell",
            feature = "NSCell",
            feature = "NSPasteboard",
            feature = "NSTextFieldCell"
        ))]
        #[optional]
        #[method(tokenFieldCell:writeRepresentedObjects:toPasteboard:)]
        unsafe fn tokenFieldCell_writeRepresentedObjects_toPasteboard(
            &self,
            token_field_cell: &NSTokenFieldCell,
            objects: &NSArray,
            pboard: &NSPasteboard,
        ) -> bool;

        #[cfg(all(
            feature = "NSActionCell",
            feature = "NSCell",
            feature = "NSPasteboard",
            feature = "NSTextFieldCell"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other tokenFieldCell:readFromPasteboard:)]
        unsafe fn tokenFieldCell_readFromPasteboard(
            &self,
            token_field_cell: &NSTokenFieldCell,
            pboard: &NSPasteboard,
        ) -> Option<Retained<NSArray>>;

        #[cfg(all(
            feature = "NSActionCell",
            feature = "NSCell",
            feature = "NSMenu",
            feature = "NSTextFieldCell"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other tokenFieldCell:menuForRepresentedObject:)]
        unsafe fn tokenFieldCell_menuForRepresentedObject(
            &self,
            token_field_cell: &NSTokenFieldCell,
            represented_object: &AnyObject,
        ) -> Option<Retained<NSMenu>>;

        #[cfg(all(
            feature = "NSActionCell",
            feature = "NSCell",
            feature = "NSTextFieldCell"
        ))]
        #[optional]
        #[method(tokenFieldCell:hasMenuForRepresentedObject:)]
        unsafe fn tokenFieldCell_hasMenuForRepresentedObject(
            &self,
            token_field_cell: &NSTokenFieldCell,
            represented_object: &AnyObject,
        ) -> bool;

        #[cfg(all(
            feature = "NSActionCell",
            feature = "NSCell",
            feature = "NSTextFieldCell"
        ))]
        #[optional]
        #[method(tokenFieldCell:styleForRepresentedObject:)]
        unsafe fn tokenFieldCell_styleForRepresentedObject(
            &self,
            token_field_cell: &NSTokenFieldCell,
            represented_object: &AnyObject,
        ) -> NSTokenStyle;
    }

    unsafe impl ProtocolType for dyn NSTokenFieldCellDelegate {}
);

pub static NSDefaultTokenStyle: NSTokenStyle = NSTokenStyle(NSTokenStyle::Default.0);

pub static NSPlainTextTokenStyle: NSTokenStyle = NSTokenStyle(NSTokenStyle::None.0);

pub static NSRoundedTokenStyle: NSTokenStyle = NSTokenStyle(NSTokenStyle::Rounded.0);
