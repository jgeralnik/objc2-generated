//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    pub static CBCentralManagerOptionShowPowerAlertKey: &'static NSString;
}

extern "C" {
    pub static CBCentralManagerOptionRestoreIdentifierKey: &'static NSString;
}

extern "C" {
    pub static CBCentralManagerOptionDeviceAccessForMedia: &'static NSString;
}

extern "C" {
    pub static CBCentralManagerScanOptionAllowDuplicatesKey: &'static NSString;
}

extern "C" {
    pub static CBCentralManagerScanOptionSolicitedServiceUUIDsKey: &'static NSString;
}

extern "C" {
    pub static CBConnectPeripheralOptionNotifyOnConnectionKey: &'static NSString;
}

extern "C" {
    pub static CBConnectPeripheralOptionNotifyOnDisconnectionKey: &'static NSString;
}

extern "C" {
    pub static CBConnectPeripheralOptionNotifyOnNotificationKey: &'static NSString;
}

extern "C" {
    pub static CBConnectPeripheralOptionStartDelayKey: &'static NSString;
}

extern "C" {
    pub static CBConnectPeripheralOptionEnableTransportBridgingKey: &'static NSString;
}

extern "C" {
    pub static CBConnectPeripheralOptionRequiresANCS: &'static NSString;
}

extern "C" {
    pub static CBCentralManagerRestoredStatePeripheralsKey: &'static NSString;
}

extern "C" {
    pub static CBCentralManagerRestoredStateScanServicesKey: &'static NSString;
}

extern "C" {
    pub static CBCentralManagerRestoredStateScanOptionsKey: &'static NSString;
}

// NS_TYPED_ENUM
pub type CBConnectionEventMatchingOption = NSString;

extern "C" {
    pub static CBConnectionEventMatchingOptionServiceUUIDs:
        &'static CBConnectionEventMatchingOption;
}

extern "C" {
    pub static CBConnectionEventMatchingOptionPeripheralUUIDs:
        &'static CBConnectionEventMatchingOption;
}

extern "C" {
    pub static CBConnectPeripheralOptionEnableAutoReconnect: &'static NSString;
}