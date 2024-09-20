//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLFunctionOptions(pub NSUInteger);
bitflags::bitflags! {
    impl MTLFunctionOptions: NSUInteger {
        const MTLFunctionOptionNone = 0;
        const MTLFunctionOptionCompileToBinary = 1<<0;
        const MTLFunctionOptionStoreFunctionInMetalPipelinesScript = 1<<1;
#[deprecated]
        const MTLFunctionOptionStoreFunctionInMetalScript = 1<<1;
        const MTLFunctionOptionFailOnBinaryArchiveMiss = 1<<2;
    }
}

unsafe impl Encode for MTLFunctionOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLFunctionOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLFunctionDescriptor;

    unsafe impl ClassType for MTLFunctionDescriptor {
        type Super = NSObject;
    }
);

unsafe impl NSCopying for MTLFunctionDescriptor {}

unsafe impl CopyingHelper for MTLFunctionDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MTLFunctionDescriptor {}

extern_methods!(
    unsafe impl MTLFunctionDescriptor {
        #[method_id(@__retain_semantics Other functionDescriptor)]
        pub fn functionDescriptor() -> Retained<MTLFunctionDescriptor>;

        #[method_id(@__retain_semantics Other name)]
        pub fn name(&self) -> Option<Retained<NSString>>;

        #[method(setName:)]
        pub fn setName(&self, name: Option<&NSString>);

        #[method_id(@__retain_semantics Other specializedName)]
        pub fn specializedName(&self) -> Option<Retained<NSString>>;

        #[method(setSpecializedName:)]
        pub fn setSpecializedName(&self, specialized_name: Option<&NSString>);

        #[cfg(feature = "MTLFunctionConstantValues")]
        #[method_id(@__retain_semantics Other constantValues)]
        pub fn constantValues(&self) -> Option<Retained<MTLFunctionConstantValues>>;

        #[cfg(feature = "MTLFunctionConstantValues")]
        #[method(setConstantValues:)]
        pub fn setConstantValues(&self, constant_values: Option<&MTLFunctionConstantValues>);

        #[method(options)]
        pub fn options(&self) -> MTLFunctionOptions;

        #[method(setOptions:)]
        pub fn setOptions(&self, options: MTLFunctionOptions);

        #[cfg(feature = "MTLBinaryArchive")]
        #[method_id(@__retain_semantics Other binaryArchives)]
        pub unsafe fn binaryArchives(
            &self,
        ) -> Option<Retained<NSArray<ProtocolObject<dyn MTLBinaryArchive>>>>;

        #[cfg(feature = "MTLBinaryArchive")]
        #[method(setBinaryArchives:)]
        pub unsafe fn setBinaryArchives(
            &self,
            binary_archives: Option<&NSArray<ProtocolObject<dyn MTLBinaryArchive>>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLFunctionDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Retained<Self>;
    }
);

impl DefaultRetained for MTLFunctionDescriptor {
    #[inline]
    fn default_retained() -> Retained<Self> {
        Self::new()
    }
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLIntersectionFunctionDescriptor;

    unsafe impl ClassType for MTLIntersectionFunctionDescriptor {
        #[inherits(NSObject)]
        type Super = MTLFunctionDescriptor;
    }
);

unsafe impl NSCopying for MTLIntersectionFunctionDescriptor {}

unsafe impl CopyingHelper for MTLIntersectionFunctionDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MTLIntersectionFunctionDescriptor {}

extern_methods!(
    unsafe impl MTLIntersectionFunctionDescriptor {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLIntersectionFunctionDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
