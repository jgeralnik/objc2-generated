//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/speech/sftranscriptionsegment?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SFTranscriptionSegment;
);

unsafe impl NSCoding for SFTranscriptionSegment {}

unsafe impl NSCopying for SFTranscriptionSegment {}

unsafe impl CopyingHelper for SFTranscriptionSegment {
    type Result = Self;
}

unsafe impl NSObjectProtocol for SFTranscriptionSegment {}

unsafe impl NSSecureCoding for SFTranscriptionSegment {}

extern_methods!(
    unsafe impl SFTranscriptionSegment {
        #[method_id(@__retain_semantics Other substring)]
        pub unsafe fn substring(&self) -> Retained<NSString>;

        #[method(substringRange)]
        pub unsafe fn substringRange(&self) -> NSRange;

        #[method(timestamp)]
        pub unsafe fn timestamp(&self) -> NSTimeInterval;

        #[method(duration)]
        pub unsafe fn duration(&self) -> NSTimeInterval;

        #[method(confidence)]
        pub unsafe fn confidence(&self) -> c_float;

        #[method_id(@__retain_semantics Other alternativeSubstrings)]
        pub unsafe fn alternativeSubstrings(&self) -> Retained<NSArray<NSString>>;

        #[cfg(feature = "SFVoiceAnalytics")]
        #[deprecated = "voiceAnalytics is moved to SFSpeechRecognitionMetadata"]
        #[method_id(@__retain_semantics Other voiceAnalytics)]
        pub unsafe fn voiceAnalytics(&self) -> Option<Retained<SFVoiceAnalytics>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SFTranscriptionSegment {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
