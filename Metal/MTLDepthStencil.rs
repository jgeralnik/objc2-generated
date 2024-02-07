//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLCompareFunction {
        #[doc(alias = "MTLCompareFunctionNever")]
        Never = 0,
        #[doc(alias = "MTLCompareFunctionLess")]
        Less = 1,
        #[doc(alias = "MTLCompareFunctionEqual")]
        Equal = 2,
        #[doc(alias = "MTLCompareFunctionLessEqual")]
        LessEqual = 3,
        #[doc(alias = "MTLCompareFunctionGreater")]
        Greater = 4,
        #[doc(alias = "MTLCompareFunctionNotEqual")]
        NotEqual = 5,
        #[doc(alias = "MTLCompareFunctionGreaterEqual")]
        GreaterEqual = 6,
        #[doc(alias = "MTLCompareFunctionAlways")]
        Always = 7,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLStencilOperation {
        #[doc(alias = "MTLStencilOperationKeep")]
        Keep = 0,
        #[doc(alias = "MTLStencilOperationZero")]
        Zero = 1,
        #[doc(alias = "MTLStencilOperationReplace")]
        Replace = 2,
        #[doc(alias = "MTLStencilOperationIncrementClamp")]
        IncrementClamp = 3,
        #[doc(alias = "MTLStencilOperationDecrementClamp")]
        DecrementClamp = 4,
        #[doc(alias = "MTLStencilOperationInvert")]
        Invert = 5,
        #[doc(alias = "MTLStencilOperationIncrementWrap")]
        IncrementWrap = 6,
        #[doc(alias = "MTLStencilOperationDecrementWrap")]
        DecrementWrap = 7,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLStencilDescriptor")]
    pub struct MTLStencilDescriptor;

    #[cfg(feature = "Metal_MTLStencilDescriptor")]
    unsafe impl ClassType for MTLStencilDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Metal_MTLStencilDescriptor")]
unsafe impl NSCopying for MTLStencilDescriptor {}

#[cfg(feature = "Metal_MTLStencilDescriptor")]
unsafe impl NSObjectProtocol for MTLStencilDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLStencilDescriptor")]
    unsafe impl MTLStencilDescriptor {
        #[method(stencilCompareFunction)]
        pub fn stencilCompareFunction(&self) -> MTLCompareFunction;

        #[method(setStencilCompareFunction:)]
        pub fn setStencilCompareFunction(&self, stencil_compare_function: MTLCompareFunction);

        #[method(stencilFailureOperation)]
        pub fn stencilFailureOperation(&self) -> MTLStencilOperation;

        #[method(setStencilFailureOperation:)]
        pub fn setStencilFailureOperation(&self, stencil_failure_operation: MTLStencilOperation);

        #[method(depthFailureOperation)]
        pub fn depthFailureOperation(&self) -> MTLStencilOperation;

        #[method(setDepthFailureOperation:)]
        pub fn setDepthFailureOperation(&self, depth_failure_operation: MTLStencilOperation);

        #[method(depthStencilPassOperation)]
        pub fn depthStencilPassOperation(&self) -> MTLStencilOperation;

        #[method(setDepthStencilPassOperation:)]
        pub fn setDepthStencilPassOperation(
            &self,
            depth_stencil_pass_operation: MTLStencilOperation,
        );

        #[method(readMask)]
        pub fn readMask(&self) -> u32;

        #[method(setReadMask:)]
        pub fn setReadMask(&self, read_mask: u32);

        #[method(writeMask)]
        pub fn writeMask(&self) -> u32;

        #[method(setWriteMask:)]
        pub fn setWriteMask(&self, write_mask: u32);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Metal_MTLStencilDescriptor")]
    unsafe impl MTLStencilDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLDepthStencilDescriptor")]
    pub struct MTLDepthStencilDescriptor;

    #[cfg(feature = "Metal_MTLDepthStencilDescriptor")]
    unsafe impl ClassType for MTLDepthStencilDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Metal_MTLDepthStencilDescriptor")]
unsafe impl NSCopying for MTLDepthStencilDescriptor {}

#[cfg(feature = "Metal_MTLDepthStencilDescriptor")]
unsafe impl NSObjectProtocol for MTLDepthStencilDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLDepthStencilDescriptor")]
    unsafe impl MTLDepthStencilDescriptor {
        #[method(depthCompareFunction)]
        pub fn depthCompareFunction(&self) -> MTLCompareFunction;

        #[method(setDepthCompareFunction:)]
        pub fn setDepthCompareFunction(&self, depth_compare_function: MTLCompareFunction);

        #[method(isDepthWriteEnabled)]
        pub fn isDepthWriteEnabled(&self) -> bool;

        #[method(setDepthWriteEnabled:)]
        pub fn setDepthWriteEnabled(&self, depth_write_enabled: bool);

        #[cfg(feature = "Metal_MTLStencilDescriptor")]
        #[method_id(@__retain_semantics Other frontFaceStencil)]
        pub fn frontFaceStencil(&self) -> Id<MTLStencilDescriptor>;

        #[cfg(feature = "Metal_MTLStencilDescriptor")]
        #[method(setFrontFaceStencil:)]
        pub fn setFrontFaceStencil(&self, front_face_stencil: Option<&MTLStencilDescriptor>);

        #[cfg(feature = "Metal_MTLStencilDescriptor")]
        #[method_id(@__retain_semantics Other backFaceStencil)]
        pub fn backFaceStencil(&self) -> Id<MTLStencilDescriptor>;

        #[cfg(feature = "Metal_MTLStencilDescriptor")]
        #[method(setBackFaceStencil:)]
        pub fn setBackFaceStencil(&self, back_face_stencil: Option<&MTLStencilDescriptor>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        pub fn label(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLabel:)]
        pub fn setLabel(&self, label: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Metal_MTLDepthStencilDescriptor")]
    unsafe impl MTLDepthStencilDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait MTLDepthStencilState: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        fn label(&self) -> Option<Id<NSString>>;

        #[method_id(@__retain_semantics Other device)]
        fn device(&self) -> Id<ProtocolObject<dyn MTLDevice>>;
    }

    unsafe impl ProtocolType for dyn MTLDepthStencilState {}
);
