//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKSyncEngine")]
    pub struct CKSyncEngine;

    #[cfg(feature = "CloudKit_CKSyncEngine")]
    unsafe impl ClassType for CKSyncEngine {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CloudKit_CKSyncEngine")]
unsafe impl Send for CKSyncEngine {}

#[cfg(feature = "CloudKit_CKSyncEngine")]
unsafe impl Sync for CKSyncEngine {}

#[cfg(feature = "CloudKit_CKSyncEngine")]
unsafe impl NSObjectProtocol for CKSyncEngine {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKSyncEngine")]
    unsafe impl CKSyncEngine {
        #[cfg(feature = "CloudKit_CKSyncEngineConfiguration")]
        #[method_id(@__retain_semantics Init initWithConfiguration:)]
        pub unsafe fn initWithConfiguration(
            this: Allocated<Self>,
            configuration: &CKSyncEngineConfiguration,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[cfg(feature = "CloudKit_CKDatabase")]
        #[method_id(@__retain_semantics Other database)]
        pub unsafe fn database(&self) -> Id<CKDatabase>;

        #[cfg(feature = "CloudKit_CKSyncEngineState")]
        #[method_id(@__retain_semantics Other state)]
        pub unsafe fn state(&self) -> Id<CKSyncEngineState>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(fetchChangesWithCompletionHandler:)]
        pub unsafe fn fetchChangesWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<dyn Fn(*mut NSError)>>,
        );

        #[cfg(all(
            feature = "CloudKit_CKSyncEngineFetchChangesOptions",
            feature = "Foundation_NSError"
        ))]
        #[method(fetchChangesWithOptions:completionHandler:)]
        pub unsafe fn fetchChangesWithOptions_completionHandler(
            &self,
            options: &CKSyncEngineFetchChangesOptions,
            completion_handler: Option<&Block<dyn Fn(*mut NSError)>>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(sendChangesWithCompletionHandler:)]
        pub unsafe fn sendChangesWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<dyn Fn(*mut NSError)>>,
        );

        #[cfg(all(
            feature = "CloudKit_CKSyncEngineSendChangesOptions",
            feature = "Foundation_NSError"
        ))]
        #[method(sendChangesWithOptions:completionHandler:)]
        pub unsafe fn sendChangesWithOptions_completionHandler(
            &self,
            options: &CKSyncEngineSendChangesOptions,
            completion_handler: Option<&Block<dyn Fn(*mut NSError)>>,
        );

        #[method(cancelOperationsWithCompletionHandler:)]
        pub unsafe fn cancelOperationsWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<dyn Fn()>>,
        );
    }
);

