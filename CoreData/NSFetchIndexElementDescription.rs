//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsfetchindexelementtype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSFetchIndexElementType(pub NSUInteger);
impl NSFetchIndexElementType {
    #[doc(alias = "NSFetchIndexElementTypeBinary")]
    pub const Binary: Self = Self(0);
    #[doc(alias = "NSFetchIndexElementTypeRTree")]
    pub const RTree: Self = Self(1);
}

unsafe impl Encode for NSFetchIndexElementType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSFetchIndexElementType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsfetchindexelementdescription?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSFetchIndexElementDescription;
);

unsafe impl NSCoding for NSFetchIndexElementDescription {}

unsafe impl NSCopying for NSFetchIndexElementDescription {}

unsafe impl CopyingHelper for NSFetchIndexElementDescription {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSFetchIndexElementDescription {}

extern_methods!(
    unsafe impl NSFetchIndexElementDescription {
        #[cfg(feature = "NSPropertyDescription")]
        #[method_id(@__retain_semantics Init initWithProperty:collationType:)]
        pub unsafe fn initWithProperty_collationType(
            this: Allocated<Self>,
            property: &NSPropertyDescription,
            collation_type: NSFetchIndexElementType,
        ) -> Retained<Self>;

        #[cfg(feature = "NSPropertyDescription")]
        #[method_id(@__retain_semantics Other property)]
        pub unsafe fn property(&self) -> Option<Retained<NSPropertyDescription>>;

        #[method_id(@__retain_semantics Other propertyName)]
        pub unsafe fn propertyName(&self) -> Option<Retained<NSString>>;

        #[method(collationType)]
        pub unsafe fn collationType(&self) -> NSFetchIndexElementType;

        #[method(setCollationType:)]
        pub unsafe fn setCollationType(&self, collation_type: NSFetchIndexElementType);

        #[method(isAscending)]
        pub unsafe fn isAscending(&self) -> bool;

        #[method(setAscending:)]
        pub unsafe fn setAscending(&self, ascending: bool);

        #[cfg(feature = "NSFetchIndexDescription")]
        #[method_id(@__retain_semantics Other indexDescription)]
        pub unsafe fn indexDescription(&self) -> Option<Retained<NSFetchIndexDescription>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSFetchIndexElementDescription {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
