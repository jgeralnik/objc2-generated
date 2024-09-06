//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

#[cfg(all(feature = "NSError", feature = "block2"))]
pub type NSUserScriptTaskCompletionHandler = *mut block2::Block<dyn Fn(*mut NSError)>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSUserScriptTask;

    unsafe impl ClassType for NSUserScriptTask {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for NSUserScriptTask {}

extern_methods!(
    unsafe impl NSUserScriptTask {
        #[cfg(all(feature = "NSError", feature = "NSURL"))]
        #[method_id(@__retain_semantics Init initWithURL:error:_)]
        pub unsafe fn initWithURL_error(
            this: Allocated<Self>,
            url: &NSURL,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(feature = "NSURL")]
        #[method_id(@__retain_semantics Other scriptURL)]
        pub unsafe fn scriptURL(&self) -> Retained<NSURL>;

        #[cfg(all(feature = "NSError", feature = "block2"))]
        #[method(executeWithCompletionHandler:)]
        pub unsafe fn executeWithCompletionHandler(
            &self,
            handler: NSUserScriptTaskCompletionHandler,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSUserScriptTask {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

#[cfg(all(feature = "NSError", feature = "block2"))]
pub type NSUserUnixTaskCompletionHandler = *mut block2::Block<dyn Fn(*mut NSError)>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSUserUnixTask;

    unsafe impl ClassType for NSUserUnixTask {
        #[inherits(NSObject)]
        type Super = NSUserScriptTask;
    }
);

unsafe impl NSObjectProtocol for NSUserUnixTask {}

extern_methods!(
    unsafe impl NSUserUnixTask {
        #[cfg(feature = "NSFileHandle")]
        #[method_id(@__retain_semantics Other standardInput)]
        pub unsafe fn standardInput(&self) -> Option<Retained<NSFileHandle>>;

        #[cfg(feature = "NSFileHandle")]
        #[method(setStandardInput:)]
        pub unsafe fn setStandardInput(&self, standard_input: Option<&NSFileHandle>);

        #[cfg(feature = "NSFileHandle")]
        #[method_id(@__retain_semantics Other standardOutput)]
        pub unsafe fn standardOutput(&self) -> Option<Retained<NSFileHandle>>;

        #[cfg(feature = "NSFileHandle")]
        #[method(setStandardOutput:)]
        pub unsafe fn setStandardOutput(&self, standard_output: Option<&NSFileHandle>);

        #[cfg(feature = "NSFileHandle")]
        #[method_id(@__retain_semantics Other standardError)]
        pub unsafe fn standardError(&self) -> Option<Retained<NSFileHandle>>;

        #[cfg(feature = "NSFileHandle")]
        #[method(setStandardError:)]
        pub unsafe fn setStandardError(&self, standard_error: Option<&NSFileHandle>);

        #[cfg(all(
            feature = "NSArray",
            feature = "NSError",
            feature = "NSString",
            feature = "block2"
        ))]
        #[method(executeWithArguments:completionHandler:)]
        pub unsafe fn executeWithArguments_completionHandler(
            &self,
            arguments: Option<&NSArray<NSString>>,
            handler: NSUserUnixTaskCompletionHandler,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSUserScriptTask`
    unsafe impl NSUserUnixTask {
        #[cfg(all(feature = "NSError", feature = "NSURL"))]
        #[method_id(@__retain_semantics Init initWithURL:error:_)]
        pub unsafe fn initWithURL_error(
            this: Allocated<Self>,
            url: &NSURL,
        ) -> Result<Retained<Self>, Retained<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSUserUnixTask {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

#[cfg(all(
    feature = "NSAppleEventDescriptor",
    feature = "NSError",
    feature = "block2"
))]
pub type NSUserAppleScriptTaskCompletionHandler =
    *mut block2::Block<dyn Fn(*mut NSAppleEventDescriptor, *mut NSError)>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSUserAppleScriptTask;

    unsafe impl ClassType for NSUserAppleScriptTask {
        #[inherits(NSObject)]
        type Super = NSUserScriptTask;
    }
);

unsafe impl NSObjectProtocol for NSUserAppleScriptTask {}

extern_methods!(
    unsafe impl NSUserAppleScriptTask {
        #[cfg(all(
            feature = "NSAppleEventDescriptor",
            feature = "NSError",
            feature = "block2"
        ))]
        #[method(executeWithAppleEvent:completionHandler:)]
        pub unsafe fn executeWithAppleEvent_completionHandler(
            &self,
            event: Option<&NSAppleEventDescriptor>,
            handler: NSUserAppleScriptTaskCompletionHandler,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSUserScriptTask`
    unsafe impl NSUserAppleScriptTask {
        #[cfg(all(feature = "NSError", feature = "NSURL"))]
        #[method_id(@__retain_semantics Init initWithURL:error:_)]
        pub unsafe fn initWithURL_error(
            this: Allocated<Self>,
            url: &NSURL,
        ) -> Result<Retained<Self>, Retained<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSUserAppleScriptTask {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

#[cfg(all(feature = "NSError", feature = "block2"))]
pub type NSUserAutomatorTaskCompletionHandler =
    *mut block2::Block<dyn Fn(*mut AnyObject, *mut NSError)>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSUserAutomatorTask;

    unsafe impl ClassType for NSUserAutomatorTask {
        #[inherits(NSObject)]
        type Super = NSUserScriptTask;
    }
);

unsafe impl NSObjectProtocol for NSUserAutomatorTask {}

extern_methods!(
    unsafe impl NSUserAutomatorTask {
        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method_id(@__retain_semantics Other variables)]
        pub unsafe fn variables(&self) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method(setVariables:)]
        pub unsafe fn setVariables(&self, variables: Option<&NSDictionary<NSString, AnyObject>>);

        #[cfg(all(feature = "NSError", feature = "NSObject", feature = "block2"))]
        #[method(executeWithInput:completionHandler:)]
        pub unsafe fn executeWithInput_completionHandler(
            &self,
            input: Option<&ProtocolObject<dyn NSSecureCoding>>,
            handler: NSUserAutomatorTaskCompletionHandler,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSUserScriptTask`
    unsafe impl NSUserAutomatorTask {
        #[cfg(all(feature = "NSError", feature = "NSURL"))]
        #[method_id(@__retain_semantics Init initWithURL:error:_)]
        pub unsafe fn initWithURL_error(
            this: Allocated<Self>,
            url: &NSURL,
        ) -> Result<Retained<Self>, Retained<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSUserAutomatorTask {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
