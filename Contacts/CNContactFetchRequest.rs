//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/contacts/cncontactfetchrequest?language=objc)
    #[unsafe(super(CNFetchRequest, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CNFetchRequest")]
    pub struct CNContactFetchRequest;
);

#[cfg(feature = "CNFetchRequest")]
unsafe impl NSCoding for CNContactFetchRequest {}

#[cfg(feature = "CNFetchRequest")]
unsafe impl NSObjectProtocol for CNContactFetchRequest {}

#[cfg(feature = "CNFetchRequest")]
unsafe impl NSSecureCoding for CNContactFetchRequest {}

extern_methods!(
    #[cfg(feature = "CNFetchRequest")]
    unsafe impl CNContactFetchRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "CNContact")]
        #[method_id(@__retain_semantics Init initWithKeysToFetch:)]
        pub unsafe fn initWithKeysToFetch(
            this: Allocated<Self>,
            keys_to_fetch: &NSArray<ProtocolObject<dyn CNKeyDescriptor>>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other predicate)]
        pub unsafe fn predicate(&self) -> Option<Retained<NSPredicate>>;

        #[method(setPredicate:)]
        pub unsafe fn setPredicate(&self, predicate: Option<&NSPredicate>);

        #[cfg(feature = "CNContact")]
        #[method_id(@__retain_semantics Other keysToFetch)]
        pub unsafe fn keysToFetch(&self) -> Retained<NSArray<ProtocolObject<dyn CNKeyDescriptor>>>;

        #[cfg(feature = "CNContact")]
        #[method(setKeysToFetch:)]
        pub unsafe fn setKeysToFetch(
            &self,
            keys_to_fetch: &NSArray<ProtocolObject<dyn CNKeyDescriptor>>,
        );

        #[method(mutableObjects)]
        pub unsafe fn mutableObjects(&self) -> bool;

        #[method(setMutableObjects:)]
        pub unsafe fn setMutableObjects(&self, mutable_objects: bool);

        #[method(unifyResults)]
        pub unsafe fn unifyResults(&self) -> bool;

        #[method(setUnifyResults:)]
        pub unsafe fn setUnifyResults(&self, unify_results: bool);

        #[cfg(feature = "CNContact")]
        #[method(sortOrder)]
        pub unsafe fn sortOrder(&self) -> CNContactSortOrder;

        #[cfg(feature = "CNContact")]
        #[method(setSortOrder:)]
        pub unsafe fn setSortOrder(&self, sort_order: CNContactSortOrder);
    }
);
