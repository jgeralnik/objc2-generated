//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MLComputePlan;

    unsafe impl ClassType for MLComputePlan {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for MLComputePlan {}

extern_methods!(
    unsafe impl MLComputePlan {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(all(feature = "MLModelConfiguration", feature = "block2"))]
        #[method(loadContentsOfURL:configuration:completionHandler:)]
        pub unsafe fn loadContentsOfURL_configuration_completionHandler(
            url: &NSURL,
            configuration: &MLModelConfiguration,
            handler: &block2::Block<dyn Fn(*mut MLComputePlan, *mut NSError)>,
        );

        #[cfg(all(
            feature = "MLModelAsset",
            feature = "MLModelConfiguration",
            feature = "block2"
        ))]
        #[method(loadModelAsset:configuration:completionHandler:)]
        pub unsafe fn loadModelAsset_configuration_completionHandler(
            asset: &MLModelAsset,
            configuration: &MLModelConfiguration,
            handler: &block2::Block<dyn Fn(*mut MLComputePlan, *mut NSError)>,
        );

        #[cfg(all(
            feature = "MLComputePlanCost",
            feature = "MLModelStructureProgramOperation"
        ))]
        #[method_id(@__retain_semantics Other estimatedCostOfMLProgramOperation:)]
        pub unsafe fn estimatedCostOfMLProgramOperation(
            &self,
            operation: &MLModelStructureProgramOperation,
        ) -> Option<Retained<MLComputePlanCost>>;

        #[cfg(all(
            feature = "MLComputePlanDeviceUsage",
            feature = "MLModelStructureNeuralNetworkLayer"
        ))]
        #[method_id(@__retain_semantics Other computeDeviceUsageForNeuralNetworkLayer:)]
        pub unsafe fn computeDeviceUsageForNeuralNetworkLayer(
            &self,
            layer: &MLModelStructureNeuralNetworkLayer,
        ) -> Option<Retained<MLComputePlanDeviceUsage>>;

        #[cfg(all(
            feature = "MLComputePlanDeviceUsage",
            feature = "MLModelStructureProgramOperation"
        ))]
        #[method_id(@__retain_semantics Other computeDeviceUsageForMLProgramOperation:)]
        pub unsafe fn computeDeviceUsageForMLProgramOperation(
            &self,
            operation: &MLModelStructureProgramOperation,
        ) -> Option<Retained<MLComputePlanDeviceUsage>>;

        #[cfg(feature = "MLModelStructure")]
        #[method_id(@__retain_semantics Other modelStructure)]
        pub unsafe fn modelStructure(&self) -> Retained<MLModelStructure>;
    }
);
