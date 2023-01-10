//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSPersistentHistoryChangeType {
        NSPersistentHistoryChangeTypeInsert = 0,
        NSPersistentHistoryChangeTypeUpdate = 1,
        NSPersistentHistoryChangeTypeDelete = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPersistentHistoryChange;

    unsafe impl ClassType for NSPersistentHistoryChange {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "CoreData_NSPersistentHistoryChange")]
    unsafe impl NSPersistentHistoryChange {
        #[cfg(all(
            feature = "CoreData_NSEntityDescription",
            feature = "CoreData_NSManagedObjectContext"
        ))]
        #[method_id(@__retain_semantics Other entityDescriptionWithContext:)]
        pub unsafe fn entityDescriptionWithContext(
            context: &NSManagedObjectContext,
        ) -> Option<Id<NSEntityDescription, Shared>>;

        #[cfg(feature = "CoreData_NSEntityDescription")]
        #[method_id(@__retain_semantics Other entityDescription)]
        pub unsafe fn entityDescription() -> Option<Id<NSEntityDescription, Shared>>;

        #[cfg(feature = "CoreData_NSFetchRequest")]
        #[method_id(@__retain_semantics Other fetchRequest)]
        pub unsafe fn fetchRequest() -> Option<Id<NSFetchRequest, Shared>>;

        #[method(changeID)]
        pub unsafe fn changeID(&self) -> i64;

        #[cfg(feature = "CoreData_NSManagedObjectID")]
        #[method_id(@__retain_semantics Other changedObjectID)]
        pub unsafe fn changedObjectID(&self) -> Id<NSManagedObjectID, Shared>;

        #[method(changeType)]
        pub unsafe fn changeType(&self) -> NSPersistentHistoryChangeType;

        #[method_id(@__retain_semantics Other tombstone)]
        pub unsafe fn tombstone(&self) -> Option<Id<NSDictionary, Shared>>;

        #[cfg(feature = "CoreData_NSPersistentHistoryTransaction")]
        #[method_id(@__retain_semantics Other transaction)]
        pub unsafe fn transaction(&self) -> Option<Id<NSPersistentHistoryTransaction, Shared>>;

        #[cfg(feature = "CoreData_NSPropertyDescription")]
        #[method_id(@__retain_semantics Other updatedProperties)]
        pub unsafe fn updatedProperties(&self) -> Option<Id<NSSet<NSPropertyDescription>, Shared>>;
    }
);
