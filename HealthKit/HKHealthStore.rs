//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKHealthStore;

    unsafe impl ClassType for HKHealthStore {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for HKHealthStore {}

unsafe impl Sync for HKHealthStore {}

unsafe impl NSObjectProtocol for HKHealthStore {}

extern_methods!(
    unsafe impl HKHealthStore {
        #[method(isHealthDataAvailable)]
        pub unsafe fn isHealthDataAvailable() -> bool;

        #[method(supportsHealthRecords)]
        pub unsafe fn supportsHealthRecords(&self) -> bool;

        #[cfg(all(feature = "HealthKit_HKDefines", feature = "HealthKit_HKObjectType"))]
        #[method(authorizationStatusForType:)]
        pub unsafe fn authorizationStatusForType(
            &self,
            r#type: &HKObjectType,
        ) -> HKAuthorizationStatus;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSSet",
            feature = "HealthKit_HKObjectType"
        ))]
        #[method(requestAuthorizationToShareTypes:readTypes:completion:)]
        pub unsafe fn requestAuthorizationToShareTypes_readTypes_completion(
            &self,
            types_to_share: Option<&NSSet<HKSampleType>>,
            types_to_read: Option<&NSSet<HKObjectType>>,
            completion: &Block<dyn Fn(Bool, *mut NSError)>,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSPredicate",
            feature = "HealthKit_HKObjectType"
        ))]
        #[method(requestPerObjectReadAuthorizationForType:predicate:completion:)]
        pub unsafe fn requestPerObjectReadAuthorizationForType_predicate_completion(
            &self,
            object_type: &HKObjectType,
            predicate: Option<&NSPredicate>,
            completion: &Block<dyn Fn(Bool, *mut NSError)>,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSSet",
            feature = "HealthKit_HKDefines",
            feature = "HealthKit_HKObjectType"
        ))]
        #[method(getRequestStatusForAuthorizationToShareTypes:readTypes:completion:)]
        pub unsafe fn getRequestStatusForAuthorizationToShareTypes_readTypes_completion(
            &self,
            types_to_share: &NSSet<HKSampleType>,
            types_to_read: &NSSet<HKObjectType>,
            completion: &Block<dyn Fn(HKAuthorizationRequestStatus, *mut NSError)>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(handleAuthorizationForExtensionWithCompletion:)]
        pub unsafe fn handleAuthorizationForExtensionWithCompletion(
            &self,
            completion: &Block<dyn Fn(Bool, *mut NSError)>,
        );

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other earliestPermittedSampleDate)]
        pub unsafe fn earliestPermittedSampleDate(&self) -> Id<NSDate>;

        #[cfg(all(feature = "Foundation_NSError", feature = "HealthKit_HKObject"))]
        #[method(saveObject:withCompletion:)]
        pub unsafe fn saveObject_withCompletion(
            &self,
            object: &HKObject,
            completion: &Block<dyn Fn(Bool, *mut NSError)>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "HealthKit_HKObject"
        ))]
        #[method(saveObjects:withCompletion:)]
        pub unsafe fn saveObjects_withCompletion(
            &self,
            objects: &NSArray<HKObject>,
            completion: &Block<dyn Fn(Bool, *mut NSError)>,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "HealthKit_HKObject"))]
        #[method(deleteObject:withCompletion:)]
        pub unsafe fn deleteObject_withCompletion(
            &self,
            object: &HKObject,
            completion: &Block<dyn Fn(Bool, *mut NSError)>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "HealthKit_HKObject"
        ))]
        #[method(deleteObjects:withCompletion:)]
        pub unsafe fn deleteObjects_withCompletion(
            &self,
            objects: &NSArray<HKObject>,
            completion: &Block<dyn Fn(Bool, *mut NSError)>,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSPredicate",
            feature = "HealthKit_HKObjectType"
        ))]
        #[method(deleteObjectsOfType:predicate:withCompletion:)]
        pub unsafe fn deleteObjectsOfType_predicate_withCompletion(
            &self,
            object_type: &HKObjectType,
            predicate: &NSPredicate,
            completion: &Block<dyn Fn(Bool, NSUInteger, *mut NSError)>,
        );

        #[cfg(feature = "HealthKit_HKQuery")]
        #[method(executeQuery:)]
        pub unsafe fn executeQuery(&self, query: &HKQuery);

        #[cfg(feature = "HealthKit_HKQuery")]
        #[method(stopQuery:)]
        pub unsafe fn stopQuery(&self, query: &HKQuery);

        #[cfg(all(
            feature = "Foundation_NSDate",
            feature = "Foundation_NSError",
            feature = "HealthKit_HKQuantity"
        ))]
        #[deprecated = "No longer supported"]
        #[method(splitTotalEnergy:startDate:endDate:resultsHandler:)]
        pub unsafe fn splitTotalEnergy_startDate_endDate_resultsHandler(
            &self,
            total_energy: &HKQuantity,
            start_date: &NSDate,
            end_date: &NSDate,
            results_handler: &Block<dyn Fn(*mut HKQuantity, *mut HKQuantity, *mut NSError)>,
        );

        #[cfg(all(feature = "Foundation_NSDate", feature = "Foundation_NSError"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other dateOfBirthWithError:_)]
        pub unsafe fn dateOfBirthWithError(&self) -> Result<Id<NSDate>, Id<NSError>>;

        #[cfg(all(feature = "Foundation_NSCalendar", feature = "Foundation_NSError"))]
        #[method_id(@__retain_semantics Other dateOfBirthComponentsWithError:_)]
        pub unsafe fn dateOfBirthComponentsWithError(
            &self,
        ) -> Result<Id<NSDateComponents>, Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "HealthKit_HKCharacteristicObjects"
        ))]
        #[method_id(@__retain_semantics Other biologicalSexWithError:_)]
        pub unsafe fn biologicalSexWithError(
            &self,
        ) -> Result<Id<HKBiologicalSexObject>, Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "HealthKit_HKCharacteristicObjects"
        ))]
        #[method_id(@__retain_semantics Other bloodTypeWithError:_)]
        pub unsafe fn bloodTypeWithError(&self) -> Result<Id<HKBloodTypeObject>, Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "HealthKit_HKCharacteristicObjects"
        ))]
        #[method_id(@__retain_semantics Other fitzpatrickSkinTypeWithError:_)]
        pub unsafe fn fitzpatrickSkinTypeWithError(
            &self,
        ) -> Result<Id<HKFitzpatrickSkinTypeObject>, Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "HealthKit_HKCharacteristicObjects"
        ))]
        #[method_id(@__retain_semantics Other wheelchairUseWithError:_)]
        pub unsafe fn wheelchairUseWithError(
            &self,
        ) -> Result<Id<HKWheelchairUseObject>, Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "HealthKit_HKCharacteristicObjects"
        ))]
        #[method_id(@__retain_semantics Other activityMoveModeWithError:_)]
        pub unsafe fn activityMoveModeWithError(
            &self,
        ) -> Result<Id<HKActivityMoveModeObject>, Id<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HKHealthStore {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// HKWorkout
    unsafe impl HKHealthStore {
        #[cfg(feature = "HealthKit_HKWorkoutSession")]
        #[method(workoutSessionMirroringStartHandler)]
        pub unsafe fn workoutSessionMirroringStartHandler(
            &self,
        ) -> *mut Block<dyn Fn(NonNull<HKWorkoutSession>)>;

        #[cfg(feature = "HealthKit_HKWorkoutSession")]
        #[method(setWorkoutSessionMirroringStartHandler:)]
        pub unsafe fn setWorkoutSessionMirroringStartHandler(
            &self,
            workout_session_mirroring_start_handler: Option<
                &Block<dyn Fn(NonNull<HKWorkoutSession>)>,
            >,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "HealthKit_HKObject",
            feature = "HealthKit_HKSample",
            feature = "HealthKit_HKWorkout"
        ))]
        #[deprecated = "Use HKWorkoutBuilder"]
        #[method(addSamples:toWorkout:completion:)]
        pub unsafe fn addSamples_toWorkout_completion(
            &self,
            samples: &NSArray<HKSample>,
            workout: &HKWorkout,
            completion: &Block<dyn Fn(Bool, *mut NSError)>,
        );

        #[cfg(feature = "HealthKit_HKWorkoutSession")]
        #[deprecated = "Use HKWorkoutSession's start method"]
        #[method(startWorkoutSession:)]
        pub unsafe fn startWorkoutSession(&self, workout_session: &HKWorkoutSession);

        #[cfg(feature = "HealthKit_HKWorkoutSession")]
        #[deprecated = "Use HKWorkoutSession's end method"]
        #[method(endWorkoutSession:)]
        pub unsafe fn endWorkoutSession(&self, workout_session: &HKWorkoutSession);

        #[cfg(feature = "HealthKit_HKWorkoutSession")]
        #[deprecated = "Use HKWorkoutSession's pause method"]
        #[method(pauseWorkoutSession:)]
        pub unsafe fn pauseWorkoutSession(&self, workout_session: &HKWorkoutSession);

        #[cfg(feature = "HealthKit_HKWorkoutSession")]
        #[deprecated = "Use HKWorkoutSession's resume method"]
        #[method(resumeWorkoutSession:)]
        pub unsafe fn resumeWorkoutSession(&self, workout_session: &HKWorkoutSession);

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "HealthKit_HKWorkoutConfiguration"
        ))]
        #[method(startWatchAppWithWorkoutConfiguration:completion:)]
        pub unsafe fn startWatchAppWithWorkoutConfiguration_completion(
            &self,
            workout_configuration: &HKWorkoutConfiguration,
            completion: &Block<dyn Fn(Bool, *mut NSError)>,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "HealthKit_HKWorkoutSession"))]
        #[method(recoverActiveWorkoutSessionWithCompletion:)]
        pub unsafe fn recoverActiveWorkoutSessionWithCompletion(
            &self,
            completion: &Block<dyn Fn(*mut HKWorkoutSession, *mut NSError)>,
        );
    }
);

