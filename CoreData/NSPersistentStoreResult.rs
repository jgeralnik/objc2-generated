//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSBatchInsertRequestResultType {
        NSBatchInsertRequestResultTypeStatusOnly = 0x0,
        NSBatchInsertRequestResultTypeObjectIDs = 0x1,
        NSBatchInsertRequestResultTypeCount = 0x2,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSBatchUpdateRequestResultType {
        NSStatusOnlyResultType = 0x0,
        NSUpdatedObjectIDsResultType = 0x1,
        NSUpdatedObjectsCountResultType = 0x2,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSBatchDeleteRequestResultType {
        NSBatchDeleteResultTypeStatusOnly = 0x0,
        NSBatchDeleteResultTypeObjectIDs = 0x1,
        NSBatchDeleteResultTypeCount = 0x2,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSPersistentHistoryResultType {
        NSPersistentHistoryResultTypeStatusOnly = 0x0,
        NSPersistentHistoryResultTypeObjectIDs = 0x1,
        NSPersistentHistoryResultTypeCount = 0x2,
        NSPersistentHistoryResultTypeTransactionsOnly = 0x3,
        NSPersistentHistoryResultTypeChangesOnly = 0x4,
        NSPersistentHistoryResultTypeTransactionsAndChanges = 0x5,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPersistentStoreResult;

    unsafe impl ClassType for NSPersistentStoreResult {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "CoreData_NSPersistentStoreResult")]
    unsafe impl NSPersistentStoreResult {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPersistentStoreAsynchronousResult;

    unsafe impl ClassType for NSPersistentStoreAsynchronousResult {
        #[inherits(NSObject)]
        type Super = NSPersistentStoreResult;
    }
);

extern_methods!(
    #[cfg(feature = "CoreData_NSPersistentStoreAsynchronousResult")]
    unsafe impl NSPersistentStoreAsynchronousResult {
        #[cfg(feature = "CoreData_NSManagedObjectContext")]
        #[method_id(@__retain_semantics Other managedObjectContext)]
        pub unsafe fn managedObjectContext(&self) -> Id<NSManagedObjectContext, Shared>;

        #[method_id(@__retain_semantics Other operationError)]
        pub unsafe fn operationError(&self) -> Option<Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other progress)]
        pub unsafe fn progress(&self) -> Option<Id<NSProgress, Shared>>;

        #[method(cancel)]
        pub unsafe fn cancel(&self);
    }
);

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSAsynchronousFetchResult<
        ResultType: Message = Object,
        ResultTypeOwnership: Ownership = Shared,
    > {
        _inner0: PhantomData<*mut (ResultType, ResultTypeOwnership)>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    unsafe impl<ResultType: Message, ResultTypeOwnership: Ownership> ClassType
        for NSAsynchronousFetchResult<ResultType, ResultTypeOwnership>
    {
        #[inherits(NSPersistentStoreResult, NSObject)]
        type Super = NSPersistentStoreAsynchronousResult;
    }
);

extern_methods!(
    #[cfg(feature = "CoreData_NSAsynchronousFetchResult")]
    unsafe impl<ResultType: Message, ResultTypeOwnership: Ownership>
        NSAsynchronousFetchResult<ResultType, ResultTypeOwnership>
    {
        #[cfg(feature = "CoreData_NSAsynchronousFetchRequest")]
        #[method_id(@__retain_semantics Other fetchRequest)]
        pub unsafe fn fetchRequest(&self) -> Id<NSAsynchronousFetchRequest<ResultType>, Shared>;

        #[method_id(@__retain_semantics Other finalResult)]
        pub unsafe fn finalResult(&self) -> Option<Id<NSArray<ResultType>, Shared>>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSBatchInsertResult;

    unsafe impl ClassType for NSBatchInsertResult {
        #[inherits(NSObject)]
        type Super = NSPersistentStoreResult;
    }
);

extern_methods!(
    #[cfg(feature = "CoreData_NSBatchInsertResult")]
    unsafe impl NSBatchInsertResult {
        #[method_id(@__retain_semantics Other result)]
        pub unsafe fn result(&self) -> Option<Id<Object, Shared>>;

        #[method(resultType)]
        pub unsafe fn resultType(&self) -> NSBatchInsertRequestResultType;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSBatchUpdateResult;

    unsafe impl ClassType for NSBatchUpdateResult {
        #[inherits(NSObject)]
        type Super = NSPersistentStoreResult;
    }
);

extern_methods!(
    #[cfg(feature = "CoreData_NSBatchUpdateResult")]
    unsafe impl NSBatchUpdateResult {
        #[method_id(@__retain_semantics Other result)]
        pub unsafe fn result(&self) -> Option<Id<Object, Shared>>;

        #[method(resultType)]
        pub unsafe fn resultType(&self) -> NSBatchUpdateRequestResultType;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSBatchDeleteResult;

    unsafe impl ClassType for NSBatchDeleteResult {
        #[inherits(NSObject)]
        type Super = NSPersistentStoreResult;
    }
);

extern_methods!(
    #[cfg(feature = "CoreData_NSBatchDeleteResult")]
    unsafe impl NSBatchDeleteResult {
        #[method_id(@__retain_semantics Other result)]
        pub unsafe fn result(&self) -> Option<Id<Object, Shared>>;

        #[method(resultType)]
        pub unsafe fn resultType(&self) -> NSBatchDeleteRequestResultType;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPersistentHistoryResult;

    unsafe impl ClassType for NSPersistentHistoryResult {
        #[inherits(NSObject)]
        type Super = NSPersistentStoreResult;
    }
);

extern_methods!(
    #[cfg(feature = "CoreData_NSPersistentHistoryResult")]
    unsafe impl NSPersistentHistoryResult {
        #[method_id(@__retain_semantics Other result)]
        pub unsafe fn result(&self) -> Option<Id<Object, Shared>>;

        #[method(resultType)]
        pub unsafe fn resultType(&self) -> NSPersistentHistoryResultType;
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSPersistentCloudKitContainerEventResultType {
        NSPersistentCloudKitContainerEventResultTypeEvents = 0,
        NSPersistentCloudKitContainerEventResultTypeCountEvents = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPersistentCloudKitContainerEventResult;

    unsafe impl ClassType for NSPersistentCloudKitContainerEventResult {
        #[inherits(NSObject)]
        type Super = NSPersistentStoreResult;
    }
);

extern_methods!(
    #[cfg(feature = "CoreData_NSPersistentCloudKitContainerEventResult")]
    unsafe impl NSPersistentCloudKitContainerEventResult {
        #[method_id(@__retain_semantics Other result)]
        pub unsafe fn result(&self) -> Option<Id<Object, Shared>>;

        #[method(resultType)]
        pub unsafe fn resultType(&self) -> NSPersistentCloudKitContainerEventResultType;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;
    }
);
