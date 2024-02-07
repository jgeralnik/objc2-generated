// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

//! # Bindings to the `OSAKit` framework
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

#[link(name = "OSAKit", kind = "framework")]
extern "C" {}

#[path = "OSALanguage.rs"]
mod __OSALanguage;
#[path = "OSALanguageInstance.rs"]
mod __OSALanguageInstance;
#[path = "OSAScript.rs"]
mod __OSAScript;
#[path = "OSAScriptController.rs"]
mod __OSAScriptController;
#[path = "OSAScriptView.rs"]
mod __OSAScriptView;

#[cfg(feature = "OSAKit_OSALanguage")]
pub use self::__OSALanguage::OSALanguage;
pub use self::__OSALanguage::OSALanguageFeatures;
#[cfg(feature = "OSAKit_OSALanguageInstance")]
pub use self::__OSALanguageInstance::OSALanguageInstance;
#[cfg(feature = "OSAKit_OSAScript")]
pub use self::__OSAScript::OSAScript;
pub use self::__OSAScript::OSAScriptErrorAppAddressKey;
pub use self::__OSAScript::OSAScriptErrorAppName;
pub use self::__OSAScript::OSAScriptErrorAppNameKey;
pub use self::__OSAScript::OSAScriptErrorBriefMessage;
pub use self::__OSAScript::OSAScriptErrorBriefMessageKey;
pub use self::__OSAScript::OSAScriptErrorExpectedTypeKey;
pub use self::__OSAScript::OSAScriptErrorMessage;
pub use self::__OSAScript::OSAScriptErrorMessageKey;
pub use self::__OSAScript::OSAScriptErrorNumber;
pub use self::__OSAScript::OSAScriptErrorNumberKey;
pub use self::__OSAScript::OSAScriptErrorOffendingObjectKey;
pub use self::__OSAScript::OSAScriptErrorPartialResultKey;
pub use self::__OSAScript::OSAScriptErrorRange;
pub use self::__OSAScript::OSAScriptErrorRangeKey;
pub use self::__OSAScript::OSAStorageApplicationBundleType;
pub use self::__OSAScript::OSAStorageApplicationType;
pub use self::__OSAScript::OSAStorageOptions;
pub use self::__OSAScript::OSAStorageScriptBundleType;
pub use self::__OSAScript::OSAStorageScriptType;
pub use self::__OSAScript::OSAStorageTextType;
#[cfg(feature = "OSAKit_OSAScriptController")]
pub use self::__OSAScriptController::OSAScriptController;
pub use self::__OSAScriptController::OSAScriptState;
#[cfg(feature = "OSAKit_OSAScriptView")]
pub use self::__OSAScriptView::OSAScriptView;
