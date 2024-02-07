// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

//! # Bindings to the `LocalAuthentication` framework
#![allow(unused_imports)]
#![allow(deprecated)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(missing_docs)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::identity_op)]
#![allow(clippy::missing_safety_doc)]

#[link(name = "LocalAuthentication", kind = "framework")]
extern "C" {}

#[path = "LABase.rs"]
mod __LABase;
#[path = "LAContext.rs"]
mod __LAContext;
#[path = "LAError.rs"]
mod __LAError;
#[path = "LAPersistedRight.rs"]
mod __LAPersistedRight;
#[path = "LAPrivateKey.rs"]
mod __LAPrivateKey;
#[path = "LAPublicDefines.rs"]
mod __LAPublicDefines;
#[path = "LAPublicKey.rs"]
mod __LAPublicKey;
#[path = "LARequirement.rs"]
mod __LARequirement;
#[path = "LARight.rs"]
mod __LARight;
#[path = "LARightStore.rs"]
mod __LARightStore;
#[path = "LASecret.rs"]
mod __LASecret;

pub use self::__LAContext::LAAccessControlOperation;
pub use self::__LAContext::LABiometryType;
#[cfg(feature = "LocalAuthentication_LAContext")]
pub use self::__LAContext::LAContext;
pub use self::__LAContext::LAPolicy;
pub use self::__LAContext::LATouchIDAuthenticationMaximumAllowableReuseDuration;
pub use self::__LAError::LAErrorDomain;
#[cfg(feature = "LocalAuthentication_LAPersistedRight")]
pub use self::__LAPersistedRight::LAPersistedRight;
#[cfg(feature = "LocalAuthentication_LAPrivateKey")]
pub use self::__LAPrivateKey::LAPrivateKey;
#[cfg(feature = "LocalAuthentication_LAPublicKey")]
pub use self::__LAPublicKey::LAPublicKey;
#[cfg(feature = "LocalAuthentication_LAAuthenticationRequirement")]
pub use self::__LARequirement::LAAuthenticationRequirement;
#[cfg(feature = "LocalAuthentication_LABiometryFallbackRequirement")]
pub use self::__LARequirement::LABiometryFallbackRequirement;
#[cfg(feature = "LocalAuthentication_LARight")]
pub use self::__LARight::LARight;
pub use self::__LARight::LARightState;
#[cfg(feature = "LocalAuthentication_LARightStore")]
pub use self::__LARightStore::LARightStore;
#[cfg(feature = "LocalAuthentication_LASecret")]
pub use self::__LASecret::LASecret;