extern_methods!(
    /// HKBackgroundDelivery
    unsafe impl HKHealthStore {
        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "HealthKit_HKDefines",
            feature = "HealthKit_HKObjectType"
        ))]
        #[method(enableBackgroundDeliveryForType:frequency:withCompletion:)]
        pub unsafe fn enableBackgroundDeliveryForType_frequency_withCompletion(
            &self,
            r#type: &HKObjectType,
            frequency: HKUpdateFrequency,
            completion: &Block<dyn Fn(Bool, *mut NSError)>,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "HealthKit_HKObjectType"))]
        #[method(disableBackgroundDeliveryForType:withCompletion:)]
        pub unsafe fn disableBackgroundDeliveryForType_withCompletion(
            &self,
            r#type: &HKObjectType,
            completion: &Block<dyn Fn(Bool, *mut NSError)>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(disableAllBackgroundDeliveryWithCompletion:)]
        pub unsafe fn disableAllBackgroundDeliveryWithCompletion(
            &self,
            completion: &Block<dyn Fn(Bool, *mut NSError)>,
        );
    }
);

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static HKUserPreferencesDidChangeNotification: &'static NSString;
}

extern_methods!(
    /// HKUserPreferences
    unsafe impl HKHealthStore {
        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSSet",
            feature = "HealthKit_HKObjectType",
            feature = "HealthKit_HKUnit"
        ))]
        #[method(preferredUnitsForQuantityTypes:completion:)]
        pub unsafe fn preferredUnitsForQuantityTypes_completion(
            &self,
            quantity_types: &NSSet<HKQuantityType>,
            completion: &Block<dyn Fn(NonNull<NSDictionary<HKQuantityType, HKUnit>>, *mut NSError)>,
        );
    }
);

extern_methods!(
    /// HKRecalibrateEstimates
    unsafe impl HKHealthStore {
        #[cfg(all(
            feature = "Foundation_NSDate",
            feature = "Foundation_NSError",
            feature = "HealthKit_HKObjectType"
        ))]
        #[method(recalibrateEstimatesForSampleType:atDate:completion:)]
        pub unsafe fn recalibrateEstimatesForSampleType_atDate_completion(
            &self,
            sample_type: &HKSampleType,
            date: &NSDate,
            completion: &Block<dyn Fn(Bool, *mut NSError)>,
        );
    }
);
