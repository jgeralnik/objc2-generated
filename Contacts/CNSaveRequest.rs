//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CNSaveRequest;
);

unsafe impl NSObjectProtocol for CNSaveRequest {}

extern_methods!(
    unsafe impl CNSaveRequest {
        #[cfg(all(feature = "CNContact", feature = "CNMutableContact"))]
        #[method(addContact:toContainerWithIdentifier:)]
        pub unsafe fn addContact_toContainerWithIdentifier(
            &self,
            contact: &CNMutableContact,
            identifier: Option<&NSString>,
        );

        #[cfg(all(feature = "CNContact", feature = "CNMutableContact"))]
        #[method(updateContact:)]
        pub unsafe fn updateContact(&self, contact: &CNMutableContact);

        #[cfg(all(feature = "CNContact", feature = "CNMutableContact"))]
        #[method(deleteContact:)]
        pub unsafe fn deleteContact(&self, contact: &CNMutableContact);

        #[cfg(all(feature = "CNGroup", feature = "CNMutableGroup"))]
        #[method(addGroup:toContainerWithIdentifier:)]
        pub unsafe fn addGroup_toContainerWithIdentifier(
            &self,
            group: &CNMutableGroup,
            identifier: Option<&NSString>,
        );

        #[cfg(all(feature = "CNGroup", feature = "CNMutableGroup"))]
        #[method(updateGroup:)]
        pub unsafe fn updateGroup(&self, group: &CNMutableGroup);

        #[cfg(all(feature = "CNGroup", feature = "CNMutableGroup"))]
        #[method(deleteGroup:)]
        pub unsafe fn deleteGroup(&self, group: &CNMutableGroup);

        #[cfg(feature = "CNGroup")]
        #[method(addSubgroup:toGroup:)]
        pub unsafe fn addSubgroup_toGroup(&self, subgroup: &CNGroup, group: &CNGroup);

        #[cfg(feature = "CNGroup")]
        #[method(removeSubgroup:fromGroup:)]
        pub unsafe fn removeSubgroup_fromGroup(&self, subgroup: &CNGroup, group: &CNGroup);

        #[cfg(all(feature = "CNContact", feature = "CNGroup"))]
        #[method(addMember:toGroup:)]
        pub unsafe fn addMember_toGroup(&self, contact: &CNContact, group: &CNGroup);

        #[cfg(all(feature = "CNContact", feature = "CNGroup"))]
        #[method(removeMember:fromGroup:)]
        pub unsafe fn removeMember_fromGroup(&self, contact: &CNContact, group: &CNGroup);

        #[method_id(@__retain_semantics Other transactionAuthor)]
        pub unsafe fn transactionAuthor(&self) -> Option<Retained<NSString>>;

        #[method(setTransactionAuthor:)]
        pub unsafe fn setTransactionAuthor(&self, transaction_author: Option<&NSString>);

        #[method(shouldRefetchContacts)]
        pub unsafe fn shouldRefetchContacts(&self) -> bool;

        #[method(setShouldRefetchContacts:)]
        pub unsafe fn setShouldRefetchContacts(&self, should_refetch_contacts: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CNSaveRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
