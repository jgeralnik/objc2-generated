//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKHealthStore;

    unsafe impl ClassType for HKHealthStore {
        type Super = NSObject;
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

        #[cfg(all(feature = "HKDefines", feature = "HKObjectType"))]
        #[method(authorizationStatusForType:)]
        pub unsafe fn authorizationStatusForType(
            &self,
            r#type: &HKObjectType,
        ) -> HKAuthorizationStatus;

        #[cfg(all(feature = "HKObjectType", feature = "block2"))]
        #[method(requestAuthorizationToShareTypes:readTypes:completion:)]
        pub unsafe fn requestAuthorizationToShareTypes_readTypes_completion(
            &self,
            types_to_share: Option<&NSSet<HKSampleType>>,
            types_to_read: Option<&NSSet<HKObjectType>>,
            completion: &block2::Block<dyn Fn(Bool, *mut NSError)>,
        );

        #[cfg(all(feature = "HKObjectType", feature = "block2"))]
        #[method(requestPerObjectReadAuthorizationForType:predicate:completion:)]
        pub unsafe fn requestPerObjectReadAuthorizationForType_predicate_completion(
            &self,
            object_type: &HKObjectType,
            predicate: Option<&NSPredicate>,
            completion: &block2::Block<dyn Fn(Bool, *mut NSError)>,
        );

        #[cfg(all(feature = "HKDefines", feature = "HKObjectType", feature = "block2"))]
        #[method(getRequestStatusForAuthorizationToShareTypes:readTypes:completion:)]
        pub unsafe fn getRequestStatusForAuthorizationToShareTypes_readTypes_completion(
            &self,
            types_to_share: &NSSet<HKSampleType>,
            types_to_read: &NSSet<HKObjectType>,
            completion: &block2::Block<dyn Fn(HKAuthorizationRequestStatus, *mut NSError)>,
        );

        #[cfg(feature = "block2")]
        #[method(handleAuthorizationForExtensionWithCompletion:)]
        pub unsafe fn handleAuthorizationForExtensionWithCompletion(
            &self,
            completion: &block2::Block<dyn Fn(Bool, *mut NSError)>,
        );

        #[method_id(@__retain_semantics Other earliestPermittedSampleDate)]
        pub unsafe fn earliestPermittedSampleDate(&self) -> Retained<NSDate>;

        #[cfg(all(feature = "HKObject", feature = "block2"))]
        #[method(saveObject:withCompletion:)]
        pub unsafe fn saveObject_withCompletion(
            &self,
            object: &HKObject,
            completion: &block2::Block<dyn Fn(Bool, *mut NSError)>,
        );

        #[cfg(all(feature = "HKObject", feature = "block2"))]
        #[method(saveObjects:withCompletion:)]
        pub unsafe fn saveObjects_withCompletion(
            &self,
            objects: &NSArray<HKObject>,
            completion: &block2::Block<dyn Fn(Bool, *mut NSError)>,
        );

        #[cfg(all(feature = "HKObject", feature = "block2"))]
        #[method(deleteObject:withCompletion:)]
        pub unsafe fn deleteObject_withCompletion(
            &self,
            object: &HKObject,
            completion: &block2::Block<dyn Fn(Bool, *mut NSError)>,
        );

        #[cfg(all(feature = "HKObject", feature = "block2"))]
        #[method(deleteObjects:withCompletion:)]
        pub unsafe fn deleteObjects_withCompletion(
            &self,
            objects: &NSArray<HKObject>,
            completion: &block2::Block<dyn Fn(Bool, *mut NSError)>,
        );

        #[cfg(all(feature = "HKObjectType", feature = "block2"))]
        #[method(deleteObjectsOfType:predicate:withCompletion:)]
        pub unsafe fn deleteObjectsOfType_predicate_withCompletion(
            &self,
            object_type: &HKObjectType,
            predicate: &NSPredicate,
            completion: &block2::Block<dyn Fn(Bool, NSUInteger, *mut NSError)>,
        );

        #[cfg(feature = "HKQuery")]
        #[method(executeQuery:)]
        pub unsafe fn executeQuery(&self, query: &HKQuery);

        #[cfg(feature = "HKQuery")]
        #[method(stopQuery:)]
        pub unsafe fn stopQuery(&self, query: &HKQuery);

        #[cfg(all(feature = "HKQuantity", feature = "block2"))]
        #[deprecated = "No longer supported"]
        #[method(splitTotalEnergy:startDate:endDate:resultsHandler:)]
        pub unsafe fn splitTotalEnergy_startDate_endDate_resultsHandler(
            &self,
            total_energy: &HKQuantity,
            start_date: &NSDate,
            end_date: &NSDate,
            results_handler: &block2::Block<dyn Fn(*mut HKQuantity, *mut HKQuantity, *mut NSError)>,
        );

        #[deprecated]
        #[method_id(@__retain_semantics Other dateOfBirthWithError:_)]
        pub unsafe fn dateOfBirthWithError(&self) -> Result<Retained<NSDate>, Retained<NSError>>;

        #[method_id(@__retain_semantics Other dateOfBirthComponentsWithError:_)]
        pub unsafe fn dateOfBirthComponentsWithError(
            &self,
        ) -> Result<Retained<NSDateComponents>, Retained<NSError>>;

        #[cfg(feature = "HKCharacteristicObjects")]
        #[method_id(@__retain_semantics Other biologicalSexWithError:_)]
        pub unsafe fn biologicalSexWithError(
            &self,
        ) -> Result<Retained<HKBiologicalSexObject>, Retained<NSError>>;

        #[cfg(feature = "HKCharacteristicObjects")]
        #[method_id(@__retain_semantics Other bloodTypeWithError:_)]
        pub unsafe fn bloodTypeWithError(
            &self,
        ) -> Result<Retained<HKBloodTypeObject>, Retained<NSError>>;

        #[cfg(feature = "HKCharacteristicObjects")]
        #[method_id(@__retain_semantics Other fitzpatrickSkinTypeWithError:_)]
        pub unsafe fn fitzpatrickSkinTypeWithError(
            &self,
        ) -> Result<Retained<HKFitzpatrickSkinTypeObject>, Retained<NSError>>;

        #[cfg(feature = "HKCharacteristicObjects")]
        #[method_id(@__retain_semantics Other wheelchairUseWithError:_)]
        pub unsafe fn wheelchairUseWithError(
            &self,
        ) -> Result<Retained<HKWheelchairUseObject>, Retained<NSError>>;

        #[cfg(feature = "HKCharacteristicObjects")]
        #[method_id(@__retain_semantics Other activityMoveModeWithError:_)]
        pub unsafe fn activityMoveModeWithError(
            &self,
        ) -> Result<Retained<HKActivityMoveModeObject>, Retained<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HKHealthStore {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// HKWorkout
    unsafe impl HKHealthStore {
        #[cfg(all(feature = "HKWorkoutSession", feature = "block2"))]
        #[method(workoutSessionMirroringStartHandler)]
        pub unsafe fn workoutSessionMirroringStartHandler(
            &self,
        ) -> *mut block2::Block<dyn Fn(NonNull<HKWorkoutSession>)>;

        #[cfg(all(feature = "HKWorkoutSession", feature = "block2"))]
        #[method(setWorkoutSessionMirroringStartHandler:)]
        pub unsafe fn setWorkoutSessionMirroringStartHandler(
            &self,
            workout_session_mirroring_start_handler: Option<
                &block2::Block<dyn Fn(NonNull<HKWorkoutSession>)>,
            >,
        );

        #[cfg(all(
            feature = "HKObject",
            feature = "HKSample",
            feature = "HKWorkout",
            feature = "block2"
        ))]
        #[deprecated = "Use HKWorkoutBuilder"]
        #[method(addSamples:toWorkout:completion:)]
        pub unsafe fn addSamples_toWorkout_completion(
            &self,
            samples: &NSArray<HKSample>,
            workout: &HKWorkout,
            completion: &block2::Block<dyn Fn(Bool, *mut NSError)>,
        );

        #[cfg(feature = "HKWorkoutSession")]
        #[deprecated = "Use HKWorkoutSession's start method"]
        #[method(startWorkoutSession:)]
        pub unsafe fn startWorkoutSession(&self, workout_session: &HKWorkoutSession);

        #[cfg(feature = "HKWorkoutSession")]
        #[deprecated = "Use HKWorkoutSession's end method"]
        #[method(endWorkoutSession:)]
        pub unsafe fn endWorkoutSession(&self, workout_session: &HKWorkoutSession);

        #[cfg(feature = "HKWorkoutSession")]
        #[deprecated = "Use HKWorkoutSession's pause method"]
        #[method(pauseWorkoutSession:)]
        pub unsafe fn pauseWorkoutSession(&self, workout_session: &HKWorkoutSession);

        #[cfg(feature = "HKWorkoutSession")]
        #[deprecated = "Use HKWorkoutSession's resume method"]
        #[method(resumeWorkoutSession:)]
        pub unsafe fn resumeWorkoutSession(&self, workout_session: &HKWorkoutSession);

        #[cfg(all(feature = "HKWorkoutConfiguration", feature = "block2"))]
        #[method(startWatchAppWithWorkoutConfiguration:completion:)]
        pub unsafe fn startWatchAppWithWorkoutConfiguration_completion(
            &self,
            workout_configuration: &HKWorkoutConfiguration,
            completion: &block2::Block<dyn Fn(Bool, *mut NSError)>,
        );

        #[cfg(all(feature = "HKWorkoutSession", feature = "block2"))]
        #[method(recoverActiveWorkoutSessionWithCompletion:)]
        pub unsafe fn recoverActiveWorkoutSessionWithCompletion(
            &self,
            completion: &block2::Block<dyn Fn(*mut HKWorkoutSession, *mut NSError)>,
        );
    }
);

