//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSTextStorageEditActions {
        NSTextStorageEditedAttributes = 1 << 0,
        NSTextStorageEditedCharacters = 1 << 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextStorage;

    unsafe impl ClassType for NSTextStorage {
        #[inherits(NSAttributedString, NSObject)]
        type Super = NSMutableAttributedString;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSTextStorage")]
    unsafe impl NSTextStorage {
        #[cfg(feature = "AppKit_NSLayoutManager")]
        #[method_id(@__retain_semantics Other layoutManagers)]
        pub unsafe fn layoutManagers(&self) -> Id<NSArray<NSLayoutManager>, Shared>;

        #[cfg(feature = "AppKit_NSLayoutManager")]
        #[method(addLayoutManager:)]
        pub unsafe fn addLayoutManager(&self, aLayoutManager: &NSLayoutManager);

        #[cfg(feature = "AppKit_NSLayoutManager")]
        #[method(removeLayoutManager:)]
        pub unsafe fn removeLayoutManager(&self, aLayoutManager: &NSLayoutManager);

        #[method(editedMask)]
        pub unsafe fn editedMask(&self) -> NSTextStorageEditActions;

        #[method(editedRange)]
        pub unsafe fn editedRange(&self) -> NSRange;

        #[method(changeInLength)]
        pub unsafe fn changeInLength(&self) -> NSInteger;

        #[cfg(feature = "AppKit_NSTextStorageDelegate")]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSTextStorageDelegate, Shared>>;

        #[cfg(feature = "AppKit_NSTextStorageDelegate")]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSTextStorageDelegate>);

        #[method(edited:range:changeInLength:)]
        pub unsafe fn edited_range_changeInLength(
            &self,
            editedMask: NSTextStorageEditActions,
            editedRange: NSRange,
            delta: NSInteger,
        );

        #[method(processEditing)]
        pub unsafe fn processEditing(&self);

        #[method(fixesAttributesLazily)]
        pub unsafe fn fixesAttributesLazily(&self) -> bool;

        #[method(invalidateAttributesInRange:)]
        pub unsafe fn invalidateAttributesInRange(&self, range: NSRange);

        #[method(ensureAttributesAreFixedInRange:)]
        pub unsafe fn ensureAttributesAreFixedInRange(&self, range: NSRange);

        #[cfg(feature = "AppKit_NSTextStorageObserving")]
        #[method_id(@__retain_semantics Other textStorageObserver)]
        pub unsafe fn textStorageObserver(&self) -> Option<Id<NSTextStorageObserving, Shared>>;

        #[cfg(feature = "AppKit_NSTextStorageObserving")]
        #[method(setTextStorageObserver:)]
        pub unsafe fn setTextStorageObserver(
            &self,
            textStorageObserver: Option<&NSTextStorageObserving>,
        );
    }
);

extern_protocol!(
    pub struct NSTextStorageDelegate;

    unsafe impl ProtocolType for NSTextStorageDelegate {
        #[optional]
        #[method(textStorage:willProcessEditing:range:changeInLength:)]
        pub unsafe fn textStorage_willProcessEditing_range_changeInLength(
            &self,
            textStorage: &NSTextStorage,
            editedMask: NSTextStorageEditActions,
            editedRange: NSRange,
            delta: NSInteger,
        );

        #[optional]
        #[method(textStorage:didProcessEditing:range:changeInLength:)]
        pub unsafe fn textStorage_didProcessEditing_range_changeInLength(
            &self,
            textStorage: &NSTextStorage,
            editedMask: NSTextStorageEditActions,
            editedRange: NSRange,
            delta: NSInteger,
        );
    }
);

extern_static!(NSTextStorageWillProcessEditingNotification: &'static NSNotificationName);

extern_static!(NSTextStorageDidProcessEditingNotification: &'static NSNotificationName);

extern_protocol!(
    pub struct NSTextStorageObserving;

    unsafe impl ProtocolType for NSTextStorageObserving {
        #[method_id(@__retain_semantics Other textStorage)]
        pub unsafe fn textStorage(&self) -> Option<Id<NSTextStorage, Shared>>;

        #[method(setTextStorage:)]
        pub unsafe fn setTextStorage(&self, textStorage: Option<&NSTextStorage>);

        #[method(processEditingForTextStorage:edited:range:changeInLength:invalidatedRange:)]
        pub unsafe fn processEditingForTextStorage_edited_range_changeInLength_invalidatedRange(
            &self,
            textStorage: &NSTextStorage,
            editMask: NSTextStorageEditActions,
            newCharRange: NSRange,
            delta: NSInteger,
            invalidatedCharRange: NSRange,
        );

        #[method(performEditingTransactionForTextStorage:usingBlock:)]
        pub unsafe fn performEditingTransactionForTextStorage_usingBlock(
            &self,
            textStorage: &NSTextStorage,
            transaction: &Block<(), ()>,
        );
    }
);

pub type NSTextStorageEditedOptions = NSUInteger;

extern_methods!(
    /// Methods declared on superclass `NSAttributedString`
    ///
    /// NSExtendedAttributedString
    #[cfg(feature = "AppKit_NSTextStorage")]
    unsafe impl NSTextStorage {
        #[method_id(@__retain_semantics Init initWithString:)]
        pub unsafe fn initWithString(
            this: Option<Allocated<Self>>,
            str: &NSString,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithString:attributes:)]
        pub unsafe fn initWithString_attributes(
            this: Option<Allocated<Self>>,
            str: &NSString,
            attrs: Option<&NSDictionary<NSAttributedStringKey, Object>>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithAttributedString:)]
        pub unsafe fn initWithAttributedString(
            this: Option<Allocated<Self>>,
            attrStr: &NSAttributedString,
        ) -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSAttributedString`
    ///
    /// NSAttributedStringCreateFromMarkdown
    #[cfg(feature = "AppKit_NSTextStorage")]
    unsafe impl NSTextStorage {
        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Init initWithContentsOfMarkdownFileAtURL:options:baseURL:error:_)]
        pub unsafe fn initWithContentsOfMarkdownFileAtURL_options_baseURL_error(
            this: Option<Allocated<Self>>,
            markdownFile: &NSURL,
            options: Option<&NSAttributedStringMarkdownParsingOptions>,
            baseURL: Option<&NSURL>,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Init initWithMarkdown:options:baseURL:error:_)]
        pub unsafe fn initWithMarkdown_options_baseURL_error(
            this: Option<Allocated<Self>>,
            markdown: &NSData,
            options: Option<&NSAttributedStringMarkdownParsingOptions>,
            baseURL: Option<&NSURL>,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Init initWithMarkdownString:options:baseURL:error:_)]
        pub unsafe fn initWithMarkdownString_options_baseURL_error(
            this: Option<Allocated<Self>>,
            markdownString: &NSString,
            options: Option<&NSAttributedStringMarkdownParsingOptions>,
            baseURL: Option<&NSURL>,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;
    }
);
