//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsderivedattributedescription?language=objc)
    #[unsafe(super(NSAttributeDescription, NSPropertyDescription, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NSAttributeDescription", feature = "NSPropertyDescription"))]
    pub struct NSDerivedAttributeDescription;
);

#[cfg(all(feature = "NSAttributeDescription", feature = "NSPropertyDescription"))]
unsafe impl NSCoding for NSDerivedAttributeDescription {}

#[cfg(all(feature = "NSAttributeDescription", feature = "NSPropertyDescription"))]
unsafe impl NSCopying for NSDerivedAttributeDescription {}

#[cfg(all(feature = "NSAttributeDescription", feature = "NSPropertyDescription"))]
unsafe impl CopyingHelper for NSDerivedAttributeDescription {
    type Result = Self;
}

#[cfg(all(feature = "NSAttributeDescription", feature = "NSPropertyDescription"))]
unsafe impl NSObjectProtocol for NSDerivedAttributeDescription {}

extern_methods!(
    #[cfg(all(feature = "NSAttributeDescription", feature = "NSPropertyDescription"))]
    unsafe impl NSDerivedAttributeDescription {
        #[method_id(@__retain_semantics Other derivationExpression)]
        pub unsafe fn derivationExpression(&self) -> Option<Retained<NSExpression>>;

        #[method(setDerivationExpression:)]
        pub unsafe fn setDerivationExpression(&self, derivation_expression: Option<&NSExpression>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "NSAttributeDescription", feature = "NSPropertyDescription"))]
    unsafe impl NSDerivedAttributeDescription {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
