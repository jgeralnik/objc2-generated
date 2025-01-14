//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlcgraphcompletionhandler?language=objc)
#[cfg(all(feature = "MLCTensor", feature = "block2"))]
pub type MLCGraphCompletionHandler =
    *mut block2::Block<dyn Fn(*mut MLCTensor, *mut NSError, NSTimeInterval)>;

/// [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlcdatatype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MLCDataType(pub i32);
impl MLCDataType {
    #[doc(alias = "MLCDataTypeInvalid")]
    pub const Invalid: Self = Self(0);
    #[doc(alias = "MLCDataTypeFloat32")]
    pub const Float32: Self = Self(1);
    #[doc(alias = "MLCDataTypeFloat16")]
    pub const Float16: Self = Self(3);
    #[doc(alias = "MLCDataTypeBoolean")]
    pub const Boolean: Self = Self(4);
    #[doc(alias = "MLCDataTypeInt64")]
    pub const Int64: Self = Self(5);
    #[doc(alias = "MLCDataTypeInt32")]
    pub const Int32: Self = Self(7);
    #[doc(alias = "MLCDataTypeInt8")]
    pub const Int8: Self = Self(8);
    #[doc(alias = "MLCDataTypeUInt8")]
    pub const UInt8: Self = Self(9);
    #[doc(alias = "MLCDataTypeCount")]
    pub const Count: Self = Self(10);
}

unsafe impl Encode for MLCDataType {
    const ENCODING: Encoding = i32::ENCODING;
}

unsafe impl RefEncode for MLCDataType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlcrandominitializertype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MLCRandomInitializerType(pub i32);
impl MLCRandomInitializerType {
    #[doc(alias = "MLCRandomInitializerTypeInvalid")]
    pub const Invalid: Self = Self(0);
    #[doc(alias = "MLCRandomInitializerTypeUniform")]
    pub const Uniform: Self = Self(1);
    #[doc(alias = "MLCRandomInitializerTypeGlorotUniform")]
    pub const GlorotUniform: Self = Self(2);
    #[doc(alias = "MLCRandomInitializerTypeXavier")]
    pub const Xavier: Self = Self(3);
    #[doc(alias = "MLCRandomInitializerTypeCount")]
    pub const Count: Self = Self(4);
}

unsafe impl Encode for MLCRandomInitializerType {
    const ENCODING: Encoding = i32::ENCODING;
}

unsafe impl RefEncode for MLCRandomInitializerType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlcdevicetype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MLCDeviceType(pub i32);
impl MLCDeviceType {
    #[doc(alias = "MLCDeviceTypeCPU")]
    pub const CPU: Self = Self(0);
    #[doc(alias = "MLCDeviceTypeGPU")]
    pub const GPU: Self = Self(1);
    #[doc(alias = "MLCDeviceTypeAny")]
    pub const Any: Self = Self(2);
    #[doc(alias = "MLCDeviceTypeANE")]
    pub const ANE: Self = Self(3);
    #[doc(alias = "MLCDeviceTypeCount")]
    pub const Count: Self = Self(4);
}

unsafe impl Encode for MLCDeviceType {
    const ENCODING: Encoding = i32::ENCODING;
}

unsafe impl RefEncode for MLCDeviceType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlcgraphcompilationoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MLCGraphCompilationOptions(pub u64);
bitflags::bitflags! {
    impl MLCGraphCompilationOptions: u64 {
        #[doc(alias = "MLCGraphCompilationOptionsNone")]
        const None = 0x00;
        #[doc(alias = "MLCGraphCompilationOptionsDebugLayers")]
        const DebugLayers = 0x01;
        #[doc(alias = "MLCGraphCompilationOptionsDisableLayerFusion")]
        const DisableLayerFusion = 0x02;
        #[doc(alias = "MLCGraphCompilationOptionsLinkGraphs")]
        const LinkGraphs = 0x04;
        #[doc(alias = "MLCGraphCompilationOptionsComputeAllGradients")]
        const ComputeAllGradients = 0x08;
    }
}

