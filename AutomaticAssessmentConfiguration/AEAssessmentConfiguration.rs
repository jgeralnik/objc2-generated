//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/automaticassessmentconfiguration/aeautocorrectmode?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AEAutocorrectMode(pub NSUInteger);
bitflags::bitflags! {
    impl AEAutocorrectMode: NSUInteger {
        #[doc(alias = "AEAutocorrectModeNone")]
        const None = 0;
        #[doc(alias = "AEAutocorrectModeSpelling")]
        const Spelling = 1<<0;
        #[doc(alias = "AEAutocorrectModePunctuation")]
        const Punctuation = 1<<1;
    }
}

unsafe impl Encode for AEAutocorrectMode {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for AEAutocorrectMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/automaticassessmentconfiguration/aeassessmentconfiguration?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AEAssessmentConfiguration;
);

unsafe impl NSCopying for AEAssessmentConfiguration {}

unsafe impl CopyingHelper for AEAssessmentConfiguration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for AEAssessmentConfiguration {}

extern_methods!(
    unsafe impl AEAssessmentConfiguration {
        #[method(autocorrectMode)]
        pub unsafe fn autocorrectMode(&self) -> AEAutocorrectMode;

        #[method(setAutocorrectMode:)]
        pub unsafe fn setAutocorrectMode(&self, autocorrect_mode: AEAutocorrectMode);

        #[method(allowsSpellCheck)]
        pub unsafe fn allowsSpellCheck(&self) -> bool;

        #[method(setAllowsSpellCheck:)]
        pub unsafe fn setAllowsSpellCheck(&self, allows_spell_check: bool);

        #[method(allowsPredictiveKeyboard)]
        pub unsafe fn allowsPredictiveKeyboard(&self) -> bool;

        #[method(setAllowsPredictiveKeyboard:)]
        pub unsafe fn setAllowsPredictiveKeyboard(&self, allows_predictive_keyboard: bool);

        #[method(allowsKeyboardShortcuts)]
        pub unsafe fn allowsKeyboardShortcuts(&self) -> bool;

        #[method(setAllowsKeyboardShortcuts:)]
        pub unsafe fn setAllowsKeyboardShortcuts(&self, allows_keyboard_shortcuts: bool);

        #[method(allowsActivityContinuation)]
        pub unsafe fn allowsActivityContinuation(&self) -> bool;

        #[method(setAllowsActivityContinuation:)]
        pub unsafe fn setAllowsActivityContinuation(&self, allows_activity_continuation: bool);

        #[method(allowsDictation)]
        pub unsafe fn allowsDictation(&self) -> bool;

        #[method(setAllowsDictation:)]
        pub unsafe fn setAllowsDictation(&self, allows_dictation: bool);

        #[method(allowsAccessibilitySpeech)]
        pub unsafe fn allowsAccessibilitySpeech(&self) -> bool;

        #[method(setAllowsAccessibilitySpeech:)]
        pub unsafe fn setAllowsAccessibilitySpeech(&self, allows_accessibility_speech: bool);

        #[method(allowsPasswordAutoFill)]
        pub unsafe fn allowsPasswordAutoFill(&self) -> bool;

        #[method(setAllowsPasswordAutoFill:)]
        pub unsafe fn setAllowsPasswordAutoFill(&self, allows_password_auto_fill: bool);

        #[method(allowsContinuousPathKeyboard)]
        pub unsafe fn allowsContinuousPathKeyboard(&self) -> bool;

        #[method(setAllowsContinuousPathKeyboard:)]
        pub unsafe fn setAllowsContinuousPathKeyboard(&self, allows_continuous_path_keyboard: bool);

        #[cfg(feature = "AEAssessmentParticipantConfiguration")]
        #[method_id(@__retain_semantics Other mainParticipantConfiguration)]
        pub unsafe fn mainParticipantConfiguration(
            &self,
        ) -> Retained<AEAssessmentParticipantConfiguration>;

        #[cfg(all(
            feature = "AEAssessmentApplication",
            feature = "AEAssessmentParticipantConfiguration"
        ))]
        #[method_id(@__retain_semantics Other configurationsByApplication)]
        pub unsafe fn configurationsByApplication(
            &self,
        ) -> Retained<NSDictionary<AEAssessmentApplication, AEAssessmentParticipantConfiguration>>;

        #[cfg(all(
            feature = "AEAssessmentApplication",
            feature = "AEAssessmentParticipantConfiguration"
        ))]
        #[method(setConfiguration:forApplication:)]
        pub unsafe fn setConfiguration_forApplication(
            &self,
            configuration: &AEAssessmentParticipantConfiguration,
            application: &AEAssessmentApplication,
        );

        #[cfg(feature = "AEAssessmentApplication")]
        #[method(removeApplication:)]
        pub unsafe fn removeApplication(&self, application: &AEAssessmentApplication);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AEAssessmentConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
