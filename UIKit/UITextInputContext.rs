//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextinputcontext?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITextInputContext;
);

unsafe impl NSObjectProtocol for UITextInputContext {}

extern_methods!(
    unsafe impl UITextInputContext {
        #[method(isPencilInputExpected)]
        pub unsafe fn isPencilInputExpected(&self) -> bool;

        #[method(setPencilInputExpected:)]
        pub unsafe fn setPencilInputExpected(&self, pencil_input_expected: bool);

        #[method(isDictationInputExpected)]
        pub unsafe fn isDictationInputExpected(&self) -> bool;

        #[method(setDictationInputExpected:)]
        pub unsafe fn setDictationInputExpected(&self, dictation_input_expected: bool);

        #[method(isHardwareKeyboardInputExpected)]
        pub unsafe fn isHardwareKeyboardInputExpected(&self) -> bool;

        #[method(setHardwareKeyboardInputExpected:)]
        pub unsafe fn setHardwareKeyboardInputExpected(
            &self,
            hardware_keyboard_input_expected: bool,
        );

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Other current)]
        pub unsafe fn current() -> Option<Retained<UITextInputContext>>;
    }
);