unsafe impl Encode for MLCGraphCompilationOptions {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MLCGraphCompilationOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlcexecutionoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MLCExecutionOptions(pub u64);
bitflags::bitflags! {
    impl MLCExecutionOptions: u64 {
        #[doc(alias = "MLCExecutionOptionsNone")]
        const None = 0x00;
        #[doc(alias = "MLCExecutionOptionsSkipWritingInputDataToDevice")]
        const SkipWritingInputDataToDevice = 0x01;
        #[doc(alias = "MLCExecutionOptionsSynchronous")]
        const Synchronous = 0x02;
        #[doc(alias = "MLCExecutionOptionsProfiling")]
        const Profiling = 0x04;
        #[doc(alias = "MLCExecutionOptionsForwardForInference")]
        const ForwardForInference = 0x08;
        #[doc(alias = "MLCExecutionOptionsPerLayerProfiling")]
        const PerLayerProfiling = 0x10;
    }
}

unsafe impl Encode for MLCExecutionOptions {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MLCExecutionOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlcarithmeticoperation?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MLCArithmeticOperation(pub i32);
impl MLCArithmeticOperation {
    #[doc(alias = "MLCArithmeticOperationAdd")]
    pub const Add: Self = Self(0);
    #[doc(alias = "MLCArithmeticOperationSubtract")]
    pub const Subtract: Self = Self(1);
    #[doc(alias = "MLCArithmeticOperationMultiply")]
    pub const Multiply: Self = Self(2);
    #[doc(alias = "MLCArithmeticOperationDivide")]
    pub const Divide: Self = Self(3);
    #[doc(alias = "MLCArithmeticOperationFloor")]
    pub const Floor: Self = Self(4);
    #[doc(alias = "MLCArithmeticOperationRound")]
    pub const Round: Self = Self(5);
    #[doc(alias = "MLCArithmeticOperationCeil")]
    pub const Ceil: Self = Self(6);
    #[doc(alias = "MLCArithmeticOperationSqrt")]
    pub const Sqrt: Self = Self(7);
    #[doc(alias = "MLCArithmeticOperationRsqrt")]
    pub const Rsqrt: Self = Self(8);
    #[doc(alias = "MLCArithmeticOperationSin")]
    pub const Sin: Self = Self(9);
    #[doc(alias = "MLCArithmeticOperationCos")]
    pub const Cos: Self = Self(10);
    #[doc(alias = "MLCArithmeticOperationTan")]
    pub const Tan: Self = Self(11);
    #[doc(alias = "MLCArithmeticOperationAsin")]
    pub const Asin: Self = Self(12);
    #[doc(alias = "MLCArithmeticOperationAcos")]
    pub const Acos: Self = Self(13);
    #[doc(alias = "MLCArithmeticOperationAtan")]
    pub const Atan: Self = Self(14);
    #[doc(alias = "MLCArithmeticOperationSinh")]
    pub const Sinh: Self = Self(15);
    #[doc(alias = "MLCArithmeticOperationCosh")]
    pub const Cosh: Self = Self(16);
    #[doc(alias = "MLCArithmeticOperationTanh")]
    pub const Tanh: Self = Self(17);
    #[doc(alias = "MLCArithmeticOperationAsinh")]
    pub const Asinh: Self = Self(18);
    #[doc(alias = "MLCArithmeticOperationAcosh")]
    pub const Acosh: Self = Self(19);
    #[doc(alias = "MLCArithmeticOperationAtanh")]
    pub const Atanh: Self = Self(20);
    #[doc(alias = "MLCArithmeticOperationPow")]
    pub const Pow: Self = Self(21);
    #[doc(alias = "MLCArithmeticOperationExp")]
    pub const Exp: Self = Self(22);
    #[doc(alias = "MLCArithmeticOperationExp2")]
    pub const Exp2: Self = Self(23);
    #[doc(alias = "MLCArithmeticOperationLog")]
    pub const Log: Self = Self(24);
    #[doc(alias = "MLCArithmeticOperationLog2")]
    pub const Log2: Self = Self(25);
    #[doc(alias = "MLCArithmeticOperationMultiplyNoNaN")]
    pub const MultiplyNoNaN: Self = Self(26);
    #[doc(alias = "MLCArithmeticOperationDivideNoNaN")]
    pub const DivideNoNaN: Self = Self(27);
    #[doc(alias = "MLCArithmeticOperationMin")]
    pub const Min: Self = Self(28);
    #[doc(alias = "MLCArithmeticOperationMax")]
    pub const Max: Self = Self(29);
    #[doc(alias = "MLCArithmeticOperationCount")]
    pub const Count: Self = Self(30);
}

unsafe impl Encode for MLCArithmeticOperation {
    const ENCODING: Encoding = i32::ENCODING;
}

unsafe impl RefEncode for MLCArithmeticOperation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlclosstype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MLCLossType(pub i32);
impl MLCLossType {
    #[doc(alias = "MLCLossTypeMeanAbsoluteError")]
    pub const MeanAbsoluteError: Self = Self(0);
    #[doc(alias = "MLCLossTypeMeanSquaredError")]
    pub const MeanSquaredError: Self = Self(1);
    #[doc(alias = "MLCLossTypeSoftmaxCrossEntropy")]
    pub const SoftmaxCrossEntropy: Self = Self(2);
    #[doc(alias = "MLCLossTypeSigmoidCrossEntropy")]
    pub const SigmoidCrossEntropy: Self = Self(3);
    #[doc(alias = "MLCLossTypeCategoricalCrossEntropy")]
    pub const CategoricalCrossEntropy: Self = Self(4);
    #[doc(alias = "MLCLossTypeHinge")]
    pub const Hinge: Self = Self(5);
    #[doc(alias = "MLCLossTypeHuber")]
    pub const Huber: Self = Self(6);
    #[doc(alias = "MLCLossTypeCosineDistance")]
    pub const CosineDistance: Self = Self(7);
    #[doc(alias = "MLCLossTypeLog")]
    pub const Log: Self = Self(8);
    #[doc(alias = "MLCLossTypeCount")]
    pub const Count: Self = Self(9);
}

unsafe impl Encode for MLCLossType {
    const ENCODING: Encoding = i32::ENCODING;
}

unsafe impl RefEncode for MLCLossType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlcactivationtype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MLCActivationType(pub i32);
impl MLCActivationType {
    #[doc(alias = "MLCActivationTypeNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "MLCActivationTypeReLU")]
    pub const ReLU: Self = Self(1);
    #[doc(alias = "MLCActivationTypeLinear")]
    pub const Linear: Self = Self(2);
    #[doc(alias = "MLCActivationTypeSigmoid")]
    pub const Sigmoid: Self = Self(3);
    #[doc(alias = "MLCActivationTypeHardSigmoid")]
    pub const HardSigmoid: Self = Self(4);
    #[doc(alias = "MLCActivationTypeTanh")]
    pub const Tanh: Self = Self(5);
    #[doc(alias = "MLCActivationTypeAbsolute")]
    pub const Absolute: Self = Self(6);
    #[doc(alias = "MLCActivationTypeSoftPlus")]
    pub const SoftPlus: Self = Self(7);
    #[doc(alias = "MLCActivationTypeSoftSign")]
    pub const SoftSign: Self = Self(8);
    #[doc(alias = "MLCActivationTypeELU")]
    pub const ELU: Self = Self(9);
    #[doc(alias = "MLCActivationTypeReLUN")]
    pub const ReLUN: Self = Self(10);
    #[doc(alias = "MLCActivationTypeLogSigmoid")]
    pub const LogSigmoid: Self = Self(11);
    #[doc(alias = "MLCActivationTypeSELU")]
    pub const SELU: Self = Self(12);
    #[doc(alias = "MLCActivationTypeCELU")]
    pub const CELU: Self = Self(13);
    #[doc(alias = "MLCActivationTypeHardShrink")]
    pub const HardShrink: Self = Self(14);
    #[doc(alias = "MLCActivationTypeSoftShrink")]
    pub const SoftShrink: Self = Self(15);
    #[doc(alias = "MLCActivationTypeTanhShrink")]
    pub const TanhShrink: Self = Self(16);
    #[doc(alias = "MLCActivationTypeThreshold")]
    pub const Threshold: Self = Self(17);
    #[doc(alias = "MLCActivationTypeGELU")]
    pub const GELU: Self = Self(18);
    #[doc(alias = "MLCActivationTypeHardSwish")]
    pub const HardSwish: Self = Self(19);
    #[doc(alias = "MLCActivationTypeClamp")]
    pub const Clamp: Self = Self(20);
    #[doc(alias = "MLCActivationTypeCount")]
    pub const Count: Self = Self(21);
}

unsafe impl Encode for MLCActivationType {
    const ENCODING: Encoding = i32::ENCODING;
}

unsafe impl RefEncode for MLCActivationType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlcconvolutiontype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MLCConvolutionType(pub i32);
impl MLCConvolutionType {
    #[doc(alias = "MLCConvolutionTypeStandard")]
    pub const Standard: Self = Self(0);
    #[doc(alias = "MLCConvolutionTypeTransposed")]
    pub const Transposed: Self = Self(1);
    #[doc(alias = "MLCConvolutionTypeDepthwise")]
    pub const Depthwise: Self = Self(2);
}

unsafe impl Encode for MLCConvolutionType {
    const ENCODING: Encoding = i32::ENCODING;
}

unsafe impl RefEncode for MLCConvolutionType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlcpaddingpolicy?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MLCPaddingPolicy(pub i32);
impl MLCPaddingPolicy {
    #[doc(alias = "MLCPaddingPolicySame")]
    pub const Same: Self = Self(0);
    #[doc(alias = "MLCPaddingPolicyValid")]
    pub const Valid: Self = Self(1);
    #[doc(alias = "MLCPaddingPolicyUsePaddingSize")]
    pub const UsePaddingSize: Self = Self(2);
}

unsafe impl Encode for MLCPaddingPolicy {
    const ENCODING: Encoding = i32::ENCODING;
}

unsafe impl RefEncode for MLCPaddingPolicy {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlcpaddingtype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MLCPaddingType(pub i32);
impl MLCPaddingType {
    #[doc(alias = "MLCPaddingTypeZero")]
    pub const Zero: Self = Self(0);
    #[doc(alias = "MLCPaddingTypeReflect")]
    pub const Reflect: Self = Self(1);
    #[doc(alias = "MLCPaddingTypeSymmetric")]
    pub const Symmetric: Self = Self(2);
    #[doc(alias = "MLCPaddingTypeConstant")]
    pub const Constant: Self = Self(3);
}

unsafe impl Encode for MLCPaddingType {
    const ENCODING: Encoding = i32::ENCODING;
}

unsafe impl RefEncode for MLCPaddingType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlcpoolingtype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MLCPoolingType(pub i32);
impl MLCPoolingType {
    #[doc(alias = "MLCPoolingTypeMax")]
    pub const Max: Self = Self(1);
    #[doc(alias = "MLCPoolingTypeAverage")]
    pub const Average: Self = Self(2);
    #[doc(alias = "MLCPoolingTypeL2Norm")]
    pub const L2Norm: Self = Self(3);
    #[doc(alias = "MLCPoolingTypeCount")]
    pub const Count: Self = Self(4);
}

unsafe impl Encode for MLCPoolingType {
    const ENCODING: Encoding = i32::ENCODING;
}

unsafe impl RefEncode for MLCPoolingType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlcreductiontype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MLCReductionType(pub i32);
impl MLCReductionType {
    #[doc(alias = "MLCReductionTypeNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "MLCReductionTypeSum")]
    pub const Sum: Self = Self(1);
    #[doc(alias = "MLCReductionTypeMean")]
    pub const Mean: Self = Self(2);
    #[doc(alias = "MLCReductionTypeMax")]
    pub const Max: Self = Self(3);
    #[doc(alias = "MLCReductionTypeMin")]
    pub const Min: Self = Self(4);
    #[doc(alias = "MLCReductionTypeArgMax")]
    pub const ArgMax: Self = Self(5);
    #[doc(alias = "MLCReductionTypeArgMin")]
    pub const ArgMin: Self = Self(6);
    #[doc(alias = "MLCReductionTypeL1Norm")]
    pub const L1Norm: Self = Self(7);
    #[doc(alias = "MLCReductionTypeAny")]
    pub const Any: Self = Self(8);
    #[doc(alias = "MLCReductionTypeAll")]
    pub const All: Self = Self(9);
    #[doc(alias = "MLCReductionTypeCount")]
    pub const Count: Self = Self(10);
}

unsafe impl Encode for MLCReductionType {
    const ENCODING: Encoding = i32::ENCODING;
}

unsafe impl RefEncode for MLCReductionType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlcregularizationtype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MLCRegularizationType(pub i32);
impl MLCRegularizationType {
    #[doc(alias = "MLCRegularizationTypeNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "MLCRegularizationTypeL1")]
    pub const L1: Self = Self(1);
    #[doc(alias = "MLCRegularizationTypeL2")]
    pub const L2: Self = Self(2);
}

unsafe impl Encode for MLCRegularizationType {
    const ENCODING: Encoding = i32::ENCODING;
}

unsafe impl RefEncode for MLCRegularizationType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlcsamplemode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MLCSampleMode(pub i32);
impl MLCSampleMode {
    #[doc(alias = "MLCSampleModeNearest")]
    pub const Nearest: Self = Self(0);
    #[doc(alias = "MLCSampleModeLinear")]
    pub const Linear: Self = Self(1);
}

unsafe impl Encode for MLCSampleMode {
    const ENCODING: Encoding = i32::ENCODING;
}

unsafe impl RefEncode for MLCSampleMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlcsoftmaxoperation?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MLCSoftmaxOperation(pub i32);
impl MLCSoftmaxOperation {
    #[doc(alias = "MLCSoftmaxOperationSoftmax")]
    pub const Softmax: Self = Self(0);
    #[doc(alias = "MLCSoftmaxOperationLogSoftmax")]
    pub const LogSoftmax: Self = Self(1);
}

unsafe impl Encode for MLCSoftmaxOperation {
    const ENCODING: Encoding = i32::ENCODING;
}

unsafe impl RefEncode for MLCSoftmaxOperation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlclstmresultmode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MLCLSTMResultMode(pub u64);
impl MLCLSTMResultMode {
    #[doc(alias = "MLCLSTMResultModeOutput")]
    pub const Output: Self = Self(0);
    #[doc(alias = "MLCLSTMResultModeOutputAndStates")]
    pub const OutputAndStates: Self = Self(1);
}

unsafe impl Encode for MLCLSTMResultMode {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MLCLSTMResultMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlccomparisonoperation?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MLCComparisonOperation(pub i32);
impl MLCComparisonOperation {
    #[doc(alias = "MLCComparisonOperationEqual")]
    pub const Equal: Self = Self(0);
    #[doc(alias = "MLCComparisonOperationNotEqual")]
    pub const NotEqual: Self = Self(1);
    #[doc(alias = "MLCComparisonOperationLess")]
    pub const Less: Self = Self(2);
    #[doc(alias = "MLCComparisonOperationGreater")]
    pub const Greater: Self = Self(3);
    #[doc(alias = "MLCComparisonOperationLessOrEqual")]
    pub const LessOrEqual: Self = Self(4);
    #[doc(alias = "MLCComparisonOperationGreaterOrEqual")]
    pub const GreaterOrEqual: Self = Self(5);
    #[doc(alias = "MLCComparisonOperationLogicalAND")]
    pub const LogicalAND: Self = Self(6);
    #[doc(alias = "MLCComparisonOperationLogicalOR")]
    pub const LogicalOR: Self = Self(7);
    #[doc(alias = "MLCComparisonOperationLogicalNOT")]
    pub const LogicalNOT: Self = Self(8);
    #[doc(alias = "MLCComparisonOperationLogicalNAND")]
    pub const LogicalNAND: Self = Self(9);
    #[doc(alias = "MLCComparisonOperationLogicalNOR")]
    pub const LogicalNOR: Self = Self(10);
    #[doc(alias = "MLCComparisonOperationLogicalXOR")]
    pub const LogicalXOR: Self = Self(11);
    #[doc(alias = "MLCComparisonOperationCount")]
    pub const Count: Self = Self(12);
}

unsafe impl Encode for MLCComparisonOperation {
    const ENCODING: Encoding = i32::ENCODING;
}

unsafe impl RefEncode for MLCComparisonOperation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlcgradientclippingtype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MLCGradientClippingType(pub i32);
impl MLCGradientClippingType {
    #[doc(alias = "MLCGradientClippingTypeByValue")]
    pub const ByValue: Self = Self(0);
    #[doc(alias = "MLCGradientClippingTypeByNorm")]
    pub const ByNorm: Self = Self(1);
    #[doc(alias = "MLCGradientClippingTypeByGlobalNorm")]
    pub const ByGlobalNorm: Self = Self(2);
}

unsafe impl Encode for MLCGradientClippingType {
    const ENCODING: Encoding = i32::ENCODING;
}

unsafe impl RefEncode for MLCGradientClippingType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    pub fn MLCActivationTypeDebugDescription(
        activation_type: MLCActivationType,
    ) -> NonNull<NSString>;
}

extern "C-unwind" {
    pub fn MLCArithmeticOperationDebugDescription(
        operation: MLCArithmeticOperation,
    ) -> NonNull<NSString>;
}

extern "C-unwind" {
    pub fn MLCPaddingPolicyDebugDescription(padding_policy: MLCPaddingPolicy) -> NonNull<NSString>;
}

extern "C-unwind" {
    pub fn MLCLossTypeDebugDescription(loss_type: MLCLossType) -> NonNull<NSString>;
}

extern "C-unwind" {
    pub fn MLCReductionTypeDebugDescription(reduction_type: MLCReductionType) -> NonNull<NSString>;
}

extern "C-unwind" {
    pub fn MLCPaddingTypeDebugDescription(padding_type: MLCPaddingType) -> NonNull<NSString>;
}

extern "C-unwind" {
    pub fn MLCConvolutionTypeDebugDescription(
        convolution_type: MLCConvolutionType,
    ) -> NonNull<NSString>;
}

extern "C-unwind" {
    pub fn MLCPoolingTypeDebugDescription(pooling_type: MLCPoolingType) -> NonNull<NSString>;
}

extern "C-unwind" {
    pub fn MLCSoftmaxOperationDebugDescription(operation: MLCSoftmaxOperation)
        -> NonNull<NSString>;
}

extern "C-unwind" {
    pub fn MLCSampleModeDebugDescription(mode: MLCSampleMode) -> NonNull<NSString>;
}

extern "C-unwind" {
    pub fn MLCLSTMResultModeDebugDescription(mode: MLCLSTMResultMode) -> NonNull<NSString>;
}

extern "C-unwind" {
    pub fn MLCComparisonOperationDebugDescription(
        operation: MLCComparisonOperation,
    ) -> NonNull<NSString>;
}

extern "C-unwind" {
    pub fn MLCGradientClippingTypeDebugDescription(
        gradient_clipping_type: MLCGradientClippingType,
    ) -> NonNull<NSString>;
}
