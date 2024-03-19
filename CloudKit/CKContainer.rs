//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static CKCurrentUserDefaultName: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static CKOwnerDefaultName: &'static NSString;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKContainer;

    unsafe impl ClassType for CKContainer {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for CKContainer {}

unsafe impl Sync for CKContainer {}

unsafe impl NSObjectProtocol for CKContainer {}

extern_methods!(
    unsafe impl CKContainer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Other defaultContainer)]
        pub unsafe fn defaultContainer() -> Id<CKContainer>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other containerWithIdentifier:)]
        pub unsafe fn containerWithIdentifier(container_identifier: &NSString) -> Id<CKContainer>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other containerIdentifier)]
        pub unsafe fn containerIdentifier(&self) -> Option<Id<NSString>>;

        #[cfg(all(feature = "CloudKit_CKOperation", feature = "Foundation_NSOperation"))]
        #[method(addOperation:)]
        pub unsafe fn addOperation(&self, operation: &CKOperation);
    }
);

extern_methods!(
    /// Database
    unsafe impl CKContainer {
        #[cfg(feature = "CloudKit_CKDatabase")]
        #[method_id(@__retain_semantics Other privateCloudDatabase)]
        pub unsafe fn privateCloudDatabase(&self) -> Id<CKDatabase>;

        #[cfg(feature = "CloudKit_CKDatabase")]
        #[method_id(@__retain_semantics Other publicCloudDatabase)]
        pub unsafe fn publicCloudDatabase(&self) -> Id<CKDatabase>;

        #[cfg(feature = "CloudKit_CKDatabase")]
        #[method_id(@__retain_semantics Other sharedCloudDatabase)]
        pub unsafe fn sharedCloudDatabase(&self) -> Id<CKDatabase>;

        #[cfg(feature = "CloudKit_CKDatabase")]
        #[method_id(@__retain_semantics Other databaseWithDatabaseScope:)]
        pub unsafe fn databaseWithDatabaseScope(
            &self,
            database_scope: CKDatabaseScope,
        ) -> Id<CKDatabase>;
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum CKAccountStatus {
        #[doc(alias = "CKAccountStatusCouldNotDetermine")]
        CouldNotDetermine = 0,
        #[doc(alias = "CKAccountStatusAvailable")]
        Available = 1,
        #[doc(alias = "CKAccountStatusRestricted")]
        Restricted = 2,
        #[doc(alias = "CKAccountStatusNoAccount")]
        NoAccount = 3,
        #[doc(alias = "CKAccountStatusTemporarilyUnavailable")]
        TemporarilyUnavailable = 4,
    }
);

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static CKAccountChangedNotification: &'static NSString;
}

extern_methods!(
    /// AccountStatus
    unsafe impl CKContainer {
        #[cfg(feature = "Foundation_NSError")]
        #[method(accountStatusWithCompletionHandler:)]
        pub unsafe fn accountStatusWithCompletionHandler(
            &self,
            completion_handler: &Block<dyn Fn(CKAccountStatus, *mut NSError)>,
        );
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum CKApplicationPermissions {
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        CKApplicationPermissionUserDiscoverability = 1 << 0,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
    pub enum CKApplicationPermissionStatus {
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[doc(alias = "CKApplicationPermissionStatusInitialState")]
        InitialState = 0,
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[doc(alias = "CKApplicationPermissionStatusCouldNotComplete")]
        CouldNotComplete = 1,
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[doc(alias = "CKApplicationPermissionStatusDenied")]
        Denied = 2,
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[doc(alias = "CKApplicationPermissionStatusGranted")]
        Granted = 3,
    }
);

#[cfg(feature = "Foundation_NSError")]
pub type CKApplicationPermissionBlock =
    *mut Block<dyn Fn(CKApplicationPermissionStatus, *mut NSError)>;

extern_methods!(
    /// ApplicationPermission
    unsafe impl CKContainer {
        #[cfg(feature = "Foundation_NSError")]
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[method(statusForApplicationPermission:completionHandler:)]
        pub unsafe fn statusForApplicationPermission_completionHandler(
            &self,
            application_permission: CKApplicationPermissions,
            completion_handler: CKApplicationPermissionBlock,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[method(requestApplicationPermission:completionHandler:)]
        pub unsafe fn requestApplicationPermission_completionHandler(
            &self,
            application_permission: CKApplicationPermissions,
            completion_handler: CKApplicationPermissionBlock,
        );
    }
);

extern_methods!(
    /// UserRecords
    unsafe impl CKContainer {
        #[cfg(all(feature = "CloudKit_CKRecordID", feature = "Foundation_NSError"))]
        #[method(fetchUserRecordIDWithCompletionHandler:)]
        pub unsafe fn fetchUserRecordIDWithCompletionHandler(
            &self,
            completion_handler: &Block<dyn Fn(*mut CKRecordID, *mut NSError)>,
        );

        #[cfg(all(
            feature = "CloudKit_CKUserIdentity",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError"
        ))]
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[method(discoverAllIdentitiesWithCompletionHandler:)]
        pub unsafe fn discoverAllIdentitiesWithCompletionHandler(
            &self,
            completion_handler: &Block<dyn Fn(*mut NSArray<CKUserIdentity>, *mut NSError)>,
        );

        #[cfg(all(
            feature = "CloudKit_CKUserIdentity",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[method(discoverUserIdentityWithEmailAddress:completionHandler:)]
        pub unsafe fn discoverUserIdentityWithEmailAddress_completionHandler(
            &self,
            email: &NSString,
            completion_handler: &Block<dyn Fn(*mut CKUserIdentity, *mut NSError)>,
        );

        #[cfg(all(
            feature = "CloudKit_CKUserIdentity",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[method(discoverUserIdentityWithPhoneNumber:completionHandler:)]
        pub unsafe fn discoverUserIdentityWithPhoneNumber_completionHandler(
            &self,
            phone_number: &NSString,
            completion_handler: &Block<dyn Fn(*mut CKUserIdentity, *mut NSError)>,
        );

        #[cfg(all(
            feature = "CloudKit_CKRecordID",
            feature = "CloudKit_CKUserIdentity",
            feature = "Foundation_NSError"
        ))]
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[method(discoverUserIdentityWithUserRecordID:completionHandler:)]
        pub unsafe fn discoverUserIdentityWithUserRecordID_completionHandler(
            &self,
            user_record_id: &CKRecordID,
            completion_handler: &Block<dyn Fn(*mut CKUserIdentity, *mut NSError)>,
        );
    }
);

