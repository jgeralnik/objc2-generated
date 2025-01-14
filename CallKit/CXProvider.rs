//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/callkit/cxcallendedreason?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CXCallEndedReason(pub NSInteger);
impl CXCallEndedReason {
    #[doc(alias = "CXCallEndedReasonFailed")]
    pub const Failed: Self = Self(1);
    #[doc(alias = "CXCallEndedReasonRemoteEnded")]
    pub const RemoteEnded: Self = Self(2);
    #[doc(alias = "CXCallEndedReasonUnanswered")]
    pub const Unanswered: Self = Self(3);
    #[doc(alias = "CXCallEndedReasonAnsweredElsewhere")]
    pub const AnsweredElsewhere: Self = Self(4);
    #[doc(alias = "CXCallEndedReasonDeclinedElsewhere")]
    pub const DeclinedElsewhere: Self = Self(5);
}

unsafe impl Encode for CXCallEndedReason {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CXCallEndedReason {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/callkit/cxproviderdelegate?language=objc)
    pub unsafe trait CXProviderDelegate: NSObjectProtocol {
        #[method(providerDidReset:)]
        unsafe fn providerDidReset(&self, provider: &CXProvider);

        #[optional]
        #[method(providerDidBegin:)]
        unsafe fn providerDidBegin(&self, provider: &CXProvider);

        #[cfg(feature = "CXTransaction")]
        #[optional]
        #[method(provider:executeTransaction:)]
        unsafe fn provider_executeTransaction(
            &self,
            provider: &CXProvider,
            transaction: &CXTransaction,
        ) -> bool;

        #[cfg(all(
            feature = "CXAction",
            feature = "CXCallAction",
            feature = "CXStartCallAction"
        ))]
        #[optional]
        #[method(provider:performStartCallAction:)]
        unsafe fn provider_performStartCallAction(
            &self,
            provider: &CXProvider,
            action: &CXStartCallAction,
        );

        #[cfg(all(
            feature = "CXAction",
            feature = "CXAnswerCallAction",
            feature = "CXCallAction"
        ))]
        #[optional]
        #[method(provider:performAnswerCallAction:)]
        unsafe fn provider_performAnswerCallAction(
            &self,
            provider: &CXProvider,
            action: &CXAnswerCallAction,
        );

        #[cfg(all(
            feature = "CXAction",
            feature = "CXCallAction",
            feature = "CXEndCallAction"
        ))]
        #[optional]
        #[method(provider:performEndCallAction:)]
        unsafe fn provider_performEndCallAction(
            &self,
            provider: &CXProvider,
            action: &CXEndCallAction,
        );

        #[cfg(all(
            feature = "CXAction",
            feature = "CXCallAction",
            feature = "CXSetHeldCallAction"
        ))]
        #[optional]
        #[method(provider:performSetHeldCallAction:)]
        unsafe fn provider_performSetHeldCallAction(
            &self,
            provider: &CXProvider,
            action: &CXSetHeldCallAction,
        );

        #[cfg(all(
            feature = "CXAction",
            feature = "CXCallAction",
            feature = "CXSetMutedCallAction"
        ))]
        #[optional]
        #[method(provider:performSetMutedCallAction:)]
        unsafe fn provider_performSetMutedCallAction(
            &self,
            provider: &CXProvider,
            action: &CXSetMutedCallAction,
        );

        #[cfg(all(
            feature = "CXAction",
            feature = "CXCallAction",
            feature = "CXSetGroupCallAction"
        ))]
        #[optional]
        #[method(provider:performSetGroupCallAction:)]
        unsafe fn provider_performSetGroupCallAction(
            &self,
            provider: &CXProvider,
            action: &CXSetGroupCallAction,
        );

        #[cfg(all(
            feature = "CXAction",
            feature = "CXCallAction",
            feature = "CXPlayDTMFCallAction"
        ))]
        #[optional]
        #[method(provider:performPlayDTMFCallAction:)]
        unsafe fn provider_performPlayDTMFCallAction(
            &self,
            provider: &CXProvider,
            action: &CXPlayDTMFCallAction,
        );

        #[cfg(feature = "CXAction")]
        #[optional]
        #[method(provider:timedOutPerformingAction:)]
        unsafe fn provider_timedOutPerformingAction(
            &self,
            provider: &CXProvider,
            action: &CXAction,
        );
    }

    unsafe impl ProtocolType for dyn CXProviderDelegate {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/callkit/cxprovider?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CXProvider;
);

unsafe impl NSObjectProtocol for CXProvider {}

extern_methods!(
    unsafe impl CXProvider {
        #[cfg(feature = "CXProviderConfiguration")]
        #[method_id(@__retain_semantics Init initWithConfiguration:)]
        pub unsafe fn initWithConfiguration(
            this: Allocated<Self>,
            configuration: &CXProviderConfiguration,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(all(feature = "CXCallUpdate", feature = "block2"))]
        #[method(reportNewIncomingCallWithUUID:update:completion:)]
        pub unsafe fn reportNewIncomingCallWithUUID_update_completion(
            &self,
            uuid: &NSUUID,
            update: &CXCallUpdate,
            completion: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "CXCallUpdate")]
        #[method(reportCallWithUUID:updated:)]
        pub unsafe fn reportCallWithUUID_updated(&self, uuid: &NSUUID, update: &CXCallUpdate);

        #[method(reportCallWithUUID:endedAtDate:reason:)]
        pub unsafe fn reportCallWithUUID_endedAtDate_reason(
            &self,
            uuid: &NSUUID,
            date_ended: Option<&NSDate>,
            ended_reason: CXCallEndedReason,
        );

        #[method(reportOutgoingCallWithUUID:startedConnectingAtDate:)]
        pub unsafe fn reportOutgoingCallWithUUID_startedConnectingAtDate(
            &self,
            uuid: &NSUUID,
            date_started_connecting: Option<&NSDate>,
        );

        #[method(reportOutgoingCallWithUUID:connectedAtDate:)]
        pub unsafe fn reportOutgoingCallWithUUID_connectedAtDate(
            &self,
            uuid: &NSUUID,
            date_connected: Option<&NSDate>,
        );

        #[cfg(feature = "block2")]
        #[method(reportNewIncomingVoIPPushPayload:completion:)]
        pub unsafe fn reportNewIncomingVoIPPushPayload_completion(
            dictionary_payload: &NSDictionary,
            completion: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        );

        #[cfg(feature = "CXProviderConfiguration")]
        #[method_id(@__retain_semantics Other configuration)]
        pub unsafe fn configuration(&self) -> Retained<CXProviderConfiguration>;

        #[cfg(feature = "CXProviderConfiguration")]
        #[method(setConfiguration:)]
        pub unsafe fn setConfiguration(&self, configuration: &CXProviderConfiguration);

        #[method(invalidate)]
        pub unsafe fn invalidate(&self);

        #[cfg(feature = "CXTransaction")]
        #[method_id(@__retain_semantics Other pendingTransactions)]
        pub unsafe fn pendingTransactions(&self) -> Retained<NSArray<CXTransaction>>;

        #[cfg(all(feature = "CXAction", feature = "CXCallAction"))]
        #[method_id(@__retain_semantics Other pendingCallActionsOfClass:withCallUUID:)]
        pub unsafe fn pendingCallActionsOfClass_withCallUUID(
            &self,
            call_action_class: &AnyClass,
            call_uuid: &NSUUID,
        ) -> Retained<NSArray<CXCallAction>>;
    }
);
