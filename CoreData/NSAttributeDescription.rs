//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsattributetype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSAttributeType(pub NSUInteger);
impl NSAttributeType {
    pub const NSUndefinedAttributeType: Self = Self(0);
    pub const NSInteger16AttributeType: Self = Self(100);
    pub const NSInteger32AttributeType: Self = Self(200);
    pub const NSInteger64AttributeType: Self = Self(300);
    pub const NSDecimalAttributeType: Self = Self(400);
    pub const NSDoubleAttributeType: Self = Self(500);
    pub const NSFloatAttributeType: Self = Self(600);
    pub const NSStringAttributeType: Self = Self(700);
    pub const NSBooleanAttributeType: Self = Self(800);
    pub const NSDateAttributeType: Self = Self(900);
    pub const NSBinaryDataAttributeType: Self = Self(1000);
    pub const NSUUIDAttributeType: Self = Self(1100);
    pub const NSURIAttributeType: Self = Self(1200);
    pub const NSTransformableAttributeType: Self = Self(1800);
    pub const NSObjectIDAttributeType: Self = Self(2000);
    pub const NSCompositeAttributeType: Self = Self(2100);
}

unsafe impl Encode for NSAttributeType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSAttributeType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsattributedescription?language=objc)
    #[unsafe(super(NSPropertyDescription, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSPropertyDescription")]
    pub struct NSAttributeDescription;
);

#[cfg(feature = "NSPropertyDescription")]
unsafe impl NSCoding for NSAttributeDescription {}

#[cfg(feature = "NSPropertyDescription")]
unsafe impl NSCopying for NSAttributeDescription {}

#[cfg(feature = "NSPropertyDescription")]
unsafe impl CopyingHelper for NSAttributeDescription {
    type Result = Self;
}

#[cfg(feature = "NSPropertyDescription")]
unsafe impl NSObjectProtocol for NSAttributeDescription {}

extern_methods!(
    #[cfg(feature = "NSPropertyDescription")]
    unsafe impl NSAttributeDescription {
        #[method(attributeType)]
        pub unsafe fn attributeType(&self) -> NSAttributeType;

        #[method(setAttributeType:)]
        pub unsafe fn setAttributeType(&self, attribute_type: NSAttributeType);

        #[method_id(@__retain_semantics Other attributeValueClassName)]
        pub unsafe fn attributeValueClassName(&self) -> Option<Retained<NSString>>;

        #[method(setAttributeValueClassName:)]
        pub unsafe fn setAttributeValueClassName(
            &self,
            attribute_value_class_name: Option<&NSString>,
        );

        #[method_id(@__retain_semantics Other defaultValue)]
        pub unsafe fn defaultValue(&self) -> Option<Retained<AnyObject>>;

        #[method(setDefaultValue:)]
        pub unsafe fn setDefaultValue(&self, default_value: Option<&AnyObject>);

        #[method_id(@__retain_semantics Other versionHash)]
        pub unsafe fn versionHash(&self) -> Retained<NSData>;

        #[method_id(@__retain_semantics Other valueTransformerName)]
        pub unsafe fn valueTransformerName(&self) -> Option<Retained<NSString>>;

        #[method(setValueTransformerName:)]
        pub unsafe fn setValueTransformerName(&self, value_transformer_name: Option<&NSString>);

        #[method(allowsExternalBinaryDataStorage)]
        pub unsafe fn allowsExternalBinaryDataStorage(&self) -> bool;

        #[method(setAllowsExternalBinaryDataStorage:)]
        pub unsafe fn setAllowsExternalBinaryDataStorage(
            &self,
            allows_external_binary_data_storage: bool,
        );

        #[method(preservesValueInHistoryOnDeletion)]
        pub unsafe fn preservesValueInHistoryOnDeletion(&self) -> bool;

        #[method(setPreservesValueInHistoryOnDeletion:)]
        pub unsafe fn setPreservesValueInHistoryOnDeletion(
            &self,
            preserves_value_in_history_on_deletion: bool,
        );

        #[method(allowsCloudEncryption)]
        pub unsafe fn allowsCloudEncryption(&self) -> bool;

        #[method(setAllowsCloudEncryption:)]
        pub unsafe fn setAllowsCloudEncryption(&self, allows_cloud_encryption: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSPropertyDescription")]
    unsafe impl NSAttributeDescription {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
