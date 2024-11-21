//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CNContactVCardSerialization;
);

unsafe impl NSObjectProtocol for CNContactVCardSerialization {}

extern_methods!(
    unsafe impl CNContactVCardSerialization {
        #[cfg(feature = "CNContact")]
        #[method_id(@__retain_semantics Other descriptorForRequiredKeys)]
        pub unsafe fn descriptorForRequiredKeys() -> Retained<ProtocolObject<dyn CNKeyDescriptor>>;

        #[cfg(feature = "CNContact")]
        #[method_id(@__retain_semantics Other dataWithContacts:error:_)]
        pub unsafe fn dataWithContacts_error(
            contacts: &NSArray<CNContact>,
        ) -> Result<Retained<NSData>, Retained<NSError>>;

        #[cfg(feature = "CNContact")]
        #[method_id(@__retain_semantics Other contactsWithData:error:_)]
        pub unsafe fn contactsWithData_error(
            data: &NSData,
        ) -> Result<Retained<NSArray<CNContact>>, Retained<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CNContactVCardSerialization {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
