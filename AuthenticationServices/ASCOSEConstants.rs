//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

typed_extensible_enum!(
    pub type ASCOSEAlgorithmIdentifier = NSInteger;
);

extern_static!(ASCOSEAlgorithmIdentifierES256: ASCOSEAlgorithmIdentifier = -7);

typed_extensible_enum!(
    pub type ASCOSEEllipticCurveIdentifier = NSInteger;
);

extern_static!(ASCOSEEllipticCurveIdentifierP256: ASCOSEEllipticCurveIdentifier = 1);
