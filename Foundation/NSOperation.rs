//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSOperationQueuePriority(pub NSInteger);
impl NSOperationQueuePriority {
    #[doc(alias = "NSOperationQueuePriorityVeryLow")]
    pub const VeryLow: Self = Self(-8);
    #[doc(alias = "NSOperationQueuePriorityLow")]
    pub const Low: Self = Self(-4);
    #[doc(alias = "NSOperationQueuePriorityNormal")]
    pub const Normal: Self = Self(0);
    #[doc(alias = "NSOperationQueuePriorityHigh")]
    pub const High: Self = Self(4);
    #[doc(alias = "NSOperationQueuePriorityVeryHigh")]
    pub const VeryHigh: Self = Self(8);
}

unsafe impl Encode for NSOperationQueuePriority {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSOperationQueuePriority {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSOperation;
);

unsafe impl Send for NSOperation {}

unsafe impl Sync for NSOperation {}

unsafe impl NSObjectProtocol for NSOperation {}

extern_methods!(
    unsafe impl NSOperation {
        #[method(start)]
        pub unsafe fn start(&self);

        #[method(main)]
        pub unsafe fn main(&self);

        #[method(isCancelled)]
        pub unsafe fn isCancelled(&self) -> bool;

        #[method(cancel)]
        pub unsafe fn cancel(&self);

        #[method(isExecuting)]
        pub unsafe fn isExecuting(&self) -> bool;

        #[method(isFinished)]
        pub unsafe fn isFinished(&self) -> bool;

        #[method(isConcurrent)]
        pub unsafe fn isConcurrent(&self) -> bool;

        #[method(isAsynchronous)]
        pub unsafe fn isAsynchronous(&self) -> bool;

        #[method(isReady)]
        pub unsafe fn isReady(&self) -> bool;

        #[method(addDependency:)]
        pub unsafe fn addDependency(&self, op: &NSOperation);

        #[method(removeDependency:)]
        pub unsafe fn removeDependency(&self, op: &NSOperation);

        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Other dependencies)]
        pub unsafe fn dependencies(&self) -> Retained<NSArray<NSOperation>>;

        #[method(queuePriority)]
        pub unsafe fn queuePriority(&self) -> NSOperationQueuePriority;

        #[method(setQueuePriority:)]
        pub unsafe fn setQueuePriority(&self, queue_priority: NSOperationQueuePriority);

        #[cfg(feature = "block2")]
        #[method(completionBlock)]
        pub unsafe fn completionBlock(&self) -> *mut block2::Block<dyn Fn()>;

        #[cfg(feature = "block2")]
        #[method(setCompletionBlock:)]
        pub unsafe fn setCompletionBlock(&self, completion_block: Option<&block2::Block<dyn Fn()>>);

        #[method(waitUntilFinished)]
        pub unsafe fn waitUntilFinished(&self);

        #[deprecated = "Not supported"]
        #[method(threadPriority)]
        pub unsafe fn threadPriority(&self) -> c_double;

        #[deprecated = "Not supported"]
        #[method(setThreadPriority:)]
        pub unsafe fn setThreadPriority(&self, thread_priority: c_double);

        #[cfg(feature = "NSObjCRuntime")]
        #[method(qualityOfService)]
        pub unsafe fn qualityOfService(&self) -> NSQualityOfService;

