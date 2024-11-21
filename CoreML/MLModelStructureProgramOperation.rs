//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MLModelStructureProgramOperation;
);

unsafe impl Send for MLModelStructureProgramOperation {}

unsafe impl Sync for MLModelStructureProgramOperation {}

unsafe impl NSObjectProtocol for MLModelStructureProgramOperation {}

extern_methods!(
    unsafe impl MLModelStructureProgramOperation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Other operatorName)]
        pub unsafe fn operatorName(&self) -> Retained<NSString>;

        #[cfg(feature = "MLModelStructureProgramArgument")]
        #[method_id(@__retain_semantics Other inputs)]
        pub unsafe fn inputs(
            &self,
        ) -> Retained<NSDictionary<NSString, MLModelStructureProgramArgument>>;

        #[cfg(feature = "MLModelStructureProgramNamedValueType")]
        #[method_id(@__retain_semantics Other outputs)]
        pub unsafe fn outputs(&self) -> Retained<NSArray<MLModelStructureProgramNamedValueType>>;

        #[cfg(feature = "MLModelStructureProgramBlock")]
        #[method_id(@__retain_semantics Other blocks)]
        pub unsafe fn blocks(&self) -> Retained<NSArray<MLModelStructureProgramBlock>>;
    }
);
