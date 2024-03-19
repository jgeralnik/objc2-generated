//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::StoreKit::*;

#[cfg(feature = "Foundation_NSString")]
typed_enum!(
    pub type SKCloudServiceSetupOptionsKey = NSString;
);

#[cfg(feature = "Foundation_NSString")]
typed_enum!(
    pub type SKCloudServiceSetupAction = NSString;
);

#[cfg(feature = "Foundation_NSString")]
typed_enum!(
    pub type SKCloudServiceSetupMessageIdentifier = NSString;
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
    pub struct SKCloudServiceSetupViewController;

    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
    unsafe impl ClassType for SKCloudServiceSetupViewController {
        #[inherits(NSResponder, NSObject)]
        type Super = NSViewController;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSViewController",
    feature = "Foundation_NSObject"
))]
unsafe impl NSCoding for SKCloudServiceSetupViewController {}

#[cfg(all(
    feature = "AppKit_NSKeyValueBinding",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSViewController"
))]
unsafe impl NSEditor for SKCloudServiceSetupViewController {}

#[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
unsafe impl NSObjectProtocol for SKCloudServiceSetupViewController {}

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSStoryboardSegue",
    feature = "AppKit_NSViewController"
))]
unsafe impl NSSeguePerforming for SKCloudServiceSetupViewController {}

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSUserInterfaceItemIdentification",
    feature = "AppKit_NSViewController"
))]
unsafe impl NSUserInterfaceItemIdentification for SKCloudServiceSetupViewController {}

extern_methods!(
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
    unsafe impl SKCloudServiceSetupViewController {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn SKCloudServiceSetupViewControllerDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn SKCloudServiceSetupViewControllerDelegate>>,
        );

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method(loadWithOptions:completionHandler:)]
        pub unsafe fn loadWithOptions_completionHandler(
            &self,
            options: &NSDictionary<SKCloudServiceSetupOptionsKey, AnyObject>,
            completion_handler: Option<&Block<dyn Fn(Bool, *mut NSError)>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
    unsafe impl SKCloudServiceSetupViewController {
        #[cfg(all(
            feature = "AppKit_NSNib",
            feature = "Foundation_NSBundle",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
    unsafe impl SKCloudServiceSetupViewController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
    unsafe impl SKCloudServiceSetupViewController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait SKCloudServiceSetupViewControllerDelegate: NSObjectProtocol {
        #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
        #[optional]
        #[method(cloudServiceSetupViewControllerDidDismiss:)]
        unsafe fn cloudServiceSetupViewControllerDidDismiss(
            &self,
            cloud_service_setup_view_controller: &SKCloudServiceSetupViewController,
        );
    }

    unsafe impl ProtocolType for dyn SKCloudServiceSetupViewControllerDelegate {}
);

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static SKCloudServiceSetupOptionsActionKey: &'static SKCloudServiceSetupOptionsKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static SKCloudServiceSetupOptionsITunesItemIdentifierKey:
        &'static SKCloudServiceSetupOptionsKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static SKCloudServiceSetupOptionsAffiliateTokenKey: &'static SKCloudServiceSetupOptionsKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static SKCloudServiceSetupOptionsCampaignTokenKey: &'static SKCloudServiceSetupOptionsKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static SKCloudServiceSetupOptionsMessageIdentifierKey:
        &'static SKCloudServiceSetupOptionsKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static SKCloudServiceSetupActionSubscribe: &'static SKCloudServiceSetupAction;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static SKCloudServiceSetupMessageIdentifierJoin:
        &'static SKCloudServiceSetupMessageIdentifier;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static SKCloudServiceSetupMessageIdentifierConnect:
        &'static SKCloudServiceSetupMessageIdentifier;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static SKCloudServiceSetupMessageIdentifierAddMusic:
        &'static SKCloudServiceSetupMessageIdentifier;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static SKCloudServiceSetupMessageIdentifierPlayMusic:
        &'static SKCloudServiceSetupMessageIdentifier;
}