        #[cfg(feature = "NSObjCRuntime")]
        #[method(setQualityOfService:)]
        pub unsafe fn setQualityOfService(&self, quality_of_service: NSQualityOfService);

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSOperation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[unsafe(super(NSOperation, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSBlockOperation;
);

unsafe impl Send for NSBlockOperation {}

unsafe impl Sync for NSBlockOperation {}

unsafe impl NSObjectProtocol for NSBlockOperation {}

extern_methods!(
    unsafe impl NSBlockOperation {
        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Other blockOperationWithBlock:)]
        pub unsafe fn blockOperationWithBlock(block: &block2::Block<dyn Fn()>) -> Retained<Self>;

        #[cfg(feature = "block2")]
        #[method(addExecutionBlock:)]
        pub unsafe fn addExecutionBlock(&self, block: &block2::Block<dyn Fn()>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSBlockOperation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[unsafe(super(NSOperation, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSInvocationOperation;
);

unsafe impl NSObjectProtocol for NSInvocationOperation {}

extern_methods!(
    unsafe impl NSInvocationOperation {
        #[method_id(@__retain_semantics Init initWithTarget:selector:object:)]
        pub unsafe fn initWithTarget_selector_object(
            this: Allocated<Self>,
            target: &AnyObject,
            sel: Sel,
            arg: Option<&AnyObject>,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "NSInvocation")]
        #[method_id(@__retain_semantics Init initWithInvocation:)]
        pub unsafe fn initWithInvocation(
            this: Allocated<Self>,
            inv: &NSInvocation,
        ) -> Retained<Self>;

        #[cfg(feature = "NSInvocation")]
        #[method_id(@__retain_semantics Other invocation)]
        pub unsafe fn invocation(&self) -> Retained<NSInvocation>;

        #[method_id(@__retain_semantics Other result)]
        pub unsafe fn result(&self) -> Option<Retained<AnyObject>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSInvocationOperation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    #[cfg(all(feature = "NSObjCRuntime", feature = "NSString"))]
    pub static NSInvocationOperationVoidResultException: &'static NSExceptionName;
}

extern "C" {
    #[cfg(all(feature = "NSObjCRuntime", feature = "NSString"))]
    pub static NSInvocationOperationCancelledException: &'static NSExceptionName;
}

pub static NSOperationQueueDefaultMaxConcurrentOperationCount: NSInteger = -1;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSOperationQueue;
);

unsafe impl Send for NSOperationQueue {}

unsafe impl Sync for NSOperationQueue {}

unsafe impl NSObjectProtocol for NSOperationQueue {}

#[cfg(feature = "NSProgress")]
unsafe impl NSProgressReporting for NSOperationQueue {}

extern_methods!(
    unsafe impl NSOperationQueue {
        #[cfg(feature = "NSProgress")]
        #[method_id(@__retain_semantics Other progress)]
        pub unsafe fn progress(&self) -> Retained<NSProgress>;

        #[method(addOperation:)]
        pub unsafe fn addOperation(&self, op: &NSOperation);

        #[cfg(feature = "NSArray")]
        #[method(addOperations:waitUntilFinished:)]
        pub unsafe fn addOperations_waitUntilFinished(
            &self,
            ops: &NSArray<NSOperation>,
            wait: bool,
        );

        #[cfg(feature = "block2")]
        #[method(addOperationWithBlock:)]
        pub unsafe fn addOperationWithBlock(&self, block: &block2::Block<dyn Fn()>);

        #[cfg(feature = "block2")]
        #[method(addBarrierBlock:)]
        pub unsafe fn addBarrierBlock(&self, barrier: &block2::Block<dyn Fn()>);

        #[method(maxConcurrentOperationCount)]
        pub unsafe fn maxConcurrentOperationCount(&self) -> NSInteger;

        #[method(setMaxConcurrentOperationCount:)]
        pub unsafe fn setMaxConcurrentOperationCount(
            &self,
            max_concurrent_operation_count: NSInteger,
        );

        #[method(isSuspended)]
        pub unsafe fn isSuspended(&self) -> bool;

        #[method(setSuspended:)]
        pub unsafe fn setSuspended(&self, suspended: bool);

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[cfg(feature = "NSObjCRuntime")]
        #[method(qualityOfService)]
        pub unsafe fn qualityOfService(&self) -> NSQualityOfService;

        #[cfg(feature = "NSObjCRuntime")]
        #[method(setQualityOfService:)]
        pub unsafe fn setQualityOfService(&self, quality_of_service: NSQualityOfService);

        #[method(cancelAllOperations)]
        pub unsafe fn cancelAllOperations(&self);

        #[method(waitUntilAllOperationsAreFinished)]
        pub unsafe fn waitUntilAllOperationsAreFinished(&self);

        #[method_id(@__retain_semantics Other currentQueue)]
        pub unsafe fn currentQueue() -> Option<Retained<NSOperationQueue>>;

        #[method_id(@__retain_semantics Other mainQueue)]
        pub unsafe fn mainQueue() -> Retained<NSOperationQueue>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSOperationQueue {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// NSDeprecated
    unsafe impl NSOperationQueue {
        #[cfg(feature = "NSArray")]
        #[deprecated = "access to operations is inherently a race condition, it should not be used. For barrier style behaviors please use addBarrierBlock: instead"]
        #[method_id(@__retain_semantics Other operations)]
        pub unsafe fn operations(&self) -> Retained<NSArray<NSOperation>>;

        #[deprecated]
        #[method(operationCount)]
        pub unsafe fn operationCount(&self) -> NSUInteger;
    }
);
