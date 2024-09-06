//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSStagedMigrationManager;

    unsafe impl ClassType for NSStagedMigrationManager {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for NSStagedMigrationManager {}

extern_methods!(
    unsafe impl NSStagedMigrationManager {
        #[cfg(feature = "NSMigrationStage")]
        #[method_id(@__retain_semantics Other stages)]
        pub unsafe fn stages(&self) -> Retained<NSArray<NSMigrationStage>>;

        #[cfg(feature = "NSPersistentContainer")]
        #[method_id(@__retain_semantics Other container)]
        pub unsafe fn container(&self) -> Option<Retained<NSPersistentContainer>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "NSMigrationStage")]
        #[method_id(@__retain_semantics Init initWithMigrationStages:)]
        pub unsafe fn initWithMigrationStages(
            this: Allocated<Self>,
            stages: &NSArray<NSMigrationStage>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSStagedMigrationManager {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
