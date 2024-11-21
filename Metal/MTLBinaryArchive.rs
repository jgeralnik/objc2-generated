//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    pub static MTLBinaryArchiveDomain: &'static NSErrorDomain;
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLBinaryArchiveError(pub NSUInteger);
impl MTLBinaryArchiveError {
    #[doc(alias = "MTLBinaryArchiveErrorNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "MTLBinaryArchiveErrorInvalidFile")]
    pub const InvalidFile: Self = Self(1);
    #[doc(alias = "MTLBinaryArchiveErrorUnexpectedElement")]
    pub const UnexpectedElement: Self = Self(2);
    #[doc(alias = "MTLBinaryArchiveErrorCompilationFailure")]
    pub const CompilationFailure: Self = Self(3);
    #[doc(alias = "MTLBinaryArchiveErrorInternalError")]
    pub const InternalError: Self = Self(4);
}

unsafe impl Encode for MTLBinaryArchiveError {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLBinaryArchiveError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLBinaryArchiveDescriptor;
);

unsafe impl NSCopying for MTLBinaryArchiveDescriptor {}

unsafe impl CopyingHelper for MTLBinaryArchiveDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MTLBinaryArchiveDescriptor {}

extern_methods!(
    unsafe impl MTLBinaryArchiveDescriptor {
        #[method_id(@__retain_semantics Other url)]
        pub fn url(&self) -> Option<Retained<NSURL>>;

        #[method(setUrl:)]
        pub fn setUrl(&self, url: Option<&NSURL>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLBinaryArchiveDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Retained<Self>;
    }
);

impl DefaultRetained for MTLBinaryArchiveDescriptor {
    #[inline]
    fn default_retained() -> Retained<Self> {
        Self::new()
    }
}

extern_protocol!(
    pub unsafe trait MTLBinaryArchive: NSObjectProtocol {
        #[method_id(@__retain_semantics Other label)]
        fn label(&self) -> Option<Retained<NSString>>;

        #[method(setLabel:)]
        fn setLabel(&self, label: Option<&NSString>);

        #[cfg(feature = "MTLDevice")]
        #[method_id(@__retain_semantics Other device)]
        fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        #[cfg(feature = "MTLComputePipeline")]
        #[method(addComputePipelineFunctionsWithDescriptor:error:_)]
        fn addComputePipelineFunctionsWithDescriptor_error(
            &self,
            descriptor: &MTLComputePipelineDescriptor,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "MTLRenderPipeline")]
        #[method(addRenderPipelineFunctionsWithDescriptor:error:_)]
        fn addRenderPipelineFunctionsWithDescriptor_error(
            &self,
            descriptor: &MTLRenderPipelineDescriptor,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "MTLRenderPipeline")]
        #[method(addTileRenderPipelineFunctionsWithDescriptor:error:_)]
        unsafe fn addTileRenderPipelineFunctionsWithDescriptor_error(
            &self,
            descriptor: &MTLTileRenderPipelineDescriptor,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "MTLRenderPipeline")]
        #[method(addMeshRenderPipelineFunctionsWithDescriptor:error:_)]
        unsafe fn addMeshRenderPipelineFunctionsWithDescriptor_error(
            &self,
            descriptor: &MTLMeshRenderPipelineDescriptor,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "MTLFunctionStitching")]
        #[method(addLibraryWithDescriptor:error:_)]
        unsafe fn addLibraryWithDescriptor_error(
            &self,
            descriptor: &MTLStitchedLibraryDescriptor,
        ) -> Result<(), Retained<NSError>>;

        #[method(serializeToURL:error:_)]
        fn serializeToURL_error(&self, url: &NSURL) -> Result<(), Retained<NSError>>;

        #[cfg(all(feature = "MTLFunctionDescriptor", feature = "MTLLibrary"))]
        #[method(addFunctionWithDescriptor:library:error:_)]
        unsafe fn addFunctionWithDescriptor_library_error(
            &self,
            descriptor: &MTLFunctionDescriptor,
            library: &ProtocolObject<dyn MTLLibrary>,
        ) -> Result<(), Retained<NSError>>;
    }

    unsafe impl ProtocolType for dyn MTLBinaryArchive {}
);
