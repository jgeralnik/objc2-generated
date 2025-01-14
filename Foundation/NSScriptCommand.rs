//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsnoscripterror?language=objc)
pub const NSNoScriptError: NSInteger = 0;
/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsreceiverevaluationscripterror?language=objc)
pub const NSReceiverEvaluationScriptError: NSInteger = 1;
/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nskeyspecifierevaluationscripterror?language=objc)
pub const NSKeySpecifierEvaluationScriptError: NSInteger = 2;
/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsargumentevaluationscripterror?language=objc)
pub const NSArgumentEvaluationScriptError: NSInteger = 3;
/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsreceiverscanthandlecommandscripterror?language=objc)
pub const NSReceiversCantHandleCommandScriptError: NSInteger = 4;
/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsrequiredargumentsmissingscripterror?language=objc)
pub const NSRequiredArgumentsMissingScriptError: NSInteger = 5;
/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsargumentswrongscripterror?language=objc)
pub const NSArgumentsWrongScriptError: NSInteger = 6;
/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsunknownkeyscripterror?language=objc)
pub const NSUnknownKeyScriptError: NSInteger = 7;
/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsinternalscripterror?language=objc)
pub const NSInternalScriptError: NSInteger = 8;
/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsoperationnotsupportedforkeyscripterror?language=objc)
pub const NSOperationNotSupportedForKeyScriptError: NSInteger = 9;
/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nscannotcreatescriptcommanderror?language=objc)
pub const NSCannotCreateScriptCommandError: NSInteger = 10;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsscriptcommand?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSScriptCommand;
);

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSScriptCommand {}

unsafe impl NSObjectProtocol for NSScriptCommand {}

extern_methods!(
    unsafe impl NSScriptCommand {
        #[cfg(feature = "NSScriptCommandDescription")]
        #[method_id(@__retain_semantics Init initWithCommandDescription:)]
        pub unsafe fn initWithCommandDescription(
            this: Allocated<Self>,
            command_def: &NSScriptCommandDescription,
        ) -> Retained<Self>;

        #[cfg(feature = "NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            in_coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "NSScriptCommandDescription")]
        #[method_id(@__retain_semantics Other commandDescription)]
        pub unsafe fn commandDescription(&self) -> Retained<NSScriptCommandDescription>;

        #[method_id(@__retain_semantics Other directParameter)]
        pub unsafe fn directParameter(&self) -> Option<Retained<AnyObject>>;

        #[method(setDirectParameter:)]
        pub unsafe fn setDirectParameter(&self, direct_parameter: Option<&AnyObject>);

        #[cfg(feature = "NSScriptObjectSpecifiers")]
        #[method_id(@__retain_semantics Other receiversSpecifier)]
        pub unsafe fn receiversSpecifier(&self) -> Option<Retained<NSScriptObjectSpecifier>>;

        #[cfg(feature = "NSScriptObjectSpecifiers")]
        #[method(setReceiversSpecifier:)]
        pub unsafe fn setReceiversSpecifier(
            &self,
            receivers_specifier: Option<&NSScriptObjectSpecifier>,
        );

        #[method_id(@__retain_semantics Other evaluatedReceivers)]
        pub unsafe fn evaluatedReceivers(&self) -> Option<Retained<AnyObject>>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method_id(@__retain_semantics Other arguments)]
        pub unsafe fn arguments(&self) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method(setArguments:)]
        pub unsafe fn setArguments(&self, arguments: Option<&NSDictionary<NSString, AnyObject>>);

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method_id(@__retain_semantics Other evaluatedArguments)]
        pub unsafe fn evaluatedArguments(
            &self,
        ) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;

        #[method(isWellFormed)]
        pub unsafe fn isWellFormed(&self) -> bool;

        #[method_id(@__retain_semantics Other performDefaultImplementation)]
        pub unsafe fn performDefaultImplementation(&self) -> Option<Retained<AnyObject>>;

        #[method_id(@__retain_semantics Other executeCommand)]
        pub unsafe fn executeCommand(&self) -> Option<Retained<AnyObject>>;

        #[method(scriptErrorNumber)]
        pub unsafe fn scriptErrorNumber(&self) -> NSInteger;

        #[method(setScriptErrorNumber:)]
        pub unsafe fn setScriptErrorNumber(&self, script_error_number: NSInteger);

        #[cfg(feature = "NSAppleEventDescriptor")]
        #[method_id(@__retain_semantics Other scriptErrorOffendingObjectDescriptor)]
        pub unsafe fn scriptErrorOffendingObjectDescriptor(
            &self,
        ) -> Option<Retained<NSAppleEventDescriptor>>;

        #[cfg(feature = "NSAppleEventDescriptor")]
        #[method(setScriptErrorOffendingObjectDescriptor:)]
        pub unsafe fn setScriptErrorOffendingObjectDescriptor(
            &self,
            script_error_offending_object_descriptor: Option<&NSAppleEventDescriptor>,
        );

        #[cfg(feature = "NSAppleEventDescriptor")]
        #[method_id(@__retain_semantics Other scriptErrorExpectedTypeDescriptor)]
        pub unsafe fn scriptErrorExpectedTypeDescriptor(
            &self,
        ) -> Option<Retained<NSAppleEventDescriptor>>;

        #[cfg(feature = "NSAppleEventDescriptor")]
        #[method(setScriptErrorExpectedTypeDescriptor:)]
        pub unsafe fn setScriptErrorExpectedTypeDescriptor(
            &self,
            script_error_expected_type_descriptor: Option<&NSAppleEventDescriptor>,
        );

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other scriptErrorString)]
        pub unsafe fn scriptErrorString(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method(setScriptErrorString:)]
        pub unsafe fn setScriptErrorString(&self, script_error_string: Option<&NSString>);

        #[method_id(@__retain_semantics Other currentCommand)]
        pub unsafe fn currentCommand() -> Option<Retained<NSScriptCommand>>;

        #[cfg(feature = "NSAppleEventDescriptor")]
        #[method_id(@__retain_semantics Other appleEvent)]
        pub unsafe fn appleEvent(&self) -> Option<Retained<NSAppleEventDescriptor>>;

        #[method(suspendExecution)]
        pub unsafe fn suspendExecution(&self);

        #[method(resumeExecutionWithResult:)]
        pub unsafe fn resumeExecutionWithResult(&self, result: Option<&AnyObject>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSScriptCommand {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
