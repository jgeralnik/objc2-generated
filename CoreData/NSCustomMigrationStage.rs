//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSMigrationStage, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSMigrationStage")]
    pub struct NSCustomMigrationStage;
);

#[cfg(feature = "NSMigrationStage")]
unsafe impl NSObjectProtocol for NSCustomMigrationStage {}

extern_methods!(
    #[cfg(feature = "NSMigrationStage")]
    unsafe impl NSCustomMigrationStage {
        #[cfg(feature = "NSManagedObjectModelReference")]
        #[method_id(@__retain_semantics Other currentModel)]
        pub unsafe fn currentModel(&self) -> Retained<NSManagedObjectModelReference>;

        #[cfg(feature = "NSManagedObjectModelReference")]
        #[method_id(@__retain_semantics Other nextModel)]
        pub unsafe fn nextModel(&self) -> Retained<NSManagedObjectModelReference>;

        #[cfg(all(feature = "NSStagedMigrationManager", feature = "block2"))]
        #[method(willMigrateHandler)]
        pub unsafe fn willMigrateHandler(
            &self,
        ) -> *mut block2::Block<
            dyn Fn(
                NonNull<NSStagedMigrationManager>,
                NonNull<NSCustomMigrationStage>,
                *mut *mut NSError,
            ) -> Bool,
        >;

        #[cfg(all(feature = "NSStagedMigrationManager", feature = "block2"))]
        #[method(setWillMigrateHandler:)]
        pub unsafe fn setWillMigrateHandler(
            &self,
            will_migrate_handler: Option<
                &block2::Block<
                    dyn Fn(
                        NonNull<NSStagedMigrationManager>,
                        NonNull<NSCustomMigrationStage>,
                        *mut *mut NSError,
                    ) -> Bool,
                >,
            >,
        );

        #[cfg(all(feature = "NSStagedMigrationManager", feature = "block2"))]
        #[method(didMigrateHandler)]
        pub unsafe fn didMigrateHandler(
            &self,
        ) -> *mut block2::Block<
            dyn Fn(
                NonNull<NSStagedMigrationManager>,
                NonNull<NSCustomMigrationStage>,
                *mut *mut NSError,
            ) -> Bool,
        >;

        #[cfg(all(feature = "NSStagedMigrationManager", feature = "block2"))]
        #[method(setDidMigrateHandler:)]
        pub unsafe fn setDidMigrateHandler(
            &self,
            did_migrate_handler: Option<
                &block2::Block<
                    dyn Fn(
                        NonNull<NSStagedMigrationManager>,
                        NonNull<NSCustomMigrationStage>,
                        *mut *mut NSError,
                    ) -> Bool,
                >,
            >,
        );

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "NSManagedObjectModelReference")]
        #[method_id(@__retain_semantics Init initWithCurrentModelReference:nextModelReference:)]
        pub unsafe fn initWithCurrentModelReference_nextModelReference(
            this: Allocated<Self>,
            current_model: &NSManagedObjectModelReference,
            next_model: &NSManagedObjectModelReference,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSMigrationStage")]
    unsafe impl NSCustomMigrationStage {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
