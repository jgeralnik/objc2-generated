//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSGlobalDomain: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSArgumentDomain: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSRegistrationDomain: &'static NSString;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSUserDefaults;

    unsafe impl ClassType for NSUserDefaults {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSUserDefaults {}

extern_methods!(
    unsafe impl NSUserDefaults {
        #[method_id(@__retain_semantics Other standardUserDefaults)]
        pub unsafe fn standardUserDefaults() -> Id<NSUserDefaults>;

        #[method(resetStandardUserDefaults)]
        pub unsafe fn resetStandardUserDefaults();

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithSuiteName:)]
        pub unsafe fn initWithSuiteName(
            this: Allocated<Self>,
            suitename: Option<&NSString>,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use -init instead"]
        #[method_id(@__retain_semantics Init initWithUser:)]
        pub unsafe fn initWithUser(this: Allocated<Self>, username: &NSString) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other objectForKey:)]
        pub unsafe fn objectForKey(&self, default_name: &NSString) -> Option<Id<AnyObject>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setObject:forKey:)]
        pub unsafe fn setObject_forKey(&self, value: Option<&AnyObject>, default_name: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method(removeObjectForKey:)]
        pub unsafe fn removeObjectForKey(&self, default_name: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other stringForKey:)]
        pub unsafe fn stringForKey(&self, default_name: &NSString) -> Option<Id<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other arrayForKey:)]
        pub unsafe fn arrayForKey(&self, default_name: &NSString) -> Option<Id<NSArray>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other dictionaryForKey:)]
        pub unsafe fn dictionaryForKey(
            &self,
            default_name: &NSString,
        ) -> Option<Id<NSDictionary<NSString, AnyObject>>>;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other dataForKey:)]
        pub unsafe fn dataForKey(&self, default_name: &NSString) -> Option<Id<NSData>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other stringArrayForKey:)]
        pub unsafe fn stringArrayForKey(
            &self,
            default_name: &NSString,
        ) -> Option<Id<NSArray<NSString>>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(integerForKey:)]
        pub unsafe fn integerForKey(&self, default_name: &NSString) -> NSInteger;

        #[cfg(feature = "Foundation_NSString")]
        #[method(floatForKey:)]
        pub unsafe fn floatForKey(&self, default_name: &NSString) -> c_float;

        #[cfg(feature = "Foundation_NSString")]
        #[method(doubleForKey:)]
        pub unsafe fn doubleForKey(&self, default_name: &NSString) -> c_double;

        #[cfg(feature = "Foundation_NSString")]
        #[method(boolForKey:)]
        pub unsafe fn boolForKey(&self, default_name: &NSString) -> bool;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other URLForKey:)]
        pub unsafe fn URLForKey(&self, default_name: &NSString) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setInteger:forKey:)]
        pub unsafe fn setInteger_forKey(&self, value: NSInteger, default_name: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method(setFloat:forKey:)]
        pub unsafe fn setFloat_forKey(&self, value: c_float, default_name: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method(setDouble:forKey:)]
        pub unsafe fn setDouble_forKey(&self, value: c_double, default_name: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method(setBool:forKey:)]
        pub unsafe fn setBool_forKey(&self, value: bool, default_name: &NSString);

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURL"))]
        #[method(setURL:forKey:)]
        pub unsafe fn setURL_forKey(&self, url: Option<&NSURL>, default_name: &NSString);

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(registerDefaults:)]
        pub unsafe fn registerDefaults(
            &self,
            registration_dictionary: &NSDictionary<NSString, AnyObject>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(addSuiteNamed:)]
        pub unsafe fn addSuiteNamed(&self, suite_name: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method(removeSuiteNamed:)]
        pub unsafe fn removeSuiteNamed(&self, suite_name: &NSString);

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other dictionaryRepresentation)]
        pub unsafe fn dictionaryRepresentation(&self) -> Id<NSDictionary<NSString, AnyObject>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other volatileDomainNames)]
        pub unsafe fn volatileDomainNames(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other volatileDomainForName:)]
        pub unsafe fn volatileDomainForName(
            &self,
            domain_name: &NSString,
        ) -> Id<NSDictionary<NSString, AnyObject>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(setVolatileDomain:forName:)]
        pub unsafe fn setVolatileDomain_forName(
            &self,
            domain: &NSDictionary<NSString, AnyObject>,
            domain_name: &NSString,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(removeVolatileDomainForName:)]
        pub unsafe fn removeVolatileDomainForName(&self, domain_name: &NSString);

        #[cfg(feature = "Foundation_NSArray")]
        #[deprecated = "Not recommended"]
        #[method_id(@__retain_semantics Other persistentDomainNames)]
        pub unsafe fn persistentDomainNames(&self) -> Id<NSArray>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other persistentDomainForName:)]
        pub unsafe fn persistentDomainForName(
            &self,
            domain_name: &NSString,
        ) -> Option<Id<NSDictionary<NSString, AnyObject>>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(setPersistentDomain:forName:)]
        pub unsafe fn setPersistentDomain_forName(
            &self,
            domain: &NSDictionary<NSString, AnyObject>,
            domain_name: &NSString,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(removePersistentDomainForName:)]
        pub unsafe fn removePersistentDomainForName(&self, domain_name: &NSString);

        #[method(synchronize)]
        pub unsafe fn synchronize(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(objectIsForcedForKey:)]
        pub unsafe fn objectIsForcedForKey(&self, key: &NSString) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(objectIsForcedForKey:inDomain:)]
        pub unsafe fn objectIsForcedForKey_inDomain(
            &self,
            key: &NSString,
            domain: &NSString,
        ) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSUserDefaults {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern "C" {
    #[cfg(all(feature = "Foundation_NSNotification", feature = "Foundation_NSString"))]
    pub static NSUserDefaultsSizeLimitExceededNotification: &'static NSNotificationName;
}

extern "C" {
    #[cfg(all(feature = "Foundation_NSNotification", feature = "Foundation_NSString"))]
    pub static NSUbiquitousUserDefaultsNoCloudAccountNotification: &'static NSNotificationName;
}

extern "C" {
    #[cfg(all(feature = "Foundation_NSNotification", feature = "Foundation_NSString"))]
    pub static NSUbiquitousUserDefaultsDidChangeAccountsNotification: &'static NSNotificationName;
}

extern "C" {
    #[cfg(all(feature = "Foundation_NSNotification", feature = "Foundation_NSString"))]
    pub static NSUbiquitousUserDefaultsCompletedInitialSyncNotification:
        &'static NSNotificationName;
}

extern "C" {
    #[cfg(all(feature = "Foundation_NSNotification", feature = "Foundation_NSString"))]
    pub static NSUserDefaultsDidChangeNotification: &'static NSNotificationName;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSWeekDayNameArray: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSShortWeekDayNameArray: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSMonthNameArray: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSShortMonthNameArray: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSTimeFormatString: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSDateFormatString: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSTimeDateFormatString: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSShortTimeDateFormatString: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSCurrencySymbol: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSDecimalSeparator: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSThousandsSeparator: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSDecimalDigits: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSAMPMDesignation: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSHourNameDesignations: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSYearMonthWeekDesignations: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSEarlierTimeDesignations: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSLaterTimeDesignations: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSThisDayDesignations: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSNextDayDesignations: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSNextNextDayDesignations: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSPriorDayDesignations: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSDateTimeOrdering: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSInternationalCurrencyString: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSShortDateFormatString: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSPositiveCurrencyFormatString: &'static NSString;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSNegativeCurrencyFormatString: &'static NSString;
}
