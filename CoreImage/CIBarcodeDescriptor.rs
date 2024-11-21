//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CIBarcodeDescriptor;
);

unsafe impl NSCoding for CIBarcodeDescriptor {}

unsafe impl NSCopying for CIBarcodeDescriptor {}

unsafe impl CopyingHelper for CIBarcodeDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CIBarcodeDescriptor {}

unsafe impl NSSecureCoding for CIBarcodeDescriptor {}

extern_methods!(
    unsafe impl CIBarcodeDescriptor {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CIBarcodeDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CIQRCodeErrorCorrectionLevel(pub NSInteger);
impl CIQRCodeErrorCorrectionLevel {
    #[doc(alias = "CIQRCodeErrorCorrectionLevelL")]
    pub const L: Self = Self(b'L' as _);
    #[doc(alias = "CIQRCodeErrorCorrectionLevelM")]
    pub const M: Self = Self(b'M' as _);
    #[doc(alias = "CIQRCodeErrorCorrectionLevelQ")]
    pub const Q: Self = Self(b'Q' as _);
    #[doc(alias = "CIQRCodeErrorCorrectionLevelH")]
    pub const H: Self = Self(b'H' as _);
}

unsafe impl Encode for CIQRCodeErrorCorrectionLevel {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CIQRCodeErrorCorrectionLevel {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[unsafe(super(CIBarcodeDescriptor, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CIQRCodeDescriptor;
);

unsafe impl NSCoding for CIQRCodeDescriptor {}

unsafe impl NSCopying for CIQRCodeDescriptor {}

unsafe impl CopyingHelper for CIQRCodeDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CIQRCodeDescriptor {}

unsafe impl NSSecureCoding for CIQRCodeDescriptor {}

extern_methods!(
    unsafe impl CIQRCodeDescriptor {
        #[method_id(@__retain_semantics Other errorCorrectedPayload)]
        pub unsafe fn errorCorrectedPayload(&self) -> Retained<NSData>;

        #[method(symbolVersion)]
        pub unsafe fn symbolVersion(&self) -> NSInteger;

        #[method(maskPattern)]
        pub unsafe fn maskPattern(&self) -> u8;

        #[method(errorCorrectionLevel)]
        pub unsafe fn errorCorrectionLevel(&self) -> CIQRCodeErrorCorrectionLevel;

        #[method_id(@__retain_semantics Init initWithPayload:symbolVersion:maskPattern:errorCorrectionLevel:)]
        pub unsafe fn initWithPayload_symbolVersion_maskPattern_errorCorrectionLevel(
            this: Allocated<Self>,
            error_corrected_payload: &NSData,
            symbol_version: NSInteger,
            mask_pattern: u8,
            error_correction_level: CIQRCodeErrorCorrectionLevel,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other descriptorWithPayload:symbolVersion:maskPattern:errorCorrectionLevel:)]
        pub unsafe fn descriptorWithPayload_symbolVersion_maskPattern_errorCorrectionLevel(
            error_corrected_payload: &NSData,
            symbol_version: NSInteger,
            mask_pattern: u8,
            error_correction_level: CIQRCodeErrorCorrectionLevel,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CIQRCodeDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[unsafe(super(CIBarcodeDescriptor, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CIAztecCodeDescriptor;
);

unsafe impl NSCoding for CIAztecCodeDescriptor {}

unsafe impl NSCopying for CIAztecCodeDescriptor {}

unsafe impl CopyingHelper for CIAztecCodeDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CIAztecCodeDescriptor {}

unsafe impl NSSecureCoding for CIAztecCodeDescriptor {}

extern_methods!(
    unsafe impl CIAztecCodeDescriptor {
        #[method_id(@__retain_semantics Other errorCorrectedPayload)]
        pub unsafe fn errorCorrectedPayload(&self) -> Retained<NSData>;

        #[method(isCompact)]
        pub unsafe fn isCompact(&self) -> bool;

        #[method(layerCount)]
        pub unsafe fn layerCount(&self) -> NSInteger;

        #[method(dataCodewordCount)]
        pub unsafe fn dataCodewordCount(&self) -> NSInteger;

        #[method_id(@__retain_semantics Init initWithPayload:isCompact:layerCount:dataCodewordCount:)]
        pub unsafe fn initWithPayload_isCompact_layerCount_dataCodewordCount(
            this: Allocated<Self>,
            error_corrected_payload: &NSData,
            is_compact: bool,
            layer_count: NSInteger,
            data_codeword_count: NSInteger,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other descriptorWithPayload:isCompact:layerCount:dataCodewordCount:)]
        pub unsafe fn descriptorWithPayload_isCompact_layerCount_dataCodewordCount(
            error_corrected_payload: &NSData,
            is_compact: bool,
            layer_count: NSInteger,
            data_codeword_count: NSInteger,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CIAztecCodeDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[unsafe(super(CIBarcodeDescriptor, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CIPDF417CodeDescriptor;
);

unsafe impl NSCoding for CIPDF417CodeDescriptor {}

unsafe impl NSCopying for CIPDF417CodeDescriptor {}

unsafe impl CopyingHelper for CIPDF417CodeDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CIPDF417CodeDescriptor {}

unsafe impl NSSecureCoding for CIPDF417CodeDescriptor {}

extern_methods!(
    unsafe impl CIPDF417CodeDescriptor {
        #[method_id(@__retain_semantics Other errorCorrectedPayload)]
        pub unsafe fn errorCorrectedPayload(&self) -> Retained<NSData>;

        #[method(isCompact)]
        pub unsafe fn isCompact(&self) -> bool;

        #[method(rowCount)]
        pub unsafe fn rowCount(&self) -> NSInteger;

        #[method(columnCount)]
        pub unsafe fn columnCount(&self) -> NSInteger;

        #[method_id(@__retain_semantics Init initWithPayload:isCompact:rowCount:columnCount:)]
        pub unsafe fn initWithPayload_isCompact_rowCount_columnCount(
            this: Allocated<Self>,
            error_corrected_payload: &NSData,
            is_compact: bool,
            row_count: NSInteger,
            column_count: NSInteger,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other descriptorWithPayload:isCompact:rowCount:columnCount:)]
        pub unsafe fn descriptorWithPayload_isCompact_rowCount_columnCount(
            error_corrected_payload: &NSData,
            is_compact: bool,
            row_count: NSInteger,
            column_count: NSInteger,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CIPDF417CodeDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CIDataMatrixCodeECCVersion(pub NSInteger);
impl CIDataMatrixCodeECCVersion {
    pub const CIDataMatrixCodeECCVersion000: Self = Self(0);
    pub const CIDataMatrixCodeECCVersion050: Self = Self(50);
    pub const CIDataMatrixCodeECCVersion080: Self = Self(80);
    pub const CIDataMatrixCodeECCVersion100: Self = Self(100);
    pub const CIDataMatrixCodeECCVersion140: Self = Self(140);
    pub const CIDataMatrixCodeECCVersion200: Self = Self(200);
}

unsafe impl Encode for CIDataMatrixCodeECCVersion {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CIDataMatrixCodeECCVersion {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[unsafe(super(CIBarcodeDescriptor, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CIDataMatrixCodeDescriptor;
);

unsafe impl NSCoding for CIDataMatrixCodeDescriptor {}

unsafe impl NSCopying for CIDataMatrixCodeDescriptor {}

unsafe impl CopyingHelper for CIDataMatrixCodeDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CIDataMatrixCodeDescriptor {}

unsafe impl NSSecureCoding for CIDataMatrixCodeDescriptor {}

extern_methods!(
    unsafe impl CIDataMatrixCodeDescriptor {
        #[method_id(@__retain_semantics Other errorCorrectedPayload)]
        pub unsafe fn errorCorrectedPayload(&self) -> Retained<NSData>;

        #[method(rowCount)]
        pub unsafe fn rowCount(&self) -> NSInteger;

        #[method(columnCount)]
        pub unsafe fn columnCount(&self) -> NSInteger;

        #[method(eccVersion)]
        pub unsafe fn eccVersion(&self) -> CIDataMatrixCodeECCVersion;

        #[method_id(@__retain_semantics Init initWithPayload:rowCount:columnCount:eccVersion:)]
        pub unsafe fn initWithPayload_rowCount_columnCount_eccVersion(
            this: Allocated<Self>,
            error_corrected_payload: &NSData,
            row_count: NSInteger,
            column_count: NSInteger,
            ecc_version: CIDataMatrixCodeECCVersion,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other descriptorWithPayload:rowCount:columnCount:eccVersion:)]
        pub unsafe fn descriptorWithPayload_rowCount_columnCount_eccVersion(
            error_corrected_payload: &NSData,
            row_count: NSInteger,
            column_count: NSInteger,
            ecc_version: CIDataMatrixCodeECCVersion,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CIDataMatrixCodeDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_category!(
    /// Category "CIBarcodeDescriptor" on [`NSUserActivity`].
    #[doc(alias = "CIBarcodeDescriptor")]
    pub unsafe trait NSUserActivityCIBarcodeDescriptor {
        #[method_id(@__retain_semantics Other detectedBarcodeDescriptor)]
        unsafe fn detectedBarcodeDescriptor(&self) -> Option<Retained<CIBarcodeDescriptor>>;
    }

    unsafe impl NSUserActivityCIBarcodeDescriptor for NSUserActivity {}
);
