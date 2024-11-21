//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_TYPED_ENUM
pub type NSRuleEditorPredicatePartKey = NSString;

extern "C" {
    pub static NSRuleEditorPredicateLeftExpression: &'static NSRuleEditorPredicatePartKey;
}

extern "C" {
    pub static NSRuleEditorPredicateRightExpression: &'static NSRuleEditorPredicatePartKey;
}

extern "C" {
    pub static NSRuleEditorPredicateComparisonModifier: &'static NSRuleEditorPredicatePartKey;
}

extern "C" {
    pub static NSRuleEditorPredicateOptions: &'static NSRuleEditorPredicatePartKey;
}

extern "C" {
    pub static NSRuleEditorPredicateOperatorType: &'static NSRuleEditorPredicatePartKey;
}

extern "C" {
    pub static NSRuleEditorPredicateCustomSelector: &'static NSRuleEditorPredicatePartKey;
}

extern "C" {
    pub static NSRuleEditorPredicateCompoundType: &'static NSRuleEditorPredicatePartKey;
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSRuleEditorNestingMode(pub NSUInteger);
impl NSRuleEditorNestingMode {
    #[doc(alias = "NSRuleEditorNestingModeSingle")]
    pub const Single: Self = Self(0);
    #[doc(alias = "NSRuleEditorNestingModeList")]
    pub const List: Self = Self(1);
    #[doc(alias = "NSRuleEditorNestingModeCompound")]
    pub const Compound: Self = Self(2);
    #[doc(alias = "NSRuleEditorNestingModeSimple")]
    pub const Simple: Self = Self(3);
}

unsafe impl Encode for NSRuleEditorNestingMode {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSRuleEditorNestingMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSRuleEditorRowType(pub NSUInteger);
impl NSRuleEditorRowType {
    #[doc(alias = "NSRuleEditorRowTypeSimple")]
    pub const Simple: Self = Self(0);
    #[doc(alias = "NSRuleEditorRowTypeCompound")]
    pub const Compound: Self = Self(1);
}

unsafe impl Encode for NSRuleEditorRowType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSRuleEditorRowType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[unsafe(super(NSControl, NSView, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    pub struct NSRuleEditor;
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibility for NSRuleEditor {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibilityElementProtocol for NSRuleEditor {}

#[cfg(all(
    feature = "NSAnimation",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAnimatablePropertyContainer for NSRuleEditor {}

#[cfg(all(
    feature = "NSAppearance",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAppearanceCustomization for NSRuleEditor {}

#[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSCoding for NSRuleEditor {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSDragging",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSDraggingDestination for NSRuleEditor {}

#[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSObjectProtocol for NSRuleEditor {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSUserInterfaceItemIdentification",
    feature = "NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for NSRuleEditor {}

extern_methods!(
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSRuleEditor {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self)
            -> Option<Retained<ProtocolObject<dyn NSRuleEditorDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSRuleEditorDelegate>>,
        );

        #[method_id(@__retain_semantics Other formattingStringsFilename)]
        pub unsafe fn formattingStringsFilename(&self) -> Option<Retained<NSString>>;

        #[method(setFormattingStringsFilename:)]
        pub unsafe fn setFormattingStringsFilename(
            &self,
            formatting_strings_filename: Option<&NSString>,
        );

        #[method_id(@__retain_semantics Other formattingDictionary)]
        pub unsafe fn formattingDictionary(
            &self,
        ) -> Option<Retained<NSDictionary<NSString, NSString>>>;

        #[method(setFormattingDictionary:)]
        pub unsafe fn setFormattingDictionary(
            &self,
            formatting_dictionary: Option<&NSDictionary<NSString, NSString>>,
        );

        #[method(reloadCriteria)]
        pub unsafe fn reloadCriteria(&self);

        #[method(nestingMode)]
        pub unsafe fn nestingMode(&self) -> NSRuleEditorNestingMode;

        #[method(setNestingMode:)]
        pub unsafe fn setNestingMode(&self, nesting_mode: NSRuleEditorNestingMode);

        #[method(rowHeight)]
        pub unsafe fn rowHeight(&self) -> CGFloat;

        #[method(setRowHeight:)]
        pub unsafe fn setRowHeight(&self, row_height: CGFloat);

        #[method(isEditable)]
        pub unsafe fn isEditable(&self) -> bool;

        #[method(setEditable:)]
        pub unsafe fn setEditable(&self, editable: bool);

        #[method(canRemoveAllRows)]
        pub unsafe fn canRemoveAllRows(&self) -> bool;

        #[method(setCanRemoveAllRows:)]
        pub unsafe fn setCanRemoveAllRows(&self, can_remove_all_rows: bool);

        #[method_id(@__retain_semantics Other predicate)]
        pub unsafe fn predicate(&self) -> Option<Retained<NSPredicate>>;

        #[method(reloadPredicate)]
        pub unsafe fn reloadPredicate(&self);

        #[method_id(@__retain_semantics Other predicateForRow:)]
        pub unsafe fn predicateForRow(&self, row: NSInteger) -> Option<Retained<NSPredicate>>;

        #[method(numberOfRows)]
        pub unsafe fn numberOfRows(&self) -> NSInteger;

        #[method_id(@__retain_semantics Other subrowIndexesForRow:)]
        pub unsafe fn subrowIndexesForRow(&self, row_index: NSInteger) -> Retained<NSIndexSet>;

        #[method_id(@__retain_semantics Other criteriaForRow:)]
        pub unsafe fn criteriaForRow(&self, row: NSInteger) -> Retained<NSArray>;

        #[method_id(@__retain_semantics Other displayValuesForRow:)]
        pub unsafe fn displayValuesForRow(&self, row: NSInteger) -> Retained<NSArray>;

        #[method(rowForDisplayValue:)]
        pub unsafe fn rowForDisplayValue(&self, display_value: &AnyObject) -> NSInteger;

        #[method(rowTypeForRow:)]
        pub unsafe fn rowTypeForRow(&self, row_index: NSInteger) -> NSRuleEditorRowType;

        #[method(parentRowForRow:)]
        pub unsafe fn parentRowForRow(&self, row_index: NSInteger) -> NSInteger;

        #[method(addRow:)]
        pub unsafe fn addRow(&self, sender: Option<&AnyObject>);

        #[method(insertRowAtIndex:withType:asSubrowOfRow:animate:)]
        pub unsafe fn insertRowAtIndex_withType_asSubrowOfRow_animate(
            &self,
            row_index: NSInteger,
            row_type: NSRuleEditorRowType,
            parent_row: NSInteger,
            should_animate: bool,
        );

        #[method(setCriteria:andDisplayValues:forRowAtIndex:)]
        pub unsafe fn setCriteria_andDisplayValues_forRowAtIndex(
            &self,
            criteria: &NSArray,
            values: &NSArray,
            row_index: NSInteger,
        );

        #[method(removeRowAtIndex:)]
        pub unsafe fn removeRowAtIndex(&self, row_index: NSInteger);

        #[method(removeRowsAtIndexes:includeSubrows:)]
        pub unsafe fn removeRowsAtIndexes_includeSubrows(
            &self,
            row_indexes: &NSIndexSet,
            include_subrows: bool,
        );

        #[method_id(@__retain_semantics Other selectedRowIndexes)]
        pub unsafe fn selectedRowIndexes(&self) -> Retained<NSIndexSet>;

        #[method(selectRowIndexes:byExtendingSelection:)]
        pub unsafe fn selectRowIndexes_byExtendingSelection(
            &self,
            indexes: &NSIndexSet,
            extend: bool,
        );

        #[method(rowClass)]
        pub unsafe fn rowClass(&self) -> &'static AnyClass;

        #[method(setRowClass:)]
        pub unsafe fn setRowClass(&self, row_class: &AnyClass);

        #[method_id(@__retain_semantics Other rowTypeKeyPath)]
        pub unsafe fn rowTypeKeyPath(&self) -> Retained<NSString>;

        #[method(setRowTypeKeyPath:)]
        pub unsafe fn setRowTypeKeyPath(&self, row_type_key_path: &NSString);

        #[method_id(@__retain_semantics Other subrowsKeyPath)]
        pub unsafe fn subrowsKeyPath(&self) -> Retained<NSString>;

        #[method(setSubrowsKeyPath:)]
        pub unsafe fn setSubrowsKeyPath(&self, subrows_key_path: &NSString);

        #[method_id(@__retain_semantics Other criteriaKeyPath)]
        pub unsafe fn criteriaKeyPath(&self) -> Retained<NSString>;

        #[method(setCriteriaKeyPath:)]
        pub unsafe fn setCriteriaKeyPath(&self, criteria_key_path: &NSString);

        #[method_id(@__retain_semantics Other displayValuesKeyPath)]
        pub unsafe fn displayValuesKeyPath(&self) -> Retained<NSString>;

        #[method(setDisplayValuesKeyPath:)]
        pub unsafe fn setDisplayValuesKeyPath(&self, display_values_key_path: &NSString);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSRuleEditor {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSRuleEditor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSRuleEditor {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    pub unsafe trait NSRuleEditorDelegate: NSObjectProtocol + MainThreadOnly {
        #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
        #[method(ruleEditor:numberOfChildrenForCriterion:withRowType:)]
        unsafe fn ruleEditor_numberOfChildrenForCriterion_withRowType(
            &self,
            editor: &NSRuleEditor,
            criterion: Option<&AnyObject>,
            row_type: NSRuleEditorRowType,
        ) -> NSInteger;

        #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
        #[method_id(@__retain_semantics Other ruleEditor:child:forCriterion:withRowType:)]
        unsafe fn ruleEditor_child_forCriterion_withRowType(
            &self,
            editor: &NSRuleEditor,
            index: NSInteger,
            criterion: Option<&AnyObject>,
            row_type: NSRuleEditorRowType,
        ) -> Retained<AnyObject>;

        #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
        #[method_id(@__retain_semantics Other ruleEditor:displayValueForCriterion:inRow:)]
        unsafe fn ruleEditor_displayValueForCriterion_inRow(
            &self,
            editor: &NSRuleEditor,
            criterion: &AnyObject,
            row: NSInteger,
        ) -> Retained<AnyObject>;

        #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
        #[optional]
        #[method_id(@__retain_semantics Other ruleEditor:predicatePartsForCriterion:withDisplayValue:inRow:)]
        unsafe fn ruleEditor_predicatePartsForCriterion_withDisplayValue_inRow(
            &self,
            editor: &NSRuleEditor,
            criterion: &AnyObject,
            value: &AnyObject,
            row: NSInteger,
        ) -> Option<Retained<NSDictionary<NSRuleEditorPredicatePartKey, AnyObject>>>;

        #[optional]
        #[method(ruleEditorRowsDidChange:)]
        unsafe fn ruleEditorRowsDidChange(&self, notification: &NSNotification);
    }

    unsafe impl ProtocolType for dyn NSRuleEditorDelegate {}
);

extern "C" {
    pub static NSRuleEditorRowsDidChangeNotification: &'static NSNotificationName;
}
