//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AMLogLevel(pub NSUInteger);
impl AMLogLevel {
    #[doc(alias = "AMLogLevelDebug")]
    pub const Debug: Self = Self(0);
    #[doc(alias = "AMLogLevelInfo")]
    pub const Info: Self = Self(1);
    #[doc(alias = "AMLogLevelWarn")]
    pub const Warn: Self = Self(2);
    #[doc(alias = "AMLogLevelError")]
    pub const Error: Self = Self(3);
}

unsafe impl Encode for AMLogLevel {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for AMLogLevel {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AMAction;

    unsafe impl ClassType for AMAction {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for AMAction {}

extern_methods!(
    unsafe impl AMAction {
        #[method_id(@__retain_semantics Init initWithDefinition:fromArchive:)]
        pub unsafe fn initWithDefinition_fromArchive(
            this: Allocated<Self>,
            dict: Option<&NSDictionary<NSString, AnyObject>>,
            archived: bool,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init initWithContentsOfURL:error:_)]
        pub unsafe fn initWithContentsOfURL_error(
            this: Allocated<Self>,
            file_url: &NSURL,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[method(ignoresInput)]
        pub unsafe fn ignoresInput(&self) -> bool;

        #[method_id(@__retain_semantics Other selectedInputType)]
        pub unsafe fn selectedInputType(&self) -> Option<Retained<NSString>>;

        #[method(setSelectedInputType:)]
        pub unsafe fn setSelectedInputType(&self, selected_input_type: Option<&NSString>);

        #[method_id(@__retain_semantics Other selectedOutputType)]
        pub unsafe fn selectedOutputType(&self) -> Option<Retained<NSString>>;

        #[method(setSelectedOutputType:)]
        pub unsafe fn setSelectedOutputType(&self, selected_output_type: Option<&NSString>);

        #[method(progressValue)]
        pub unsafe fn progressValue(&self) -> CGFloat;

        #[method(setProgressValue:)]
        pub unsafe fn setProgressValue(&self, progress_value: CGFloat);

        #[deprecated]
        #[method_id(@__retain_semantics Other runWithInput:fromAction:error:)]
        pub unsafe fn runWithInput_fromAction_error(
            &self,
            input: Option<&AnyObject>,
            an_action: Option<&AMAction>,
            error_info: Option<&mut Option<Retained<NSDictionary<NSString, AnyObject>>>>,
        ) -> Option<Retained<AnyObject>>;

        #[method_id(@__retain_semantics Other runWithInput:error:_)]
        pub unsafe fn runWithInput_error(
            &self,
            input: Option<&AnyObject>,
        ) -> Result<Retained<AnyObject>, Retained<NSError>>;

        #[method(runAsynchronouslyWithInput:)]
        pub unsafe fn runAsynchronouslyWithInput(&self, input: Option<&AnyObject>);

        #[method(willFinishRunning)]
        pub unsafe fn willFinishRunning(&self);

        #[deprecated]
        #[method(didFinishRunningWithError:)]
        pub unsafe fn didFinishRunningWithError(
            &self,
            error_info: Option<&NSDictionary<NSString, AnyObject>>,
        );

        #[method(finishRunningWithError:)]
        pub unsafe fn finishRunningWithError(&self, error: Option<&NSError>);

        #[method_id(@__retain_semantics Other output)]
        pub unsafe fn output(&self) -> Option<Retained<AnyObject>>;

        #[method(setOutput:)]
        pub unsafe fn setOutput(&self, output: Option<&AnyObject>);

        #[method(stop)]
        pub unsafe fn stop(&self);

        #[method(reset)]
        pub unsafe fn reset(&self);

        #[method(writeToDictionary:)]
        pub unsafe fn writeToDictionary(
            &self,
            dictionary: &NSMutableDictionary<NSString, AnyObject>,
        );

        #[method(opened)]
        pub unsafe fn opened(&self);

        #[method(activated)]
        pub unsafe fn activated(&self);

        #[method(closed)]
        pub unsafe fn closed(&self);

        #[method(updateParameters)]
        pub unsafe fn updateParameters(&self);

        #[method(parametersUpdated)]
        pub unsafe fn parametersUpdated(&self);

        #[method(isStopped)]
        pub unsafe fn isStopped(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AMAction {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
