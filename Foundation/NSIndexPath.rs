//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSIndexPath;

    unsafe impl ClassType for NSIndexPath {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSIndexPath {}

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSIndexPath {}

#[cfg(feature = "NSObject")]
unsafe impl CopyingHelper for NSIndexPath {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSIndexPath {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSIndexPath {}

extern_methods!(
    unsafe impl NSIndexPath {
        #[method_id(@__retain_semantics Other indexPathWithIndex:)]
        pub unsafe fn indexPathWithIndex(index: NSUInteger) -> Retained<Self>;

        #[method_id(@__retain_semantics Other indexPathWithIndexes:length:)]
        pub unsafe fn indexPathWithIndexes_length(
            indexes: *mut NSUInteger,
            length: NSUInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithIndexes:length:)]
        pub unsafe fn initWithIndexes_length(
            this: Allocated<Self>,
            indexes: *mut NSUInteger,
            length: NSUInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithIndex:)]
        pub unsafe fn initWithIndex(this: Allocated<Self>, index: NSUInteger) -> Retained<Self>;

        #[method_id(@__retain_semantics Other indexPathByAddingIndex:)]
        pub unsafe fn indexPathByAddingIndex(&self, index: NSUInteger) -> Retained<NSIndexPath>;

        #[method_id(@__retain_semantics Other indexPathByRemovingLastIndex)]
        pub unsafe fn indexPathByRemovingLastIndex(&self) -> Retained<NSIndexPath>;

        #[method(indexAtPosition:)]
        pub unsafe fn indexAtPosition(&self, position: NSUInteger) -> NSUInteger;

        #[method(length)]
        pub unsafe fn length(&self) -> NSUInteger;

        #[cfg(feature = "NSRange")]
        #[method(getIndexes:range:)]
        pub unsafe fn getIndexes_range(
            &self,
            indexes: NonNull<NSUInteger>,
            position_range: NSRange,
        );

        #[cfg(feature = "NSObjCRuntime")]
        #[method(compare:)]
        pub unsafe fn compare(&self, other_object: &NSIndexPath) -> NSComparisonResult;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSIndexPath {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// NSDeprecated
    unsafe impl NSIndexPath {
        #[deprecated]
        #[method(getIndexes:)]
        pub unsafe fn getIndexes(&self, indexes: NonNull<NSUInteger>);
    }
);
