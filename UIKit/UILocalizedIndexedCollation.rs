//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UILocalizedIndexedCollation;

    unsafe impl ClassType for UILocalizedIndexedCollation {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UILocalizedIndexedCollation {}

extern_methods!(
    unsafe impl UILocalizedIndexedCollation {
        #[method_id(@__retain_semantics Other currentCollation)]
        pub unsafe fn currentCollation(mtm: MainThreadMarker) -> Id<Self>;

        #[method_id(@__retain_semantics Other sectionTitles)]
        pub unsafe fn sectionTitles(&self) -> Id<NSArray<NSString>>;

        #[method_id(@__retain_semantics Other sectionIndexTitles)]
        pub unsafe fn sectionIndexTitles(&self) -> Id<NSArray<NSString>>;

        #[method(sectionForSectionIndexTitleAtIndex:)]
        pub unsafe fn sectionForSectionIndexTitleAtIndex(
            &self,
            index_title_index: NSInteger,
        ) -> NSInteger;

        #[method(sectionForObject:collationStringSelector:)]
        pub unsafe fn sectionForObject_collationStringSelector(
            &self,
            object: &AnyObject,
            selector: Sel,
        ) -> NSInteger;

        #[method_id(@__retain_semantics Other sortedArrayFromArray:collationStringSelector:)]
        pub unsafe fn sortedArrayFromArray_collationStringSelector(
            &self,
            array: &NSArray,
            selector: Sel,
        ) -> Id<NSArray>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UILocalizedIndexedCollation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);