extern_methods!(
    /// Sharing
    unsafe impl CKContainer {
        #[cfg(all(
            feature = "CloudKit_CKShareParticipant",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method(fetchShareParticipantWithEmailAddress:completionHandler:)]
        pub unsafe fn fetchShareParticipantWithEmailAddress_completionHandler(
            &self,
            email_address: &NSString,
            completion_handler: &Block<dyn Fn(*mut CKShareParticipant, *mut NSError)>,
        );

        #[cfg(all(
            feature = "CloudKit_CKShareParticipant",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method(fetchShareParticipantWithPhoneNumber:completionHandler:)]
        pub unsafe fn fetchShareParticipantWithPhoneNumber_completionHandler(
            &self,
            phone_number: &NSString,
            completion_handler: &Block<dyn Fn(*mut CKShareParticipant, *mut NSError)>,
        );

        #[cfg(all(
            feature = "CloudKit_CKRecordID",
            feature = "CloudKit_CKShareParticipant",
            feature = "Foundation_NSError"
        ))]
        #[method(fetchShareParticipantWithUserRecordID:completionHandler:)]
        pub unsafe fn fetchShareParticipantWithUserRecordID_completionHandler(
            &self,
            user_record_id: &CKRecordID,
            completion_handler: &Block<dyn Fn(*mut CKShareParticipant, *mut NSError)>,
        );

        #[cfg(all(
            feature = "CloudKit_CKShareMetadata",
            feature = "Foundation_NSError",
            feature = "Foundation_NSURL"
        ))]
        #[method(fetchShareMetadataWithURL:completionHandler:)]
        pub unsafe fn fetchShareMetadataWithURL_completionHandler(
            &self,
            url: &NSURL,
            completion_handler: &Block<dyn Fn(*mut CKShareMetadata, *mut NSError)>,
        );

        #[cfg(all(
            feature = "CloudKit_CKRecord",
            feature = "CloudKit_CKShare",
            feature = "CloudKit_CKShareMetadata",
            feature = "Foundation_NSError"
        ))]
        #[method(acceptShareMetadata:completionHandler:)]
        pub unsafe fn acceptShareMetadata_completionHandler(
            &self,
            metadata: &CKShareMetadata,
            completion_handler: &Block<dyn Fn(*mut CKShare, *mut NSError)>,
        );
    }
);

extern_methods!(
    /// CKLongLivedOperations
    unsafe impl CKContainer {
        #[cfg(all(
            feature = "CloudKit_CKOperation",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method(fetchAllLongLivedOperationIDsWithCompletionHandler:)]
        pub unsafe fn fetchAllLongLivedOperationIDsWithCompletionHandler(
            &self,
            completion_handler: &Block<dyn Fn(*mut NSArray<CKOperationID>, *mut NSError)>,
        );

        #[cfg(all(
            feature = "CloudKit_CKOperation",
            feature = "Foundation_NSError",
            feature = "Foundation_NSOperation",
            feature = "Foundation_NSString"
        ))]
        #[method(fetchLongLivedOperationWithID:completionHandler:)]
        pub unsafe fn fetchLongLivedOperationWithID_completionHandler(
            &self,
            operation_id: &CKOperationID,
            completion_handler: &Block<dyn Fn(*mut CKOperation, *mut NSError)>,
        );
    }
);
