//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_static!(NSDefaultRunLoopMode: &'static NSRunLoopMode);

extern_static!(NSRunLoopCommonModes: &'static NSRunLoopMode);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSRunLoop;

    unsafe impl ClassType for NSRunLoop {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSRunLoop")]
    unsafe impl NSRunLoop {
        #[method_id(@__retain_semantics Other currentRunLoop)]
        pub unsafe fn currentRunLoop() -> Id<NSRunLoop, Shared>;

        #[method_id(@__retain_semantics Other mainRunLoop)]
        pub unsafe fn mainRunLoop() -> Id<NSRunLoop, Shared>;

        #[method_id(@__retain_semantics Other currentMode)]
        pub unsafe fn currentMode(&self) -> Option<Id<NSRunLoopMode, Shared>>;

        #[method(addTimer:forMode:)]
        pub unsafe fn addTimer_forMode(&self, timer: &NSTimer, mode: &NSRunLoopMode);

        #[method(addPort:forMode:)]
        pub unsafe fn addPort_forMode(&self, aPort: &NSPort, mode: &NSRunLoopMode);

        #[method(removePort:forMode:)]
        pub unsafe fn removePort_forMode(&self, aPort: &NSPort, mode: &NSRunLoopMode);

        #[method_id(@__retain_semantics Other limitDateForMode:)]
        pub unsafe fn limitDateForMode(&self, mode: &NSRunLoopMode) -> Option<Id<NSDate, Shared>>;

        #[method(acceptInputForMode:beforeDate:)]
        pub unsafe fn acceptInputForMode_beforeDate(
            &self,
            mode: &NSRunLoopMode,
            limitDate: &NSDate,
        );
    }
);

extern_methods!(
    /// NSRunLoopConveniences
    #[cfg(feature = "Foundation_NSRunLoop")]
    unsafe impl NSRunLoop {
        #[method(run)]
        pub unsafe fn run(&self);

        #[method(runUntilDate:)]
        pub unsafe fn runUntilDate(&self, limitDate: &NSDate);

        #[method(runMode:beforeDate:)]
        pub unsafe fn runMode_beforeDate(&self, mode: &NSRunLoopMode, limitDate: &NSDate) -> bool;

        #[method(configureAsServer)]
        pub unsafe fn configureAsServer(&self);

        #[method(performInModes:block:)]
        pub unsafe fn performInModes_block(
            &self,
            modes: &NSArray<NSRunLoopMode>,
            block: &Block<(), ()>,
        );

        #[method(performBlock:)]
        pub unsafe fn performBlock(&self, block: &Block<(), ()>);
    }
);

extern_methods!(
    /// NSOrderedPerform
    #[cfg(feature = "Foundation_NSRunLoop")]
    unsafe impl NSRunLoop {
        #[method(performSelector:target:argument:order:modes:)]
        pub unsafe fn performSelector_target_argument_order_modes(
            &self,
            aSelector: Sel,
            target: &Object,
            arg: Option<&Object>,
            order: NSUInteger,
            modes: &NSArray<NSRunLoopMode>,
        );

        #[method(cancelPerformSelector:target:argument:)]
        pub unsafe fn cancelPerformSelector_target_argument(
            &self,
            aSelector: Sel,
            target: &Object,
            arg: Option<&Object>,
        );

        #[method(cancelPerformSelectorsWithTarget:)]
        pub unsafe fn cancelPerformSelectorsWithTarget(&self, target: &Object);
    }
);
