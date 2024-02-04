//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

typed_extensible_enum!(
    pub type NSBindingName = NSString;
);

typed_enum!(
    pub type NSBindingOption = NSString;
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSBindingSelectionMarker")]
    pub struct NSBindingSelectionMarker;

    #[cfg(feature = "AppKit_NSBindingSelectionMarker")]
    unsafe impl ClassType for NSBindingSelectionMarker {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSBindingSelectionMarker")]
unsafe impl NSCopying for NSBindingSelectionMarker {}

#[cfg(feature = "AppKit_NSBindingSelectionMarker")]
unsafe impl NSObjectProtocol for NSBindingSelectionMarker {}

extern_methods!(
    #[cfg(feature = "AppKit_NSBindingSelectionMarker")]
    unsafe impl NSBindingSelectionMarker {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Other multipleValuesSelectionMarker)]
        pub unsafe fn multipleValuesSelectionMarker() -> Id<NSBindingSelectionMarker>;

        #[method_id(@__retain_semantics Other noSelectionMarker)]
        pub unsafe fn noSelectionMarker() -> Id<NSBindingSelectionMarker>;

        #[method_id(@__retain_semantics Other notApplicableSelectionMarker)]
        pub unsafe fn notApplicableSelectionMarker() -> Id<NSBindingSelectionMarker>;

        #[method(setDefaultPlaceholder:forMarker:onClass:withBinding:)]
        pub unsafe fn setDefaultPlaceholder_forMarker_onClass_withBinding(
            placeholder: Option<&AnyObject>,
            marker: Option<&NSBindingSelectionMarker>,
            object_class: &AnyClass,
            binding: &NSBindingName,
        );

        #[method_id(@__retain_semantics Other defaultPlaceholderForMarker:onClass:withBinding:)]
        pub unsafe fn defaultPlaceholderForMarker_onClass_withBinding(
            marker: Option<&NSBindingSelectionMarker>,
            object_class: &AnyClass,
            binding: &NSBindingName,
        ) -> Option<Id<AnyObject>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSBindingSelectionMarker")]
    unsafe impl NSBindingSelectionMarker {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_static!(NSMultipleValuesMarker: &'static AnyObject);

extern_static!(NSNoSelectionMarker: &'static AnyObject);

extern_static!(NSNotApplicableMarker: &'static AnyObject);

extern_fn!(
    pub unsafe fn NSIsControllerMarker(object: Option<&AnyObject>) -> Bool;
);

typed_enum!(
    pub type NSBindingInfoKey = NSString;
);

extern_static!(NSObservedObjectKey: &'static NSBindingInfoKey);

extern_static!(NSObservedKeyPathKey: &'static NSBindingInfoKey);

extern_static!(NSOptionsKey: &'static NSBindingInfoKey);

extern_protocol!(
    pub unsafe trait NSEditor: NSObjectProtocol + IsMainThreadOnly {
        #[method(discardEditing)]
        unsafe fn discardEditing(&self);

        #[method(commitEditing)]
        unsafe fn commitEditing(&self) -> bool;

        #[method(commitEditingWithDelegate:didCommitSelector:contextInfo:)]
        unsafe fn commitEditingWithDelegate_didCommitSelector_contextInfo(
            &self,
            delegate: Option<&AnyObject>,
            did_commit_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(commitEditingAndReturnError:_)]
        unsafe fn commitEditingAndReturnError(&self) -> Result<(), Id<NSError>>;
    }

    unsafe impl ProtocolType for dyn NSEditor {}
);

extern_protocol!(
    pub unsafe trait NSEditorRegistration: NSObjectProtocol + IsMainThreadOnly {
        #[optional]
        #[method(objectDidBeginEditing:)]
        unsafe fn objectDidBeginEditing(&self, editor: &ProtocolObject<dyn NSEditor>);

        #[optional]
        #[method(objectDidEndEditing:)]
        unsafe fn objectDidEndEditing(&self, editor: &ProtocolObject<dyn NSEditor>);
    }

    unsafe impl ProtocolType for dyn NSEditorRegistration {}
);

extern_static!(NSAlignmentBinding: &'static NSBindingName);

extern_static!(NSAlternateImageBinding: &'static NSBindingName);

extern_static!(NSAlternateTitleBinding: &'static NSBindingName);

extern_static!(NSAnimateBinding: &'static NSBindingName);

extern_static!(NSAnimationDelayBinding: &'static NSBindingName);

extern_static!(NSArgumentBinding: &'static NSBindingName);

extern_static!(NSAttributedStringBinding: &'static NSBindingName);

extern_static!(NSContentArrayBinding: &'static NSBindingName);

extern_static!(NSContentArrayForMultipleSelectionBinding: &'static NSBindingName);

extern_static!(NSContentBinding: &'static NSBindingName);

extern_static!(NSContentDictionaryBinding: &'static NSBindingName);

extern_static!(NSContentHeightBinding: &'static NSBindingName);

extern_static!(NSContentObjectBinding: &'static NSBindingName);

extern_static!(NSContentObjectsBinding: &'static NSBindingName);

extern_static!(NSContentSetBinding: &'static NSBindingName);

extern_static!(NSContentValuesBinding: &'static NSBindingName);

extern_static!(NSContentWidthBinding: &'static NSBindingName);

extern_static!(NSCriticalValueBinding: &'static NSBindingName);

extern_static!(NSDataBinding: &'static NSBindingName);

extern_static!(NSDisplayPatternTitleBinding: &'static NSBindingName);

extern_static!(NSDisplayPatternValueBinding: &'static NSBindingName);

extern_static!(NSDocumentEditedBinding: &'static NSBindingName);

extern_static!(NSDoubleClickArgumentBinding: &'static NSBindingName);

extern_static!(NSDoubleClickTargetBinding: &'static NSBindingName);

extern_static!(NSEditableBinding: &'static NSBindingName);

extern_static!(NSEnabledBinding: &'static NSBindingName);

extern_static!(NSExcludedKeysBinding: &'static NSBindingName);

extern_static!(NSFilterPredicateBinding: &'static NSBindingName);

extern_static!(NSFontBinding: &'static NSBindingName);

extern_static!(NSFontBoldBinding: &'static NSBindingName);

extern_static!(NSFontFamilyNameBinding: &'static NSBindingName);

extern_static!(NSFontItalicBinding: &'static NSBindingName);

extern_static!(NSFontNameBinding: &'static NSBindingName);

extern_static!(NSFontSizeBinding: &'static NSBindingName);

extern_static!(NSHeaderTitleBinding: &'static NSBindingName);

extern_static!(NSHiddenBinding: &'static NSBindingName);

extern_static!(NSImageBinding: &'static NSBindingName);

extern_static!(NSIncludedKeysBinding: &'static NSBindingName);

extern_static!(NSInitialKeyBinding: &'static NSBindingName);

extern_static!(NSInitialValueBinding: &'static NSBindingName);

extern_static!(NSIsIndeterminateBinding: &'static NSBindingName);

extern_static!(NSLabelBinding: &'static NSBindingName);

extern_static!(NSLocalizedKeyDictionaryBinding: &'static NSBindingName);

extern_static!(NSManagedObjectContextBinding: &'static NSBindingName);

extern_static!(NSMaximumRecentsBinding: &'static NSBindingName);

extern_static!(NSMaxValueBinding: &'static NSBindingName);

extern_static!(NSMaxWidthBinding: &'static NSBindingName);

extern_static!(NSMinValueBinding: &'static NSBindingName);

extern_static!(NSMinWidthBinding: &'static NSBindingName);

extern_static!(NSMixedStateImageBinding: &'static NSBindingName);

extern_static!(NSOffStateImageBinding: &'static NSBindingName);

extern_static!(NSOnStateImageBinding: &'static NSBindingName);

extern_static!(NSPositioningRectBinding: &'static NSBindingName);

extern_static!(NSPredicateBinding: &'static NSBindingName);

extern_static!(NSRecentSearchesBinding: &'static NSBindingName);

extern_static!(NSRepresentedFilenameBinding: &'static NSBindingName);

extern_static!(NSRowHeightBinding: &'static NSBindingName);

extern_static!(NSSelectedIdentifierBinding: &'static NSBindingName);

extern_static!(NSSelectedIndexBinding: &'static NSBindingName);

extern_static!(NSSelectedLabelBinding: &'static NSBindingName);

extern_static!(NSSelectedObjectBinding: &'static NSBindingName);

extern_static!(NSSelectedObjectsBinding: &'static NSBindingName);

extern_static!(NSSelectedTagBinding: &'static NSBindingName);

extern_static!(NSSelectedValueBinding: &'static NSBindingName);

extern_static!(NSSelectedValuesBinding: &'static NSBindingName);

extern_static!(NSSelectionIndexesBinding: &'static NSBindingName);

extern_static!(NSSelectionIndexPathsBinding: &'static NSBindingName);

extern_static!(NSSortDescriptorsBinding: &'static NSBindingName);

extern_static!(NSTargetBinding: &'static NSBindingName);

extern_static!(NSTextColorBinding: &'static NSBindingName);

extern_static!(NSTitleBinding: &'static NSBindingName);

extern_static!(NSToolTipBinding: &'static NSBindingName);

extern_static!(NSTransparentBinding: &'static NSBindingName);

extern_static!(NSValueBinding: &'static NSBindingName);

extern_static!(NSValuePathBinding: &'static NSBindingName);

extern_static!(NSValueURLBinding: &'static NSBindingName);

extern_static!(NSVisibleBinding: &'static NSBindingName);

extern_static!(NSWarningValueBinding: &'static NSBindingName);

extern_static!(NSWidthBinding: &'static NSBindingName);

extern_static!(NSAllowsEditingMultipleValuesSelectionBindingOption: &'static NSBindingOption);

extern_static!(NSAllowsNullArgumentBindingOption: &'static NSBindingOption);

extern_static!(NSAlwaysPresentsApplicationModalAlertsBindingOption: &'static NSBindingOption);

extern_static!(NSConditionallySetsEditableBindingOption: &'static NSBindingOption);

extern_static!(NSConditionallySetsEnabledBindingOption: &'static NSBindingOption);

extern_static!(NSConditionallySetsHiddenBindingOption: &'static NSBindingOption);

extern_static!(NSContinuouslyUpdatesValueBindingOption: &'static NSBindingOption);

extern_static!(NSCreatesSortDescriptorBindingOption: &'static NSBindingOption);

extern_static!(NSDeletesObjectsOnRemoveBindingsOption: &'static NSBindingOption);

extern_static!(NSDisplayNameBindingOption: &'static NSBindingOption);

extern_static!(NSDisplayPatternBindingOption: &'static NSBindingOption);

extern_static!(NSContentPlacementTagBindingOption: &'static NSBindingOption);

extern_static!(NSHandlesContentAsCompoundValueBindingOption: &'static NSBindingOption);

extern_static!(NSInsertsNullPlaceholderBindingOption: &'static NSBindingOption);

extern_static!(NSInvokesSeparatelyWithArrayObjectsBindingOption: &'static NSBindingOption);

extern_static!(NSMultipleValuesPlaceholderBindingOption: &'static NSBindingOption);

extern_static!(NSNoSelectionPlaceholderBindingOption: &'static NSBindingOption);

extern_static!(NSNotApplicablePlaceholderBindingOption: &'static NSBindingOption);

extern_static!(NSNullPlaceholderBindingOption: &'static NSBindingOption);

extern_static!(NSRaisesForNotApplicableKeysBindingOption: &'static NSBindingOption);

extern_static!(NSPredicateFormatBindingOption: &'static NSBindingOption);

extern_static!(NSSelectorNameBindingOption: &'static NSBindingOption);

extern_static!(NSSelectsAllWhenSettingContentBindingOption: &'static NSBindingOption);

extern_static!(NSValidatesImmediatelyBindingOption: &'static NSBindingOption);

extern_static!(NSValueTransformerNameBindingOption: &'static NSBindingOption);

extern_static!(NSValueTransformerBindingOption: &'static NSBindingOption);
