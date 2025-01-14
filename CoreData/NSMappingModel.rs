//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsmappingmodel?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSMappingModel;
);

unsafe impl NSObjectProtocol for NSMappingModel {}

extern_methods!(
    unsafe impl NSMappingModel {
        #[cfg(feature = "NSManagedObjectModel")]
        #[method_id(@__retain_semantics Other mappingModelFromBundles:forSourceModel:destinationModel:)]
        pub unsafe fn mappingModelFromBundles_forSourceModel_destinationModel(
            bundles: Option<&NSArray<NSBundle>>,
            source_model: Option<&NSManagedObjectModel>,
            destination_model: Option<&NSManagedObjectModel>,
        ) -> Option<Retained<NSMappingModel>>;

        #[cfg(feature = "NSManagedObjectModel")]
        #[method_id(@__retain_semantics Other inferredMappingModelForSourceModel:destinationModel:error:_)]
        pub unsafe fn inferredMappingModelForSourceModel_destinationModel_error(
            source_model: &NSManagedObjectModel,
            destination_model: &NSManagedObjectModel,
        ) -> Result<Retained<NSMappingModel>, Retained<NSError>>;

        #[method_id(@__retain_semantics Init initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            this: Allocated<Self>,
            url: Option<&NSURL>,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "NSEntityMapping")]
        #[method_id(@__retain_semantics Other entityMappings)]
        pub unsafe fn entityMappings(&self) -> Option<Retained<NSArray<NSEntityMapping>>>;

        #[cfg(feature = "NSEntityMapping")]
        #[method(setEntityMappings:)]
        pub unsafe fn setEntityMappings(&self, entity_mappings: Option<&NSArray<NSEntityMapping>>);

        #[cfg(feature = "NSEntityMapping")]
        #[method_id(@__retain_semantics Other entityMappingsByName)]
        pub unsafe fn entityMappingsByName(
            &self,
        ) -> Retained<NSDictionary<NSString, NSEntityMapping>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSMappingModel {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
