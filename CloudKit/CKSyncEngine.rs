//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/cksyncengine?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngine;
);

unsafe impl Send for CKSyncEngine {}

unsafe impl Sync for CKSyncEngine {}

unsafe impl NSObjectProtocol for CKSyncEngine {}

extern_methods!(
    unsafe impl CKSyncEngine {
        #[cfg(feature = "CKSyncEngineConfiguration")]
        #[method_id(@__retain_semantics Init initWithConfiguration:)]
        pub unsafe fn initWithConfiguration(
            this: Allocated<Self>,
            configuration: &CKSyncEngineConfiguration,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "CKDatabase")]
        #[method_id(@__retain_semantics Other database)]
        pub unsafe fn database(&self) -> Retained<CKDatabase>;

        #[cfg(feature = "CKSyncEngineState")]
        #[method_id(@__retain_semantics Other state)]
        pub unsafe fn state(&self) -> Retained<CKSyncEngineState>;

        #[cfg(feature = "block2")]
        #[method(fetchChangesWithCompletionHandler:)]
        pub unsafe fn fetchChangesWithCompletionHandler(
            &self,
            completion_handler: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        );

        #[cfg(feature = "block2")]
        #[method(fetchChangesWithOptions:completionHandler:)]
        pub unsafe fn fetchChangesWithOptions_completionHandler(
            &self,
            options: &CKSyncEngineFetchChangesOptions,
            completion_handler: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        );

        #[cfg(feature = "block2")]
        #[method(sendChangesWithCompletionHandler:)]
        pub unsafe fn sendChangesWithCompletionHandler(
            &self,
            completion_handler: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        );

        #[cfg(feature = "block2")]
        #[method(sendChangesWithOptions:completionHandler:)]
        pub unsafe fn sendChangesWithOptions_completionHandler(
            &self,
            options: &CKSyncEngineSendChangesOptions,
            completion_handler: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        );

        #[cfg(feature = "block2")]
        #[method(cancelOperationsWithCompletionHandler:)]
        pub unsafe fn cancelOperationsWithCompletionHandler(
            &self,
            completion_handler: Option<&block2::Block<dyn Fn()>>,
        );
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/cksyncenginedelegate?language=objc)
    pub unsafe trait CKSyncEngineDelegate: NSObjectProtocol {
        #[cfg(feature = "CKSyncEngineEvent")]
        #[method(syncEngine:handleEvent:)]
        unsafe fn syncEngine_handleEvent(
            &self,
            sync_engine: &CKSyncEngine,
            event: &CKSyncEngineEvent,
        );

        #[cfg(feature = "CKSyncEngineRecordZoneChangeBatch")]
        #[method_id(@__retain_semantics Other syncEngine:nextRecordZoneChangeBatchForContext:)]
        unsafe fn syncEngine_nextRecordZoneChangeBatchForContext(
            &self,
            sync_engine: &CKSyncEngine,
            context: &CKSyncEngineSendChangesContext,
        ) -> Option<Retained<CKSyncEngineRecordZoneChangeBatch>>;

        #[optional]
        #[method_id(@__retain_semantics Other syncEngine:nextFetchChangesOptionsForContext:)]
        unsafe fn syncEngine_nextFetchChangesOptionsForContext(
            &self,
            sync_engine: &CKSyncEngine,
            context: &CKSyncEngineFetchChangesContext,
        ) -> Retained<CKSyncEngineFetchChangesOptions>;
    }

    unsafe impl ProtocolType for dyn CKSyncEngineDelegate {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/cksyncenginefetchchangesoptions?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineFetchChangesOptions;
);

unsafe impl Send for CKSyncEngineFetchChangesOptions {}

unsafe impl Sync for CKSyncEngineFetchChangesOptions {}

unsafe impl NSCopying for CKSyncEngineFetchChangesOptions {}