extern_protocol!(
    pub unsafe trait CKSyncEngineDelegate: NSObjectProtocol {
        #[cfg(all(
            feature = "CloudKit_CKSyncEngine",
            feature = "CloudKit_CKSyncEngineEvent"
        ))]
        #[method(syncEngine:handleEvent:)]
        unsafe fn syncEngine_handleEvent(
            &self,
            sync_engine: &CKSyncEngine,
            event: &CKSyncEngineEvent,
        );

        #[cfg(all(
            feature = "CloudKit_CKSyncEngine",
            feature = "CloudKit_CKSyncEngineRecordZoneChangeBatch",
            feature = "CloudKit_CKSyncEngineSendChangesContext"
        ))]
        #[method_id(@__retain_semantics Other syncEngine:nextRecordZoneChangeBatchForContext:)]
        unsafe fn syncEngine_nextRecordZoneChangeBatchForContext(
            &self,
            sync_engine: &CKSyncEngine,
            context: &CKSyncEngineSendChangesContext,
        ) -> Option<Id<CKSyncEngineRecordZoneChangeBatch>>;

        #[cfg(all(
            feature = "CloudKit_CKSyncEngine",
            feature = "CloudKit_CKSyncEngineFetchChangesContext",
            feature = "CloudKit_CKSyncEngineFetchChangesOptions"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other syncEngine:nextFetchChangesOptionsForContext:)]
        unsafe fn syncEngine_nextFetchChangesOptionsForContext(
            &self,
            sync_engine: &CKSyncEngine,
            context: &CKSyncEngineFetchChangesContext,
        ) -> Id<CKSyncEngineFetchChangesOptions>;
    }

    unsafe impl ProtocolType for dyn CKSyncEngineDelegate {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKSyncEngineFetchChangesOptions")]
    pub struct CKSyncEngineFetchChangesOptions;

    #[cfg(feature = "CloudKit_CKSyncEngineFetchChangesOptions")]
    unsafe impl ClassType for CKSyncEngineFetchChangesOptions {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CloudKit_CKSyncEngineFetchChangesOptions")]
unsafe impl Send for CKSyncEngineFetchChangesOptions {}

#[cfg(feature = "CloudKit_CKSyncEngineFetchChangesOptions")]
unsafe impl Sync for CKSyncEngineFetchChangesOptions {}

#[cfg(feature = "CloudKit_CKSyncEngineFetchChangesOptions")]
unsafe impl NSCopying for CKSyncEngineFetchChangesOptions {}

#[cfg(feature = "CloudKit_CKSyncEngineFetchChangesOptions")]
unsafe impl NSObjectProtocol for CKSyncEngineFetchChangesOptions {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKSyncEngineFetchChangesOptions")]
    unsafe impl CKSyncEngineFetchChangesOptions {
        #[cfg(feature = "CloudKit_CKSyncEngineFetchChangesScope")]
        #[method_id(@__retain_semantics Other scope)]
        pub unsafe fn scope(&self) -> Id<CKSyncEngineFetchChangesScope>;

        #[cfg(feature = "CloudKit_CKSyncEngineFetchChangesScope")]
        #[method(setScope:)]
        pub unsafe fn setScope(&self, scope: &CKSyncEngineFetchChangesScope);

        #[cfg(feature = "CloudKit_CKOperationGroup")]
        #[method_id(@__retain_semantics Other operationGroup)]
        pub unsafe fn operationGroup(&self) -> Id<CKOperationGroup>;

        #[cfg(feature = "CloudKit_CKOperationGroup")]
        #[method(setOperationGroup:)]
        pub unsafe fn setOperationGroup(&self, operation_group: &CKOperationGroup);

        #[cfg(all(feature = "CloudKit_CKRecordZoneID", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other prioritizedZoneIDs)]
        pub unsafe fn prioritizedZoneIDs(&self) -> Id<NSArray<CKRecordZoneID>>;

        #[cfg(all(feature = "CloudKit_CKRecordZoneID", feature = "Foundation_NSArray"))]
        #[method(setPrioritizedZoneIDs:)]
        pub unsafe fn setPrioritizedZoneIDs(&self, prioritized_zone_i_ds: &NSArray<CKRecordZoneID>);

        #[cfg(feature = "CloudKit_CKSyncEngineFetchChangesScope")]
        #[method_id(@__retain_semantics Init initWithScope:)]
        pub unsafe fn initWithScope(
            this: Allocated<Self>,
            scope: Option<&CKSyncEngineFetchChangesScope>,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CloudKit_CKSyncEngineFetchChangesOptions")]
    unsafe impl CKSyncEngineFetchChangesOptions {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKSyncEngineFetchChangesScope")]
    pub struct CKSyncEngineFetchChangesScope;

    #[cfg(feature = "CloudKit_CKSyncEngineFetchChangesScope")]
    unsafe impl ClassType for CKSyncEngineFetchChangesScope {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CloudKit_CKSyncEngineFetchChangesScope")]
unsafe impl Send for CKSyncEngineFetchChangesScope {}

#[cfg(feature = "CloudKit_CKSyncEngineFetchChangesScope")]
unsafe impl Sync for CKSyncEngineFetchChangesScope {}

#[cfg(feature = "CloudKit_CKSyncEngineFetchChangesScope")]
unsafe impl NSCopying for CKSyncEngineFetchChangesScope {}

#[cfg(feature = "CloudKit_CKSyncEngineFetchChangesScope")]
unsafe impl NSObjectProtocol for CKSyncEngineFetchChangesScope {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKSyncEngineFetchChangesScope")]
    unsafe impl CKSyncEngineFetchChangesScope {
        #[cfg(all(feature = "CloudKit_CKRecordZoneID", feature = "Foundation_NSSet"))]
        #[method_id(@__retain_semantics Other zoneIDs)]
        pub unsafe fn zoneIDs(&self) -> Option<Id<NSSet<CKRecordZoneID>>>;

        #[cfg(all(feature = "CloudKit_CKRecordZoneID", feature = "Foundation_NSSet"))]
        #[method_id(@__retain_semantics Other excludedZoneIDs)]
        pub unsafe fn excludedZoneIDs(&self) -> Id<NSSet<CKRecordZoneID>>;

        #[cfg(all(feature = "CloudKit_CKRecordZoneID", feature = "Foundation_NSSet"))]
        #[method_id(@__retain_semantics Init initWithZoneIDs:)]
        pub unsafe fn initWithZoneIDs(
            this: Allocated<Self>,
            zone_i_ds: Option<&NSSet<CKRecordZoneID>>,
        ) -> Id<Self>;

        #[cfg(all(feature = "CloudKit_CKRecordZoneID", feature = "Foundation_NSSet"))]
        #[method_id(@__retain_semantics Init initWithExcludedZoneIDs:)]
        pub unsafe fn initWithExcludedZoneIDs(
            this: Allocated<Self>,
            zone_i_ds: &NSSet<CKRecordZoneID>,
        ) -> Id<Self>;

        #[cfg(feature = "CloudKit_CKRecordZoneID")]
        #[method(containsZoneID:)]
        pub unsafe fn containsZoneID(&self, zone_id: &CKRecordZoneID) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CloudKit_CKSyncEngineFetchChangesScope")]
    unsafe impl CKSyncEngineFetchChangesScope {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKSyncEngineSendChangesOptions")]
    pub struct CKSyncEngineSendChangesOptions;

    #[cfg(feature = "CloudKit_CKSyncEngineSendChangesOptions")]
    unsafe impl ClassType for CKSyncEngineSendChangesOptions {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CloudKit_CKSyncEngineSendChangesOptions")]
unsafe impl Send for CKSyncEngineSendChangesOptions {}

#[cfg(feature = "CloudKit_CKSyncEngineSendChangesOptions")]
unsafe impl Sync for CKSyncEngineSendChangesOptions {}

#[cfg(feature = "CloudKit_CKSyncEngineSendChangesOptions")]
unsafe impl NSCopying for CKSyncEngineSendChangesOptions {}

#[cfg(feature = "CloudKit_CKSyncEngineSendChangesOptions")]
unsafe impl NSObjectProtocol for CKSyncEngineSendChangesOptions {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKSyncEngineSendChangesOptions")]
    unsafe impl CKSyncEngineSendChangesOptions {
        #[cfg(feature = "CloudKit_CKSyncEngineSendChangesScope")]
        #[method_id(@__retain_semantics Other scope)]
        pub unsafe fn scope(&self) -> Id<CKSyncEngineSendChangesScope>;

        #[cfg(feature = "CloudKit_CKSyncEngineSendChangesScope")]
        #[method(setScope:)]
        pub unsafe fn setScope(&self, scope: &CKSyncEngineSendChangesScope);

        #[cfg(feature = "CloudKit_CKOperationGroup")]
        #[method_id(@__retain_semantics Other operationGroup)]
        pub unsafe fn operationGroup(&self) -> Id<CKOperationGroup>;

        #[cfg(feature = "CloudKit_CKOperationGroup")]
        #[method(setOperationGroup:)]
        pub unsafe fn setOperationGroup(&self, operation_group: &CKOperationGroup);

        #[cfg(feature = "CloudKit_CKSyncEngineSendChangesScope")]
        #[method_id(@__retain_semantics Init initWithScope:)]
        pub unsafe fn initWithScope(
            this: Allocated<Self>,
            scope: Option<&CKSyncEngineSendChangesScope>,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CloudKit_CKSyncEngineSendChangesOptions")]
    unsafe impl CKSyncEngineSendChangesOptions {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKSyncEngineSendChangesScope")]
    pub struct CKSyncEngineSendChangesScope;

    #[cfg(feature = "CloudKit_CKSyncEngineSendChangesScope")]
    unsafe impl ClassType for CKSyncEngineSendChangesScope {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CloudKit_CKSyncEngineSendChangesScope")]
unsafe impl Send for CKSyncEngineSendChangesScope {}

#[cfg(feature = "CloudKit_CKSyncEngineSendChangesScope")]
unsafe impl Sync for CKSyncEngineSendChangesScope {}

#[cfg(feature = "CloudKit_CKSyncEngineSendChangesScope")]
unsafe impl NSCopying for CKSyncEngineSendChangesScope {}

#[cfg(feature = "CloudKit_CKSyncEngineSendChangesScope")]
unsafe impl NSObjectProtocol for CKSyncEngineSendChangesScope {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKSyncEngineSendChangesScope")]
    unsafe impl CKSyncEngineSendChangesScope {
        #[cfg(all(feature = "CloudKit_CKRecordZoneID", feature = "Foundation_NSSet"))]
        #[method_id(@__retain_semantics Other zoneIDs)]
        pub unsafe fn zoneIDs(&self) -> Option<Id<NSSet<CKRecordZoneID>>>;

        #[cfg(all(feature = "CloudKit_CKRecordZoneID", feature = "Foundation_NSSet"))]
        #[method_id(@__retain_semantics Other excludedZoneIDs)]
        pub unsafe fn excludedZoneIDs(&self) -> Id<NSSet<CKRecordZoneID>>;

        #[cfg(all(feature = "CloudKit_CKRecordID", feature = "Foundation_NSSet"))]
        #[method_id(@__retain_semantics Other recordIDs)]
        pub unsafe fn recordIDs(&self) -> Option<Id<NSSet<CKRecordID>>>;

        #[cfg(all(feature = "CloudKit_CKRecordZoneID", feature = "Foundation_NSSet"))]
        #[method_id(@__retain_semantics Init initWithZoneIDs:)]
        pub unsafe fn initWithZoneIDs(
            this: Allocated<Self>,
            zone_i_ds: Option<&NSSet<CKRecordZoneID>>,
        ) -> Id<Self>;

        #[cfg(all(feature = "CloudKit_CKRecordZoneID", feature = "Foundation_NSSet"))]
        #[method_id(@__retain_semantics Init initWithExcludedZoneIDs:)]
        pub unsafe fn initWithExcludedZoneIDs(
            this: Allocated<Self>,
            excluded_zone_i_ds: &NSSet<CKRecordZoneID>,
        ) -> Id<Self>;

        #[cfg(all(feature = "CloudKit_CKRecordID", feature = "Foundation_NSSet"))]
        #[method_id(@__retain_semantics Init initWithRecordIDs:)]
        pub unsafe fn initWithRecordIDs(
            this: Allocated<Self>,
            record_i_ds: Option<&NSSet<CKRecordID>>,
        ) -> Id<Self>;

        #[cfg(feature = "CloudKit_CKRecordID")]
        #[method(containsRecordID:)]
        pub unsafe fn containsRecordID(&self, record_id: &CKRecordID) -> bool;

        #[cfg(feature = "CloudKit_CKSyncEnginePendingRecordZoneChange")]
        #[method(containsPendingRecordZoneChange:)]
        pub unsafe fn containsPendingRecordZoneChange(
            &self,
            pending_record_zone_change: &CKSyncEnginePendingRecordZoneChange,
        ) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CloudKit_CKSyncEngineSendChangesScope")]
    unsafe impl CKSyncEngineSendChangesScope {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum CKSyncEngineSyncReason {
        #[doc(alias = "CKSyncEngineSyncReasonScheduled")]
        Scheduled = 0,
        #[doc(alias = "CKSyncEngineSyncReasonManual")]
        Manual = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKSyncEngineFetchChangesContext")]
    pub struct CKSyncEngineFetchChangesContext;

    #[cfg(feature = "CloudKit_CKSyncEngineFetchChangesContext")]
    unsafe impl ClassType for CKSyncEngineFetchChangesContext {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CloudKit_CKSyncEngineFetchChangesContext")]
unsafe impl Send for CKSyncEngineFetchChangesContext {}

#[cfg(feature = "CloudKit_CKSyncEngineFetchChangesContext")]
unsafe impl Sync for CKSyncEngineFetchChangesContext {}

#[cfg(feature = "CloudKit_CKSyncEngineFetchChangesContext")]
unsafe impl NSObjectProtocol for CKSyncEngineFetchChangesContext {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKSyncEngineFetchChangesContext")]
    unsafe impl CKSyncEngineFetchChangesContext {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method(reason)]
        pub unsafe fn reason(&self) -> CKSyncEngineSyncReason;

        #[cfg(feature = "CloudKit_CKSyncEngineFetchChangesOptions")]
        #[method_id(@__retain_semantics Other options)]
        pub unsafe fn options(&self) -> Id<CKSyncEngineFetchChangesOptions>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKSyncEngineSendChangesContext")]
    pub struct CKSyncEngineSendChangesContext;

    #[cfg(feature = "CloudKit_CKSyncEngineSendChangesContext")]
    unsafe impl ClassType for CKSyncEngineSendChangesContext {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CloudKit_CKSyncEngineSendChangesContext")]
unsafe impl Send for CKSyncEngineSendChangesContext {}

#[cfg(feature = "CloudKit_CKSyncEngineSendChangesContext")]
unsafe impl Sync for CKSyncEngineSendChangesContext {}

#[cfg(feature = "CloudKit_CKSyncEngineSendChangesContext")]
unsafe impl NSObjectProtocol for CKSyncEngineSendChangesContext {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKSyncEngineSendChangesContext")]
    unsafe impl CKSyncEngineSendChangesContext {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method(reason)]
        pub unsafe fn reason(&self) -> CKSyncEngineSyncReason;

        #[cfg(feature = "CloudKit_CKSyncEngineSendChangesOptions")]
        #[method_id(@__retain_semantics Other options)]
        pub unsafe fn options(&self) -> Id<CKSyncEngineSendChangesOptions>;
    }
);
