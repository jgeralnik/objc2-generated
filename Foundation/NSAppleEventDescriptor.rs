//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSAppleEventSendOptions {
        NSAppleEventSendNoReply = 1,
        NSAppleEventSendQueueReply = 2,
        NSAppleEventSendWaitForReply = 3,
        NSAppleEventSendNeverInteract = 16,
        NSAppleEventSendCanInteract = 32,
        NSAppleEventSendAlwaysInteract = 48,
        NSAppleEventSendCanSwitchLayer = 64,
        NSAppleEventSendDontRecord = 4096,
        NSAppleEventSendDontExecute = 8192,
        NSAppleEventSendDontAnnotate = 65536,
        NSAppleEventSendDefaultOptions = 35,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSAppleEventDescriptor;

    unsafe impl ClassType for NSAppleEventDescriptor {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSAppleEventDescriptor")]
    unsafe impl NSAppleEventDescriptor {
        #[method_id(@__retain_semantics Other nullDescriptor)]
        pub unsafe fn nullDescriptor() -> Id<NSAppleEventDescriptor, Shared>;

        #[method_id(@__retain_semantics Other descriptorWithBoolean:)]
        pub unsafe fn descriptorWithBoolean(boolean: Boolean)
            -> Id<NSAppleEventDescriptor, Shared>;

        #[method_id(@__retain_semantics Other descriptorWithEnumCode:)]
        pub unsafe fn descriptorWithEnumCode(
            enumerator: OSType,
        ) -> Id<NSAppleEventDescriptor, Shared>;

        #[method_id(@__retain_semantics Other descriptorWithInt32:)]
        pub unsafe fn descriptorWithInt32(signedInt: i32) -> Id<NSAppleEventDescriptor, Shared>;

        #[method_id(@__retain_semantics Other descriptorWithDouble:)]
        pub unsafe fn descriptorWithDouble(
            doubleValue: c_double,
        ) -> Id<NSAppleEventDescriptor, Shared>;

        #[method_id(@__retain_semantics Other descriptorWithTypeCode:)]
        pub unsafe fn descriptorWithTypeCode(
            typeCode: OSType,
        ) -> Id<NSAppleEventDescriptor, Shared>;

        #[method_id(@__retain_semantics Other descriptorWithString:)]
        pub unsafe fn descriptorWithString(string: &NSString)
            -> Id<NSAppleEventDescriptor, Shared>;

        #[method_id(@__retain_semantics Other descriptorWithDate:)]
        pub unsafe fn descriptorWithDate(date: &NSDate) -> Id<NSAppleEventDescriptor, Shared>;

        #[method_id(@__retain_semantics Other descriptorWithFileURL:)]
        pub unsafe fn descriptorWithFileURL(fileURL: &NSURL) -> Id<NSAppleEventDescriptor, Shared>;

        #[method_id(@__retain_semantics Other listDescriptor)]
        pub unsafe fn listDescriptor() -> Id<NSAppleEventDescriptor, Shared>;

        #[method_id(@__retain_semantics Other recordDescriptor)]
        pub unsafe fn recordDescriptor() -> Id<NSAppleEventDescriptor, Shared>;

        #[method_id(@__retain_semantics Other currentProcessDescriptor)]
        pub unsafe fn currentProcessDescriptor() -> Id<NSAppleEventDescriptor, Shared>;

        #[method_id(@__retain_semantics Other descriptorWithBundleIdentifier:)]
        pub unsafe fn descriptorWithBundleIdentifier(
            bundleIdentifier: &NSString,
        ) -> Id<NSAppleEventDescriptor, Shared>;

        #[method_id(@__retain_semantics Other descriptorWithApplicationURL:)]
        pub unsafe fn descriptorWithApplicationURL(
            applicationURL: &NSURL,
        ) -> Id<NSAppleEventDescriptor, Shared>;

        #[method_id(@__retain_semantics Init initListDescriptor)]
        pub unsafe fn initListDescriptor(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initRecordDescriptor)]
        pub unsafe fn initRecordDescriptor(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other data)]
        pub unsafe fn data(&self) -> Id<NSData, Shared>;

        #[method(booleanValue)]
        pub unsafe fn booleanValue(&self) -> Boolean;

        #[method(enumCodeValue)]
        pub unsafe fn enumCodeValue(&self) -> OSType;

        #[method(int32Value)]
        pub unsafe fn int32Value(&self) -> i32;

        #[method(doubleValue)]
        pub unsafe fn doubleValue(&self) -> c_double;

        #[method(typeCodeValue)]
        pub unsafe fn typeCodeValue(&self) -> OSType;

        #[method_id(@__retain_semantics Other stringValue)]
        pub unsafe fn stringValue(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other dateValue)]
        pub unsafe fn dateValue(&self) -> Option<Id<NSDate, Shared>>;

        #[method_id(@__retain_semantics Other fileURLValue)]
        pub unsafe fn fileURLValue(&self) -> Option<Id<NSURL, Shared>>;

        #[method(isRecordDescriptor)]
        pub unsafe fn isRecordDescriptor(&self) -> bool;

        #[method(numberOfItems)]
        pub unsafe fn numberOfItems(&self) -> NSInteger;

        #[method(insertDescriptor:atIndex:)]
        pub unsafe fn insertDescriptor_atIndex(
            &self,
            descriptor: &NSAppleEventDescriptor,
            index: NSInteger,
        );

        #[method_id(@__retain_semantics Other descriptorAtIndex:)]
        pub unsafe fn descriptorAtIndex(
            &self,
            index: NSInteger,
        ) -> Option<Id<NSAppleEventDescriptor, Shared>>;

        #[method(removeDescriptorAtIndex:)]
        pub unsafe fn removeDescriptorAtIndex(&self, index: NSInteger);
    }
);