unsafe impl CopyingHelper for CKSyncEngineFetchChangesOptions {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CKSyncEngineFetchChangesOptions {}

extern_methods!(
    unsafe impl CKSyncEngineFetchChangesOptions {
        #[method_id(@__retain_semantics Other scope)]
        pub unsafe fn scope(&self) -> Retained<CKSyncEngineFetchChangesScope>;

        #[method(setScope:)]
        pub unsafe fn setScope(&self, scope: &CKSyncEngineFetchChangesScope);

        #[cfg(feature = "CKOperationGroup")]
        #[method_id(@__retain_semantics Other operationGroup)]
        pub unsafe fn operationGroup(&self) -> Retained<CKOperationGroup>;

        #[cfg(feature = "CKOperationGroup")]
        #[method(setOperationGroup:)]
        pub unsafe fn setOperationGroup(&self, operation_group: &CKOperationGroup);

        #[cfg(feature = "CKRecordZoneID")]
        #[method_id(@__retain_semantics Other prioritizedZoneIDs)]
        pub unsafe fn prioritizedZoneIDs(&self) -> Retained<NSArray<CKRecordZoneID>>;

        #[cfg(feature = "CKRecordZoneID")]
        #[method(setPrioritizedZoneIDs:)]
        pub unsafe fn setPrioritizedZoneIDs(&self, prioritized_zone_i_ds: &NSArray<CKRecordZoneID>);

        #[method_id(@__retain_semantics Init initWithScope:)]
        pub unsafe fn initWithScope(
            this: Allocated<Self>,
            scope: Option<&CKSyncEngineFetchChangesScope>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CKSyncEngineFetchChangesOptions {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/cksyncenginefetchchangesscope?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineFetchChangesScope;
);

unsafe impl Send for CKSyncEngineFetchChangesScope {}

unsafe impl Sync for CKSyncEngineFetchChangesScope {}

unsafe impl NSCopying for CKSyncEngineFetchChangesScope {}

unsafe impl CopyingHelper for CKSyncEngineFetchChangesScope {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CKSyncEngineFetchChangesScope {}

extern_methods!(
    unsafe impl CKSyncEngineFetchChangesScope {
        #[cfg(feature = "CKRecordZoneID")]
        #[method_id(@__retain_semantics Other zoneIDs)]
        pub unsafe fn zoneIDs(&self) -> Option<Retained<NSSet<CKRecordZoneID>>>;

        #[cfg(feature = "CKRecordZoneID")]
        #[method_id(@__retain_semantics Other excludedZoneIDs)]
        pub unsafe fn excludedZoneIDs(&self) -> Retained<NSSet<CKRecordZoneID>>;

        #[cfg(feature = "CKRecordZoneID")]
        #[method_id(@__retain_semantics Init initWithZoneIDs:)]
        pub unsafe fn initWithZoneIDs(
            this: Allocated<Self>,
            zone_i_ds: Option<&NSSet<CKRecordZoneID>>,
        ) -> Retained<Self>;

        #[cfg(feature = "CKRecordZoneID")]
        #[method_id(@__retain_semantics Init initWithExcludedZoneIDs:)]
        pub unsafe fn initWithExcludedZoneIDs(
            this: Allocated<Self>,
            zone_i_ds: &NSSet<CKRecordZoneID>,
        ) -> Retained<Self>;

        #[cfg(feature = "CKRecordZoneID")]
        #[method(containsZoneID:)]
        pub unsafe fn containsZoneID(&self, zone_id: &CKRecordZoneID) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CKSyncEngineFetchChangesScope {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/cksyncenginesendchangesoptions?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineSendChangesOptions;
);

unsafe impl Send for CKSyncEngineSendChangesOptions {}

unsafe impl Sync for CKSyncEngineSendChangesOptions {}

unsafe impl NSCopying for CKSyncEngineSendChangesOptions {}

unsafe impl CopyingHelper for CKSyncEngineSendChangesOptions {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CKSyncEngineSendChangesOptions {}

extern_methods!(
    unsafe impl CKSyncEngineSendChangesOptions {
        #[method_id(@__retain_semantics Other scope)]
        pub unsafe fn scope(&self) -> Retained<CKSyncEngineSendChangesScope>;

        #[method(setScope:)]
        pub unsafe fn setScope(&self, scope: &CKSyncEngineSendChangesScope);

        #[cfg(feature = "CKOperationGroup")]
        #[method_id(@__retain_semantics Other operationGroup)]
        pub unsafe fn operationGroup(&self) -> Retained<CKOperationGroup>;

        #[cfg(feature = "CKOperationGroup")]
        #[method(setOperationGroup:)]
        pub unsafe fn setOperationGroup(&self, operation_group: &CKOperationGroup);

        #[method_id(@__retain_semantics Init initWithScope:)]
        pub unsafe fn initWithScope(
            this: Allocated<Self>,
            scope: Option<&CKSyncEngineSendChangesScope>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CKSyncEngineSendChangesOptions {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/cksyncenginesendchangesscope?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineSendChangesScope;
);

unsafe impl Send for CKSyncEngineSendChangesScope {}

unsafe impl Sync for CKSyncEngineSendChangesScope {}

unsafe impl NSCopying for CKSyncEngineSendChangesScope {}

unsafe impl CopyingHelper for CKSyncEngineSendChangesScope {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CKSyncEngineSendChangesScope {}

extern_methods!(
    unsafe impl CKSyncEngineSendChangesScope {
        #[cfg(feature = "CKRecordZoneID")]
        #[method_id(@__retain_semantics Other zoneIDs)]
        pub unsafe fn zoneIDs(&self) -> Option<Retained<NSSet<CKRecordZoneID>>>;

        #[cfg(feature = "CKRecordZoneID")]
        #[method_id(@__retain_semantics Other excludedZoneIDs)]
        pub unsafe fn excludedZoneIDs(&self) -> Retained<NSSet<CKRecordZoneID>>;

        #[cfg(feature = "CKRecordID")]
        #[method_id(@__retain_semantics Other recordIDs)]
        pub unsafe fn recordIDs(&self) -> Option<Retained<NSSet<CKRecordID>>>;

        #[cfg(feature = "CKRecordZoneID")]
        #[method_id(@__retain_semantics Init initWithZoneIDs:)]
        pub unsafe fn initWithZoneIDs(
            this: Allocated<Self>,
            zone_i_ds: Option<&NSSet<CKRecordZoneID>>,
        ) -> Retained<Self>;

        #[cfg(feature = "CKRecordZoneID")]
        #[method_id(@__retain_semantics Init initWithExcludedZoneIDs:)]
        pub unsafe fn initWithExcludedZoneIDs(
            this: Allocated<Self>,
            excluded_zone_i_ds: &NSSet<CKRecordZoneID>,
        ) -> Retained<Self>;

        #[cfg(feature = "CKRecordID")]
        #[method_id(@__retain_semantics Init initWithRecordIDs:)]
        pub unsafe fn initWithRecordIDs(
            this: Allocated<Self>,
            record_i_ds: Option<&NSSet<CKRecordID>>,
        ) -> Retained<Self>;

        #[cfg(feature = "CKRecordID")]
        #[method(containsRecordID:)]
        pub unsafe fn containsRecordID(&self, record_id: &CKRecordID) -> bool;

        #[cfg(feature = "CKSyncEngineState")]
        #[method(containsPendingRecordZoneChange:)]
        pub unsafe fn containsPendingRecordZoneChange(
            &self,
            pending_record_zone_change: &CKSyncEnginePendingRecordZoneChange,
        ) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CKSyncEngineSendChangesScope {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/cksyncenginesyncreason?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CKSyncEngineSyncReason(pub NSInteger);
impl CKSyncEngineSyncReason {
    #[doc(alias = "CKSyncEngineSyncReasonScheduled")]
    pub const Scheduled: Self = Self(0);
    #[doc(alias = "CKSyncEngineSyncReasonManual")]
    pub const Manual: Self = Self(1);
}

unsafe impl Encode for CKSyncEngineSyncReason {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CKSyncEngineSyncReason {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/cksyncenginefetchchangescontext?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineFetchChangesContext;
);

unsafe impl Send for CKSyncEngineFetchChangesContext {}

unsafe impl Sync for CKSyncEngineFetchChangesContext {}

unsafe impl NSObjectProtocol for CKSyncEngineFetchChangesContext {}

extern_methods!(
    unsafe impl CKSyncEngineFetchChangesContext {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method(reason)]
        pub unsafe fn reason(&self) -> CKSyncEngineSyncReason;

        #[method_id(@__retain_semantics Other options)]
        pub unsafe fn options(&self) -> Retained<CKSyncEngineFetchChangesOptions>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/cksyncenginesendchangescontext?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineSendChangesContext;
);

unsafe impl Send for CKSyncEngineSendChangesContext {}

unsafe impl Sync for CKSyncEngineSendChangesContext {}

unsafe impl NSObjectProtocol for CKSyncEngineSendChangesContext {}

extern_methods!(
    unsafe impl CKSyncEngineSendChangesContext {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method(reason)]
        pub unsafe fn reason(&self) -> CKSyncEngineSyncReason;

        #[method_id(@__retain_semantics Other options)]
        pub unsafe fn options(&self) -> Retained<CKSyncEngineSendChangesOptions>;
    }
);
