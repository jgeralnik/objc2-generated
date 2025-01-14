//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/networkextension/nefilterprovider?language=objc)
    #[unsafe(super(NEProvider, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NEProvider")]
    pub struct NEFilterProvider;
);

#[cfg(feature = "NEProvider")]
unsafe impl NSObjectProtocol for NEFilterProvider {}

extern_methods!(
    #[cfg(feature = "NEProvider")]
    unsafe impl NEFilterProvider {
        #[cfg(feature = "block2")]
        #[method(startFilterWithCompletionHandler:)]
        pub unsafe fn startFilterWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "block2")]
        #[method(stopFilterWithReason:completionHandler:)]
        pub unsafe fn stopFilterWithReason_completionHandler(
            &self,
            reason: NEProviderStopReason,
            completion_handler: &block2::Block<dyn Fn()>,
        );

        #[cfg(feature = "NEFilterProviderConfiguration")]
        #[method_id(@__retain_semantics Other filterConfiguration)]
        pub unsafe fn filterConfiguration(&self) -> Retained<NEFilterProviderConfiguration>;

        #[method(handleReport:)]
        pub unsafe fn handleReport(&self, report: &NEFilterReport);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NEProvider")]
    unsafe impl NEFilterProvider {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/networkextension/nefilterreportfrequency?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NEFilterReportFrequency(pub NSInteger);
impl NEFilterReportFrequency {
    #[doc(alias = "NEFilterReportFrequencyNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "NEFilterReportFrequencyLow")]
    pub const Low: Self = Self(1);
    #[doc(alias = "NEFilterReportFrequencyMedium")]
    pub const Medium: Self = Self(2);
    #[doc(alias = "NEFilterReportFrequencyHigh")]
    pub const High: Self = Self(3);
}

unsafe impl Encode for NEFilterReportFrequency {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NEFilterReportFrequency {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/networkextension/nefilterverdict?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NEFilterVerdict;
);

unsafe impl NSCoding for NEFilterVerdict {}

unsafe impl NSCopying for NEFilterVerdict {}

unsafe impl CopyingHelper for NEFilterVerdict {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NEFilterVerdict {}

unsafe impl NSSecureCoding for NEFilterVerdict {}

extern_methods!(
    unsafe impl NEFilterVerdict {
        #[method(shouldReport)]
        pub unsafe fn shouldReport(&self) -> bool;

        #[method(setShouldReport:)]
        pub unsafe fn setShouldReport(&self, should_report: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NEFilterVerdict {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/networkextension/nefilternewflowverdict?language=objc)
    #[unsafe(super(NEFilterVerdict, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NEFilterNewFlowVerdict;
);

unsafe impl NSCoding for NEFilterNewFlowVerdict {}

unsafe impl NSCopying for NEFilterNewFlowVerdict {}

unsafe impl CopyingHelper for NEFilterNewFlowVerdict {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NEFilterNewFlowVerdict {}

unsafe impl NSSecureCoding for NEFilterNewFlowVerdict {}

extern_methods!(
    unsafe impl NEFilterNewFlowVerdict {
        #[method(statisticsReportFrequency)]
        pub unsafe fn statisticsReportFrequency(&self) -> NEFilterReportFrequency;

        #[method(setStatisticsReportFrequency:)]
        pub unsafe fn setStatisticsReportFrequency(
            &self,
            statistics_report_frequency: NEFilterReportFrequency,
        );

        #[method_id(@__retain_semantics Other needRulesVerdict)]
        pub unsafe fn needRulesVerdict() -> Retained<NEFilterNewFlowVerdict>;

        #[method_id(@__retain_semantics Other allowVerdict)]
        pub unsafe fn allowVerdict() -> Retained<NEFilterNewFlowVerdict>;

        #[method_id(@__retain_semantics Other dropVerdict)]
        pub unsafe fn dropVerdict() -> Retained<NEFilterNewFlowVerdict>;

        #[method_id(@__retain_semantics Other remediateVerdictWithRemediationURLMapKey:remediationButtonTextMapKey:)]
        pub unsafe fn remediateVerdictWithRemediationURLMapKey_remediationButtonTextMapKey(
            remediation_url_map_key: &NSString,
            remediation_button_text_map_key: &NSString,
        ) -> Retained<NEFilterNewFlowVerdict>;

        #[method_id(@__retain_semantics Other URLAppendStringVerdictWithMapKey:)]
        pub unsafe fn URLAppendStringVerdictWithMapKey(
            url_append_map_key: &NSString,
        ) -> Retained<NEFilterNewFlowVerdict>;

        #[method_id(@__retain_semantics Other filterDataVerdictWithFilterInbound:peekInboundBytes:filterOutbound:peekOutboundBytes:)]
        pub unsafe fn filterDataVerdictWithFilterInbound_peekInboundBytes_filterOutbound_peekOutboundBytes(
            filter_inbound: bool,
            peek_inbound_bytes: NSUInteger,
            filter_outbound: bool,
            peek_outbound_bytes: NSUInteger,
        ) -> Retained<NEFilterNewFlowVerdict>;

        #[method_id(@__retain_semantics Other pauseVerdict)]
        pub unsafe fn pauseVerdict() -> Retained<NEFilterNewFlowVerdict>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NEFilterNewFlowVerdict {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/networkextension/nefiltercontrolverdict?language=objc)
    #[unsafe(super(NEFilterNewFlowVerdict, NEFilterVerdict, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NEFilterControlVerdict;
);

unsafe impl NSCoding for NEFilterControlVerdict {}

unsafe impl NSCopying for NEFilterControlVerdict {}

unsafe impl CopyingHelper for NEFilterControlVerdict {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NEFilterControlVerdict {}

unsafe impl NSSecureCoding for NEFilterControlVerdict {}

extern_methods!(
    unsafe impl NEFilterControlVerdict {
        #[method_id(@__retain_semantics Other allowVerdictWithUpdateRules:)]
        pub unsafe fn allowVerdictWithUpdateRules(
            update_rules: bool,
        ) -> Retained<NEFilterControlVerdict>;

        #[method_id(@__retain_semantics Other dropVerdictWithUpdateRules:)]
        pub unsafe fn dropVerdictWithUpdateRules(
            update_rules: bool,
        ) -> Retained<NEFilterControlVerdict>;

        #[method_id(@__retain_semantics Other updateRules)]
        pub unsafe fn updateRules() -> Retained<NEFilterControlVerdict>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NEFilterControlVerdict {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/networkextension/nefilteraction?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NEFilterAction(pub NSInteger);
impl NEFilterAction {
    #[doc(alias = "NEFilterActionInvalid")]
    pub const Invalid: Self = Self(0);
    #[doc(alias = "NEFilterActionAllow")]
    pub const Allow: Self = Self(1);
    #[doc(alias = "NEFilterActionDrop")]
    pub const Drop: Self = Self(2);
    #[doc(alias = "NEFilterActionRemediate")]
    pub const Remediate: Self = Self(3);
    #[doc(alias = "NEFilterActionFilterData")]
    pub const FilterData: Self = Self(4);
}

unsafe impl Encode for NEFilterAction {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NEFilterAction {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/networkextension/nefilterreportevent?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NEFilterReportEvent(pub NSInteger);
impl NEFilterReportEvent {
    #[doc(alias = "NEFilterReportEventNewFlow")]
    pub const NewFlow: Self = Self(1);
    #[doc(alias = "NEFilterReportEventDataDecision")]
    pub const DataDecision: Self = Self(2);
    #[doc(alias = "NEFilterReportEventFlowClosed")]
    pub const FlowClosed: Self = Self(3);
    #[doc(alias = "NEFilterReportEventStatistics")]
    pub const Statistics: Self = Self(4);
}

unsafe impl Encode for NEFilterReportEvent {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NEFilterReportEvent {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/networkextension/nefilterreport?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NEFilterReport;
);

unsafe impl NSCoding for NEFilterReport {}

unsafe impl NSCopying for NEFilterReport {}

unsafe impl CopyingHelper for NEFilterReport {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NEFilterReport {}

unsafe impl NSSecureCoding for NEFilterReport {}

extern_methods!(
    unsafe impl NEFilterReport {
        #[cfg(feature = "NEFilterFlow")]
        #[method_id(@__retain_semantics Other flow)]
        pub unsafe fn flow(&self) -> Option<Retained<NEFilterFlow>>;

        #[method(action)]
        pub unsafe fn action(&self) -> NEFilterAction;

        #[method(event)]
        pub unsafe fn event(&self) -> NEFilterReportEvent;

        #[method(bytesInboundCount)]
        pub unsafe fn bytesInboundCount(&self) -> NSUInteger;

        #[method(bytesOutboundCount)]
        pub unsafe fn bytesOutboundCount(&self) -> NSUInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NEFilterReport {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
