//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::StoreKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
    pub struct SKStoreProductViewController;

    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
    unsafe impl ClassType for SKStoreProductViewController {
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
unsafe impl NSCoding for SKStoreProductViewController {}

#[cfg(all(
    feature = "AppKit_NSKeyValueBinding",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSViewController"
))]
unsafe impl NSEditor for SKStoreProductViewController {}

#[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
unsafe impl NSObjectProtocol for SKStoreProductViewController {}

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSStoryboardSegue",
    feature = "AppKit_NSViewController"
))]
unsafe impl NSSeguePerforming for SKStoreProductViewController {}

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSUserInterfaceItemIdentification",
    feature = "AppKit_NSViewController"
))]
unsafe impl NSUserInterfaceItemIdentification for SKStoreProductViewController {}

extern_methods!(
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
    unsafe impl SKStoreProductViewController {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn SKStoreProductViewControllerDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn SKStoreProductViewControllerDelegate>>,
        );

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method(loadProductWithParameters:completionBlock:)]
        pub unsafe fn loadProductWithParameters_completionBlock(
            &self,
            parameters: &NSDictionary<NSString, AnyObject>,
            block: Option<&Block<dyn Fn(Bool, *mut NSError)>>,
        );

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "StoreKit_SKAdImpression"
        ))]
        #[method(loadProductWithParameters:impression:completionBlock:)]
        pub unsafe fn loadProductWithParameters_impression_completionBlock(
            &self,
            parameters: &NSDictionary<NSString, AnyObject>,
            impression: &SKAdImpression,
            block: Option<&Block<dyn Fn(Bool, *mut NSError)>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
    unsafe impl SKStoreProductViewController {
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
    unsafe impl SKStoreProductViewController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
    unsafe impl SKStoreProductViewController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait SKStoreProductViewControllerDelegate: NSObjectProtocol {
        #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSViewController"))]
        #[optional]
        #[method(productViewControllerDidFinish:)]
        unsafe fn productViewControllerDidFinish(
            &self,
            view_controller: &SKStoreProductViewController,
        );
    }

    unsafe impl ProtocolType for dyn SKStoreProductViewControllerDelegate {}
);

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static SKStoreProductParameterITunesItemIdentifier: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static SKStoreProductParameterProductIdentifier: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static SKStoreProductParameterCustomProductPageIdentifier: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static SKStoreProductParameterAffiliateToken: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static SKStoreProductParameterCampaignToken: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static SKStoreProductParameterProviderToken: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static SKStoreProductParameterAdvertisingPartnerToken: &'static NSString;
}
