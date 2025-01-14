//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-symbols")]
use objc2_symbols::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uisymboleffectcompletion?language=objc)
#[cfg(feature = "block2")]
pub type UISymbolEffectCompletion =
    *mut block2::Block<dyn Fn(NonNull<UISymbolEffectCompletionContext>)>;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uisymboleffectcompletioncontext?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UISymbolEffectCompletionContext;
);

unsafe impl NSObjectProtocol for UISymbolEffectCompletionContext {}

extern_methods!(
    unsafe impl UISymbolEffectCompletionContext {
        #[method(isFinished)]
        pub unsafe fn isFinished(&self) -> bool;

        #[method_id(@__retain_semantics Other sender)]
        pub unsafe fn sender(&self) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "objc2-symbols")]
        #[method_id(@__retain_semantics Other effect)]
        pub unsafe fn effect(&self) -> Option<Retained<NSSymbolEffect>>;

        #[cfg(feature = "objc2-symbols")]
        #[method_id(@__retain_semantics Other contentTransition)]
        pub unsafe fn contentTransition(&self) -> Option<Retained<NSSymbolContentTransition>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
