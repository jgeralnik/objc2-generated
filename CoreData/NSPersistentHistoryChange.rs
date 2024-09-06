//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPersistentHistoryChangeType(pub NSInteger);
impl NSPersistentHistoryChangeType {
    #[doc(alias = "NSPersistentHistoryChangeTypeInsert")]
    pub const Insert: Self = Self(0);
    #[doc(alias = "NSPersistentHistoryChangeTypeUpdate")]
    pub const Update: Self = Self(1);
    #[doc(alias = "NSPersistentHistoryChangeTypeDelete")]
    pub const Delete: Self = Self(2);
}

unsafe impl Encode for NSPersistentHistoryChangeType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSPersistentHistoryChangeType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPersistentHistoryChange;

    unsafe impl ClassType for NSPersistentHistoryChange {
        type Super = NSObject;
    }
);

unsafe impl NSCopying for NSPersistentHistoryChange {}

unsafe impl CopyingHelper for NSPersistentHistoryChange {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSPersistentHistoryChange {}

extern_methods!(
    unsafe impl NSPersistentHistoryChange {
        #[cfg(all(feature = "NSEntityDescription", feature = "NSManagedObjectContext"))]
        #[method_id(@__retain_semantics Other entityDescriptionWithContext:)]
        pub unsafe fn entityDescriptionWithContext(
            context: &NSManagedObjectContext,
        ) -> Option<Retained<NSEntityDescription>>;

        #[cfg(feature = "NSEntityDescription")]
        #[method_id(@__retain_semantics Other entityDescription)]
        pub unsafe fn entityDescription() -> Option<Retained<NSEntityDescription>>;

        #[cfg(all(feature = "NSFetchRequest", feature = "NSPersistentStoreRequest"))]
        #[method_id(@__retain_semantics Other fetchRequest)]
        pub unsafe fn fetchRequest() -> Option<Retained<NSFetchRequest>>;

        #[method(changeID)]
        pub unsafe fn changeID(&self) -> i64;

        #[cfg(feature = "NSManagedObjectID")]
        #[method_id(@__retain_semantics Other changedObjectID)]
        pub unsafe fn changedObjectID(&self) -> Retained<NSManagedObjectID>;

        #[method(changeType)]
        pub unsafe fn changeType(&self) -> NSPersistentHistoryChangeType;

        #[method_id(@__retain_semantics Other tombstone)]
        pub unsafe fn tombstone(&self) -> Option<Retained<NSDictionary>>;

        #[cfg(feature = "NSPersistentHistoryTransaction")]
        #[method_id(@__retain_semantics Other transaction)]
        pub unsafe fn transaction(&self) -> Option<Retained<NSPersistentHistoryTransaction>>;

        #[cfg(feature = "NSPropertyDescription")]
        #[method_id(@__retain_semantics Other updatedProperties)]
        pub unsafe fn updatedProperties(&self) -> Option<Retained<NSSet<NSPropertyDescription>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSPersistentHistoryChange {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
