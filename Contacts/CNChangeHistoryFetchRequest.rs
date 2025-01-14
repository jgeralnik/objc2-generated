//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/contacts/cnchangehistoryfetchrequest?language=objc)
    #[unsafe(super(CNFetchRequest, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CNFetchRequest")]
    pub struct CNChangeHistoryFetchRequest;
);

#[cfg(feature = "CNFetchRequest")]
unsafe impl NSCoding for CNChangeHistoryFetchRequest {}

#[cfg(feature = "CNFetchRequest")]
unsafe impl NSObjectProtocol for CNChangeHistoryFetchRequest {}

#[cfg(feature = "CNFetchRequest")]
unsafe impl NSSecureCoding for CNChangeHistoryFetchRequest {}

extern_methods!(
    #[cfg(feature = "CNFetchRequest")]
    unsafe impl CNChangeHistoryFetchRequest {
        #[method_id(@__retain_semantics Other startingToken)]
        pub unsafe fn startingToken(&self) -> Option<Retained<NSData>>;

        #[method(setStartingToken:)]
        pub unsafe fn setStartingToken(&self, starting_token: Option<&NSData>);

        #[cfg(feature = "CNContact")]
        #[method_id(@__retain_semantics Other additionalContactKeyDescriptors)]
        pub unsafe fn additionalContactKeyDescriptors(
            &self,
        ) -> Option<Retained<NSArray<ProtocolObject<dyn CNKeyDescriptor>>>>;

        #[cfg(feature = "CNContact")]
        #[method(setAdditionalContactKeyDescriptors:)]
        pub unsafe fn setAdditionalContactKeyDescriptors(
            &self,
            additional_contact_key_descriptors: Option<
                &NSArray<ProtocolObject<dyn CNKeyDescriptor>>,
            >,
        );

        #[method(shouldUnifyResults)]
        pub unsafe fn shouldUnifyResults(&self) -> bool;

        #[method(setShouldUnifyResults:)]
        pub unsafe fn setShouldUnifyResults(&self, should_unify_results: bool);

        #[method(mutableObjects)]
        pub unsafe fn mutableObjects(&self) -> bool;

        #[method(setMutableObjects:)]
        pub unsafe fn setMutableObjects(&self, mutable_objects: bool);

        #[method(includeGroupChanges)]
        pub unsafe fn includeGroupChanges(&self) -> bool;

        #[method(setIncludeGroupChanges:)]
        pub unsafe fn setIncludeGroupChanges(&self, include_group_changes: bool);

        #[method_id(@__retain_semantics Other excludedTransactionAuthors)]
        pub unsafe fn excludedTransactionAuthors(&self) -> Option<Retained<NSArray<NSString>>>;

        #[method(setExcludedTransactionAuthors:)]
        pub unsafe fn setExcludedTransactionAuthors(
            &self,
            excluded_transaction_authors: Option<&NSArray<NSString>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CNFetchRequest")]
    unsafe impl CNChangeHistoryFetchRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
