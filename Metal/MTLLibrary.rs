//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLPatchType(pub NSUInteger);
impl MTLPatchType {
    #[doc(alias = "MTLPatchTypeNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "MTLPatchTypeTriangle")]
    pub const Triangle: Self = Self(1);
    #[doc(alias = "MTLPatchTypeQuad")]
    pub const Quad: Self = Self(2);
}

unsafe impl Encode for MTLPatchType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLPatchType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLVertexAttribute;
);

unsafe impl NSObjectProtocol for MTLVertexAttribute {}

extern_methods!(
    unsafe impl MTLVertexAttribute {
        #[method_id(@__retain_semantics Other name)]
        pub fn name(&self) -> Retained<NSString>;

        #[method(attributeIndex)]
        pub fn attributeIndex(&self) -> NSUInteger;

        #[cfg(feature = "MTLArgument")]
        #[method(attributeType)]
        pub fn attributeType(&self) -> MTLDataType;

        #[method(isActive)]
        pub fn isActive(&self) -> bool;

        #[method(isPatchData)]
        pub fn isPatchData(&self) -> bool;

        #[method(isPatchControlPointData)]
        pub fn isPatchControlPointData(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLVertexAttribute {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLAttribute;
);

unsafe impl NSObjectProtocol for MTLAttribute {}

extern_methods!(
    unsafe impl MTLAttribute {
        #[method_id(@__retain_semantics Other name)]
        pub fn name(&self) -> Retained<NSString>;

        #[method(attributeIndex)]
        pub fn attributeIndex(&self) -> NSUInteger;

        #[cfg(feature = "MTLArgument")]
        #[method(attributeType)]
        pub fn attributeType(&self) -> MTLDataType;

        #[method(isActive)]
        pub fn isActive(&self) -> bool;

        #[method(isPatchData)]
        pub fn isPatchData(&self) -> bool;

        #[method(isPatchControlPointData)]
        pub fn isPatchControlPointData(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLAttribute {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLFunctionType(pub NSUInteger);
impl MTLFunctionType {
    #[doc(alias = "MTLFunctionTypeVertex")]
    pub const Vertex: Self = Self(1);
    #[doc(alias = "MTLFunctionTypeFragment")]
    pub const Fragment: Self = Self(2);
    #[doc(alias = "MTLFunctionTypeKernel")]
    pub const Kernel: Self = Self(3);
    #[doc(alias = "MTLFunctionTypeVisible")]
    pub const Visible: Self = Self(5);
    #[doc(alias = "MTLFunctionTypeIntersection")]
    pub const Intersection: Self = Self(6);
    #[doc(alias = "MTLFunctionTypeMesh")]
    pub const Mesh: Self = Self(7);
    #[doc(alias = "MTLFunctionTypeObject")]
    pub const Object: Self = Self(8);
}

unsafe impl Encode for MTLFunctionType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLFunctionType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLFunctionConstant;
);

unsafe impl NSObjectProtocol for MTLFunctionConstant {}

extern_methods!(
    unsafe impl MTLFunctionConstant {
        #[method_id(@__retain_semantics Other name)]
        pub fn name(&self) -> Retained<NSString>;

        #[cfg(feature = "MTLArgument")]
        #[method(type)]
        pub unsafe fn r#type(&self) -> MTLDataType;

        #[method(index)]
        pub fn index(&self) -> NSUInteger;

        #[method(required)]
        pub fn required(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLFunctionConstant {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    pub unsafe trait MTLFunction: NSObjectProtocol {
        #[method_id(@__retain_semantics Other label)]
        fn label(&self) -> Option<Retained<NSString>>;

        #[method(setLabel:)]
        fn setLabel(&self, label: Option<&NSString>);

        #[cfg(feature = "MTLDevice")]
        #[method_id(@__retain_semantics Other device)]
        fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        #[method(functionType)]
        fn functionType(&self) -> MTLFunctionType;

        #[method(patchType)]
        fn patchType(&self) -> MTLPatchType;

        #[method(patchControlPointCount)]
        fn patchControlPointCount(&self) -> NSInteger;

        #[method_id(@__retain_semantics Other vertexAttributes)]
        fn vertexAttributes(&self) -> Option<Retained<NSArray<MTLVertexAttribute>>>;

        #[method_id(@__retain_semantics Other stageInputAttributes)]
        fn stageInputAttributes(&self) -> Option<Retained<NSArray<MTLAttribute>>>;

        #[method_id(@__retain_semantics Other name)]
        fn name(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other functionConstantsDictionary)]
        fn functionConstantsDictionary(
            &self,
        ) -> Retained<NSDictionary<NSString, MTLFunctionConstant>>;

        #[cfg(feature = "MTLArgumentEncoder")]
        #[method_id(@__retain_semantics New newArgumentEncoderWithBufferIndex:)]
        unsafe fn newArgumentEncoderWithBufferIndex(
            &self,
            buffer_index: NSUInteger,
        ) -> Retained<ProtocolObject<dyn MTLArgumentEncoder>>;

        #[cfg(feature = "MTLFunctionDescriptor")]
        #[method(options)]
        fn options(&self) -> MTLFunctionOptions;
    }

    unsafe impl ProtocolType for dyn MTLFunction {}
);

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLLanguageVersion(pub NSUInteger);
impl MTLLanguageVersion {
    #[deprecated = "Use a newer language standard"]
    pub const MTLLanguageVersion1_0: Self = Self(1 << 16);
    pub const MTLLanguageVersion1_1: Self = Self((1 << 16) + 1);
    pub const MTLLanguageVersion1_2: Self = Self((1 << 16) + 2);
    pub const MTLLanguageVersion2_0: Self = Self(2 << 16);
    pub const MTLLanguageVersion2_1: Self = Self((2 << 16) + 1);
    pub const MTLLanguageVersion2_2: Self = Self((2 << 16) + 2);
    pub const MTLLanguageVersion2_3: Self = Self((2 << 16) + 3);
    pub const MTLLanguageVersion2_4: Self = Self((2 << 16) + 4);
    pub const MTLLanguageVersion3_0: Self = Self((3 << 16) + 0);
    pub const MTLLanguageVersion3_1: Self = Self((3 << 16) + 1);
    pub const MTLLanguageVersion3_2: Self = Self((3 << 16) + 2);
}

unsafe impl Encode for MTLLanguageVersion {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLLanguageVersion {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLLibraryType(pub NSInteger);
impl MTLLibraryType {
    #[doc(alias = "MTLLibraryTypeExecutable")]
    pub const Executable: Self = Self(0);
    #[doc(alias = "MTLLibraryTypeDynamic")]
    pub const Dynamic: Self = Self(1);
}

unsafe impl Encode for MTLLibraryType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MTLLibraryType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLLibraryOptimizationLevel(pub NSInteger);
impl MTLLibraryOptimizationLevel {
    #[doc(alias = "MTLLibraryOptimizationLevelDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "MTLLibraryOptimizationLevelSize")]
    pub const Size: Self = Self(1);
}

unsafe impl Encode for MTLLibraryOptimizationLevel {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MTLLibraryOptimizationLevel {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLCompileSymbolVisibility(pub NSInteger);
impl MTLCompileSymbolVisibility {
    #[doc(alias = "MTLCompileSymbolVisibilityDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "MTLCompileSymbolVisibilityHidden")]
    pub const Hidden: Self = Self(1);
}

unsafe impl Encode for MTLCompileSymbolVisibility {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MTLCompileSymbolVisibility {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLMathMode(pub NSInteger);
impl MTLMathMode {
    #[doc(alias = "MTLMathModeSafe")]
    pub const Safe: Self = Self(0);
    #[doc(alias = "MTLMathModeRelaxed")]
    pub const Relaxed: Self = Self(1);
    #[doc(alias = "MTLMathModeFast")]
    pub const Fast: Self = Self(2);
}

unsafe impl Encode for MTLMathMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MTLMathMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLMathFloatingPointFunctions(pub NSInteger);
impl MTLMathFloatingPointFunctions {
    #[doc(alias = "MTLMathFloatingPointFunctionsFast")]
    pub const Fast: Self = Self(0);
    #[doc(alias = "MTLMathFloatingPointFunctionsPrecise")]
    pub const Precise: Self = Self(1);
}

unsafe impl Encode for MTLMathFloatingPointFunctions {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MTLMathFloatingPointFunctions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLCompileOptions;
);

unsafe impl NSCopying for MTLCompileOptions {}

unsafe impl CopyingHelper for MTLCompileOptions {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MTLCompileOptions {}

extern_methods!(
    unsafe impl MTLCompileOptions {
        #[method_id(@__retain_semantics Other preprocessorMacros)]
        pub fn preprocessorMacros(&self) -> Option<Retained<NSDictionary<NSString, NSObject>>>;

        #[method(setPreprocessorMacros:)]
        pub unsafe fn setPreprocessorMacros(
            &self,
            preprocessor_macros: Option<&NSDictionary<NSString, NSObject>>,
        );

        #[deprecated = "Use mathMode instead"]
        #[method(fastMathEnabled)]
        pub fn fastMathEnabled(&self) -> bool;

        #[deprecated = "Use mathMode instead"]
        #[method(setFastMathEnabled:)]
        pub fn setFastMathEnabled(&self, fast_math_enabled: bool);

        #[method(mathMode)]
        pub unsafe fn mathMode(&self) -> MTLMathMode;

        #[method(setMathMode:)]
        pub unsafe fn setMathMode(&self, math_mode: MTLMathMode);

        #[method(mathFloatingPointFunctions)]
        pub unsafe fn mathFloatingPointFunctions(&self) -> MTLMathFloatingPointFunctions;

        #[method(setMathFloatingPointFunctions:)]
        pub unsafe fn setMathFloatingPointFunctions(
            &self,
            math_floating_point_functions: MTLMathFloatingPointFunctions,
        );

        #[method(languageVersion)]
        pub fn languageVersion(&self) -> MTLLanguageVersion;

        #[method(setLanguageVersion:)]
        pub fn setLanguageVersion(&self, language_version: MTLLanguageVersion);

        #[method(libraryType)]
        pub fn libraryType(&self) -> MTLLibraryType;

        #[method(setLibraryType:)]
        pub fn setLibraryType(&self, library_type: MTLLibraryType);

        #[method_id(@__retain_semantics Other installName)]
        pub fn installName(&self) -> Option<Retained<NSString>>;

        #[method(setInstallName:)]
        pub unsafe fn setInstallName(&self, install_name: Option<&NSString>);

        #[cfg(feature = "MTLDynamicLibrary")]
        #[method_id(@__retain_semantics Other libraries)]
        pub fn libraries(&self)
            -> Option<Retained<NSArray<ProtocolObject<dyn MTLDynamicLibrary>>>>;

        #[cfg(feature = "MTLDynamicLibrary")]
        #[method(setLibraries:)]
        pub fn setLibraries(
            &self,
            libraries: Option<&NSArray<ProtocolObject<dyn MTLDynamicLibrary>>>,
        );

        #[method(preserveInvariance)]
        pub fn preserveInvariance(&self) -> bool;

        #[method(setPreserveInvariance:)]
        pub fn setPreserveInvariance(&self, preserve_invariance: bool);

        #[method(optimizationLevel)]
        pub unsafe fn optimizationLevel(&self) -> MTLLibraryOptimizationLevel;

        #[method(setOptimizationLevel:)]
        pub unsafe fn setOptimizationLevel(&self, optimization_level: MTLLibraryOptimizationLevel);

        #[method(compileSymbolVisibility)]
        pub unsafe fn compileSymbolVisibility(&self) -> MTLCompileSymbolVisibility;

        #[method(setCompileSymbolVisibility:)]
        pub unsafe fn setCompileSymbolVisibility(
            &self,
            compile_symbol_visibility: MTLCompileSymbolVisibility,
        );

        #[method(allowReferencingUndefinedSymbols)]
        pub unsafe fn allowReferencingUndefinedSymbols(&self) -> bool;

        #[method(setAllowReferencingUndefinedSymbols:)]
        pub unsafe fn setAllowReferencingUndefinedSymbols(
            &self,
            allow_referencing_undefined_symbols: bool,
        );

        #[method(maxTotalThreadsPerThreadgroup)]
        pub unsafe fn maxTotalThreadsPerThreadgroup(&self) -> NSUInteger;

        #[method(setMaxTotalThreadsPerThreadgroup:)]
        pub unsafe fn setMaxTotalThreadsPerThreadgroup(
            &self,
            max_total_threads_per_threadgroup: NSUInteger,
        );

        #[method(enableLogging)]
        pub unsafe fn enableLogging(&self) -> bool;

        #[method(setEnableLogging:)]
        pub unsafe fn setEnableLogging(&self, enable_logging: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLCompileOptions {
        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Retained<Self>;
    }
);

impl DefaultRetained for MTLCompileOptions {
    #[inline]
    fn default_retained() -> Retained<Self> {
        Self::new()
    }
}

extern "C" {
    pub static MTLLibraryErrorDomain: &'static NSErrorDomain;
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLLibraryError(pub NSUInteger);
impl MTLLibraryError {
    #[doc(alias = "MTLLibraryErrorUnsupported")]
    pub const Unsupported: Self = Self(1);
    #[doc(alias = "MTLLibraryErrorInternal")]
    pub const Internal: Self = Self(2);
    #[doc(alias = "MTLLibraryErrorCompileFailure")]
    pub const CompileFailure: Self = Self(3);
    #[doc(alias = "MTLLibraryErrorCompileWarning")]
    pub const CompileWarning: Self = Self(4);
    #[doc(alias = "MTLLibraryErrorFunctionNotFound")]
    pub const FunctionNotFound: Self = Self(5);
    #[doc(alias = "MTLLibraryErrorFileNotFound")]
    pub const FileNotFound: Self = Self(6);
}

unsafe impl Encode for MTLLibraryError {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLLibraryError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    pub unsafe trait MTLLibrary: NSObjectProtocol {
        #[method_id(@__retain_semantics Other label)]
        fn label(&self) -> Option<Retained<NSString>>;

        #[method(setLabel:)]
        fn setLabel(&self, label: Option<&NSString>);

        #[cfg(feature = "MTLDevice")]
        #[method_id(@__retain_semantics Other device)]
        fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        #[method_id(@__retain_semantics New newFunctionWithName:)]
        fn newFunctionWithName(
            &self,
            function_name: &NSString,
        ) -> Option<Retained<ProtocolObject<dyn MTLFunction>>>;

        #[cfg(feature = "MTLFunctionConstantValues")]
        #[method_id(@__retain_semantics New newFunctionWithName:constantValues:error:_)]
        fn newFunctionWithName_constantValues_error(
            &self,
            name: &NSString,
            constant_values: &MTLFunctionConstantValues,
        ) -> Result<Retained<ProtocolObject<dyn MTLFunction>>, Retained<NSError>>;

        #[cfg(all(feature = "MTLFunctionConstantValues", feature = "block2"))]
        #[method(newFunctionWithName:constantValues:completionHandler:)]
        unsafe fn newFunctionWithName_constantValues_completionHandler(
            &self,
            name: &NSString,
            constant_values: &MTLFunctionConstantValues,
            completion_handler: &block2::Block<
                dyn Fn(*mut ProtocolObject<dyn MTLFunction>, *mut NSError),
            >,
        );

        #[cfg(all(feature = "MTLFunctionDescriptor", feature = "block2"))]
        #[method(newFunctionWithDescriptor:completionHandler:)]
        unsafe fn newFunctionWithDescriptor_completionHandler(
            &self,
            descriptor: &MTLFunctionDescriptor,
            completion_handler: &block2::Block<
                dyn Fn(*mut ProtocolObject<dyn MTLFunction>, *mut NSError),
            >,
        );

        #[cfg(feature = "MTLFunctionDescriptor")]
        #[method_id(@__retain_semantics New newFunctionWithDescriptor:error:_)]
        fn newFunctionWithDescriptor_error(
            &self,
            descriptor: &MTLFunctionDescriptor,
        ) -> Result<Retained<ProtocolObject<dyn MTLFunction>>, Retained<NSError>>;

        #[cfg(all(feature = "MTLFunctionDescriptor", feature = "block2"))]
        #[method(newIntersectionFunctionWithDescriptor:completionHandler:)]
        unsafe fn newIntersectionFunctionWithDescriptor_completionHandler(
            &self,
            descriptor: &MTLIntersectionFunctionDescriptor,
            completion_handler: &block2::Block<
                dyn Fn(*mut ProtocolObject<dyn MTLFunction>, *mut NSError),
            >,
        );

        #[cfg(feature = "MTLFunctionDescriptor")]
        #[method_id(@__retain_semantics New newIntersectionFunctionWithDescriptor:error:_)]
        fn newIntersectionFunctionWithDescriptor_error(
            &self,
            descriptor: &MTLIntersectionFunctionDescriptor,
        ) -> Result<Retained<ProtocolObject<dyn MTLFunction>>, Retained<NSError>>;

        #[method_id(@__retain_semantics Other functionNames)]
        fn functionNames(&self) -> Retained<NSArray<NSString>>;

        #[method(type)]
        unsafe fn r#type(&self) -> MTLLibraryType;

        #[method_id(@__retain_semantics Other installName)]
        fn installName(&self) -> Option<Retained<NSString>>;
    }

    unsafe impl ProtocolType for dyn MTLLibrary {}
);
