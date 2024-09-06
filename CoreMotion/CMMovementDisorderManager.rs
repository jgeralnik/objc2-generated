//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CMDyskineticSymptomResult;

    unsafe impl ClassType for CMDyskineticSymptomResult {
        type Super = NSObject;
    }
);

unsafe impl NSCoding for CMDyskineticSymptomResult {}

unsafe impl NSCopying for CMDyskineticSymptomResult {}

unsafe impl CopyingHelper for CMDyskineticSymptomResult {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CMDyskineticSymptomResult {}

unsafe impl NSSecureCoding for CMDyskineticSymptomResult {}

extern_methods!(
    unsafe impl CMDyskineticSymptomResult {
        #[method_id(@__retain_semantics Other startDate)]
        pub unsafe fn startDate(&self) -> Retained<NSDate>;

        #[method_id(@__retain_semantics Other endDate)]
        pub unsafe fn endDate(&self) -> Retained<NSDate>;

        #[method(percentUnlikely)]
        pub unsafe fn percentUnlikely(&self) -> c_float;

        #[method(percentLikely)]
        pub unsafe fn percentLikely(&self) -> c_float;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CMDyskineticSymptomResult {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CMTremorResult;

    unsafe impl ClassType for CMTremorResult {
        type Super = NSObject;
    }
);

unsafe impl NSCoding for CMTremorResult {}

unsafe impl NSCopying for CMTremorResult {}

unsafe impl CopyingHelper for CMTremorResult {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CMTremorResult {}

unsafe impl NSSecureCoding for CMTremorResult {}

extern_methods!(
    unsafe impl CMTremorResult {
        #[method_id(@__retain_semantics Other startDate)]
        pub unsafe fn startDate(&self) -> Retained<NSDate>;

        #[method_id(@__retain_semantics Other endDate)]
        pub unsafe fn endDate(&self) -> Retained<NSDate>;

        #[method(percentUnknown)]
        pub unsafe fn percentUnknown(&self) -> c_float;

        #[method(percentNone)]
        pub unsafe fn percentNone(&self) -> c_float;

        #[method(percentSlight)]
        pub unsafe fn percentSlight(&self) -> c_float;

        #[method(percentMild)]
        pub unsafe fn percentMild(&self) -> c_float;

        #[method(percentModerate)]
        pub unsafe fn percentModerate(&self) -> c_float;

        #[method(percentStrong)]
        pub unsafe fn percentStrong(&self) -> c_float;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CMTremorResult {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

#[cfg(feature = "block2")]
pub type CMDyskineticSymptomResultHandler =
    *mut block2::Block<dyn Fn(NonNull<NSArray<CMDyskineticSymptomResult>>, *mut NSError)>;

#[cfg(feature = "block2")]
pub type CMTremorResultHandler =
    *mut block2::Block<dyn Fn(NonNull<NSArray<CMTremorResult>>, *mut NSError)>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CMMovementDisorderManager;

    unsafe impl ClassType for CMMovementDisorderManager {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for CMMovementDisorderManager {}

extern_methods!(
    unsafe impl CMMovementDisorderManager {
        #[method(isAvailable)]
        pub unsafe fn isAvailable() -> bool;

        #[method_id(@__retain_semantics Other version)]
        pub unsafe fn version() -> Option<Retained<NSString>>;

        #[cfg(feature = "CMAuthorization")]
        #[method(authorizationStatus)]
        pub unsafe fn authorizationStatus() -> CMAuthorizationStatus;

        #[method(monitorKinesiasForDuration:)]
        pub unsafe fn monitorKinesiasForDuration(&self, duration: NSTimeInterval);

        #[cfg(feature = "block2")]
        #[method(queryDyskineticSymptomFromDate:toDate:withHandler:)]
        pub unsafe fn queryDyskineticSymptomFromDate_toDate_withHandler(
            &self,
            from_date: &NSDate,
            to_date: &NSDate,
            handler: CMDyskineticSymptomResultHandler,
        );

        #[cfg(feature = "block2")]
        #[method(queryTremorFromDate:toDate:withHandler:)]
        pub unsafe fn queryTremorFromDate_toDate_withHandler(
            &self,
            from_date: &NSDate,
            to_date: &NSDate,
            handler: CMTremorResultHandler,
        );

        #[method_id(@__retain_semantics Other lastProcessedDate)]
        pub unsafe fn lastProcessedDate(&self) -> Option<Retained<NSDate>>;

        #[method_id(@__retain_semantics Other monitorKinesiasExpirationDate)]
        pub unsafe fn monitorKinesiasExpirationDate(&self) -> Option<Retained<NSDate>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CMMovementDisorderManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