extern_methods!(
    /// HKBackgroundDelivery
    unsafe impl HKHealthStore {
        #[cfg(all(feature = "HKDefines", feature = "HKObjectType", feature = "block2"))]
        #[method(enableBackgroundDeliveryForType:frequency:withCompletion:)]
        pub unsafe fn enableBackgroundDeliveryForType_frequency_withCompletion(
            &self,
            r#type: &HKObjectType,
            frequency: HKUpdateFrequency,
            completion: &block2::Block<dyn Fn(Bool, *mut NSError)>,
        );

        #[cfg(all(feature = "HKObjectType", feature = "block2"))]
        #[method(disableBackgroundDeliveryForType:withCompletion:)]
        pub unsafe fn disableBackgroundDeliveryForType_withCompletion(
            &self,
            r#type: &HKObjectType,
            completion: &block2::Block<dyn Fn(Bool, *mut NSError)>,
        );

        #[cfg(feature = "block2")]
        #[method(disableAllBackgroundDeliveryWithCompletion:)]
        pub unsafe fn disableAllBackgroundDeliveryWithCompletion(
            &self,
            completion: &block2::Block<dyn Fn(Bool, *mut NSError)>,
        );
    }
);

extern "C" {
    pub static HKUserPreferencesDidChangeNotification: &'static NSString;
}

extern_methods!(
    /// HKUserPreferences
    unsafe impl HKHealthStore {
        #[cfg(all(feature = "HKObjectType", feature = "HKUnit", feature = "block2"))]
        #[method(preferredUnitsForQuantityTypes:completion:)]
        pub unsafe fn preferredUnitsForQuantityTypes_completion(
            &self,
            quantity_types: &NSSet<HKQuantityType>,
            completion: &block2::Block<
                dyn Fn(NonNull<NSDictionary<HKQuantityType, HKUnit>>, *mut NSError),
            >,
        );
    }
);

extern_methods!(
    /// HKRecalibrateEstimates
    unsafe impl HKHealthStore {
        #[cfg(all(feature = "HKObjectType", feature = "block2"))]
        #[method(recalibrateEstimatesForSampleType:atDate:completion:)]
        pub unsafe fn recalibrateEstimatesForSampleType_atDate_completion(
            &self,
            sample_type: &HKSampleType,
            date: &NSDate,
            completion: &block2::Block<dyn Fn(Bool, *mut NSError)>,
        );
    }
);
