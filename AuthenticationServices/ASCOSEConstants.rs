//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

typed_extensible_enum!(
    pub type ASCOSEAlgorithmIdentifier = NSInteger;
);

pub static ASCOSEAlgorithmIdentifierES256: ASCOSEAlgorithmIdentifier = -7;

typed_extensible_enum!(
    pub type ASCOSEEllipticCurveIdentifier = NSInteger;
);

pub static ASCOSEEllipticCurveIdentifierP256: ASCOSEEllipticCurveIdentifier = 1;
