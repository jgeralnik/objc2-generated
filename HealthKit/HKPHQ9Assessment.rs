//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkphq9assessmentrisk?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HKPHQ9AssessmentRisk(pub NSInteger);
impl HKPHQ9AssessmentRisk {
    #[doc(alias = "HKPHQ9AssessmentRiskNoneToMinimal")]
    pub const NoneToMinimal: Self = Self(1);
    #[doc(alias = "HKPHQ9AssessmentRiskMild")]
    pub const Mild: Self = Self(2);
    #[doc(alias = "HKPHQ9AssessmentRiskModerate")]
    pub const Moderate: Self = Self(3);
    #[doc(alias = "HKPHQ9AssessmentRiskModeratelySevere")]
    pub const ModeratelySevere: Self = Self(4);
    #[doc(alias = "HKPHQ9AssessmentRiskSevere")]
    pub const Severe: Self = Self(5);
}

unsafe impl Encode for HKPHQ9AssessmentRisk {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for HKPHQ9AssessmentRisk {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkphq9assessmentanswer?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HKPHQ9AssessmentAnswer(pub NSInteger);
impl HKPHQ9AssessmentAnswer {
    #[doc(alias = "HKPHQ9AssessmentAnswerNotAtAll")]
    pub const NotAtAll: Self = Self(0);
    #[doc(alias = "HKPHQ9AssessmentAnswerSeveralDays")]
    pub const SeveralDays: Self = Self(1);
    #[doc(alias = "HKPHQ9AssessmentAnswerMoreThanHalfTheDays")]
    pub const MoreThanHalfTheDays: Self = Self(2);
    #[doc(alias = "HKPHQ9AssessmentAnswerNearlyEveryDay")]
    pub const NearlyEveryDay: Self = Self(3);
    #[doc(alias = "HKPHQ9AssessmentAnswerPreferNotToAnswer")]
    pub const PreferNotToAnswer: Self = Self(4);
}

unsafe impl Encode for HKPHQ9AssessmentAnswer {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for HKPHQ9AssessmentAnswer {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    pub fn HKMinimumScoreForPHQ9AssessmentRisk(risk: HKPHQ9AssessmentRisk) -> NSInteger;
}

extern "C-unwind" {
    pub fn HKMaximumScoreForPHQ9AssessmentRisk(risk: HKPHQ9AssessmentRisk) -> NSInteger;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkphq9assessment?language=objc)
    #[unsafe(super(HKScoredAssessment, HKSample, HKObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "HKObject",
        feature = "HKSample",
        feature = "HKScoredAssessment"
    ))]
    pub struct HKPHQ9Assessment;
);

#[cfg(all(
    feature = "HKObject",
    feature = "HKSample",
    feature = "HKScoredAssessment"
))]
unsafe impl Send for HKPHQ9Assessment {}

#[cfg(all(
    feature = "HKObject",
    feature = "HKSample",
    feature = "HKScoredAssessment"
))]
unsafe impl Sync for HKPHQ9Assessment {}

#[cfg(all(
    feature = "HKObject",
    feature = "HKSample",
    feature = "HKScoredAssessment"
))]
unsafe impl NSCoding for HKPHQ9Assessment {}

#[cfg(all(
    feature = "HKObject",
    feature = "HKSample",
    feature = "HKScoredAssessment"
))]
unsafe impl NSCopying for HKPHQ9Assessment {}

#[cfg(all(
    feature = "HKObject",
    feature = "HKSample",
    feature = "HKScoredAssessment"
))]
unsafe impl CopyingHelper for HKPHQ9Assessment {
    type Result = Self;
}

#[cfg(all(
    feature = "HKObject",
    feature = "HKSample",
    feature = "HKScoredAssessment"
))]
unsafe impl NSObjectProtocol for HKPHQ9Assessment {}

#[cfg(all(
    feature = "HKObject",
    feature = "HKSample",
    feature = "HKScoredAssessment"
))]
unsafe impl NSSecureCoding for HKPHQ9Assessment {}

extern_methods!(
    #[cfg(all(
        feature = "HKObject",
        feature = "HKSample",
        feature = "HKScoredAssessment"
    ))]
    unsafe impl HKPHQ9Assessment {
        #[method_id(@__retain_semantics Other answers)]
        pub unsafe fn answers(&self) -> Retained<NSArray<NSNumber>>;

        #[method(risk)]
        pub unsafe fn risk(&self) -> HKPHQ9AssessmentRisk;

        #[method_id(@__retain_semantics Other assessmentWithDate:answers:)]
        pub unsafe fn assessmentWithDate_answers(
            date: &NSDate,
            answers: &NSArray<NSNumber>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other assessmentWithDate:answers:metadata:)]
        pub unsafe fn assessmentWithDate_answers_metadata(
            date: &NSDate,
            answers: &NSArray<NSNumber>,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
