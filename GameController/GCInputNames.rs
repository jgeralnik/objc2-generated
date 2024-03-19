//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

extern_protocol!(
    pub unsafe trait GCPhysicalInputElementName {}

    unsafe impl ProtocolType for dyn GCPhysicalInputElementName {}
);

extern_protocol!(
    pub unsafe trait GCButtonElementName: GCPhysicalInputElementName {}

    unsafe impl ProtocolType for dyn GCButtonElementName {}
);

extern_protocol!(
    pub unsafe trait GCAxisElementName: GCPhysicalInputElementName {}

    unsafe impl ProtocolType for dyn GCAxisElementName {}
);

extern_protocol!(
    pub unsafe trait GCSwitchElementName: GCPhysicalInputElementName {}

    unsafe impl ProtocolType for dyn GCSwitchElementName {}
);

extern_protocol!(
    pub unsafe trait GCDirectionPadElementName: GCPhysicalInputElementName {}

    unsafe impl ProtocolType for dyn GCDirectionPadElementName {}
);

#[cfg(feature = "Foundation_NSString")]
extern_static!(GCInputLeftBumper: Option<&'static GCInputButtonName>);

#[cfg(feature = "Foundation_NSString")]
extern_static!(GCInputRightBumper: Option<&'static GCInputButtonName>);

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub fn GCInputBackLeftButton(position: NSInteger) -> *mut GCInputButtonName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub fn GCInputBackRightButton(position: NSInteger) -> *mut GCInputButtonName;
}
