#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod etsi_its_cdd {
    extern crate alloc;
    use core::borrow::Borrow;
    use lazy_static::lazy_static;
    use rasn::prelude::*;
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = " Specification of CDD Data Frames:"]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*"]
    #[doc = " * This DF represents an acceleration vector with associated confidence value."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field polarAcceleration: the representation of the acceleration vector in a polar or cylindrical coordinate system. "]
    #[doc = " * "]
    #[doc = " * @field cartesianAcceleration: the representation of the acceleration vector in a cartesian coordinate system."]
    #[doc = " * "]
    #[doc = " * @category: Kinematic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum Acceleration3dWithConfidence {
        polarAcceleration(AccelerationPolarWithZ),
        cartesianAcceleration(AccelerationCartesian),
    }
    #[doc = "*"]
    #[doc = " * This DF represents a acceleration vector in a cartesian coordinate system."]
    #[doc = " "]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field xAcceleration: the x component of the acceleration vector with the associated confidence value."]
    #[doc = " * "]
    #[doc = " * @field yAcceleration: the y component of the acceleration vector with the associated confidence value."]
    #[doc = " *"]
    #[doc = " * @field zAcceleration: the optional z component of the acceleration vector with the associated confidence value."]
    #[doc = " * "]
    #[doc = " * @category: Kinematic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct AccelerationCartesian {
        #[rasn(identifier = "xAcceleration")]
        pub x_acceleration: AccelerationComponent,
        #[rasn(identifier = "yAcceleration")]
        pub y_acceleration: AccelerationComponent,
        #[rasn(identifier = "zAcceleration")]
        pub z_acceleration: Option<AccelerationComponent>,
    }
    impl AccelerationCartesian {
        pub fn new(
            x_acceleration: AccelerationComponent,
            y_acceleration: AccelerationComponent,
            z_acceleration: Option<AccelerationComponent>,
        ) -> Self {
            Self {
                x_acceleration,
                y_acceleration,
                z_acceleration,
            }
        }
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = " Specification of CDD Data Elements: "]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "* "]
    #[doc = " * This DE indicates a change of acceleration."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 - `accelerate` - if the magnitude of the horizontal velocity vector increases."]
    #[doc = " * - 1 - `decelerate` - if the magnitude of the horizontal velocity vector decreases."]
    #[doc = " *"]
    #[doc = " * @category: Kinematic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AccelerationChange {
        accelerate = 0,
        decelerate = 1,
    }
    #[doc = "* "]
    #[doc = " * This DF represents information associated to changes in acceleration. "]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field accelOrDecel: the indication of an acceleration change."]
    #[doc = " *"]
    #[doc = " * @field actionDeltaTime: the period over which the acceleration change action is performed."]
    #[doc = " *"]
    #[doc = " * @category: Kinematic Information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct AccelerationChangeIndication {
        #[rasn(identifier = "accelOrDecel")]
        pub accel_or_decel: AccelerationChange,
        #[rasn(identifier = "actionDeltaTime")]
        pub action_delta_time: DeltaTimeTenthOfSecond,
    }
    impl AccelerationChangeIndication {
        pub fn new(
            accel_or_decel: AccelerationChange,
            action_delta_time: DeltaTimeTenthOfSecond,
        ) -> Self {
            Self {
                accel_or_decel,
                action_delta_time,
            }
        }
    }
    #[doc = "* "]
    #[doc = " * This DF represents an acceleration component along with a confidence value."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field value: the value of the acceleration component which can be estimated as the mean of the current distribution."]
    #[doc = " *"]
    #[doc = " * @field confidence: the confidence value associated to the provided value."]
    #[doc = " *"]
    #[doc = " * @category: Kinematic Information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct AccelerationComponent {
        pub value: AccelerationValue,
        pub confidence: AccelerationConfidence,
    }
    impl AccelerationComponent {
        pub fn new(value: AccelerationValue, confidence: AccelerationConfidence) -> Self {
            Self { value, confidence }
        }
    }
    #[doc = "*"]
    #[doc = " * This DE indicates the acceleration confidence value which represents the estimated absolute accuracy of an acceleration value with a default confidence level of 95 %. "]
    #[doc = " * If required, the confidence level can be defined by the corresponding standards applying this DE."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `n` (`n > 0` and `n < 101`) if the confidence value is equal to or less than n x 0,1 m/s^2, and greater than (n-1) x 0,1 m/s^2,"]
    #[doc = " * - `101` if the confidence value is out of range i.e. greater than 10 m/s^2,"]
    #[doc = " * - `102` if the confidence value is unavailable."]
    #[doc = " *"]
    #[doc = " * The value 0 shall not be used."]
    #[doc = " *"]
    #[doc = " * @note: The fact that an acceleration value is received with confidence value set to `unavailable(102)` can be caused by several reasons, such as:"]
    #[doc = " * - the sensor cannot deliver the accuracy at the defined confidence level because it is a low-end sensor,"]
    #[doc = " * - the sensor cannot calculate the accuracy due to lack of variables, or"]
    #[doc = " * - there has been a vehicle bus (e.g. CAN bus) error."]
    #[doc = " * In all 3 cases above, the acceleration value may be valid and used by the application."]
    #[doc = " * "]
    #[doc = " * @note: If an acceleration value is received and its confidence value is set to `outOfRange(101)`, it means that the value is not valid and therefore cannot be trusted. Such value is not useful for the application."]
    #[doc = " *"]
    #[doc = " * @unit 0,1 m/s^2"]
    #[doc = " * @category: Kinematic information"]
    #[doc = " * @revision: Description revised in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=102"))]
    pub struct AccelerationConfidence(pub u8);
    #[doc = "*"]
    #[doc = " * This DE indicates the status of the controlling mechanisms for longitudinal movement of the vehicle."]
    #[doc = " * The data may be provided via the in-vehicle network. It indicates whether a specific in-vehicle"]
    #[doc = " * acceleration control system is engaged or not. "]
    #[doc = " *"]
    #[doc = " * The corresponding bit shall be set to 1 under the following conditions:"]
    #[doc = " * - 0 - `brakePedalEngaged`      - Driver is stepping on the brake pedal,"]
    #[doc = " * - 1 - `gasPedalEngaged`        - Driver is stepping on the gas pedal,"]
    #[doc = " * - 2 - `emergencyBrakeEngaged`  - emergency brake system is engaged,"]
    #[doc = " * - 3 - `collisionWarningEngaged`- collision warning system is engaged,"]
    #[doc = " * - 4 - `accEngaged`             - ACC is engaged,"]
    #[doc = " * - 5 - `cruiseControlEngaged`   - cruise control is engaged,"]
    #[doc = " * - 6 - `speedLimiterEngaged`    - speed limiter is engaged."]
    #[doc = " *"]
    #[doc = " * Otherwise (for example when the corresponding system is not available due to non equipped system"]
    #[doc = " * or information is unavailable), the corresponding bit shall be set to 0."]
    #[doc = " *"]
    #[doc = " * @note: The system engagement condition is OEM specific and therefore out of scope of the present document."]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: V1.3.1, description revised in V2.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct AccelerationControl(pub FixedBitString<7usize>);
    #[doc = "*"]
    #[doc = " * This DE represents the extension of DE AccelerationControl and should only be used together with DE AccelerationControl."]
    #[doc = " *"]
    #[doc = " * The corresponding bit shall be set to 1 under the following conditions:"]
    #[doc = " * - 0  - `rearCrossTrafficAlertEngaged`       - rear cross traffic alert system is engaged"]
    #[doc = " * - 1  - `emergencyBrakeRearEngaged`          - emergency brake system for rear driving is engaged"]
    #[doc = " * - 2  - `assistedParkingLongitudinalEngaged` - assisted parking system (longitudinal control) is engaged"]
    #[doc = " *"]
    #[doc = " * Otherwise (for example when the corresponding system is not available due to non-equipped system "]
    #[doc = " * or information is unavailable), the corresponding bit shall be set to 0. "]
    #[doc = " *"]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: Created in V2.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("3", extensible))]
    pub struct AccelerationControlExtension(pub BitString);
    #[doc = "*"]
    #[doc = " * This DF represents the magnitude of the acceleration vector and associated confidence value."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field accelerationMagnitudeValue: the magnitude of the acceleration vector."]
    #[doc = " * "]
    #[doc = " * @field accelerationConfidence: the confidence value of the magnitude value."]
    #[doc = " *"]
    #[doc = " * @category: Kinematic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct AccelerationMagnitude {
        #[rasn(identifier = "accelerationMagnitudeValue")]
        pub acceleration_magnitude_value: AccelerationMagnitudeValue,
        #[rasn(identifier = "accelerationConfidence")]
        pub acceleration_confidence: AccelerationConfidence,
    }
    impl AccelerationMagnitude {
        pub fn new(
            acceleration_magnitude_value: AccelerationMagnitudeValue,
            acceleration_confidence: AccelerationConfidence,
        ) -> Self {
            Self {
                acceleration_magnitude_value,
                acceleration_confidence,
            }
        }
    }
    #[doc = "* "]
    #[doc = " * This DE represents the magnitude of the acceleration vector in a defined coordinate system."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `0` to indicate no acceleration,"]
    #[doc = " * - `n` (`n > 0` and `n < 160`) to indicate acceleration equal to or less than n x 0,1 m/s^2, and greater than (n-1) x 0,1 m/s^2,"]
    #[doc = " * - `160` for acceleration values greater than 15,9 m/s^2,"]
    #[doc = " * - `161` when the data is unavailable."]
    #[doc = " *"]
    #[doc = " * @unit 0,1 m/s^2"]
    #[doc = " * @category: Kinematic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=161"))]
    pub struct AccelerationMagnitudeValue(pub u8);
    #[doc = "*"]
    #[doc = " * This DF represents an acceleration vector in a polar or cylindrical coordinate system."]
    #[doc = " "]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field accelerationMagnitude: magnitude of the acceleration vector projected onto the reference plane, with the associated confidence value."]
    #[doc = " * "]
    #[doc = " * @field accelerationDirection: polar angle of the acceleration vector projected onto the reference plane, with the associated confidence value."]
    #[doc = " *"]
    #[doc = " * @field zAcceleration: the optional z component of the acceleration vector along the reference axis of the cylindrical coordinate system, with the associated confidence value."]
    #[doc = " * "]
    #[doc = " * @category: Kinematic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct AccelerationPolarWithZ {
        #[rasn(identifier = "accelerationMagnitude")]
        pub acceleration_magnitude: AccelerationMagnitude,
        #[rasn(identifier = "accelerationDirection")]
        pub acceleration_direction: CartesianAngle,
        #[rasn(identifier = "zAcceleration")]
        pub z_acceleration: Option<AccelerationComponent>,
    }
    impl AccelerationPolarWithZ {
        pub fn new(
            acceleration_magnitude: AccelerationMagnitude,
            acceleration_direction: CartesianAngle,
            z_acceleration: Option<AccelerationComponent>,
        ) -> Self {
            Self {
                acceleration_magnitude,
                acceleration_direction,
                z_acceleration,
            }
        }
    }
    #[doc = "* "]
    #[doc = " * This DE represents the value of an acceleration component in a defined coordinate system."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `-160` for acceleration values equal to or less than -16 m/s^2,"]
    #[doc = " * - `n` (`n > -160` and `n <= 0`) to indicate negative acceleration equal to or less than n x 0,1 m/s^2, and greater than (n-1) x 0,1 m/s^2,"]
    #[doc = " * - `n` (`n > 0` and `n < 160`) to indicate positive acceleration equal to or less than n x 0,1 m/s^2, and greater than (n-1) x 0,1 m/s^2,"]
    #[doc = " * - `160` for acceleration values greater than 15,9 m/s^2,"]
    #[doc = " * - `161` when the data is unavailable."]
    #[doc = " *"]
    #[doc = " * @note: the formula for values > -160 and <160 results in rounding up to the next value. Zero acceleration is indicated using n=0."]
    #[doc = " * @unit 0,1 m/s^2"]
    #[doc = " * @category: Kinematic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-160..=161"))]
    pub struct AccelerationValue(pub i16);
    #[doc = "*"]
    #[doc = " * This DE indicates an access technology."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `0`: in case of any access technology class,"]
    #[doc = " * - `1`: in case of ITS-G5 access technology class,"]
    #[doc = " * - `2`: in case of LTE-V2X access technology class,"]
    #[doc = " * - `3`: in case of NR-V2X access technology class."]
    #[doc = " * "]
    #[doc = " * @category: Communication information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum AccessTechnologyClass {
        any = 0,
        itsg5Class = 1,
        ltev2xClass = 2,
        nrv2xClass = 3,
    }
    #[doc = "*"]
    #[doc = " * This DE represents the value of the sub cause code of the @ref CauseCode `accident`."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 - `unavailable`                        - in case the information on the sub cause of the accident is unavailable,"]
    #[doc = " * - 1 - `multiVehicleAccident`               - in case more than two vehicles are involved in accident,"]
    #[doc = " * - 2 - `heavyAccident`                      - in case the airbag of the vehicle involved in the accident is triggered, "]
    #[doc = " *                                              the accident requires important rescue and/or recovery work,"]
    #[doc = " * - 3 - `accidentInvolvingLorry`             - in case the accident involves a lorry,"]
    #[doc = " * - 4 - `accidentInvolvingBus`               - in case the accident involves a bus,"]
    #[doc = " * - 5 - `accidentInvolvingHazardousMaterials`- in case the accident involves hazardous material,"]
    #[doc = " * - 6 - `accidentOnOppositeLane-deprecated`  - deprecated,"]
    #[doc = " * - 7 - `unsecuredAccident`                  - in case the accident is not secured,"]
    #[doc = " * - 8 - `assistanceRequested`                - in case rescue and assistance are requested,"]
    #[doc = " * - 9-255                                    - reserved for future usage. "]
    #[doc = " *"]
    #[doc = " * @category: Traffic information"]
    #[doc = " * @revision: V1.3.1, value 6 deprecated in V2.4.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct AccidentSubCauseCode(pub u8);
    #[doc = "*"]
    #[doc = " * This DF represents an identifier used to describe a protocol action taken by an ITS-S."]
    #[doc = " * "]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field originatingStationId: Id of the ITS-S that takes the action. "]
    #[doc = " * "]
    #[doc = " * @field sequenceNumber: a sequence number. "]
    #[doc = " * "]
    #[doc = " * @note: this DF is kept for backwards compatibility reasons only. It is recommended to use the @ref ActionId instead. "]
    #[doc = " * @category: Communication information"]
    #[doc = " * @revision: V1.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct ActionID {
        #[rasn(identifier = "originatingStationId")]
        pub originating_station_id: StationID,
        #[rasn(identifier = "sequenceNumber")]
        pub sequence_number: SequenceNumber,
    }
    impl ActionID {
        pub fn new(originating_station_id: StationID, sequence_number: SequenceNumber) -> Self {
            Self {
                originating_station_id,
                sequence_number,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DF represents an identifier used to describe a protocol action taken by an ITS-S."]
    #[doc = " * "]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field originatingStationId: Id of the ITS-S that takes the action. "]
    #[doc = " * "]
    #[doc = " * @field sequenceNumber: a sequence number. "]
    #[doc = " * "]
    #[doc = " * @category: Communication information"]
    #[doc = " * @revision: Created in V2.1.1 based on @ref ActionID."]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct ActionId {
        #[rasn(identifier = "originatingStationId")]
        pub originating_station_id: StationId,
        #[rasn(identifier = "sequenceNumber")]
        pub sequence_number: SequenceNumber,
    }
    impl ActionId {
        pub fn new(originating_station_id: StationId, sequence_number: SequenceNumber) -> Self {
            Self {
                originating_station_id,
                sequence_number,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DF shall contain a list of @ref ActionId. "]
    #[doc = ""]
    #[doc = " * @category: Communication Information"]
    #[doc = " * @revision: Created in V2.1.1 based on ReferenceDenms from DENM Release 1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8", extensible))]
    pub struct ActionIdList(pub SequenceOf<ActionId>);
    #[doc = "*"]
    #[doc = " * This DE represents the value of the sub cause code of the @ref CauseCode `adhesion`. "]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 - `unavailable`     - in case information on the cause of the low road adhesion is unavailable,"]
    #[doc = " * - 1 - `heavyFrostOnRoad`- in case the low road adhesion is due to heavy frost on the road,"]
    #[doc = " * - 2 - `fuelOnRoad`      - in case the low road adhesion is due to fuel on the road,"]
    #[doc = " * - 3 - `mudOnRoad`       - in case the low road adhesion is due to mud on the road,"]
    #[doc = " * - 4 - `snowOnRoad`      - in case the low road adhesion is due to snow on the road,"]
    #[doc = " * - 5 - `iceOnRoad`       - in case the low road adhesion is due to ice on the road,"]
    #[doc = " * - 6 - `blackIceOnRoad`  - in case the low road adhesion is due to black ice on the road,"]
    #[doc = " * - 7 - `oilOnRoad`       - in case the low road adhesion is due to oil on the road,"]
    #[doc = " * - 8 - `looseChippings`  - in case the low road adhesion is due to loose gravel or stone fragments on the road,"]
    #[doc = " * - 9 - `instantBlackIce` - in case the low road adhesion is due to instant black ice on the road surface,"]
    #[doc = " * - 10 - `roadsSalted`    - when the low road adhesion is due to salted road,"]
    #[doc = " * - 11 - `flooding`       - in case low road adhesion is due to flooding of the road,"]
    #[doc = " * - 12 - `waterOnRoad`    - in case low road adhesion is due to a shallow layer of standing water on the road (not flooding). "]
    #[doc = " * - 12-255                - are reserved for future usage."]
    #[doc = " *"]
    #[doc = " * @category: Traffic information"]
    #[doc = " * @revision: V1.3.1, name changed to AdhesionSubCauseCode in V2.4.1, value 11 moved from hazardousLocation-SurfaceCondition to this DE and value 12 added in V2.4.1 "]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct AdhesionSubCauseCode(pub u8);
    #[doc = "*"]
    #[doc = " * This DE represents the value of the sub cause codes of the @ref CauseCode `adverseWeatherCondition-Precipitation`. "]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 - `unavailable`   - in case information on the type of precipitation is unavailable,"]
    #[doc = " * - 1 - `rain`          - in case the type of precipitation is rain,"]
    #[doc = " * - 2 - `snowfall`      - in case the type of precipitation is snowfall,"]
    #[doc = " * - 3 - `hail`          - in case the type of precipitation is hail."]
    #[doc = " * - 4-255               - are reserved for future usage"]
    #[doc = " *"]
    #[doc = " * @category: Traffic information"]
    #[doc = " * @revision: V1.3.1, values 1,2,3 renamed in V2.4.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        identifier = "AdverseWeatherCondition-PrecipitationSubCauseCode",
        value("0..=255")
    )]
    pub struct AdverseWeatherConditionPrecipitationSubCauseCode(pub u8);
    #[doc = "*"]
    #[doc = " * This DE represents the value of the sub cause codes of the @ref CauseCode `adverseWeatherCondition-Visibility`."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 - `unavailable`    - in case information on the cause of low visibility is unavailable,"]
    #[doc = " * - 1 - `fog`            - in case the cause of low visibility is fog,"]
    #[doc = " * - 2 - `smoke`          - in case the cause of low visibility is smoke,"]
    #[doc = " * - 3 - `snowfall`       - in case the cause of low visibility is snow fall,"]
    #[doc = " * - 4 - `rain`           - in case the cause of low visibility is rain,"]
    #[doc = " * - 5 - `hail`           - in case the cause of low visibility is hail,"]
    #[doc = " * - 6 - `lowSunGlare`    - in case the cause of low visibility is sun glare,"]
    #[doc = " * - 7 - `sandstorms`     - in case the cause of low visibility is sand storm,"]
    #[doc = " * - 8 - `swarmsOfInsects`- in case the cause of low visibility is swarm of insects."]
    #[doc = " * - 9-255                - are reserved for future usage"]
    #[doc = " *"]
    #[doc = " * @category: Traffic information"]
    #[doc = " * @revision: V1.3.1, values 3, 4, 5 renamed in V2.4.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        identifier = "AdverseWeatherCondition-VisibilitySubCauseCode",
        value("0..=255")
    )]
    pub struct AdverseWeatherConditionVisibilitySubCauseCode(pub u8);
    #[doc = "*"]
    #[doc = " * This DE represents the value of the sub cause codes of the @ref CauseCode `adverseWeatherCondition-Wind`."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 - `unavailable`             - in case information on the type of wind is unavailable,"]
    #[doc = " * - 1 - `strongWinds`             - in case the type of wind is strong wind such as gale or storm (e.g. Beaufort scale number 9-11),"]
    #[doc = " * - 2 - `damagingHail-deprecated` - deprecated since not representing a wind related event,"]
    #[doc = " * - 3 - `hurricane`               - in case the type of storm is hurricane (e.g. Beaufort scale number 12),"]
    #[doc = " * - 4 - `thunderstorm`            - in case the type of storm is thunderstorm,"]
    #[doc = " * - 5 - `tornado`                 - in case the type of storm is tornado,"]
    #[doc = " * - 6 - `blizzard`                - in case the type of storm is blizzard."]
    #[doc = " * - 7-255                         - are reserved for future usage."]
    #[doc = " *"]
    #[doc = " * @category: Traffic information"]
    #[doc = " * @revision: V1.3.1, DE renamed in V2.4.1, value 2 deprecated, description of value 1 and 3  amended in V2.4.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        identifier = "AdverseWeatherCondition-WindSubCauseCode",
        value("0..=255")
    )]
    pub struct AdverseWeatherConditionWindSubCauseCode(pub u8);
    #[doc = "*"]
    #[doc = " * This DE represents the air humidity in tenths of percent."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `n` (`n > 0` and `n < 1001`) indicates that the applicable value is equal to or less than n x 0,1 percent and greater than (n-1) x 0,1 percent."]
    #[doc = " * - `1001` indicates that the air humidity is unavailable."]
    #[doc = " *"]
    #[doc = " * @category: Basic information"]
    #[doc = " * @unit: 0,1 % "]
    #[doc = " * @revision: created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=1001"))]
    pub struct AirHumidity(pub u16);
    #[doc = "*"]
    #[doc = " * This DF provides the altitude and confidence level of an altitude information in a WGS84 coordinate system."]
    #[doc = " * The specific WGS84 coordinate system is specified by the corresponding standards applying this DE."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field altitudeValue: altitude of a geographical point."]
    #[doc = " *"]
    #[doc = " * @field altitudeConfidence: confidence level of the altitudeValue."]
    #[doc = " *"]
    #[doc = " * @note: this DF is kept for backwards compatibility reasons only. It is recommended to use the @ref AltitudeWithConfidence instead. "]
    #[doc = " * @category: GeoReference information"]
    #[doc = " * @revision: Description revised in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct Altitude {
        #[rasn(identifier = "altitudeValue")]
        pub altitude_value: AltitudeValue,
        #[rasn(identifier = "altitudeConfidence")]
        pub altitude_confidence: AltitudeConfidence,
    }
    impl Altitude {
        pub fn new(altitude_value: AltitudeValue, altitude_confidence: AltitudeConfidence) -> Self {
            Self {
                altitude_value,
                altitude_confidence,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DE indicates the altitude confidence value which represents the estimated absolute accuracy of an altitude value of a geographical point with a default confidence level of 95 %."]
    #[doc = " * If required, the confidence level can be defined by the corresponding standards applying this DE."]
    #[doc = " *"]
    #[doc = " * The value shall be set to: "]
    #[doc = " *   - 0  - `alt-000-01`   - if the confidence value is equal to or less than 0,01 metre,"]
    #[doc = " *   - 1  - `alt-000-02`   - if the confidence value is equal to or less than 0,02 metre and greater than 0,01 metre,"]
    #[doc = " *   - 2  - `alt-000-05`   - if the confidence value is equal to or less than 0,05 metre and greater than 0,02 metre,            "]
    #[doc = " *   - 3  - `alt-000-10`   - if the confidence value is equal to or less than 0,1 metre and greater than 0,05 metre,            "]
    #[doc = " *   - 4  - `alt-000-20`   - if the confidence value is equal to or less than 0,2 metre and greater than 0,1 metre,            "]
    #[doc = " *   - 5  - `alt-000-50`   - if the confidence value is equal to or less than 0,5 metre and greater than 0,2 metre,             "]
    #[doc = " *   - 6  - `alt-001-00`   - if the confidence value is equal to or less than 1 metre and greater than 0,5 metre,             "]
    #[doc = " *   - 7  - `alt-002-00`   - if the confidence value is equal to or less than 2 metres and greater than 1 metre,             "]
    #[doc = " *   - 8  - `alt-005-00`   - if the confidence value is equal to or less than 5 metres and greater than 2 metres,              "]
    #[doc = " *   - 9  - `alt-010-00`   - if the confidence value is equal to or less than 10 metres and greater than 5 metres,             "]
    #[doc = " *   - 10 - `alt-020-00`   - if the confidence value is equal to or less than 20 metres and greater than 10 metres,            "]
    #[doc = " *   - 11 - `alt-050-00`   - if the confidence value is equal to or less than 50 metres and greater than 20 metres,            "]
    #[doc = " *   - 12 - `alt-100-00`   - if the confidence value is equal to or less than 100 metres and greater than 50 metres,           "]
    #[doc = " *   - 13 - `alt-200-00`   - if the confidence value is equal to or less than 200 metres and greater than 100 metres,           "]
    #[doc = " *   - 14 - `outOfRange`   - if the confidence value is out of range, i.e. greater than 200 metres,"]
    #[doc = " *   - 15 - `unavailable`  - if the confidence value is unavailable.       "]
    #[doc = " *"]
    #[doc = " * @note: The fact that an altitude value is received with confidence value set to `unavailable(15)` can be caused"]
    #[doc = " * by several reasons, such as:"]
    #[doc = " * - the sensor cannot deliver the accuracy at the defined confidence level because it is a low-end sensor,"]
    #[doc = " * - the sensor cannot calculate the accuracy due to lack of variables, or"]
    #[doc = " * - there has been a vehicle bus (e.g. CAN bus) error."]
    #[doc = " * In all 3 cases above, the altitude value may be valid and used by the application."]
    #[doc = " * "]
    #[doc = " * @note: If an altitude value is received and its confidence value is set to `outOfRange(14)`, it means that the  "]
    #[doc = " * altitude value is not valid and therefore cannot be trusted. Such value is not useful for the application.             "]
    #[doc = " *"]
    #[doc = " * @category: GeoReference information"]
    #[doc = " * @revision: Description revised in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AltitudeConfidence {
        #[rasn(identifier = "alt-000-01")]
        alt_000_01 = 0,
        #[rasn(identifier = "alt-000-02")]
        alt_000_02 = 1,
        #[rasn(identifier = "alt-000-05")]
        alt_000_05 = 2,
        #[rasn(identifier = "alt-000-10")]
        alt_000_10 = 3,
        #[rasn(identifier = "alt-000-20")]
        alt_000_20 = 4,
        #[rasn(identifier = "alt-000-50")]
        alt_000_50 = 5,
        #[rasn(identifier = "alt-001-00")]
        alt_001_00 = 6,
        #[rasn(identifier = "alt-002-00")]
        alt_002_00 = 7,
        #[rasn(identifier = "alt-005-00")]
        alt_005_00 = 8,
        #[rasn(identifier = "alt-010-00")]
        alt_010_00 = 9,
        #[rasn(identifier = "alt-020-00")]
        alt_020_00 = 10,
        #[rasn(identifier = "alt-050-00")]
        alt_050_00 = 11,
        #[rasn(identifier = "alt-100-00")]
        alt_100_00 = 12,
        #[rasn(identifier = "alt-200-00")]
        alt_200_00 = 13,
        outOfRange = 14,
        unavailable = 15,
    }
    #[doc = "*"]
    #[doc = " * This DE represents the altitude value in a WGS84 coordinate system."]
    #[doc = " * The specific WGS84 coordinate system is specified by the corresponding standards applying this DE."]
    #[doc = " *"]
    #[doc = " * The value shall be set to: "]
    #[doc = " * - `-100 000` if the altitude is equal to or less than -1 000 m,"]
    #[doc = " * - `n` (`n > -100 000` and `n < 800 000`) if the altitude is equal to or less than n  x 0,01 metre and greater than (n-1) x 0,01 metre,"]
    #[doc = " * - `800 000` if the altitude  greater than 7 999,99 m,"]
    #[doc = " * - `800 001` if the information is not available."]
    #[doc = " *"]
    #[doc = " * @note: the range of this DE does not use the full binary encoding range, but all reasonable values are covered. In order to cover all possible altitude ranges a larger encoding would be necessary."]
    #[doc = " * @unit: 0,01 metre"]
    #[doc = " * @category: GeoReference information"]
    #[doc = " * @revision: Description revised in V2.1.1 (definition of 800 000 has slightly changed) "]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-100000..=800001"))]
    pub struct AltitudeValue(pub i32);
    #[doc = "* "]
    #[doc = " * This DE indicates the angle confidence value which represents the estimated absolute accuracy of an angle value with a default confidence level of 95 %."]
    #[doc = " * If required, the confidence level can be defined by the corresponding standards applying this DE."]
    #[doc = " *"]
    #[doc = " * The value shall be set to: "]
    #[doc = " * - `n` (`n > 0` and `n < 126`)  if the accuracy is equal to or less than n * 0,1 degrees and greater than (n-1) x * 0,1 degrees,"]
    #[doc = " * - `126` if the  accuracy is out of range, i.e. greater than 12,5 degrees,"]
    #[doc = " * - `127` if the accuracy information is not available."]
    #[doc = " *"]
    #[doc = " * @unit: 0,1 degrees"]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=127"))]
    pub struct AngleConfidence(pub u8);
    #[doc = "* "]
    #[doc = " * This DE indicates the angular acceleration confidence value which represents the estimated accuracy of an angular acceleration value with a default confidence level of 95 %."]
    #[doc = " * If required, the confidence level can be defined by the corresponding standards applying this DE."]
    #[doc = " * For correlation computation, maximum interval levels shall be assumed."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 - `degSecSquared-01` - if the accuracy is equal to or less than 1 degree/second^2,"]
    #[doc = " * - 1 - `degSecSquared-02` - if the accuracy is equal to or less than 2 degrees/second^2 and greater than 1 degree/second^2,"]
    #[doc = " * - 2 - `degSecSquared-05` - if the accuracy is equal to or less than 5 degrees/second^2 and greater than 1 degree/second^2,"]
    #[doc = " * - 3 - `degSecSquared-10` - if the accuracy is equal to or less than 10 degrees/second^2 and greater than 5 degrees/second^2,"]
    #[doc = " * - 4 - `degSecSquared-20` - if the accuracy is equal to or less than 20 degrees/second^2 and greater than 10 degrees/second^2,"]
    #[doc = " * - 5 - `degSecSquared-50` - if the accuracy is equal to or less than 50 degrees/second^2 and greater than 20 degrees/second^2,"]
    #[doc = " * - 6 - `outOfRange`       - if the accuracy is out of range, i.e. greater than 50 degrees/second^2,"]
    #[doc = " * - 7 - `unavailable`      - if the accuracy information is unavailable."]
    #[doc = " *"]
    #[doc = " * @category: Kinematic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AngularAccelerationConfidence {
        #[rasn(identifier = "degSecSquared-01")]
        degSecSquared_01 = 0,
        #[rasn(identifier = "degSecSquared-02")]
        degSecSquared_02 = 1,
        #[rasn(identifier = "degSecSquared-05")]
        degSecSquared_05 = 2,
        #[rasn(identifier = "degSecSquared-10")]
        degSecSquared_10 = 3,
        #[rasn(identifier = "degSecSquared-20")]
        degSecSquared_20 = 4,
        #[rasn(identifier = "degSecSquared-50")]
        degSecSquared_50 = 5,
        outOfRange = 6,
        unavailable = 7,
    }
    #[doc = "* "]
    #[doc = " * This DE indicates the angular speed confidence value which represents the estimated absolute accuracy of an angular speed value with a default confidence level of 95 %."]
    #[doc = " * If required, the confidence level can be defined by the corresponding standards applying this DE."]
    #[doc = " * For correlation computation, maximum interval levels can be assumed."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 - `degSec-01`   - if the accuracy is equal to or less than 1 degree/second,"]
    #[doc = " * - 1 - `degSec-02`   - if the accuracy is equal to or less than 2 degrees/second and greater than 1 degree/second,"]
    #[doc = " * - 2 - `degSec-05`   - if the accuracy is equal to or less than 5 degrees/second and greater than 2 degrees/second,"]
    #[doc = " * - 3 - `degSec-10`   - if the accuracy is equal to or less than 10 degrees/second and greater than 5 degrees/second,"]
    #[doc = " * - 4 - `degSec-20`   - if the accuracy is equal to or less than 20 degrees/second and greater than 10 degrees/second,"]
    #[doc = " * - 5 - `degSec-50`   - if the accuracy is equal to or less than 50 degrees/second and greater than 20 degrees/second,"]
    #[doc = " * - 6 - `outOfRange`  - if the accuracy is out of range, i.e. greater than 50 degrees/second,"]
    #[doc = " * - 7 - `unavailable` - if the accuracy information is unavailable."]
    #[doc = " * "]
    #[doc = " * @category: Kinematic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AngularSpeedConfidence {
        #[rasn(identifier = "degSec-01")]
        degSec_01 = 0,
        #[rasn(identifier = "degSec-02")]
        degSec_02 = 1,
        #[rasn(identifier = "degSec-05")]
        degSec_05 = 2,
        #[rasn(identifier = "degSec-10")]
        degSec_10 = 3,
        #[rasn(identifier = "degSec-20")]
        degSec_20 = 4,
        #[rasn(identifier = "degSec-50")]
        degSec_50 = 5,
        outOfRange = 6,
        unavailable = 7,
    }
    #[doc = "*"]
    #[doc = " * This DE indicates the status of the controlling mechanisms for the lateral and combined lateral and longitudinal movements of the vehicle."]
    #[doc = " * The data may be provided via the in-vehicle network. It indicates whether a specific in-vehicle"]
    #[doc = " * acceleration control system combined with steering of the direction of the vehicle is engaged or not. "]
    #[doc = " *"]
    #[doc = " * The corresponding bit shall be set to 1 under the following conditions:"]
    #[doc = " * - 0  - `emergencySteeringSystemEngaged`       - emergency steering system is engaged,"]
    #[doc = " * - 1  - `autonomousEmergencySteeringEngaged`   - autonomous emergency steering system is engaged,"]
    #[doc = " * - 2  - `automaticLaneChangeEngaged`           - automatic lane change system is engaged,"]
    #[doc = " * - 3  - `laneKeepingAssistEngaged`             - lane keeping assist is engaged,"]
    #[doc = " * - 4  - `assistedParkingLateralEngaged`        - assisted parking system (lateral  control) is engaged,"]
    #[doc = " * - 5  - `emergencyAssistEngaged`               - emergency assist (lateral and longitudinal control) is engaged."]
    #[doc = " *"]
    #[doc = " * Otherwise (for example when the corresponding system is not available due to non-equipped system or information is unavailable), "]
    #[doc = " * the corresponding bit shall be set to 0. "]
    #[doc = " *"]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: Created in V2.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("6", extensible))]
    pub struct AutomationControl(pub BitString);
    #[doc = "*"]
    #[doc = " * This DE indicates the number of axles of a passing train."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `n` (`n > 2` and `n < 1001`) indicates that the train has n x axles,"]
    #[doc = " * - `1001`indicates that the number of axles is out of range,"]
    #[doc = " * - `1002` the information is unavailable."]
    #[doc = " *"]
    #[doc = " * "]
    #[doc = " * @unit: Number of axles"]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("2..=1002"))]
    pub struct AxlesCount(pub u16);
    #[doc = "*"]
    #[doc = " * This DE represents the measured uncompensated atmospheric pressure."]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `2999` indicates that the applicable value is less than 29990 Pa,"]
    #[doc = " * - `n` (`n > 2999` and `n <= 12000`) indicates that the applicable value is equal to or less than n x 10 Pa and greater than (n-1) x 10 Pa, "]
    #[doc = " * - `12001` indicates that the values is greater than 120000 Pa,"]
    #[doc = " * - `12002` indicates that the information is not available."]
    #[doc = " *"]
    #[doc = " * @category: Basic information"]
    #[doc = " * @unit: 10 Pascal"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("2999..=12002"))]
    pub struct BarometricPressure(pub u16);
    #[doc = "* "]
    #[doc = " * This DE represents a general container for usage in various types of messages."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field stationType: the type of technical context in which the ITS-S that has generated the message is integrated in."]
    #[doc = " *"]
    #[doc = " * @field referencePosition: the reference position of the station that has generated the message that contains the basic container."]
    #[doc = " *"]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct BasicContainer {
        #[rasn(identifier = "stationType")]
        pub station_type: TrafficParticipantType,
        #[rasn(identifier = "referencePosition")]
        pub reference_position: ReferencePositionWithConfidence,
    }
    impl BasicContainer {
        pub fn new(
            station_type: TrafficParticipantType,
            reference_position: ReferencePositionWithConfidence,
        ) -> Self {
            Self {
                station_type,
                reference_position,
            }
        }
    }
    #[doc = "* "]
    #[doc = " * This DF provides information about the configuration of a road section in terms of lanes using a list of @ref LanePositionAndType ."]
    #[doc = " *"]
    #[doc = " * @category: Road topology information"]
    #[doc = " * @revision: Created in V2.2.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16", extensible))]
    pub struct BasicLaneConfiguration(pub SequenceOf<BasicLaneInformation>);
    #[doc = "* "]
    #[doc = " * This DF provides basic information about a single lane of a road segment."]
    #[doc = " * It includes the following components: "]
    #[doc = " * "]
    #[doc = " * @field laneNumber: the number associated to the lane that provides a transversal identification. "]
    #[doc = " * "]
    #[doc = " * @field direction: the direction of traffic flow allowed on the lane. "]
    #[doc = " * "]
    #[doc = " * @field laneWidth: the optional width of the lane."]
    #[doc = " *"]
    #[doc = " * @field connectingLane: the number of the connecting lane in the next road section, i.e. the number of the lane which the vehicle will use when travelling from one section to the next,"]
    #[doc = " * if it does not actively change lanes. If this component is absent, the lane name number remains the same in the next section."]
    #[doc = " *"]
    #[doc = " * @field connectingRoadSection: the identifier of the next road section in direction of traffic, that is connecting to the current road section. "]
    #[doc = " * If this component is absent, the connecting road section is the one following the instance where this DF is placed in the @ref RoadConfigurationSectionList."]
    #[doc = " *"]
    #[doc = " * @category: Road topology information"]
    #[doc = " * @revision: Created in V2.2.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct BasicLaneInformation {
        #[rasn(identifier = "laneNumber")]
        pub lane_number: LanePosition,
        pub direction: Direction,
        #[rasn(identifier = "laneWidth")]
        pub lane_width: Option<LaneWidth>,
        #[rasn(identifier = "connectingLane")]
        pub connecting_lane: Option<LanePosition>,
        #[rasn(identifier = "connectingRoadSection")]
        pub connecting_road_section: Option<RoadSectionId>,
    }
    impl BasicLaneInformation {
        pub fn new(
            lane_number: LanePosition,
            direction: Direction,
            lane_width: Option<LaneWidth>,
            connecting_lane: Option<LanePosition>,
            connecting_road_section: Option<RoadSectionId>,
        ) -> Self {
            Self {
                lane_number,
                direction,
                lane_width,
                connecting_lane,
                connecting_road_section,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DE indicates the cardinal number of bogies of a train."]
    #[doc = " *"]
    #[doc = " * The value shall be set to: "]
    #[doc = " * - `n` (`n > 1` and `n < 100`) indicates that the train has n x bogies,"]
    #[doc = " * - `100`indicates that the number of bogies is out of range, "]
    #[doc = " * - `101` the information is unavailable."]
    #[doc = " *"]
    #[doc = " * @unit: Number of bogies"]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("2..=101"))]
    pub struct BogiesCount(pub u8);
    #[doc = "*"]
    #[doc = " * This DE indicates the status of the vehicleÂ´s brake control system during an externally defined period of time. "]
    #[doc = " *"]
    #[doc = " * The corresponding bit shall be set to 1 under the following conditions:"]
    #[doc = " * - 0 `abs`        - the anti-lock braking system is engaged or has been engaged,"]
    #[doc = " * - 1 `tcs`        - the traction control system is engaged or has been engaged,"]
    #[doc = " * - 2 `esc`        - the electronic stability control system is engaged or has been engaged."]
    #[doc = " *"]
    #[doc = " * Otherwise (for example when the corresponding system is not available due to non equipped system"]
    #[doc = " * or information is unavailable), the corresponding bit shall be set to 0."]
    #[doc = " *"]
    #[doc = " * @category: Vehicle information "]
    #[doc = " * @revision: created in V2.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("3", extensible))]
    pub struct BrakeControl(pub BitString);
    #[doc = "*"]
    #[doc = " * The DE represents a cardinal number that counts the size of a set. "]
    #[doc = " * "]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct CardinalNumber1B(pub u8);
    #[doc = "*"]
    #[doc = " * The DE represents a cardinal number that counts the size of a set. "]
    #[doc = " * "]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=8"))]
    pub struct CardinalNumber3b(pub u8);
    #[doc = "* "]
    #[doc = " * This DF represents a general Data Frame to describe an angle component along with a confidence value in a cartesian coordinate system."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field value: The angle value which can be estimated as the mean of the current distribution."]
    #[doc = " *"]
    #[doc = " * @field confidence: The confidence value associated to the provided value."]
    #[doc = " *"]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct CartesianAngle {
        pub value: CartesianAngleValue,
        pub confidence: AngleConfidence,
    }
    impl CartesianAngle {
        pub fn new(value: CartesianAngleValue, confidence: AngleConfidence) -> Self {
            Self { value, confidence }
        }
    }
    #[doc = "* "]
    #[doc = " * This DE represents an angle value described in a local Cartesian coordinate system, per default counted positive in"]
    #[doc = " * a right-hand local coordinate system from the abscissa."]
    #[doc = " *"]
    #[doc = " * The value shall be set to: "]
    #[doc = " * - `n` (`n >= 0` and `n < 3600`) if the angle is equal to or less than n x 0,1 degrees, and greater than (n-1) x 0,1 degrees,"]
    #[doc = " * - `3601` if the information is not available."]
    #[doc = " *"]
    #[doc = " * The value 3600 shall not be used. "]
    #[doc = " * "]
    #[doc = " * @unit 0,1 degrees"]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Created in V2.1.1, description and value for 3601 corrected in V2.2.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=3601"))]
    pub struct CartesianAngleValue(pub u16);
    #[doc = "* "]
    #[doc = " * This DF represents a general Data Frame to describe an angular acceleration component along with a confidence value in a cartesian coordinate system."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field value: The angular acceleration component value."]
    #[doc = " *"]
    #[doc = " * @field confidence: The confidence value associated to the provided value."]
    #[doc = " * "]
    #[doc = " * @category: Kinematic information"]
    #[doc = " * @revision: Created in V2.1.1 "]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct CartesianAngularAccelerationComponent {
        pub value: CartesianAngularAccelerationComponentValue,
        pub confidence: AngularAccelerationConfidence,
    }
    impl CartesianAngularAccelerationComponent {
        pub fn new(
            value: CartesianAngularAccelerationComponentValue,
            confidence: AngularAccelerationConfidence,
        ) -> Self {
            Self { value, confidence }
        }
    }
    #[doc = "*"]
    #[doc = " * This DE represents an angular acceleration value described in a local Cartesian coordinate system, per default counted positive in"]
    #[doc = " * a right-hand local coordinate system from the abscissa."]
    #[doc = " *"]
    #[doc = "  * The value shall be set to: "]
    #[doc = " * - `-255` if the acceleration is equal to or less than -255 degrees/s^2,"]
    #[doc = " * - `n` (`n > -255` and `n < 255`) if the acceleration is equal to or less than n x 1 degree/s^2,"]
    #[doc = "      and greater than `(n-1)` x 0,01 degree/s^2,"]
    #[doc = " * - `255` if the acceleration is greater than 254 degrees/s^2,"]
    #[doc = " * - `256` if the information is unavailable."]
    #[doc = " *"]
    #[doc = " * @unit:  degree/s^2 (degrees per second squared)"]
    #[doc = " * @category: Kinematic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-255..=256"))]
    pub struct CartesianAngularAccelerationComponentValue(pub i16);
    #[doc = "* "]
    #[doc = " * This DF represents an angular velocity component along with a confidence value in a cartesian coordinate system."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field value: The angular velocity component."]
    #[doc = " *"]
    #[doc = " * @field confidence: The confidence value associated to the provided value."]
    #[doc = " *"]
    #[doc = " * @category: Kinematic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct CartesianAngularVelocityComponent {
        pub value: CartesianAngularVelocityComponentValue,
        pub confidence: AngularSpeedConfidence,
    }
    impl CartesianAngularVelocityComponent {
        pub fn new(
            value: CartesianAngularVelocityComponentValue,
            confidence: AngularSpeedConfidence,
        ) -> Self {
            Self { value, confidence }
        }
    }
    #[doc = "*"]
    #[doc = " * This DE represents an angular velocity component described in a local Cartesian coordinate system, per default counted positive in"]
    #[doc = " * a right-hand local coordinate system from the abscissa."]
    #[doc = " *"]
    #[doc = " * The value shall be set to: "]
    #[doc = " * - `-255` if the velocity is equal to or less than -255 degrees/s,"]
    #[doc = " * - `n` (`n > -255` and `n < 255`) if the velocity is equal to or less than n x 1 degree/s, and greater than (n-1) x 1 degree/s,"]
    #[doc = " * - `255` if the velocity is greater than 254 degrees/s,"]
    #[doc = " * - `256` if the information is unavailable."]
    #[doc = " *"]
    #[doc = " * @unit: degree/s"]
    #[doc = " * @category: Kinematic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-255..=256"))]
    pub struct CartesianAngularVelocityComponentValue(pub i16);
    #[doc = "*"]
    #[doc = " * This DF represents the value of a cartesian coordinate with a range of -327,68 metres to +327,66 metres."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `-32 768` if the longitudinal offset is out of range, i.e. less than or equal to -327,68 metres,"]
    #[doc = " * - `n` (`n > -32 768` and `n < 32 767`) if the longitudinal offset information is equal to or less than n x 0,01 metre and more than (n-1) x 0,01 metre,"]
    #[doc = " * - `32 767` if the longitudinal offset is out of range, i.e. greater than + 327,66 metres."]
    #[doc = " *"]
    #[doc = " * @unit 0,01 m"]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-32768..=32767"))]
    pub struct CartesianCoordinate(pub i16);
    #[doc = "*"]
    #[doc = " * This DF represents the value of a cartesian coordinate with a range of -1 310,72 metres to +1 310,70 metres."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `-131072` if the longitudinal offset is out of range, i.e. less than or equal to -1 310,72 metres,"]
    #[doc = " * - `n` (`n > 131 072` and `n < 131 071`) if the longitudinal offset information is equal to or less than n x 0,01 metre and more than (n-1) x 0,01 metre,"]
    #[doc = " * - `131 071` if the longitudinal offset is out of range, i.e. greater than + 1 310,70 metres."]
    #[doc = " *  "]
    #[doc = " * @unit 0,01 m"]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-131072..=131071"))]
    pub struct CartesianCoordinateLarge(pub i32);
    #[doc = "*"]
    #[doc = " * This DF represents the value of a cartesian coordinate with a range of -30,94 metres to +10,00 metres."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `3094` if the longitudinal offset is out of range, i.e. less than or equal to -30,94 metres,"]
    #[doc = " * - `n` (`n > -3 094` and `n < 1 001`) if the longitudinal offset information is equal to or less than n x 0,01 metre and more than (n-1) x 0,01 metre,"]
    #[doc = " * - `1001` if the longitudinal offset is out of range, i.e. greater than 10 metres."]
    #[doc = " *"]
    #[doc = " * @unit 0,01 m"]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-3094..=1001"))]
    pub struct CartesianCoordinateSmall(pub i16);
    #[doc = "*"]
    #[doc = " * This DF represents a coordinate along with a confidence value in a cartesian reference system."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field value: the coordinate value, which can be estimated as the mean of the current distribution."]
    #[doc = " * "]
    #[doc = " * @field confidence: the coordinate confidence value associated to the provided value."]
    #[doc = " *"]
    #[doc = " * @category: GeoReference information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct CartesianCoordinateWithConfidence {
        pub value: CartesianCoordinateLarge,
        pub confidence: CoordinateConfidence,
    }
    impl CartesianCoordinateWithConfidence {
        pub fn new(value: CartesianCoordinateLarge, confidence: CoordinateConfidence) -> Self {
            Self { value, confidence }
        }
    }
    #[doc = "* "]
    #[doc = " * This DF represents a  position in a two- or three-dimensional cartesian coordinate system."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field xCoordinate: the X coordinate value."]
    #[doc = " *"]
    #[doc = " * @field yCoordinate: the Y coordinate value."]
    #[doc = " *"]
    #[doc = " * @field zCoordinate: the optional Z coordinate value."]
    #[doc = " * "]
    #[doc = " * @category: GeoReference information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct CartesianPosition3d {
        #[rasn(identifier = "xCoordinate")]
        pub x_coordinate: CartesianCoordinate,
        #[rasn(identifier = "yCoordinate")]
        pub y_coordinate: CartesianCoordinate,
        #[rasn(identifier = "zCoordinate")]
        pub z_coordinate: Option<CartesianCoordinate>,
    }
    impl CartesianPosition3d {
        pub fn new(
            x_coordinate: CartesianCoordinate,
            y_coordinate: CartesianCoordinate,
            z_coordinate: Option<CartesianCoordinate>,
        ) -> Self {
            Self {
                x_coordinate,
                y_coordinate,
                z_coordinate,
            }
        }
    }
    #[doc = "* "]
    #[doc = " * This DF represents a  position in a two- or three-dimensional cartesian coordinate system with an associated confidence level for each coordinate."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field xCoordinate: the X coordinate value with the associated confidence level."]
    #[doc = " *"]
    #[doc = " * @field yCoordinate: the Y coordinate value with the associated confidence level."]
    #[doc = " *"]
    #[doc = " * @field zCoordinate: the optional Z coordinate value with the associated confidence level."]
    #[doc = " * "]
    #[doc = " * @category: GeoReference information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct CartesianPosition3dWithConfidence {
        #[rasn(identifier = "xCoordinate")]
        pub x_coordinate: CartesianCoordinateWithConfidence,
        #[rasn(identifier = "yCoordinate")]
        pub y_coordinate: CartesianCoordinateWithConfidence,
        #[rasn(identifier = "zCoordinate")]
        pub z_coordinate: Option<CartesianCoordinateWithConfidence>,
    }
    impl CartesianPosition3dWithConfidence {
        pub fn new(
            x_coordinate: CartesianCoordinateWithConfidence,
            y_coordinate: CartesianCoordinateWithConfidence,
            z_coordinate: Option<CartesianCoordinateWithConfidence>,
        ) -> Self {
            Self {
                x_coordinate,
                y_coordinate,
                z_coordinate,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DF is a representation of the cause code value of a traffic event. "]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field causeCode: the main cause of a detected event. "]
    #[doc = " *"]
    #[doc = " * @field subCauseCode: the subordinate cause of a detected event. "]
    #[doc = " *"]
    #[doc = " * The semantics of the entire DF are completely defined by the component causeCode. The interpretation of the subCauseCode may "]
    #[doc = " * provide additional information that is not strictly necessary to understand the causeCode itself, and is therefore optional."]
    #[doc = " *"]
    #[doc = " * @note: this DF is kept for backwards compatibility reasons only. It is recommended to use the @ref CauseCodeV2 instead. "]
    #[doc = " *"]
    #[doc = " * @category: Traffic information"]
    #[doc = " * @revision: Editorial update in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct CauseCode {
        #[rasn(identifier = "causeCode")]
        pub cause_code: CauseCodeType,
        #[rasn(identifier = "subCauseCode")]
        pub sub_cause_code: SubCauseCodeType,
    }
    impl CauseCode {
        pub fn new(cause_code: CauseCodeType, sub_cause_code: SubCauseCodeType) -> Self {
            Self {
                cause_code,
                sub_cause_code,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DF is a representation of the cause code value and associated sub cause code value of a traffic event. "]
    #[doc = " *"]
    #[doc = " * The following options are available:"]
    #[doc = " * - 0                                                        - reserved for future use,"]
    #[doc = " * - 1  - `trafficCondition1`                                 - in case the type of event is an abnormal traffic condition,"]
    #[doc = " * - 2  - `accident2`                                         - in case the type of event is a road accident, "]
    #[doc = " * - 3  - `roadworks3`                                        - in case the type of event is roadwork, based on authoritative information,"]
    #[doc = " * - 4  - `detectedRoadworks4`                                - in case the type of event is roadworks, based on non-authoritative information such as factual detections,"]
    #[doc = " * - 5  - `impassability5`                                    - in case the  type of event is unmanaged road blocking, referring to any"]
    #[doc = " *                                                              blocking of a road, partial or total, which has not been adequately secured and signposted,"]
    #[doc = " * - 6  - `adhesion6`                                         - in case the  type of event is low adhesion of the road surface,"]
    #[doc = " * - 7  - `aquaplaning7`                                      - danger of aquaplaning on the road,"]
    #[doc = " * - 8                                                        - reserved for future usage,"]
    #[doc = " * - 9  - `hazardousLocation-SurfaceCondition9`               - in case the type of event is abnormal road surface condition not covered by `adhesion6`, "]
    #[doc = " * - 10 - `hazardousLocation-ObstacleOnTheRoad10`             - in case the type of event is obstacle on the road,"]
    #[doc = " * - 11 - `hazardousLocation-AnimalOnTheRoad11`               - in case the type of event is animal on the road,"]
    #[doc = " * - 12 - `humanPresenceOnTheRoad`                            - in case the type of event is presence of human vulnerable road user on the road,"]
    #[doc = " * - 13                                                       - reserved for future usage,"]
    #[doc = " * - 14 - `wrongWayDriving14`                                 - in case the type of the event is vehicle driving in wrong way,"]
    #[doc = " * - 15 - `rescueRecoveryAndMaintenanceWorkInProgress15`      - in case the type of event is rescue, recovery and maintenance work for accident or for a road hazard in progress,"]
    #[doc = " * - 16                                                       - reserved for future usage,"]
    #[doc = " * - 17 - `adverseWeatherCondition-Wind17`                    - in case the type of event is wind,"]
    #[doc = " * - 18 - `adverseWeatherCondition-Visibility18`              - in case the type of event is low visibility,"]
    #[doc = " * - 19 - `adverseWeatherCondition-Precipitation19`           - in case the type of event is precipitation,"]
    #[doc = " * - 20 - `violence20`                                        - in case the the type of event is human violence on or near the road,"]
    #[doc = " * - 21-25                                                    - reserved for future usage,"]
    #[doc = " * - 26 - `slowVehicle26`                                     - in case the type of event is slow vehicle driving on the road,"]
    #[doc = " * - 27 - `dangerousEndOfQueue27`                             - in case the type of event is dangerous end of vehicle queue,"]
    #[doc = " * - 28 - `publicTransportVehicleApproaching                  - in case the type of event is a public transport vehicle approaching, with a priority defined by applicable traffic regulations,"]
    #[doc = " * - 29-90                                                    - are reserved for future usage,"]
    #[doc = " * - 91 - `vehicleBreakdown91`                                - in case the type of event is break down vehicle on the road,"]
    #[doc = " * - 92 - `postCrash92`                                       - in case the type of event is a detected crash,"]
    #[doc = " * - 93 - `humanProblem93`                                    - in case the type of event is human health problem in vehicles involved in traffic,"]
    #[doc = " * - 94 - `stationaryVehicle94`                               - in case the type of event is stationary vehicle,"]
    #[doc = " * - 95 - `emergencyVehicleApproaching95`                     - in case the type of event is an approaching vehicle operating on a mission for which the "]
    #[doc = "                                                                applicable traffic regulations provide it with defined priority rights in traffic. "]
    #[doc = " * - 96 - `hazardousLocation-DangerousCurve96`                - in case the type of event is dangerous curve,"]
    #[doc = " * - 97 - `collisionRisk97`                                   - in case the type of event is a collision risk,"]
    #[doc = " * - 98 - `signalViolation98`                                 - in case the type of event is signal violation,"]
    #[doc = " * - 99 - `dangerousSituation99`                              - in case the type of event is dangerous situation in which autonomous safety system in vehicle "]
    #[doc = " *                                                              is activated,"]
    #[doc = " * - 100 - `railwayLevelCrossing100`                          - in case the type of event is a railway level crossing. "]
    #[doc = " * - 101-255                                                  - are reserved for future usage."]
    #[doc = " *"]
    #[doc = " * @note: this DF is defined for use as part of CauseCodeV2. It is recommended to use CauseCodeV2."]
    #[doc = " * @category: Traffic information"]
    #[doc = " * @revision: Created in V2.1.1, the type of impassability5 changed to ImpassabilitySubCauseCode in V2.2.1, value 28 added in V2.2.1, definition of value 12 and 95 changed in V2.2.1"]
    #[doc = " *            name and/or definition of values 3, 6, 9, 15 and 17 changed in V2.4.1, value 4 assigned in V2.4.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum CauseCodeChoice {
        reserved0(SubCauseCodeType),
        trafficCondition1(TrafficConditionSubCauseCode),
        accident2(AccidentSubCauseCode),
        roadworks3(RoadworksSubCauseCode),
        detectedRoadworks4(SubCauseCodeType),
        impassability5(ImpassabilitySubCauseCode),
        adhesion6(AdhesionSubCauseCode),
        aquaplaning7(SubCauseCodeType),
        reserved8(SubCauseCodeType),
        #[rasn(identifier = "hazardousLocation-SurfaceCondition9")]
        hazardousLocation_SurfaceCondition9(HazardousLocationSurfaceConditionSubCauseCode),
        #[rasn(identifier = "hazardousLocation-ObstacleOnTheRoad10")]
        hazardousLocation_ObstacleOnTheRoad10(HazardousLocationObstacleOnTheRoadSubCauseCode),
        #[rasn(identifier = "hazardousLocation-AnimalOnTheRoad11")]
        hazardousLocation_AnimalOnTheRoad11(HazardousLocationAnimalOnTheRoadSubCauseCode),
        humanPresenceOnTheRoad12(HumanPresenceOnTheRoadSubCauseCode),
        reserved13(SubCauseCodeType),
        wrongWayDriving14(WrongWayDrivingSubCauseCode),
        rescueRecoveryAndMaintenanceWorkInProgress15(
            RescueRecoveryAndMaintenanceWorkInProgressSubCauseCode,
        ),
        reserved16(SubCauseCodeType),
        #[rasn(identifier = "adverseWeatherCondition-Wind17")]
        adverseWeatherCondition_Wind17(AdverseWeatherConditionWindSubCauseCode),
        #[rasn(identifier = "adverseWeatherCondition-Visibility18")]
        adverseWeatherCondition_Visibility18(AdverseWeatherConditionVisibilitySubCauseCode),
        #[rasn(identifier = "adverseWeatherCondition-Precipitation19")]
        adverseWeatherCondition_Precipitation19(AdverseWeatherConditionPrecipitationSubCauseCode),
        violence20(SubCauseCodeType),
        reserved21(SubCauseCodeType),
        reserved22(SubCauseCodeType),
        reserved23(SubCauseCodeType),
        reserved24(SubCauseCodeType),
        reserved25(SubCauseCodeType),
        slowVehicle26(SlowVehicleSubCauseCode),
        dangerousEndOfQueue27(DangerousEndOfQueueSubCauseCode),
        publicTransportVehicleApproaching28(SubCauseCodeType),
        reserved29(SubCauseCodeType),
        reserved30(SubCauseCodeType),
        reserved31(SubCauseCodeType),
        reserved32(SubCauseCodeType),
        reserved33(SubCauseCodeType),
        reserved34(SubCauseCodeType),
        reserved35(SubCauseCodeType),
        reserved36(SubCauseCodeType),
        reserved37(SubCauseCodeType),
        reserved38(SubCauseCodeType),
        reserved39(SubCauseCodeType),
        reserved40(SubCauseCodeType),
        reserved41(SubCauseCodeType),
        dontPanic42(SubCauseCodeType),
        reserved43(SubCauseCodeType),
        reserved44(SubCauseCodeType),
        reserved45(SubCauseCodeType),
        reserved46(SubCauseCodeType),
        reserved47(SubCauseCodeType),
        reserved48(SubCauseCodeType),
        reserved49(SubCauseCodeType),
        reserved50(SubCauseCodeType),
        reserved51(SubCauseCodeType),
        reserved52(SubCauseCodeType),
        reserved53(SubCauseCodeType),
        reserved54(SubCauseCodeType),
        reserved55(SubCauseCodeType),
        reserved56(SubCauseCodeType),
        reserved57(SubCauseCodeType),
        reserved58(SubCauseCodeType),
        reserved59(SubCauseCodeType),
        reserved60(SubCauseCodeType),
        reserved61(SubCauseCodeType),
        reserved62(SubCauseCodeType),
        reserved63(SubCauseCodeType),
        reserved64(SubCauseCodeType),
        reserved65(SubCauseCodeType),
        reserved66(SubCauseCodeType),
        reserved67(SubCauseCodeType),
        reserved68(SubCauseCodeType),
        reserved69(SubCauseCodeType),
        reserved70(SubCauseCodeType),
        reserved71(SubCauseCodeType),
        reserved72(SubCauseCodeType),
        reserved73(SubCauseCodeType),
        reserved74(SubCauseCodeType),
        reserved75(SubCauseCodeType),
        reserved76(SubCauseCodeType),
        reserved77(SubCauseCodeType),
        reserved78(SubCauseCodeType),
        reserved79(SubCauseCodeType),
        reserved80(SubCauseCodeType),
        reserved81(SubCauseCodeType),
        reserved82(SubCauseCodeType),
        reserved83(SubCauseCodeType),
        reserved84(SubCauseCodeType),
        reserved85(SubCauseCodeType),
        reserved86(SubCauseCodeType),
        reserved87(SubCauseCodeType),
        reserved88(SubCauseCodeType),
        reserved89(SubCauseCodeType),
        reserved90(SubCauseCodeType),
        vehicleBreakdown91(VehicleBreakdownSubCauseCode),
        postCrash92(PostCrashSubCauseCode),
        humanProblem93(HumanProblemSubCauseCode),
        stationaryVehicle94(StationaryVehicleSubCauseCode),
        emergencyVehicleApproaching95(EmergencyVehicleApproachingSubCauseCode),
        #[rasn(identifier = "hazardousLocation-DangerousCurve96")]
        hazardousLocation_DangerousCurve96(HazardousLocationDangerousCurveSubCauseCode),
        collisionRisk97(CollisionRiskSubCauseCode),
        signalViolation98(SignalViolationSubCauseCode),
        dangerousSituation99(DangerousSituationSubCauseCode),
        railwayLevelCrossing100(RailwayLevelCrossingSubCauseCode),
        reserved101(SubCauseCodeType),
        reserved102(SubCauseCodeType),
        reserved103(SubCauseCodeType),
        reserved104(SubCauseCodeType),
        reserved105(SubCauseCodeType),
        reserved106(SubCauseCodeType),
        reserved107(SubCauseCodeType),
        reserved108(SubCauseCodeType),
        reserved109(SubCauseCodeType),
        reserved110(SubCauseCodeType),
        reserved111(SubCauseCodeType),
        reserved112(SubCauseCodeType),
        reserved113(SubCauseCodeType),
        reserved114(SubCauseCodeType),
        reserved115(SubCauseCodeType),
        reserved116(SubCauseCodeType),
        reserved117(SubCauseCodeType),
        reserved118(SubCauseCodeType),
        reserved119(SubCauseCodeType),
        reserved120(SubCauseCodeType),
        reserved121(SubCauseCodeType),
        reserved122(SubCauseCodeType),
        reserved123(SubCauseCodeType),
        reserved124(SubCauseCodeType),
        reserved125(SubCauseCodeType),
        reserved126(SubCauseCodeType),
        reserved127(SubCauseCodeType),
        reserved128(SubCauseCodeType),
    }
    #[doc = "*"]
    #[doc = " *The DE represents the value of the cause code of an event. "]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0                                                     - reserved for future use,"]
    #[doc = " * - 1  - `trafficCondition`                               - in case the type of event is an abnormal traffic condition,"]
    #[doc = " * - 2  - `accident`                                       - in case the type of event is a road accident,"]
    #[doc = " * - 3  - `roadworks`                                      - in case the type of event is roadworks, based on authoritative information,"]
    #[doc = " * - 4  - `detectedRoadworks`                              - in case the type of event is roadworks, based on non-authoritative information such as factual detections, "]
    #[doc = " * - 5  - `impassability`                                  - in case the  type of event is unmanaged road blocking, referring to any"]
    #[doc = " *                                                           blocking of a road, partial or total, which has not been adequately"]
    #[doc = " *                                                           secured and signposted,"]
    #[doc = " * - 6  - `adhesion`                                       - in case the  type of event is low adhesion of the road surface,"]
    #[doc = " * - 7  - `aquaplaning`                                    - danger of aquaplaning on the road,"]
    #[doc = " * - 8                                                     - reserved for future usage,"]
    #[doc = " * - 9  - `hazardousLocation-SurfaceCondition`             - in case the type of event is abnormal road surface condition not covered by `adhesion`"]
    #[doc = " * - 10 - `hazardousLocation-ObstacleOnTheRoad`            - in case the type of event is obstacle on the road,"]
    #[doc = " * - 11 - `hazardousLocation-AnimalOnTheRoad`              - in case the type of event is animal on the road,"]
    #[doc = " * - 12 - `humanPresenceOnTheRoad`                         - in case the type of event is presence of human vulnerable road user on the road,"]
    #[doc = " * - 13                                                    - reserved for future usage,"]
    #[doc = " * - 14 - `wrongWayDriving`                                - in case the type of the event is vehicle driving in wrong way,"]
    #[doc = " * - 15 - `rescueRecoveryAndMaintenanceWorkInProgress`     - in case the type of event is rescue, recovery and maintenance work for accident or for a road hazard in progress,"]
    #[doc = " * - 16                                                    - reserved for future usage,"]
    #[doc = " * - 17 - `adverseWeatherCondition-Wind                   `- in case the type of event is wind,"]
    #[doc = " * - 18 - `adverseWeatherCondition-Visibility`             - in case the type of event is low visibility,"]
    #[doc = " * - 19 - `adverseWeatherCondition-Precipitation`          - in case the type of event is precipitation,"]
    #[doc = " * - 20 - `violence`                                       - in case the the type of event is human violence on or near the road,"]
    #[doc = " * - 21-25                                                 - reserved for future usage,"]
    #[doc = " * - 26 - `slowVehicle`                                    - in case the type of event is slow vehicle driving on the road,"]
    #[doc = " * - 27 - `dangerousEndOfQueue`                            - in case the type of event is dangerous end of vehicle queue,"]
    #[doc = " * - 28 - `publicTransportVehicleApproaching               - in case the type of event is a public transport vehicle approaching, with a priority defined by applicable traffic regulations,"]
    #[doc = " * - 29-90                                                 - are reserved for future usage,"]
    #[doc = " * - 91 - `vehicleBreakdown`                               - in case the type of event is break down vehicle on the road,"]
    #[doc = " * - 92 - `postCrash`                                      - in case the type of event is a detected crash,"]
    #[doc = " * - 93 - `humanProblem`                                   - in case the type of event is human health problem in vehicles involved in traffic,"]
    #[doc = " * - 94 - `stationaryVehicle`                              - in case the type of event is stationary vehicle,"]
    #[doc = " * - 95 - `emergencyVehicleApproaching`                    - in case the type of event is an approaching vehicle operating on a mission for which the applicable "]
    #[doc = "                                                             traffic regulations provide it with defined priority rights in traffic. "]
    #[doc = " * - 96 - `hazardousLocation-DangerousCurve`               - in case the type of event is dangerous curve,"]
    #[doc = " * - 97 - `collisionRisk`                                  - in case the type of event is a collision risk,"]
    #[doc = " * - 98 - `signalViolation`                                - in case the type of event is signal violation,"]
    #[doc = " * - 99 - `dangerousSituation`                             - in case the type of event is dangerous situation in which autonomous safety system in vehicle "]
    #[doc = " *                                                             is activated,"]
    #[doc = " * - 100 - `railwayLevelCrossing`                          - in case the type of event is a railway level crossing. "]
    #[doc = " * - 101-255                                               - are reserved for future usage."]
    #[doc = " *"]
    #[doc = " * @category: Traffic information"]
    #[doc = " * @revision: V1.3.1, value 20 and 100 added in V2.1.1, value 28 added in V2.2.1, definition of values 12 and 95 changed in V2.2.1,"]
    #[doc = " *            name and/or definition of values 3, 6, 9, 15 and 17 changed in V2.4.1, value 4 assigned in V2.4.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct CauseCodeType(pub u8);
    #[doc = "*"]
    #[doc = " * This DF is an alternative representation of the cause code value of a traffic event. "]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field ccAndScc: the main cause of a detected event. Each entry is of a different type and represents the sub cause code."]
    #[doc = ""]
    #[doc = " * The semantics of the entire DF are completely defined by the choice value which represents the cause code value. "]
    #[doc = " * The interpretation of the sub cause code value may provide additional information that is not strictly necessary to understand "]
    #[doc = " * the cause code itself, and is therefore optional."]
    #[doc = " *"]
    #[doc = " * @category: Traffic information"]
    #[doc = " * @revision: Created in V2.1.1, description amended in V2.2.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct CauseCodeV2 {
        #[rasn(identifier = "ccAndScc")]
        pub cc_and_scc: CauseCodeChoice,
    }
    impl CauseCodeV2 {
        pub fn new(cc_and_scc: CauseCodeChoice) -> Self {
            Self { cc_and_scc }
        }
    }
    #[doc = "*"]
    #[doc = " * The DF describes the position of a CEN DSRC road side equipment."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field protectedZoneLatitude: the latitude of the CEN DSRC road side equipment."]
    #[doc = " * "]
    #[doc = " * @field protectedZoneLongitude: the latitude of the CEN DSRC road side equipment. "]
    #[doc = " * "]
    #[doc = " * @field cenDsrcTollingZoneID: the optional ID of the CEN DSRC road side equipment."]
    #[doc = " * "]
    #[doc = " * @category: Infrastructure information, Communication information"]
    #[doc = " * @revision: revised in V2.1.1 (cenDsrcTollingZoneId is directly of type ProtectedZoneId)"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct CenDsrcTollingZone {
        #[rasn(identifier = "protectedZoneLatitude")]
        pub protected_zone_latitude: Latitude,
        #[rasn(identifier = "protectedZoneLongitude")]
        pub protected_zone_longitude: Longitude,
        #[rasn(identifier = "cenDsrcTollingZoneId")]
        pub cen_dsrc_tolling_zone_id: Option<ProtectedZoneId>,
    }
    impl CenDsrcTollingZone {
        pub fn new(
            protected_zone_latitude: Latitude,
            protected_zone_longitude: Longitude,
            cen_dsrc_tolling_zone_id: Option<ProtectedZoneId>,
        ) -> Self {
            Self {
                protected_zone_latitude,
                protected_zone_longitude,
                cen_dsrc_tolling_zone_id,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DE represents the ID of a CEN DSRC tolling zone. "]
    #[doc = " * "]
    #[doc = " * @category: Communication information"]
    #[doc = " * @revision: V1.3.1"]
    #[doc = " * @note: this DE is deprecated and shall not be used anymore.  "]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct CenDsrcTollingZoneID(pub ProtectedZoneId);
    #[doc = "* "]
    #[doc = " * "]
    #[doc = " * This DF represents the shape of a circular area or a right cylinder that is centred on the shape's reference point. "]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field shapeReferencePoint: optional reference point that represents the centre of the circle, relative to an externally specified reference position. "]
    #[doc = " * If this component is absent, the externally specified reference position represents the shape's reference point. "]
    #[doc = " *"]
    #[doc = " * @field radius: the radius of the circular area."]
    #[doc = " *"]
    #[doc = " * @field height: the optional height, present if the shape is a right cylinder extending in the positive z-axis. "]
    #[doc = " *"]
    #[doc = " * "]
    #[doc = " * @category: GeoReference information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct CircularShape {
        #[rasn(identifier = "shapeReferencePoint")]
        pub shape_reference_point: Option<CartesianPosition3d>,
        pub radius: StandardLength12b,
        pub height: Option<StandardLength12b>,
    }
    impl CircularShape {
        pub fn new(
            shape_reference_point: Option<CartesianPosition3d>,
            radius: StandardLength12b,
            height: Option<StandardLength12b>,
        ) -> Self {
            Self {
                shape_reference_point,
                radius,
                height,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DF indicates the opening/closure status of the lanes of a carriageway."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field innerhardShoulderStatus: this information is optional and shall be included if an inner hard shoulder is present and the information is known."]
    #[doc = " * It indicates the open/closing status of inner hard shoulder lanes. "]
    #[doc = " * "]
    #[doc = " * @field outerhardShoulderStatus: this information is optional and shall be included if an outer hard shoulder is present and the information is known."]
    #[doc = " * It indicates the open/closing status of outer hard shoulder lanes. "]
    #[doc = " * "]
    #[doc = " * @field drivingLaneStatus: this information is optional and shall be included if the information is known."]
    #[doc = " * It indicates the open/closing status of driving lanes. "]
    #[doc = " * For carriageways with more than 13 driving lanes, the drivingLaneStatus component shall not be present."]
    #[doc = " * "]
    #[doc = " * @category: Infrastructure information, Road topology information"]
    #[doc = " * @revision: Description revised in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ClosedLanes {
        #[rasn(identifier = "innerhardShoulderStatus")]
        pub innerhard_shoulder_status: Option<HardShoulderStatus>,
        #[rasn(identifier = "outerhardShoulderStatus")]
        pub outerhard_shoulder_status: Option<HardShoulderStatus>,
        #[rasn(identifier = "drivingLaneStatus")]
        pub driving_lane_status: Option<DrivingLaneStatus>,
    }
    impl ClosedLanes {
        pub fn new(
            innerhard_shoulder_status: Option<HardShoulderStatus>,
            outerhard_shoulder_status: Option<HardShoulderStatus>,
            driving_lane_status: Option<DrivingLaneStatus>,
        ) -> Self {
            Self {
                innerhard_shoulder_status,
                outerhard_shoulder_status,
                driving_lane_status,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DF provides information about the breakup of a cluster."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field clusterBreakupReason: indicates the reason for breakup."]
    #[doc = " * "]
    #[doc = " * @field breakupTime: indicates the time of breakup. "]
    #[doc = " *"]
    #[doc = " * @category: Cluster Information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ClusterBreakupInfo {
        #[rasn(identifier = "clusterBreakupReason")]
        pub cluster_breakup_reason: ClusterBreakupReason,
        #[rasn(identifier = "breakupTime")]
        pub breakup_time: DeltaTimeQuarterSecond,
    }
    impl ClusterBreakupInfo {
        pub fn new(
            cluster_breakup_reason: ClusterBreakupReason,
            breakup_time: DeltaTimeQuarterSecond,
        ) -> Self {
            Self {
                cluster_breakup_reason,
                breakup_time,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DE indicates the reason why a cluster leader intends to break up the cluster."]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 - `notProvided`                          - if the information is not provided,"]
    #[doc = " * - 1 - `clusteringPurposeCompleted`           - if the cluster purpose has been completed,"]
    #[doc = " * - 2 - `leaderMovedOutOfClusterBoundingBox`   - if the leader moved out of the cluster's bounding box,"]
    #[doc = " * - 3 - `joiningAnotherCluster`                - if the cluster leader is about to join another cluster,"]
    #[doc = " * - 4 - `enteringLowRiskAreaBasedOnMaps`       - if the cluster is entering an area idenrified as low risk based on the use of maps,"]
    #[doc = " * - 5 - `receptionOfCpmContainingCluster`      - if the leader received a Collective Perception Message containing information about the same cluster. "]
    #[doc = " * - 6 to 15                                    - are reserved for future use.                                    "]
    #[doc = " *"]
    #[doc = " * @category: Cluster information"]
    #[doc = " * @revision: Created in V2.1.1, type changed from ENUMERATED to INTEGER in V2.2.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=15"))]
    pub struct ClusterBreakupReason(pub u8);
    #[doc = "*"]
    #[doc = " * This DF provides information about the joining of a cluster."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field clusterId: indicates the identifier of the cluster."]
    #[doc = " * "]
    #[doc = " * @field joinTime: indicates the time of joining. "]
    #[doc = " *"]
    #[doc = " * @category: Cluster Information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ClusterJoinInfo {
        #[rasn(identifier = "clusterId")]
        pub cluster_id: Identifier1B,
        #[rasn(identifier = "joinTime")]
        pub join_time: DeltaTimeQuarterSecond,
    }
    impl ClusterJoinInfo {
        pub fn new(cluster_id: Identifier1B, join_time: DeltaTimeQuarterSecond) -> Self {
            Self {
                cluster_id,
                join_time,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * The DF provides information about the leaving of a cluster."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field clusterId: indicates the cluster."]
    #[doc = " * "]
    #[doc = " * @field clusterLeaveReason: indicates the reason for leaving. "]
    #[doc = " *"]
    #[doc = " * @category: Cluster Information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ClusterLeaveInfo {
        #[rasn(identifier = "clusterId")]
        pub cluster_id: Identifier1B,
        #[rasn(identifier = "clusterLeaveReason")]
        pub cluster_leave_reason: ClusterLeaveReason,
    }
    impl ClusterLeaveInfo {
        pub fn new(cluster_id: Identifier1B, cluster_leave_reason: ClusterLeaveReason) -> Self {
            Self {
                cluster_id,
                cluster_leave_reason,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DE indicates the reason why a cluster participant is leaving the cluster."]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 - `notProvided `                 - if the information is not provided,"]
    #[doc = " * - 1 - `clusterLeaderLost`            - if the cluster leader cannot be found anymore,   "]
    #[doc = " * - 2 - `clusterDisbandedByLeader`     - if the cluster has been disbanded by the leader,"]
    #[doc = " * - 3 - `outOfClusterBoundingBox`      - if the participants moved out of the cluster's bounding box,"]
    #[doc = " * - 4 - `outOfClusterSpeedRange`       - if the cluster speed moved out of a defined range, "]
    #[doc = " * - 5 - `joiningAnotherCluster`        - if the participant is joining another cluster,"]
    #[doc = " * - 6 - `cancelledJoin`                - if the participant is cancelling a joining procedure,"]
    #[doc = " * - 7 - `failedJoin`                   - if the participant failed to join the cluster,"]
    #[doc = " * - 8 - `safetyCondition`              - if a safety condition applies."]
    #[doc = " * - 9 to 15                            - are reserved for future use                             "]
    #[doc = " *"]
    #[doc = " * @category: Cluster information"]
    #[doc = " * @revision: Created in V2.1.1, type changed from ENUMERATED to INTEGER in V2.2.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=15"))]
    pub struct ClusterLeaveReason(pub u8);
    #[doc = "*"]
    #[doc = " * This DE represents the sub cause codes of the @ref CauseCode `collisionRisk`."]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 - `unavailable`                    - in case information on the type of collision risk is unavailable,"]
    #[doc = " * - 1 - `longitudinalCollisionRisk`      - in case the type of detected collision risk is longitudinal collision risk, "]
    #[doc = " *                                          e.g. forward collision or face to face collision,"]
    #[doc = " * - 2 - `crossingCollisionRisk`          - in case the type of detected collision risk is crossing collision risk,"]
    #[doc = " * - 3 - `lateralCollisionRisk`           - in case the type of detected collision risk is lateral collision risk,"]
    #[doc = " * - 4 - `vulnerableRoadUser`             - in case the type of detected collision risk involves vulnerable road users"]
    #[doc = " *                                          e.g. pedestrians or bicycles."]
    #[doc = " * - 5 - `collisionRiskWithPedestrian`    - in case the type of detected collision risk involves at least one pedestrian, "]
    #[doc = " * - 6 - `collisionRiskWithCyclist`       - in case the type of detected collision risk involves at least one cyclist (and no pedestrians),"]
    #[doc = " * - 7 - `collisionRiskWithMotorVehicle`  - in case the type of detected collision risk involves at least one motor vehicle (and no pedestrians or cyclists),"]
    #[doc = " * - 8 - `erraticDriving`                 - in case the collision risk is due a vehicle exhibiting inconsistent and unpredictable actions like swerving, abrupt lane changes, and inconsistent speed."]
    #[doc = " * - 9 - `recklessDriving`                - in case the collision risk is due a vehicle exhibiting aggressive manoeuvres like tailgating and sudden lane changes and which ignores traffic rules. "]
    #[doc = " * - 10-255                               - are reserved for future usage."]
    #[doc = " *"]
    #[doc = " * @category: Traffic information"]
    #[doc = " * @revision: V1.3.1, values 5-7 assigned in V2.2.1, values 8-9 added in V2.4.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct CollisionRiskSubCauseCode(pub u8);
    #[doc = "* "]
    #[doc = " * This DE represents a confidence level in percentage."]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `n` (`n > 0` and `n < 101`) : for the confidence level in %,"]
    #[doc = " * - `101`                   : in case the confidence level is not available."]
    #[doc = " *"]
    #[doc = " * @unit Percent "]
    #[doc = " * @category: Basic information "]
    #[doc = " * @revision: Created in V2.1.1 "]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=101"))]
    pub struct ConfidenceLevel(pub u8);
    #[doc = "*"]
    #[doc = "* This DF shall contain a list of @ref ConfidenceLevel."]
    #[doc = "*"]
    #[doc = "* @category: Basic information"]
    #[doc = "* @revision: created in V2.3.1 "]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32", extensible))]
    pub struct ConfidenceLevels(pub SequenceOf<ConfidenceLevel>);
    #[doc = "* "]
    #[doc = " * This DE indicates the coordinate confidence value which represents the estimated absolute accuracy of a position coordinate with a default confidence level of 95 %."]
    #[doc = " * If required, the confidence level can be defined by the corresponding standards applying this DE."]
    #[doc = " *"]
    #[doc = " * The value shall be set to: "]
    #[doc = " * - `n` (`n > 0` and `n < 4095`) if the confidence value is is equal to or less than n x 0,01 metre, and greater than (n-1) x 0,01 metre,"]
    #[doc = " * - `4095` if the confidence value is greater than 40,94 metres,"]
    #[doc = " * - `4096` if the confidence value is not available."]
    #[doc = " *"]
    #[doc = " * @unit 0,01 m"]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=4096"))]
    pub struct CoordinateConfidence(pub u16);
    #[doc = "* "]
    #[doc = " * This DE represents the Bravais-Pearson correlation value for each cell of a lower triangular correlation matrix."]
    #[doc = " *"]
    #[doc = " * The value shall be set to: "]
    #[doc = " * - `-100` in case of full negative correlation,"]
    #[doc = " * - `n` (`n > -100` and `n < 0`) if the correlation is negative and equal to n/100,"]
    #[doc = " * - `0` in case of no correlation,"]
    #[doc = " * - `n` (`n > 0` and `n < 100`) if the correlation is positive and equal to n/100,"]
    #[doc = " * - `100` in case of full positive correlation,"]
    #[doc = " * - `101` in case the correlation information is unavailable. "]
    #[doc = " *"]
    #[doc = " * @unit: the value is scaled by 100"]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Created in V2.1.1, corrected the value to n/100 on V2.4.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-100..=101"))]
    pub struct CorrelationCellValue(pub i8);
    #[doc = "*"]
    #[doc = " * This DF represents a column of a lower triangular positive semi-definite matrix and consists of a list of correlation cell values ordered by rows."]
    #[doc = " * Given a matrix \"A\" of size n x n, the number of columns to be included in the lower triangular matrix is k=n-1."]
    #[doc = " * Each column \"i\" of the lower triangular matrix then contains k-(i-1) values (ordered by rows from 1 to n-1), where \"i\" refers to the column number count"]
    #[doc = " * starting at 1 from the left."]
    #[doc = " *"]
    #[doc = " * @category: Sensing Information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=13", extensible))]
    pub struct CorrelationColumn(pub SequenceOf<CorrelationCellValue>);
    #[doc = "* "]
    #[doc = " * This DE represents an ISO 3166-1 [25] country code encoded using ITA-2 encoding."]
    #[doc = " *"]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Created in V2.2.1 based on ISO 14816 [23]"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct CountryCode(pub FixedBitString<10usize>);
    #[doc = "*"]
    #[doc = " * This DF represents the curvature of the vehicle trajectory and the associated confidence value."]
    #[doc = " * The curvature detected by a vehicle represents the curvature of actual vehicle trajectory."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field curvatureValue: Detected curvature of the vehicle trajectory."]
    #[doc = " * "]
    #[doc = " * @field curvatureConfidence: along with a confidence value of the curvature value with a predefined confidence level. "]
    #[doc = " * "]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: Description revised in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct Curvature {
        #[rasn(identifier = "curvatureValue")]
        pub curvature_value: CurvatureValue,
        #[rasn(identifier = "curvatureConfidence")]
        pub curvature_confidence: CurvatureConfidence,
    }
    impl Curvature {
        pub fn new(
            curvature_value: CurvatureValue,
            curvature_confidence: CurvatureConfidence,
        ) -> Self {
            Self {
                curvature_value,
                curvature_confidence,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * The DE describes whether the yaw rate is used to calculate the curvature for a curvature value."]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 - `yawRateUsed`    - if the yaw rate is used,"]
    #[doc = " * - 1 - `yawRateNotUsed` - if the yaw rate is not used,"]
    #[doc = " * - 2 - `unavailable`    - if the information of curvature calculation mode is unknown."]
    #[doc = " *"]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: V1.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum CurvatureCalculationMode {
        yawRateUsed = 0,
        yawRateNotUsed = 1,
        unavailable = 2,
    }
    #[doc = "*"]
    #[doc = " * This DE indicates the acceleration confidence value which represents the estimated absolute accuracy range of a curvature value with a confidence level of 95 %."]
    #[doc = " * If required, the confidence level can be defined by the corresponding standards applying this DE."]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 - `onePerMeter-0-00002` - if the confidence value is less than or equal to 0,00002 m-1,"]
    #[doc = " * - 1 - `onePerMeter-0-0001`  - if the confidence value is less than or equal to 0,0001 m-1 and greater than 0,00002 m-1,"]
    #[doc = " * - 2 - `onePerMeter-0-0005`  - if the confidence value is less than or equal to 0,0005 m-1 and greater than 0,0001 m-1,"]
    #[doc = " * - 3 - `onePerMeter-0-002`   - if the confidence value is less than or equal to 0,002 m-1 and greater than 0,0005 m-1,"]
    #[doc = " * - 4 - `nePerMeter-0-01`     - if the confidence value is less than or equal to 0,01 m-1 and greater than 0,002 m-1,"]
    #[doc = " * - 5 - `nePerMeter-0-1`      - if the confidence value is less than or equal to 0,1 m-1  and greater than 0,01 m-1,"]
    #[doc = " * - 6 - `outOfRange`          - if the confidence value is out of range, i.e. greater than 0,1 m-1,"]
    #[doc = " * - 7 - `unavailable`         - if the confidence value is not available."]
    #[doc = " * "]
    #[doc = " * @note:\tThe fact that a curvature value is received with confidence value set to `unavailable(7)` can be caused by"]
    #[doc = " * several reasons, such as:"]
    #[doc = " * - the sensor cannot deliver the accuracy at the defined confidence level because it is a low-end sensor,"]
    #[doc = " * - the sensor cannot calculate the accuracy due to lack of variables, or"]
    #[doc = " * - there has been a vehicle bus (e.g. CAN bus) error."]
    #[doc = " * In all 3 cases above, the curvature value may be valid and used by the application."]
    #[doc = " * "]
    #[doc = " * @note: If a curvature value is received and its confidence value is set to `outOfRange(6)`, it means that the curvature value is not valid "]
    #[doc = " * and therefore cannot be trusted. Such value is not useful for the application."]
    #[doc = " * "]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: Description revised in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum CurvatureConfidence {
        #[rasn(identifier = "onePerMeter-0-00002")]
        onePerMeter_0_00002 = 0,
        #[rasn(identifier = "onePerMeter-0-0001")]
        onePerMeter_0_0001 = 1,
        #[rasn(identifier = "onePerMeter-0-0005")]
        onePerMeter_0_0005 = 2,
        #[rasn(identifier = "onePerMeter-0-002")]
        onePerMeter_0_002 = 3,
        #[rasn(identifier = "onePerMeter-0-01")]
        onePerMeter_0_01 = 4,
        #[rasn(identifier = "onePerMeter-0-1")]
        onePerMeter_0_1 = 5,
        outOfRange = 6,
        unavailable = 7,
    }
    #[doc = "*"]
    #[doc = " * This DE describes vehicle turning curve with the following information:"]
    #[doc = " * ```"]
    #[doc = " *     Value = 1 / Radius * 10000"]
    #[doc = " * ```"]
    #[doc = " * wherein radius is the vehicle turning curve radius in metres. "]
    #[doc = " * "]
    #[doc = " * Positive values indicate a turning curve to the left hand side of the driver."]
    #[doc = " * It corresponds to the vehicle coordinate system as defined in ISO 8855 [21]."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `-1023` for  values smaller than -1023,"]
    #[doc = " * - `n` (`n > -1023` and `n < 0`) for negative values equal to or less than `n`, and greater than `(n-1)`,"]
    #[doc = " * - `0` when the vehicle is moving straight,"]
    #[doc = " * - `n` (`n > 0` and `n < 1022`) for positive values equal to or less than `n`, and greater than `(n-1)`,"]
    #[doc = " * - `1022`, for values  greater than 1021,"]
    #[doc = " * - `1023`, if the information is not available."]
    #[doc = " * "]
    #[doc = " * @note: The present DE is limited to vehicle types as defined in ISO 8855 [21]."]
    #[doc = " * "]
    #[doc = " * @unit: 1 over 10 000 metres"]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: description revised in V2.1.1 (the definition of value 1022 has changed slightly)"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-1023..=1023"))]
    pub struct CurvatureValue(pub i16);
    #[doc = "*"]
    #[doc = " * This DE represents the value of the sub cause codes of the @ref CauseCode `dangerousEndOfQueue`. "]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 - `unavailable`     - in case information on the type of dangerous queue is unavailable,"]
    #[doc = " * - 1 - `suddenEndOfQueue`- in case a sudden end of queue is detected, e.g. due to accident or obstacle,"]
    #[doc = " * - 2 - `queueOverHill`   - in case the dangerous end of queue is detected on the road hill,"]
    #[doc = " * - 3 - `queueAroundBend` - in case the dangerous end of queue is detected around the road bend,"]
    #[doc = " * - 4 - `queueInTunnel`   - in case queue is detected in tunnel,"]
    #[doc = " * - 5-255                 - reserved for future usage."]
    #[doc = " *"]
    #[doc = " * @category: Traffic information"]
    #[doc = " * @revision: V1.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct DangerousEndOfQueueSubCauseCode(pub u8);
    #[doc = "*"]
    #[doc = " * This DE indicates the type of the dangerous goods being carried by a heavy vehicle."]
    #[doc = " * The value is assigned according to `class` and `division` definitions of dangerous goods as specified in part II,"]
    #[doc = " * chapter 2.1.1.1 of European Agreement concerning the International Carriage of Dangerous Goods by Road [3]."]
    #[doc = " * "]
    #[doc = " * "]
    #[doc = " * @category Vehicle information"]
    #[doc = " * @revision: V1.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum DangerousGoodsBasic {
        explosives1 = 0,
        explosives2 = 1,
        explosives3 = 2,
        explosives4 = 3,
        explosives5 = 4,
        explosives6 = 5,
        flammableGases = 6,
        nonFlammableGases = 7,
        toxicGases = 8,
        flammableLiquids = 9,
        flammableSolids = 10,
        substancesLiableToSpontaneousCombustion = 11,
        substancesEmittingFlammableGasesUponContactWithWater = 12,
        oxidizingSubstances = 13,
        organicPeroxides = 14,
        toxicSubstances = 15,
        infectiousSubstances = 16,
        radioactiveMaterial = 17,
        corrosiveSubstances = 18,
        miscellaneousDangerousSubstances = 19,
    }
    #[doc = "*"]
    #[doc = " * This DF provides a description of dangerous goods being carried by a heavy vehicle."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field dangerousGoodsType: Type of dangerous goods."]
    #[doc = " * "]
    #[doc = " * @field unNumber: a 4-digit number that identifies the substance of the dangerous goods as specified in"]
    #[doc = " * United Nations Recommendations on the Transport of Dangerous Goods - Model Regulations [4],"]
    #[doc = " * "]
    #[doc = " * @field elevatedTemperature: whether the carried dangerous goods are transported at high temperature."]
    #[doc = " * If yes, the value shall be set to TRUE,"]
    #[doc = " * "]
    #[doc = " * @field tunnelsRestricted: whether the heavy vehicle carrying dangerous goods is restricted to enter tunnels."]
    #[doc = " * If yes, the value shall be set to TRUE,"]
    #[doc = " * "]
    #[doc = " * @field limitedQuantity: whether the carried dangerous goods are packed with limited quantity."]
    #[doc = " * If yes, the value shall be set to TRUE,"]
    #[doc = " * "]
    #[doc = " * @field emergencyActionCode: physical signage placard at the vehicle that carries information on how an emergency"]
    #[doc = " * service should deal with an incident. This component is optional; it shall be present if the information is available."]
    #[doc = " * "]
    #[doc = " * @field phoneNumber: contact phone number of assistance service in case of incident or accident."]
    #[doc = " * This component is optional, it shall be present if the information is available."]
    #[doc = " * "]
    #[doc = " * @field companyName: name of company that manages the transportation of the dangerous goods."]
    #[doc = " * This component is optional; it shall be present if the information is available."]
    #[doc = " * "]
    #[doc = " * @category Vehicle information"]
    #[doc = " * @revision: V1.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct DangerousGoodsExtended {
        #[rasn(identifier = "dangerousGoodsType")]
        pub dangerous_goods_type: DangerousGoodsBasic,
        #[rasn(value("0..=9999"), identifier = "unNumber")]
        pub un_number: u16,
        #[rasn(identifier = "elevatedTemperature")]
        pub elevated_temperature: bool,
        #[rasn(identifier = "tunnelsRestricted")]
        pub tunnels_restricted: bool,
        #[rasn(identifier = "limitedQuantity")]
        pub limited_quantity: bool,
        #[rasn(size("1..=24"), identifier = "emergencyActionCode")]
        pub emergency_action_code: Option<Ia5String>,
        #[rasn(identifier = "phoneNumber")]
        pub phone_number: Option<PhoneNumber>,
        #[rasn(identifier = "companyName")]
        pub company_name: Option<Utf8String>,
    }
    impl DangerousGoodsExtended {
        pub fn new(
            dangerous_goods_type: DangerousGoodsBasic,
            un_number: u16,
            elevated_temperature: bool,
            tunnels_restricted: bool,
            limited_quantity: bool,
            emergency_action_code: Option<Ia5String>,
            phone_number: Option<PhoneNumber>,
            company_name: Option<Utf8String>,
        ) -> Self {
            Self {
                dangerous_goods_type,
                un_number,
                elevated_temperature,
                tunnels_restricted,
                limited_quantity,
                emergency_action_code,
                phone_number,
                company_name,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DE represents the value of the sub cause codes of the @ref CauseCode `dangerousSituation` "]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 - `unavailable`                      - in case information on the type of dangerous situation is unavailable,"]
    #[doc = " * - 1 - `emergencyElectronicBrakeEngaged`  - in case emergency electronic brake is engaged,"]
    #[doc = " * - 2 - `preCrashSystemEngaged`            - in case pre-crash system is engaged,"]
    #[doc = " * - 3 - `espEngaged`                       - in case Electronic Stability Program (ESP) system is engaged,"]
    #[doc = " * - 4 - `absEngaged`                       - in case Anti-lock Braking System (ABS) is engaged,"]
    #[doc = " * - 5 - `aebEngaged`                       - in case Autonomous Emergency Braking (AEB) system is engaged,"]
    #[doc = " * - 6 - `brakeWarningEngaged`              - in case brake warning is engaged,"]
    #[doc = " * - 7 - `collisionRiskWarningEngaged`      - in case collision risk warning is engaged,"]
    #[doc = " * - 8 - `riskMitigationFunctionEngaged`    - in case the Risk Mitigation Function according to UNECE Regulation 79 is engaged,"]
    #[doc = " * - 9-255                                  - reserved for future usage."]
    #[doc = " *"]
    #[doc = " * @category: Traffic information"]
    #[doc = " * @revision: V1.3.1, value 8 assigned in V2.4.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct DangerousSituationSubCauseCode(pub u8);
    #[doc = "*"]
    #[doc = " * This DE represents an offset altitude with regards to a defined altitude value."]
    #[doc = " * It may be used to describe a geographical point with regards to a specific reference geographical position."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `-12 700` for values equal to or lower than -127 metres,"]
    #[doc = " * - `n` (`n > -12 700` and `n <= 0`) for altitude offset n x 0,01 metre below the reference position,"]
    #[doc = " * - `0` for no altitudinal offset,"]
    #[doc = " * - `n` (`n > 0` and `n < 12799`) for altitude offset n x 0,01 metre above the reference position,"]
    #[doc = " * - `12 799` for values equal to or greater than 127,99 metres,"]
    #[doc = " * - `12 800` when the information is unavailable."]
    #[doc = " *"]
    #[doc = " * @unit: 0,01 metre"]
    #[doc = " * @category: GeoReference information"]
    #[doc = " * @revision: editorial update in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-12700..=12800"))]
    pub struct DeltaAltitude(pub i16);
    #[doc = "*"]
    #[doc = " * This DE represents an offset latitude with regards to a defined latitude value."]
    #[doc = " * It may be used to describe a geographical point with regards to a specific reference geographical position."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `n` (`n >= -131 071` and `n < 0`) for offset n x 10^-7 degree towards the south from the reference position,"]
    #[doc = " * - `0` for no latitudinal offset,"]
    #[doc = " * - `n` (`n > 0` and `n < 131 072`) for offset n x 10^-7 degree towards the north from the reference position,"]
    #[doc = " * - `131 072` when the information is unavailable."]
    #[doc = " *"]
    #[doc = " * @unit: 10^-7 degree"]
    #[doc = " * @category: GeoReference information"]
    #[doc = " * @revision: editorial update in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-131071..=131072"))]
    pub struct DeltaLatitude(pub i32);
    #[doc = "*"]
    #[doc = " * This DE represents an offset longitude with regards to a defined longitude value."]
    #[doc = " * It may be used to describe a geographical point with regards to a specific reference geographical position."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `n` (`n >= -131 071` and `n < 0`) for offset n x 10^-7 degree towards the west from the reference position,"]
    #[doc = " * - `0` for no longitudinal offset,"]
    #[doc = " * - `n` (`n > 0` and `n < 131 072`) for offset n x 10^-7 degree towards the east from the reference position, "]
    #[doc = " * - `131 072` when the information is unavailable."]
    #[doc = " *"]
    #[doc = " * @unit: 10^-7 degree"]
    #[doc = " * @category: GeoReference information"]
    #[doc = " * @revision: editorial update in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-131071..=131072"))]
    pub struct DeltaLongitude(pub i32);
    #[doc = "*"]
    #[doc = "* This DF defines a geographical point position as a 2 dimensional offset position to a geographical reference point."]
    #[doc = "*"]
    #[doc = "* It shall include the following components: "]
    #[doc = "*"]
    #[doc = "* @field deltaLatitude: A delta latitude offset with regards to the latitude value of the reference position."]
    #[doc = "*"]
    #[doc = "* @field deltaLongitude: A delta longitude offset with regards to the longitude value of the reference position."]
    #[doc = "*"]
    #[doc = "* @category: GeoReference information"]
    #[doc = "* @revision: created in V2.3.1 based on ISO TS 19321"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct DeltaPosition {
        #[rasn(identifier = "deltaLatitude")]
        pub delta_latitude: DeltaLatitude,
        #[rasn(identifier = "deltaLongitude")]
        pub delta_longitude: DeltaLongitude,
    }
    impl DeltaPosition {
        pub fn new(delta_latitude: DeltaLatitude, delta_longitude: DeltaLongitude) -> Self {
            Self {
                delta_latitude,
                delta_longitude,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* This DF shall contain a list of @ref DeltaPosition."]
    #[doc = "*"]
    #[doc = "* @category: GeoReference information"]
    #[doc = "* @revision: created in V2.3.1 based on ISO TS 19321 (DF DeltaPosition)"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32", extensible))]
    pub struct DeltaPositions(pub SequenceOf<DeltaPosition>);
    #[doc = "*"]
    #[doc = " * This DF defines a geographical point position as a 3 dimensional offset position to a geographical reference point."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field deltaLatitude: A delta latitude offset with regards to the latitude value of the reference position."]
    #[doc = " *"]
    #[doc = " * @field deltaLongitude: A delta longitude offset with regards to the longitude value of the reference position."]
    #[doc = " *"]
    #[doc = " * @field deltaAltitude: A delta altitude offset with regards to the altitude value of the reference position."]
    #[doc = " *"]
    #[doc = " * @category: GeoReference information"]
    #[doc = " * @revision:  V1.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct DeltaReferencePosition {
        #[rasn(identifier = "deltaLatitude")]
        pub delta_latitude: DeltaLatitude,
        #[rasn(identifier = "deltaLongitude")]
        pub delta_longitude: DeltaLongitude,
        #[rasn(identifier = "deltaAltitude")]
        pub delta_altitude: DeltaAltitude,
    }
    impl DeltaReferencePosition {
        pub fn new(
            delta_latitude: DeltaLatitude,
            delta_longitude: DeltaLongitude,
            delta_altitude: DeltaAltitude,
        ) -> Self {
            Self {
                delta_latitude,
                delta_longitude,
                delta_altitude,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* This DF shall contain a list of @ref DeltaReferencePosition."]
    #[doc = "*"]
    #[doc = "* @category: GeoReference information"]
    #[doc = "* @revision: created in V2.3.1 based on ISO TS 19321 (DF DeltaReferencePositions)"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32", extensible))]
    pub struct DeltaReferencePositions(pub SequenceOf<DeltaReferencePosition>);
    #[doc = "*"]
    #[doc = " * This DE represents a difference in time with respect to a reference time."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `n` (`n > 0` and `n < 10001`) to indicate a time value equal to or less than n x 0,001 s, and greater than (n-1) x 0,001 s,"]
    #[doc = " *"]
    #[doc = " * Example: a time interval between two consecutive message transmissions."]
    #[doc = " * "]
    #[doc = " * @unit: 0,001 s"]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Created in V2.1.1 from the DE TransmissionInterval in [2]"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=10000"))]
    pub struct DeltaTimeMilliSecondPositive(pub u16);
    #[doc = "* "]
    #[doc = " * This DE represents a signed difference in time with respect to a reference time."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `-2048` for time values equal to or less than -2,048 s,"]
    #[doc = " * - `n` (`n > -2048` and `n < 2047`) to indicate a time value equal to or less than n x 0,001 s, and greater than (n-1) x 0,001 s,"]
    #[doc = " * - `2047` for time values greater than 2,046 s"]
    #[doc = " *"]
    #[doc = " * @unit: 0,001 s"]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-2048..=2047"))]
    pub struct DeltaTimeMilliSecondSigned(pub i16);
    #[doc = "* "]
    #[doc = " * This DE represents a difference in time with respect to a reference time."]
    #[doc = " * It can be interpreted as the first 8 bits of a GenerationDeltaTime. To convert it to a @ref GenerationDeltaTime, "]
    #[doc = " * multiply by 256 (i.e. append a `00` byte)"]
    #[doc = " *"]
    #[doc = " * @unit: 256 * 0,001 s "]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=255"))]
    pub struct DeltaTimeQuarterSecond(pub u8);
    #[doc = "* "]
    #[doc = " * This DE represents a  difference in time with respect to a reference time."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `-0` for a difference in time of 0 seconds. "]
    #[doc = " * - `n` (`n > 0` and `n <= 86400`) to indicate a time value equal to or less than n x 1 s, and greater than (n-1) x 1 s,"]
    #[doc = " *"]
    #[doc = " * @unit: 1 s"]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Created in V2.1.1 from ValidityDuration"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=86400"))]
    pub struct DeltaTimeSecond(pub u32);
    #[doc = "*"]
    #[doc = " * This DE represents a difference in time with respect to a reference time."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `-0` for a difference in time of 0 seconds. "]
    #[doc = " * - `n` (`n > 0` and `n < 128`) to indicate a time value equal to or less than n x 10 s, and greater than (n-1) x 10 s,"]
    #[doc = " *"]
    #[doc = " * @unit: 10 s"]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Created in V2.2.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=127"))]
    pub struct DeltaTimeTenSeconds(pub u8);
    #[doc = "* "]
    #[doc = " * This DE represents a difference in time with respect to a reference time."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `0` for a difference in time of 0 seconds. "]
    #[doc = " * - `n` (`n > 0` and `n < 128`) to indicate a time value equal to or less than n x 0,1 s, and greater than (n-1) x 0,1 s,"]
    #[doc = " *"]
    #[doc = " * @unit: 0,1 s"]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=127"))]
    pub struct DeltaTimeTenthOfSecond(pub u8);
    #[doc = "*"]
    #[doc = " * This DF represents a portion of digital map. It shall contain a list of waypoints @ref ReferencePosition."]
    #[doc = " * "]
    #[doc = " * @category: GeoReference information"]
    #[doc = " * @revision:  V1.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=256"))]
    pub struct DigitalMap(pub SequenceOf<ReferencePosition>);
    #[doc = "*"]
    #[doc = " * This DE indicates a direction with respect to a defined reference direction."]
    #[doc = " * Example: a reference direction may be implicitly defined by the definition of a geographical zone."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 - `sameDirection`     - to indicate the same direction as the reference direction,"]
    #[doc = " * - 1 - `oppositeDirection` - to indicate opposite direction as the reference direction,"]
    #[doc = " * - 2 - `bothDirections`    - to indicate both directions, i.e. the same and the opposite direction,"]
    #[doc = " * - 3 - `unavailable`       - to indicate that the information is unavailable."]
    #[doc = " *"]
    #[doc = " * @category: GeoReference information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=3"))]
    pub struct Direction(pub u8);
    #[doc = "*"]
    #[doc = " * This DE indicates in which direction something is moving."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 - `forward`     - to indicate it is moving forward,"]
    #[doc = " * - 1 - `backwards`   - to indicate it is moving backwards,"]
    #[doc = " * - 2 - `unavailable` - to indicate that the information is unavailable."]
    #[doc = " *"]
    #[doc = " * @category: Kinematic information"]
    #[doc = " * @revision: editorial update in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum DriveDirection {
        forward = 0,
        backward = 1,
        unavailable = 2,
    }
    #[doc = "*"]
    #[doc = " * This DE indicates whether a driving lane is open to traffic."]
    #[doc = " * "]
    #[doc = " * A lane is counted from inside border of the road excluding the hard shoulder. The size of the bit string shall"]
    #[doc = " * correspond to the total number of the driving lanes in the carriageway."]
    #[doc = " * "]
    #[doc = " * The numbering is matched to @ref LanePosition."]
    #[doc = " * The bit `0` is used to indicate the innermost lane, bit `1` is used to indicate the second lane from inside border."]
    #[doc = " * "]
    #[doc = " * If a lane is closed to traffic, the corresponding bit shall be set to `1`. Otherwise, it shall be set to `0`."]
    #[doc = " * "]
    #[doc = " * @note: hard shoulder status is not provided by this DE but in @ref HardShoulderStatus."]
    #[doc = " * "]
    #[doc = " * @category: Traffic information"]
    #[doc = " * @revision: V1.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=13"))]
    pub struct DrivingLaneStatus(pub BitString);
    #[doc = "* "]
    #[doc = " * "]
    #[doc = " * This DF represents the shape of an elliptical area or right elliptical cylinder that is centred "]
    #[doc = " * on the shape's reference point defined outside of the context of this DF and oriented w.r.t. a  "]
    #[doc = " * cartesian coordinate system defined outside of the context of this DF. "]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field shapeReferencePoint: optional reference point which represents the centre of the ellipse, "]
    #[doc = " * relative to an externally specified reference position. If this component is absent, the "]
    #[doc = " * externally specified reference position represents the shape's reference point. "]
    #[doc = " *"]
    #[doc = " * @field semiMajorAxisLength: half length of the major axis of the ellipse located in the X-Y Plane."]
    #[doc = " * "]
    #[doc = " * @field semiMinorAxisLength: half length of the minor axis of the ellipse located in the X-Y Plane."]
    #[doc = " *"]
    #[doc = " * @field orientation: the optional orientation of the major axis of the ellipse, measured with "]
    #[doc = " * positive values turning around the z-axis using the right-hand rule, starting from the X-axis."]
    #[doc = " * If absent, the orientation is equal to the value zero. "]
    #[doc = " * "]
    #[doc = " * @field height: the optional height, present if the shape is a right elliptical cylinder extending "]
    #[doc = " * in the positive Z-axis."]
    #[doc = " *"]
    #[doc = " * @category: GeoReference information"]
    #[doc = " * @revision: Created in V2.1.1, the type of the field orientation changed and the description revised in V2.2.1, added note on orientation in V2.4.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct EllipticalShape {
        #[rasn(identifier = "shapeReferencePoint")]
        pub shape_reference_point: Option<CartesianPosition3d>,
        #[rasn(identifier = "semiMajorAxisLength")]
        pub semi_major_axis_length: StandardLength12b,
        #[rasn(identifier = "semiMinorAxisLength")]
        pub semi_minor_axis_length: StandardLength12b,
        pub orientation: Option<CartesianAngleValue>,
        pub height: Option<StandardLength12b>,
    }
    impl EllipticalShape {
        pub fn new(
            shape_reference_point: Option<CartesianPosition3d>,
            semi_major_axis_length: StandardLength12b,
            semi_minor_axis_length: StandardLength12b,
            orientation: Option<CartesianAngleValue>,
            height: Option<StandardLength12b>,
        ) -> Self {
            Self {
                shape_reference_point,
                semi_major_axis_length,
                semi_minor_axis_length,
                orientation,
                height,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DE indicates whether a vehicle (e.g. public transport vehicle, truck) is under the embarkation process."]
    #[doc = " * If that is the case, the value is *TRUE*, otherwise *FALSE*."]
    #[doc = " *"]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: editorial update in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(delegate)]
    pub struct EmbarkationStatus(pub bool);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct EmergencyPriority(pub FixedBitString<2usize>);
    #[doc = "*"]
    #[doc = " * This DE represents the value of the sub cause codes of the @ref CauseCode \"emergencyVehicleApproaching\". "]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 - `unavailable`                   - in case further detailed information on the emergency vehicle approaching event "]
    #[doc = " *                                         is unavailable,"]
    #[doc = " * - 1 - `emergencyVehicleApproaching`   - in case an operating emergency vehicle is approaching,"]
    #[doc = " * - 2 - `prioritizedVehicleApproaching` - in case a prioritized vehicle is approaching,"]
    #[doc = " * - 3-255                               - reserved for future usage."]
    #[doc = " *"]
    #[doc = " * @category: Traffic information"]
    #[doc = " * @revision: V1.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct EmergencyVehicleApproachingSubCauseCode(pub u8);
    #[doc = "*"]
    #[doc = " * This DE indicated the type of energy being used and stored in vehicle."]
    #[doc = " *"]
    #[doc = " * The corresponding bit shall be set to 1 under the following conditions:"]
    #[doc = " * - 0 - `hydrogenStorage`       - when hydrogen is being used and stored in vehicle,"]
    #[doc = " * - 1 - `electricEnergyStorage` - when electric energy is being used and stored in vehicle,"]
    #[doc = " * - 2 - `liquidPropaneGas`      - when liquid Propane Gas (LPG) is being used and stored in vehicle,   "]
    #[doc = " * - 3 - `compressedNaturalGas ` - when compressedNaturalGas (CNG) is being used and stored in vehicle,"]
    #[doc = " * - 4 - `diesel`                - when diesel is being used and stored in vehicle,"]
    #[doc = " * - 5 - `gasoline`              - when gasoline is being used and stored in vehicle,"]
    #[doc = " * - 6 - `ammonia`               - when ammonia is being used and stored in vehicle."]
    #[doc = " *"]
    #[doc = " * - Otherwise, the corresponding bit shall be set to `0`."]
    #[doc = " *"]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: editorial revision in V2.1.1 "]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct EnergyStorageType(pub FixedBitString<7usize>);
    #[doc = "* "]
    #[doc = " * "]
    #[doc = " * This DF represents a vehicle category according to the UNECE/TRANS/WP.29/78/Rev.4 [16]."]
    #[doc = " * The following options are available:"]
    #[doc = " * "]
    #[doc = " * @field euVehicleCategoryL: indicates a vehicle in the L category."]
    #[doc = " *"]
    #[doc = " * @field euVehicleCategoryM: indicates a vehicle in the M category."]
    #[doc = " *"]
    #[doc = " * @field euVehicleCategoryN: indicates a vehicle in the N category."]
    #[doc = " *"]
    #[doc = " * @field euVehicleCategoryO: indicates a vehicle in the O category."]
    #[doc = " *"]
    #[doc = " * @field euVehicleCategoryT: indicates a vehicle in the T category."]
    #[doc = " *"]
    #[doc = " * @field euVehicleCategoryG: indicates a vehicle in the G category."]
    #[doc = " * "]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum EuVehicleCategoryCode {
        euVehicleCategoryL(EuVehicleCategoryL),
        euVehicleCategoryM(EuVehicleCategoryM),
        euVehicleCategoryN(EuVehicleCategoryN),
        euVehicleCategoryO(EuVehicleCategoryO),
        euVehicleCategoryT(()),
        euVehicleCategoryG(()),
    }
    #[doc = "*"]
    #[doc = " * This DE represents one of the specific categories in the L category: L1, L2, L3, L4, L5, L6, or L7 according to UNECE/TRANS/WP.29/78/Rev.4 [16]."]
    #[doc = " *"]
    #[doc = " *"]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum EuVehicleCategoryL {
        l1 = 0,
        l2 = 1,
        l3 = 2,
        l4 = 3,
        l5 = 4,
        l6 = 5,
        l7 = 6,
    }
    #[doc = "*"]
    #[doc = " * This DE represents one of the specific categories in the M category: M1, M2, or M3 according to UNECE/TRANS/WP.29/78/Rev.4 [16]."]
    #[doc = " *"]
    #[doc = " *"]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum EuVehicleCategoryM {
        m1 = 0,
        m2 = 1,
        m3 = 2,
    }
    #[doc = "*"]
    #[doc = " * This DE represents one of the specific categories in the N category: N1, N2, or N3 according to UNECE/TRANS/WP.29/78/Rev.4 [16]."]
    #[doc = " *"]
    #[doc = " *"]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum EuVehicleCategoryN {
        n1 = 0,
        n2 = 1,
        n3 = 2,
    }
    #[doc = "*"]
    #[doc = " * This DE represents one of the specific categories in the O category: O1, O2, O3 or O4 according to UNECE/TRANS/WP.29/78/Rev.4 [16]."]
    #[doc = " *"]
    #[doc = " *"]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum EuVehicleCategoryO {
        o1 = 0,
        o2 = 1,
        o3 = 2,
        o4 = 3,
    }
    #[doc = "* "]
    #[doc = " * This DF represents the Euler angles which describe the orientation of an object bounding box in a Cartesian coordinate system with an associated confidence level for each angle."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field zAngle: z-angle of object bounding box at the time of measurement, with the associated confidence."]
    #[doc = " * The angle is measured with positive values considering the object orientation turning around the z-axis using the right-hand rule, starting from the x-axis. "]
    #[doc = " * This extrinsic rotation shall be applied around the centre point of the object bounding box before all other rotations."]
    #[doc = " *"]
    #[doc = " * @field yAngle: optional y-angle of object bounding box at the time of measurement, with the associated confidence."]
    #[doc = " * The angle is measured with positive values considering the object orientation turning around the y-axis using the right-hand rule, starting from the z-axis. "]
    #[doc = " * This extrinsic rotation shall be applied around the centre point of the object bounding box after the rotation by zAngle and before the rotation by xAngle."]
    #[doc = " *"]
    #[doc = " * @field xAngle: optional x-angle of object bounding box at the time of measurement, with the associated confidence."]
    #[doc = " * The angle is measured with positive values considering the object orientation turning around the x-axis using the right-hand rule, starting from the z-axis. "]
    #[doc = " * This extrinsic rotation shall be applied around the centre point of the object bounding box after all other rotations."]
    #[doc = " * "]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct EulerAnglesWithConfidence {
        #[rasn(identifier = "zAngle")]
        pub z_angle: CartesianAngle,
        #[rasn(identifier = "yAngle")]
        pub y_angle: Option<CartesianAngle>,
        #[rasn(identifier = "xAngle")]
        pub x_angle: Option<CartesianAngle>,
    }
    impl EulerAnglesWithConfidence {
        pub fn new(
            z_angle: CartesianAngle,
            y_angle: Option<CartesianAngle>,
            x_angle: Option<CartesianAngle>,
        ) -> Self {
            Self {
                z_angle,
                y_angle,
                x_angle,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * The DF shall contain a list of @ref EventPoint.  "]
    #[doc = " *"]
    #[doc = " * The eventPosition of each @ref EventPoint is defined with respect to the previous @ref EventPoint in the list. "]
    #[doc = " * Except for the first @ref EventPoint which is defined with respect to a position outside of the context of this DF."]
    #[doc = " *"]
    #[doc = " * @category: GeoReference information, Traffic information"]
    #[doc = " * @note: this DF is kept for backwards compatibility reasons only. It is recommended to use the @ref EventZone instead. "]
    #[doc = " * @revision: Generalized the semantics in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=23"))]
    pub struct EventHistory(pub SequenceOf<EventPoint>);
    #[doc = "*"]
    #[doc = " * This DF provides information related to an event at a defined position."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field eventPosition: offset position of a detected event point to a defined position. "]
    #[doc = " * "]
    #[doc = " * @field eventDeltaTime: optional time travelled by the detecting ITS-S since the previous detected event point."]
    #[doc = " * "]
    #[doc = " * @field informationQuality: Information quality of the detection for this event point."]
    #[doc = " * "]
    #[doc = " * @category: GeoReference information, Traffic information"]
    #[doc = " * @revision: generalized the semantics in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct EventPoint {
        #[rasn(identifier = "eventPosition")]
        pub event_position: DeltaReferencePosition,
        #[rasn(identifier = "eventDeltaTime")]
        pub event_delta_time: Option<PathDeltaTime>,
        #[rasn(identifier = "informationQuality")]
        pub information_quality: InformationQuality,
    }
    impl EventPoint {
        pub fn new(
            event_position: DeltaReferencePosition,
            event_delta_time: Option<PathDeltaTime>,
            information_quality: InformationQuality,
        ) -> Self {
            Self {
                event_position,
                event_delta_time,
                information_quality,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * The DF shall contain a list of @ref EventPoint, where all @ref EventPoint either contain the COMPONENT eventDeltaTime "]
    #[doc = " * or do not contain the COMPONENT eventDeltaTime.  "]
    #[doc = " *"]
    #[doc = " * The eventPosition of each @ref EventPoint is defined with respect to the previous @ref EventPoint in the list. "]
    #[doc = " * Except for the first @ref EventPoint which is defined with respect to a position outside of the context of this DF."]
    #[doc = " *"]
    #[doc = " * @category: GeoReference information, Traffic information"]
    #[doc = " * @revision: created in V2.1.1 based on EventHistory"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct EventZone(pub EventHistory);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice)]
    pub enum Ext1 {
        #[rasn(value("128..=16511"), tag(context, 0))]
        content(u16),
        #[rasn(tag(context, 1))]
        extension(Ext2),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice)]
    pub enum Ext2 {
        #[rasn(value("16512..=2113663"), tag(context, 0))]
        content(u32),
        #[rasn(tag(context, 1))]
        extension(Ext3),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("2113664..=270549119", extensible))]
    pub struct Ext3(pub Integer);
    #[doc = "*"]
    #[doc = " * This DE describes the status of the exterior light switches of a vehicle incl. VRU vehicles."]
    #[doc = " *"]
    #[doc = " * The corresponding bit shall be set to 1 under the following conditions:"]
    #[doc = " * - 0 - `lowBeamHeadlightsOn`    - when the low beam head light switch is on,"]
    #[doc = " * - 1 - `highBeamHeadlightsOn`   - when the high beam head light switch is on,"]
    #[doc = " * - 2 - `leftTurnSignalOn`       - when the left turnSignal switch is on,"]
    #[doc = " * - 3 - `rightTurnSignalOn`      - when the right turn signal switch is on,"]
    #[doc = " * - 4 - `daytimeRunningLightsOn` - when the daytime running light switch is on,"]
    #[doc = " * - 5 - `reverseLightOn`         - when the reverse light switch is on,"]
    #[doc = " * - 6 - `fogLightOn`             - when the tail fog light switch is on,"]
    #[doc = " * - 7 - `parkingLightsOn`        - when the parking light switch is on."]
    #[doc = " * "]
    #[doc = " * @note: The value of each bit indicates the state of the switch, which commands the corresponding light."]
    #[doc = " * The bit corresponding to a specific light is set to `1`, when the corresponding switch is turned on,"]
    #[doc = " * either manually by the driver or automatically by a vehicle system. The bit value does not indicate"]
    #[doc = " * if the corresponding lamps are alight or not."]
    #[doc = " * "]
    #[doc = " * If a vehicle is not equipped with a certain light or if the light switch status information is not available,"]
    #[doc = " * the corresponding bit shall be set to `0`."]
    #[doc = " * "]
    #[doc = " * As the bit value indicates only the state of the switch, the turn signal and hazard signal bit values shall not"]
    #[doc = " * alternate with the blinking interval."]
    #[doc = " * "]
    #[doc = " * For hazard indicator, the `leftTurnSignalOn (2)` and `rightTurnSignalOn (3)` shall be both set to `1`."]
    #[doc = " * "]
    #[doc = " * @category Vehicle information"]
    #[doc = " * @revision: Description revised in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct ExteriorLights(pub FixedBitString<8usize>);
    #[doc = "*"]
    #[doc = " * This DF represents the top-level DF to represent a lane position. A lane position is a transversal position on the carriageway at a specific longitudinal position, in resolution of lanes of the carriageway."]
    #[doc = " *"]
    #[doc = " * @note: This DF is the most general way to represent a lane position: it provides a complete set of information regarding a transversal (dimensionless) position on the carriageway at a specific "]
    #[doc = " * reference position, i.e. it provides different options and synonyms to represent the lane at which the reference position (the point) is located. A confidence is used to describe the probability "]
    #[doc = " * that the object is located in the provided lane. The dimension of the object or extension of an area are not considered: See @ref OccupiedLanesWithConfidence for describing the occupation of lanes, "]
    #[doc = " * where the dimensions of an object or the extension of an area is considered."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field lanePositionBased: lane position information for a defined reference position."]
    #[doc = " * "]
    #[doc = " * @field mapBased: optional lane position information described in the context of a MAPEM as specified in ETSI TS 103 301 [15]. "]
    #[doc = " * If present, it shall describe the same reference position using the lane identification in the MAPEM. This component can be used only if a MAPEM is available for the reference position "]
    #[doc = " * (e.g. on an intersection): In this case it is used as a synonym to the mandatory component lanePositionBased. "]
    #[doc = " * "]
    #[doc = " * @field confidence: confidence information for expressing the probability that the object is located at the indicated lane.  "]
    #[doc = " * If the value of the component lanePositionBased is generated directly from the absolute reference position and reference topology information, "]
    #[doc = " * no sensor shall be indicated in the component usedDetectionInformation of the @ref MetaInformation."]
    #[doc = " *"]
    #[doc = " * @category: Road Topology information"]
    #[doc = " * @revision: newly created in V2.2.1. The previous DF GeneralizedLanePosition is now renamed to @ref LanePositionOptions. "]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct GeneralizedLanePosition {
        #[rasn(identifier = "lanePositionBased")]
        pub lane_position_based: LanePositionOptions,
        #[rasn(identifier = "mapBased")]
        pub map_based: Option<MapPosition>,
        pub confidence: MetaInformation,
    }
    impl GeneralizedLanePosition {
        pub fn new(
            lane_position_based: LanePositionOptions,
            map_based: Option<MapPosition>,
            confidence: MetaInformation,
        ) -> Self {
            Self {
                lane_position_based,
                map_based,
                confidence,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DF represents transversal position information with respect to the road, at an externally defined reference position. It shall contain a set of up to `4` @ref GeneralizedLanePosition."]
    #[doc = " * Multiple entries can be used to describe several lane positions with the associated confidence, in cases where the reference position cannot be mapped to a single lane."]
    #[doc = " *"]
    #[doc = " * @category: Road Topology information"]
    #[doc = " * @revision: Created in V2.2.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct GeneralizedLanePositions(pub SequenceOf<GeneralizedLanePosition>);
    #[doc = "*"]
    #[doc = " * This DE represents a timestamp based on TimestampIts modulo 65 536."]
    #[doc = " * This means that generationDeltaTime = TimestampIts mod 65 536."]
    #[doc = " *"]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Created in V2.1.1 based on ETSI TS 103 900 [1]"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=65535"))]
    pub struct GenerationDeltaTime(pub u16);
    #[doc = "* "]
    #[doc = " * This DF indicates a geographical position."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field latitude: the latitude of the geographical position."]
    #[doc = " *"]
    #[doc = " * @field longitude: the longitude of the geographical position."]
    #[doc = " *"]
    #[doc = " * @field altitude: the altitude of the geographical position with default value unavailable."]
    #[doc = " *"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct GeoPosition {
        pub latitude: Latitude,
        pub longitude: Longitude,
        #[rasn(default = "geo_position_altitude_default")]
        pub altitude: AltitudeValue,
    }
    impl GeoPosition {
        pub fn new(latitude: Latitude, longitude: Longitude, altitude: AltitudeValue) -> Self {
            Self {
                latitude,
                longitude,
                altitude,
            }
        }
    }
    fn geo_position_altitude_default() -> AltitudeValue {
        AltitudeValue(800001)
    }
    #[doc = "*"]
    #[doc = "* This DE indicates a geographical position with altitude."]
    #[doc = "*"]
    #[doc = "* It shall include the following components: "]
    #[doc = "*"]
    #[doc = "* @field latitude: the latitude of the geographical position."]
    #[doc = "*"]
    #[doc = "* @field longitude: the longitude of the geographical position."]
    #[doc = "*"]
    #[doc = "* @field altitude: the altitude of the geographical position."]
    #[doc = "*"]
    #[doc = "* @category: GeoReference information"]
    #[doc = "* @revision: created in V2.3.1 based on ISO TS 19321 (DF AbsolutePositionWAltitude)"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct GeoPositionWAltitude {
        pub latitude: Latitude,
        pub longitude: Longitude,
        pub altitude: Altitude,
    }
    impl GeoPositionWAltitude {
        pub fn new(latitude: Latitude, longitude: Longitude, altitude: Altitude) -> Self {
            Self {
                latitude,
                longitude,
                altitude,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* This DF indicates a geographical position without altitude."]
    #[doc = "*"]
    #[doc = "* It shall include the following components: "]
    #[doc = "*"]
    #[doc = "* @field latitude: the latitude of the geographical position."]
    #[doc = "*"]
    #[doc = "* @field longitude: the longitude of the geographical position."]
    #[doc = "*"]
    #[doc = "* @category: GeoReference information"]
    #[doc = "* @revision: created in V2.3.1 based on ISO TS 19321 (DF AbsolutePosition)"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct GeoPositionWoAltitude {
        pub latitude: Latitude,
        pub longitude: Longitude,
    }
    impl GeoPositionWoAltitude {
        pub fn new(latitude: Latitude, longitude: Longitude) -> Self {
            Self {
                latitude,
                longitude,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* This DF shall contain a list of @ref AbsolutePositionWAltitude."]
    #[doc = "*"]
    #[doc = "* @category: GeoReference information"]
    #[doc = "* @revision: created in V2.3.1 based on ISO TS 19321 (DF AbsolutePositionsWAltitude)"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8", extensible))]
    pub struct GeoPositionsWAltitude(pub SequenceOf<GeoPositionWAltitude>);
    #[doc = "*"]
    #[doc = "* This DF shall contain a list of @ref GeoPositionWoAltitude."]
    #[doc = "*"]
    #[doc = "* @category: GeoReference information"]
    #[doc = "* @revision: created in V2.3.1 based on ISO TS 19321 (AbsolutePositions)"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8", extensible))]
    pub struct GeoPositionsWoAltitude(pub SequenceOf<GeoPositionWoAltitude>);
    #[doc = "*"]
    #[doc = " * This DE indicates the current status of a hard shoulder: whether it is available for special usage"]
    #[doc = " * (e.g. for stopping or for driving) or closed for all vehicles."]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 - `availableForStopping` - if the hard shoulder is available for stopping in e.g. emergency situations,"]
    #[doc = " * - 1 - `closed`               - if the hard shoulder is closed and cannot be occupied in any case,"]
    #[doc = " * - 2 - `availableForDriving`  - if the hard shoulder is available for regular driving."]
    #[doc = " *"]
    #[doc = " * @category: Traffic information"]
    #[doc = " * @revision: Description revised in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum HardShoulderStatus {
        availableForStopping = 0,
        closed = 1,
        availableForDriving = 2,
    }
    #[doc = "*"]
    #[doc = " * This DE represents the value of the sub cause code of the @ref CauseCode `hazardousLocation-AnimalOnTheRoad`."]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 - `unavailable`          - in case further detailed information on the animal(s) on the road is unavailable,"]
    #[doc = " * - 1 - `wildAnimals`          - in case wild animals of unknown size are present on the road,"]
    #[doc = " * - 2 - `herdOfAnimals`        - in case a herd of animals is present on the road,"]
    #[doc = " * - 3 - `smallAnimals`         - in case small size animals of unknown type are present on the road,"]
    #[doc = " * - 4 - `largeAnimals`         - in case large size animals of unknown type are present on the road,"]
    #[doc = " * - 5 - `wildAnimalsSmall`     - in case small size wild animal(s) are present on the road,"]
    #[doc = " * - 6 - `wildAnimalsLarge`     - in case large size wild animal(s) are present on the road,"]
    #[doc = " * - 7 - `domesticAnimals`      - in case domestic animal(s) of unknown size are detected on the road,"]
    #[doc = " * - 8 - `domesticAnimalsSmall` - in case small size domestic animal(s) are present on the road,"]
    #[doc = " * - 9 - `domesticAnimalsLarge` - in case large size domestic animal(s) are present on the road."]
    #[doc = " * - 10-255                     - are reserved for future usage."]
    #[doc = " *"]
    #[doc = " * @category: Traffic information"]
    #[doc = " * @revision: V1.3.1, named values 5 to 9 added in V2.2.1 "]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        identifier = "HazardousLocation-AnimalOnTheRoadSubCauseCode",
        value("0..=255")
    )]
    pub struct HazardousLocationAnimalOnTheRoadSubCauseCode(pub u8);
    #[doc = "*"]
    #[doc = " * This DE represents the sub cause code of the @ref CauseCode  `hazardousLocation-DangerousCurve`."]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 - `unavailable`                                        - in case further detailed information on the dangerous curve is unavailable,"]
    #[doc = " * - 1 - `dangerousLeftTurnCurve`                             - in case the dangerous curve is a left turn curve,"]
    #[doc = " * - 2 - `dangerousRightTurnCurve`                            - in case the dangerous curve is a right turn curve,"]
    #[doc = " * - 3 - `multipleCurvesStartingWithUnknownTurningDirection`  - in case of multiple curves for which the starting curve turning direction is not known,"]
    #[doc = " * - 4 - `multipleCurvesStartingWithLeftTurn`                 - in case of multiple curves starting with a left turn curve,"]
    #[doc = " * - 5 - `multipleCurvesStartingWithRightTurn`                - in case of multiple curves starting with a right turn curve."]
    #[doc = " * - 6-255                                                    - are reserved for future usage."]
    #[doc = " * "]
    #[doc = " * The definition of whether a curve is dangerous may vary according to region and according to vehicle types/mass"]
    #[doc = " * and vehicle speed driving on the curve. This definition is out of scope of the present document."]
    #[doc = " *"]
    #[doc = " * @category: Traffic information"]
    #[doc = " * @revision: V1.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        identifier = "HazardousLocation-DangerousCurveSubCauseCode",
        value("0..=255")
    )]
    pub struct HazardousLocationDangerousCurveSubCauseCode(pub u8);
    #[doc = "*"]
    #[doc = " * This DE represents the value of the sub cause code of the @ref CauseCode `hazardousLocation-ObstacleOnTheRoad`. "]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 - `unavailable`                - in case further detailed information on the detected obstacle is unavailable,"]
    #[doc = " * - 1 - `shedLoad`                   - in case detected obstacle is large amount of obstacles (shedload),"]
    #[doc = " * - 2 - `partsOfVehicles`            - in case detected obstacles are parts of vehicles,"]
    #[doc = " * - 3 - `partsOfTyres`               - in case the detected obstacles are parts of tyres,"]
    #[doc = " * - 4 - `bigObjects`                 - in case the detected obstacles are big objects,"]
    #[doc = " * - 5 - `fallenTrees`                - in case the detected obstacles are fallen trees,"]
    #[doc = " * - 6 - `hubCaps`                    - in case the detected obstacles are hub caps,"]
    #[doc = " * - 7 - `waitingVehicles-deprecated` - deprecated since not representing an obstacle and already covered by StationaryVehicleSubCauseCode."]
    #[doc = " * - 8-255                - are reserved for future usage."]
    #[doc = " *"]
    #[doc = " * @category: Traffic information"]
    #[doc = " * @revision: V1.3.1, value 7 deprecated in V2.4.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        identifier = "HazardousLocation-ObstacleOnTheRoadSubCauseCode",
        value("0..=255")
    )]
    pub struct HazardousLocationObstacleOnTheRoadSubCauseCode(pub u8);
    #[doc = "*"]
    #[doc = " * This DE represents the value of the sub cause code of the @ref CauseCode `hazardousLocation-SurfaceCondition`. "]
    #[doc = " * "]
    #[doc = "The value shall be set to:"]
    #[doc = " * - 0  - `unavailable`                  - in case further detailed information on the road surface condition is unavailable,"]
    #[doc = " * - 1  - `rockfalls`                    - in case rock falls have fallen on the road surface,"]
    #[doc = " * - 2  - `earthquakeDamage-deprecated`  - deprecated since it is covered by 1 rockfalls and 4 subsidence,"]
    #[doc = " * - 3  - `sinkhole`                     - in case of a partial collapse of the road surface that creates a depression (hole) in the pavement,"]
    #[doc = " * - 4  - `subsidence`                   - in case road surface is damaged by subsidence,"]
    #[doc = " * - 5  - `snowDrifts-deprecated`        - deprecated since not representing a road damage,"]
    #[doc = " * - 6  - `stormDamage-deprecated`       - deprecated since not representing a road damage,"]
    #[doc = " * - 7  - `burstPipe-deprecated`         - deprecated since not representing a road damage,"]
    #[doc = " * - 8  - `lava           `              - in case road surface is damaged due to  lava on the road,"]
    #[doc = " * - 9  - `fallingIce-deprecated`        - deprecated since not representing a road damage,"]
    #[doc = " * - 10 - `fire`                         - in case there is fire on or near to the road surface,"]
    #[doc = " * - 11 - `flooding-deprecated`          - deprecated since not representing a road damage."]
    #[doc = " * - 12 - `wearAndTear`                  - in case the road surface is damaged by wear and tear."]
    #[doc = " * - 13-255                              - are reserved for future usage."]
    #[doc = " *"]
    #[doc = " * @category: Traffic information"]
    #[doc = " * @revision: V1.3.1, value 10 added in V2.1.1, value 11 added in V2.3.1, "]
    #[doc = "              name and definition of value 3 and 8 changed in V2.4.1, values 2, 5, 6, 7, 9 and 11 deprecated in V2.4.1, vaue 12 added in V2.4.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        identifier = "HazardousLocation-SurfaceConditionSubCauseCode",
        value("0..=255")
    )]
    pub struct HazardousLocationSurfaceConditionSubCauseCode(pub u8);
    #[doc = "*"]
    #[doc = " * This DF represents the Heading in a WGS84 co-ordinates system."]
    #[doc = " * The specific WGS84 coordinate system is specified by the corresponding standards applying this DE."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field headingValue: the heading value."]
    #[doc = " * "]
    #[doc = " * @field headingConfidence: the confidence value of the heading value with a predefined confidence level."]
    #[doc = " * "]
    #[doc = " * @note: this DF is kept for backwards compatibility reasons only. It is recommended to use the @ref Wgs84Angle instead. "]
    #[doc = " * @category: Kinematic Information"]
    #[doc = " * @revision: Description revised in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct Heading {
        #[rasn(identifier = "headingValue")]
        pub heading_value: HeadingValue,
        #[rasn(identifier = "headingConfidence")]
        pub heading_confidence: HeadingConfidence,
    }
    impl Heading {
        pub fn new(heading_value: HeadingValue, heading_confidence: HeadingConfidence) -> Self {
            Self {
                heading_value,
                heading_confidence,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DF  provides  information  associated to heading  change indicators  such as  a  change  of  direction."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field direction: the direction of heading change value."]
    #[doc = " * "]
    #[doc = " * @field actionDeltaTime: the period over which a direction change action is performed. "]
    #[doc = " * "]
    #[doc = " * @category: Kinematic Information"]
    #[doc = " * @revision: created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct HeadingChangeIndication {
        pub direction: TurningDirection,
        #[rasn(identifier = "actionDeltaTime")]
        pub action_delta_time: DeltaTimeTenthOfSecond,
    }
    impl HeadingChangeIndication {
        pub fn new(direction: TurningDirection, action_delta_time: DeltaTimeTenthOfSecond) -> Self {
            Self {
                direction,
                action_delta_time,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DE indicates the heading confidence value which represents the estimated absolute accuracy of a heading value with a confidence level of 95 %."]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `n` (`n > 0` and `n < 126`) if the confidence value is equal to or less than n x 0,1 degree and more than (n-1) x 0,1 degree,"]
    #[doc = " * - `126` if the confidence value is out of range, i.e. greater than 12,5 degrees,"]
    #[doc = " * - `127` if the confidence value information is not available."]
    #[doc = " * "]
    #[doc = " * @note:\tThe fact that a value is received with confidence value set to `unavailable(127)` can be caused by several reasons,"]
    #[doc = " * such as:"]
    #[doc = " * - the sensor cannot deliver the accuracy at the defined confidence level because it is a low-end sensor,"]
    #[doc = " * - the sensor cannot calculate the accuracy due to lack of variables, or"]
    #[doc = " * - there has been a vehicle bus (e.g. CAN bus) error."]
    #[doc = " * In all 3 cases above, the heading value may be valid and used by the application."]
    #[doc = " *"]
    #[doc = " * @note: If a heading value is received and its confidence value is set to `outOfRange(126)`, it means that the "]
    #[doc = " * heading value is not valid and therefore cannot be trusted. Such value is not useful for the application."]
    #[doc = " * @note: this DE is kept for backwards compatibility reasons only. It is recommended to use the @ref Wgs84AngleConfidence instead. "]
    #[doc = " * "]
    #[doc = " * @unit: 0,1 degree"]
    #[doc = " * @category: GeoReference information"]
    #[doc = " * @revision: Description revised in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=127"))]
    pub struct HeadingConfidence(pub u8);
    #[doc = "*"]
    #[doc = " * This DE represents the orientation of the horizontal velocity vector with regards to the WGS84 north."]
    #[doc = " * When the information is not available, the DE shall be set to 3 601. The value 3600 shall not be used."]
    #[doc = " *"]
    #[doc = " * @note: this DE is kept for backwards compatibility reasons only. It is recommended to use the @ref Wgs84AngleValue instead. "]
    #[doc = " *"]
    #[doc = " * Unit: 0,1 degree"]
    #[doc = " * Categories: GeoReference information"]
    #[doc = " * @revision: Description revised in V2.1.1 (usage of value 3600 specified) "]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=3601"))]
    pub struct HeadingValue(pub u16);
    #[doc = "*"]
    #[doc = " * This DE represents the height of the left or right longitude carrier of vehicle from base to top (left or right carrier seen from vehicle"]
    #[doc = " * rear to front). "]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `n` (`n >= 1` and `n < 99`) if the height information is equal to or less than n x 0,01 metre and more than (n-1) x 0,01 metre,"]
    #[doc = " * - `99` if the height is out of range, i.e. equal to or greater than 0,98 m,"]
    #[doc = " * - `100` if the height information is not available."]
    #[doc = " *"]
    #[doc = " * @unit 0,01 metre"]
    #[doc = " * @category Vehicle information"]
    #[doc = " * @revision: Description revised in V2.1.1 (the definition of 99 has changed slightly) "]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=100"))]
    pub struct HeightLonCarr(pub u8);
    #[doc = "*"]
    #[doc = " * This DE represents the value of the sub cause code of the @ref CauseCode `humanPresenceOnTheRoad`."]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 - `unavailable`                    - in case further detailed information abou the human presence on the road is unavailable,"]
    #[doc = " * - 1 - `childrenOnRoadway`              - in case children are present on the road,"]
    #[doc = " * - 2 - `cyclistOnRoadway`               - in case cyclist(s) are present on the road,"]
    #[doc = " * - 3 - `motorcyclistOnRoadway`          - in case motorcyclist(s) are present on the road,"]
    #[doc = " * - 4 - `pedestrian`                     - in case pedestrian(s) of any type are present on the road,"]
    #[doc = " * - 5 - `ordinary-pedestrian`            - in case pedestrian(s) to which no more-specific profile applies are present on the road,"]
    #[doc = " * - 6 - `road-worker`                    - in case pedestrian(s) with the role of a road worker applies are present on the road,"]
    #[doc = " * - 7 - `first-responder`                - in case pedestrian(s) with the role of a first responder applies are present on the road,  "]
    #[doc = " * - 8 - `lightVruVehicle                 - in case light vru vehicle(s) of any type are present on the road,"]
    #[doc = " * - 9 - `bicyclist `                     - in case cycle(s) and their bicyclist(s) are present on the road,"]
    #[doc = " * - 10 - `wheelchair-user`               - in case wheelchair(s) and their user(s) are present on the road,"]
    #[doc = " * - 11 - `horse-and-rider`               - in case horse(s) and rider(s) are present on the road,"]
    #[doc = " * - 12 - `rollerskater`                  - in case rolleskater(s) and skater(s) are present on the road,"]
    #[doc = " * - 13 - `e-scooter`                     - in case e-scooter(s) and rider(s) are present on the road,"]
    #[doc = " * - 14 - `personal-transporter`          - in case personal-transporter(s) and rider(s) are present on the road,"]
    #[doc = " * - 15 - `pedelec`                       - in case pedelec(s) and rider(s) are present on the road,"]
    #[doc = " * - 16 - `speed-pedelec`                 - in case speed-pedelec(s) and rider(s) are present on the road,"]
    #[doc = " * - 17 - `ptw`                           - in case powered-two-wheeler(s) of any type are present on the road,"]
    #[doc = " * - 18 - `moped`                         - in case moped(s) and rider(s) are present on the road,"]
    #[doc = " * - 19 - `motorcycle`                    - in case motorcycle(s) and rider(s) are present on the road,"]
    #[doc = " * - 20 - `motorcycle-and-sidecar-right`  - in case motorcycle(s) with sidecar(s) on the right and rider are present on the road,"]
    #[doc = " * - 21 - `motorcycle-and-sidecar-left`   - in case motorcycle(s) with sidecar(s) on the left and rider are present on the road."]
    #[doc = " * - 22-255                               - are reserved for future usage."]
    #[doc = " *"]
    #[doc = " * @category: Traffic information"]
    #[doc = " * @revision: editorial revision in V2.1.1, named values 4-21 added in V2.2.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct HumanPresenceOnTheRoadSubCauseCode(pub u8);
    #[doc = "*"]
    #[doc = " * This DE represents the value of the sub cause codes of the @ref CauseCode \"humanProblem\"."]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 - `unavailable`        - in case further detailed information on human health problem is unavailable,"]
    #[doc = " * - 1 - `glycemiaProblem`    - in case human problem is due to glycaemia problem,"]
    #[doc = " * - 2 - `heartProblem`       - in case human problem is due to heart problem,"]
    #[doc = " * - 3 - `unresponsiveDriver` - in case an unresponsive driver is detected."]
    #[doc = " * - 3-255                    - reserved for future usage."]
    #[doc = " *"]
    #[doc = " * @category: Traffic information"]
    #[doc = " * @revision: V1.3.1, value 3 assigned in V2.4.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct HumanProblemSubCauseCode(pub u8);
    #[doc = "* "]
    #[doc = " * This DE is a general identifier."]
    #[doc = " * "]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct Identifier1B(pub u8);
    #[doc = "* "]
    #[doc = " * This DE is a general identifier."]
    #[doc = " * "]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=65535"))]
    pub struct Identifier2B(pub u16);
    #[doc = "*"]
    #[doc = " * This DE represents the value of the sub cause codes of the @ref CauseCode `impassability`"]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 `unavailable`              - in case further detailed information about the unmanaged road blockage is unavailable,"]
    #[doc = " * - 1 `flooding          `       - in case the road is affected by flooding,"]
    #[doc = " * - 2 `dangerOfAvalanches`       - in case the road is at risk of being affected or blocked by avalanches,"]
    #[doc = " * - 3 `blastingOfAvalanches`     - in case there is an active blasting of avalanches on or near the road,"]
    #[doc = " * - 4 `landslips`                - in case the road is affected by landslips,"]
    #[doc = " * - 5 `chemicalSpillage`         - in case the road is affected by chemical spillage,"]
    #[doc = " * - 6 `winterClosure`            - in case the road is impassable due to a winter closure."]
    #[doc = " * - 7 `sinkhole`                 - in case the road is impassable due to large holes in the road surface."]
    #[doc = " * - 8 `earthquakeDamage`         - in case the road is obstructed or partially obstructed because of damage caused by an earthquake."]
    #[doc = " * - 9 `fallenTrees`              - in case the road is obstructed or partially obstructed by one or more fallen trees. "]
    #[doc = " * - 10 `rockfalls`               - in case the road is obstructed or partially obstructed due to fallen rocks."]
    #[doc = " * - 11 `sewerOverflow`           - in case the road is obstructed or partially obstructed by overflows from one or more sewers. "]
    #[doc = " * - 12 `stormDamage`             - in case the road is obstructed or partially obstructed by debris caused by strong winds."]
    #[doc = " * - 13 `subsidence`              - in case the road surface has sunken or collapsed in places."]
    #[doc = " * - 14 `burstPipe`               - in case the road surface has sunken or collapsed in places due to burst pipes."]
    #[doc = " * - 15 `burstWaterMain`          - in case the road is obstructed due to local flooding and/or subsidence. "]
    #[doc = " * - 16 `fallenPowerCables`       - in case the road is obstructed or partly obstructed by one or more fallen power cables."]
    #[doc = " * - 17 `snowDrifts`              - in case the road is obstructed or partially obstructed by snow drifting in progress or patches of deep snow due to earlier drifting."]
    #[doc = " * - 15-255                       - are reserved for future usage."]
    #[doc = " *"]
    #[doc = " * @category: Traffic information"]
    #[doc = " * @revision: Created in V2.2.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct ImpassabilitySubCauseCode(pub u8);
    #[doc = "*"]
    #[doc = " * This DE represents the quality level of provided information."]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `0` if the information is unavailable,"]
    #[doc = " * - `1` if the quality level is lowest,"]
    #[doc = " * - `n` (`n > 1` and `n < 7`) if the quality level is n, "]
    #[doc = " * - `7` if the quality level is highest."]
    #[doc = " *"]
    #[doc = " * @note: Definition of quality level is out of scope of the present document."]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Editorial update in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=7"))]
    pub struct InformationQuality(pub u8);
    #[doc = "*"]
    #[doc = " * This DF represents a frequency channel "]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field centreFrequency: the centre frequency of the channel in 10^(exp+2) Hz (where exp is exponent)"]
    #[doc = " * "]
    #[doc = " * @field channelWidth: width of the channel in 10^exp Hz (where exp is exponent)"]
    #[doc = " *"]
    #[doc = " * @field exponent: exponent of the power of 10 used in the calculation of the components above."]
    #[doc = " *"]
    #[doc = " * @category: Communication information"]
    #[doc = " * @revision: created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct InterferenceManagementChannel {
        #[rasn(value("1..=99999"), identifier = "centreFrequency")]
        pub centre_frequency: u32,
        #[rasn(value("0..=9999"), identifier = "channelWidth")]
        pub channel_width: u16,
        #[rasn(value("0..=15"))]
        pub exponent: u8,
    }
    impl InterferenceManagementChannel {
        pub fn new(centre_frequency: u32, channel_width: u16, exponent: u8) -> Self {
            Self {
                centre_frequency,
                channel_width,
                exponent,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DF shall contain a list of up to 16 definitions containing interference management information, per affected frequency channels."]
    #[doc = " *  "]
    #[doc = " * @category: Communication information."]
    #[doc = " * @revision: created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16", extensible))]
    pub struct InterferenceManagementInfo(pub SequenceOf<InterferenceManagementInfoPerChannel>);
    #[doc = "*"]
    #[doc = " * This DF contains interference management information for one affected frequency channel."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field interferenceManagementChannel: frequency channel for which the zone should be applied interference management "]
    #[doc = " *"]
    #[doc = " * @field interferenceManagementZoneType: type of the interference management zone. "]
    #[doc = " *"]
    #[doc = " * @field interferenceManagementMitigationType: optional type of the mitigation to be used in the interference management zone. "]
    #[doc = " * In the case where no mitigation should be applied by the ITS-S, this is indicated by the field interferenceManagementMitigationType being absent."]
    #[doc = " *"]
    #[doc = " * @field expiryTime: optional time at which the validity of the interference management communication zone will expire. "]
    #[doc = " * This component is present when the interference management is temporarily valid"]
    #[doc = " *"]
    #[doc = " * @category: Communication information"]
    #[doc = " * @revision: created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct InterferenceManagementInfoPerChannel {
        #[rasn(identifier = "interferenceManagementChannel")]
        pub interference_management_channel: InterferenceManagementChannel,
        #[rasn(identifier = "interferenceManagementZoneType")]
        pub interference_management_zone_type: InterferenceManagementZoneType,
        #[rasn(identifier = "interferenceManagementMitigationType")]
        pub interference_management_mitigation_type: Option<MitigationForTechnologies>,
        #[rasn(identifier = "expiryTime")]
        pub expiry_time: Option<TimestampIts>,
    }
    impl InterferenceManagementInfoPerChannel {
        pub fn new(
            interference_management_channel: InterferenceManagementChannel,
            interference_management_zone_type: InterferenceManagementZoneType,
            interference_management_mitigation_type: Option<MitigationForTechnologies>,
            expiry_time: Option<TimestampIts>,
        ) -> Self {
            Self {
                interference_management_channel,
                interference_management_zone_type,
                interference_management_mitigation_type,
                expiry_time,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * "]
    #[doc = " * This DF represents a zone  inside which the ITS communication should be restricted in order to manage interference."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field zoneDefinition: contains the geographical definition of the zone."]
    #[doc = " *"]
    #[doc = " * @field managementInfo: contains interference management information applicable in the zone defined in the component zoneDefinition."]
    #[doc = " *"]
    #[doc = " * @category: Communication information"]
    #[doc = " * @revision: created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct InterferenceManagementZone {
        #[rasn(identifier = "zoneDefinition")]
        pub zone_definition: InterferenceManagementZoneDefinition,
        #[rasn(identifier = "managementInfo")]
        pub management_info: InterferenceManagementInfo,
    }
    impl InterferenceManagementZone {
        pub fn new(
            zone_definition: InterferenceManagementZoneDefinition,
            management_info: InterferenceManagementInfo,
        ) -> Self {
            Self {
                zone_definition,
                management_info,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DF represents the geographical definition of the zone where band sharing occurs. "]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field interferenceManagementZoneLatitude: Latitude of the centre point of the interference management zone."]
    #[doc = " *"]
    #[doc = " * @field interferenceManagementZoneLongitude: Longitude of the centre point of the interference management zone."]
    #[doc = " *"]
    #[doc = " * @field interferenceManagementZoneId: optional identification of the interference management zone. "]
    #[doc = " *"]
    #[doc = " * @field interferenceManagementZoneShape: shape of the interference management zone placed at the centre point. "]
    #[doc = " *"]
    #[doc = " * @category: Communication information"]
    #[doc = " * @revision: created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct InterferenceManagementZoneDefinition {
        #[rasn(identifier = "interferenceManagementZoneLatitude")]
        pub interference_management_zone_latitude: Latitude,
        #[rasn(identifier = "interferenceManagementZoneLongitude")]
        pub interference_management_zone_longitude: Longitude,
        #[rasn(identifier = "interferenceManagementZoneId")]
        pub interference_management_zone_id: Option<ProtectedZoneId>,
        #[rasn(value("0.."), identifier = "interferenceManagementZoneShape")]
        pub interference_management_zone_shape: Option<Shape>,
    }
    impl InterferenceManagementZoneDefinition {
        pub fn new(
            interference_management_zone_latitude: Latitude,
            interference_management_zone_longitude: Longitude,
            interference_management_zone_id: Option<ProtectedZoneId>,
            interference_management_zone_shape: Option<Shape>,
        ) -> Self {
            Self {
                interference_management_zone_latitude,
                interference_management_zone_longitude,
                interference_management_zone_id,
                interference_management_zone_shape,
            }
        }
    }
    #[doc = "* "]
    #[doc = " * This DE defines the type of an interference management zone, so that an ITS-S can "]
    #[doc = " * assert the actions to do while passing by such zone (e.g. reduce the transmit power in case of a DSRC tolling station)."]
    #[doc = " * It is an extension of the type @ref ProtectedZoneType."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 - `permanentCenDsrcTolling` - as specified in ETSI TS 102 792 [14],"]
    #[doc = " * - 1 - `temporaryCenDsrcTolling` - as specified in ETSI TS 102 792 [14],"]
    #[doc = " * - 2 - `unavailable`             - default value. Set to 2 for backwards compatibility with DSRC tolling,"]
    #[doc = " * - 3 - `urbanRail`               - as specified in ETSI TS 103 724 [13], clause 7,"]
    #[doc = " * - 4 - `satelliteStation`        - as specified in ETSI TS 103 724 [13], clause 7,"]
    #[doc = " * - 5 - `fixedLinks`              - as specified in ETSI TS 103 724 [13], clause 7."]
    #[doc = " *"]
    #[doc = " * @category: Communication information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum InterferenceManagementZoneType {
        permanentCenDsrcTolling = 0,
        temporaryCenDsrcTolling = 1,
        unavailable = 2,
        urbanRail = 3,
        satelliteStation = 4,
        fixedLinks = 5,
    }
    #[doc = "*"]
    #[doc = " * This DF shall contain a list of up to 16 interference  management zones.  "]
    #[doc = " *"]
    #[doc = " * **EXAMPLE**: An interference management communication zone may be defined around a CEN DSRC road side equipment or an urban rail operational area."]
    #[doc = " * "]
    #[doc = " * @category: Communication information"]
    #[doc = " * @revision: created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16", extensible))]
    pub struct InterferenceManagementZones(pub SequenceOf<InterferenceManagementZone>);
    #[doc = "*"]
    #[doc = " * This DF represents a unique id for an intersection, in accordance with ETSI TS 103 301 [15]."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field region: the optional identifier of the entity that is responsible for the region in which the intersection is placed."]
    #[doc = " * It is the duty of that entity to guarantee that the @ref Id is unique within the region."]
    #[doc = " *"]
    #[doc = " * @field id: the identifier of the intersection"]
    #[doc = " *"]
    #[doc = " * @note: when the component region is present, the IntersectionReferenceId is guaranteed to be globally unique."]
    #[doc = " * @category: Road topology information"]
    #[doc = " * @revision: created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct IntersectionReferenceId {
        pub region: Option<Identifier2B>,
        pub id: Identifier2B,
    }
    impl IntersectionReferenceId {
        pub fn new(region: Option<Identifier2B>, id: Identifier2B) -> Self {
            Self { region, id }
        }
    }
    #[doc = "*"]
    #[doc = " * This DE represents the vehicle type according to ISO 3833 [22]."]
    #[doc = " * A \"term No\" refers to the number of the corresponding term and its definition in ISO 3833."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0\t- `passengerCar`              - term No 3.1.1"]
    #[doc = " * - 1\t- `saloon`                    - term No 3.1.1.1 (sedan)"]
    #[doc = " * - 2\t- `convertibleSaloon`         - term No 3.1.1.2"]
    #[doc = " * - 3\t- `pullmanSaloon`             - term No 3.1.1.3"]
    #[doc = " * - 4\t- `stationWagon`              - term No 3.1.1.4"]
    #[doc = " * - 5\t- `truckStationWagon`         - term No 3.1.1.4.1"]
    #[doc = " * - 6\t- `coupe`                     - term No 3.1.1.5 (coupe)"]
    #[doc = " * - 7\t- `convertible`               - term No 3.1.1.6 (open tourer, roadstar, spider)"]
    #[doc = " * - 8\t- `multipurposePassengerCar`  - term No 3.1.1.7"]
    #[doc = " * - 9\t- `forwardControlPassengerCar`- term No 3.1.1.8"]
    #[doc = " * - 10\t- `specialPassengerCar`       - term No 3.1.1.9"]
    #[doc = " * - 11\t- `bus`                       - term No 3.1.2"]
    #[doc = " * - 12\t- `minibus`                   - term No 3.1.2.1"]
    #[doc = " * - 13\t- `urbanBus`                  - term No 3.1.2.2"]
    #[doc = " * - 14\t- `interurbanCoach`           - term No 3.1.2.3"]
    #[doc = " * - 15\t- `longDistanceCoach`         - term No 3.1.2.4"]
    #[doc = " * - 16\t- `articulatedBus`            - term No 3.1.2.5"]
    #[doc = " * - 17\t- `trolleyBus\t`             - term No 3.1.2.6"]
    #[doc = " * - 18\t- `specialBus`                - term No 3.1.2.7"]
    #[doc = " * - 19\t- `commercialVehicle`         - term No 3.1.3"]
    #[doc = " * - 20\t- `specialCommercialVehicle`  - term No 3.1.3.1"]
    #[doc = " * - 21\t- `specialVehicle`            - term No 3.1.4"]
    #[doc = " * - 22\t- `trailingTowingVehicle`     - term No 3.1.5 (draw-bar tractor)"]
    #[doc = " * - 23\t- `semiTrailerTowingVehicle`  - term No 3.1.6 (fifth wheel tractor)"]
    #[doc = " * - 24\t- `trailer`                   - term No 3.2.1"]
    #[doc = " * - 25\t- `busTrailer`                - term No 3.2.1.1"]
    #[doc = " * - 26\t- `generalPurposeTrailer`     - term No 3.2.1.2"]
    #[doc = " * - 27\t- `caravan`                   - term No 3.2.1.3"]
    #[doc = " * - 28\t- `specialTrailer`            - term No 3.2.1.4"]
    #[doc = " * - 29\t- `semiTrailer`               - term No 3.2.2"]
    #[doc = " * - 30\t- `busSemiTrailer`            - term No 3.2.2.1"]
    #[doc = " * - 31\t- `generalPurposeSemiTrailer` - term No 3.2.2.2"]
    #[doc = " * - 32\t- `specialSemiTrailer`        - term No 3.2.2.3"]
    #[doc = " * - 33\t- `roadTrain`                 - term No 3.3.1"]
    #[doc = " * - 34\t- `passengerRoadTrain`        - term No 3.3.2"]
    #[doc = " * - 35\t- `articulatedRoadTrain`      - term No 3.3.3"]
    #[doc = " * - 36\t- `doubleRoadTrain`           - term No 3.3.4"]
    #[doc = " * - 37\t- `compositeRoadTrain`        - term No 3.3.5"]
    #[doc = " * - 38\t- `specialRoadTrain`          - term No 3.3.6"]
    #[doc = " * - 39\t- `moped`                     - term No 3.4"]
    #[doc = " * - 40\t- `motorCycle`                - term No 3.5"]
    #[doc = " * - 41-255                           - reserved for future use"]
    #[doc = " * "]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct Iso3833VehicleType(pub u8);
    #[doc = "* "]
    #[doc = " * This DE represent the identifier of an organization according to the applicable registry."]
    #[doc = " *"]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Created in V2.2.1 based on ISO 14816 [23]"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=16383"))]
    pub struct IssuerIdentifier(pub u16);
    #[doc = "*"]
    #[doc = " * This DF shall contain  a list of waypoints @ref ReferencePosition."]
    #[doc = " *"]
    #[doc = " * @category: GeoReference information"]
    #[doc = " * @revision: Editorial update in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=40"))]
    pub struct ItineraryPath(pub SequenceOf<ReferencePosition>);
    #[doc = "*"]
    #[doc = " * This DF represents a common message header for application and facilities layer messages."]
    #[doc = " * It is included at the beginning of an ITS message as the message header."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field protocolVersion: version of the ITS message."]
    #[doc = " *"]
    #[doc = " * @field messageId: type of the ITS message."]
    #[doc = " *"]
    #[doc = " * @field stationId: the identifier of the ITS-S that generated the ITS message."]
    #[doc = " *"]
    #[doc = " * @category: Communication information"]
    #[doc = " * @revision:  update in V2.1.1: messageID and stationID changed to messageId and stationId; messageId is of type MessageId."]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct ItsPduHeader {
        #[rasn(identifier = "protocolVersion")]
        pub protocol_version: OrdinalNumber1B,
        #[rasn(identifier = "messageId")]
        pub message_id: MessageId,
        #[rasn(identifier = "stationId")]
        pub station_id: StationId,
    }
    impl ItsPduHeader {
        pub fn new(
            protocol_version: OrdinalNumber1B,
            message_id: MessageId,
            station_id: StationId,
        ) -> Self {
            Self {
                protocol_version,
                message_id,
                station_id,
            }
        }
    }
    #[doc = "* "]
    #[doc = " * This DE represents the identifier of the IVIM."]
    #[doc = " *"]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Created in V2.2.1 based on ETSI TS 103 301 [15]"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=32767", extensible))]
    pub struct IviIdentificationNumber(pub Integer);
    #[doc = "*"]
    #[doc = " * This DF provides the reference to the information contained in a IVIM according to ETSI TS 103 301 [15]. "]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field serviceProviderId: identifier of the organization that provided the IVIM."]
    #[doc = " *"]
    #[doc = " * @field iviIdentificationNumber: identifier of the IVIM, as assigned by the organization identified in serviceProviderId."]
    #[doc = " *"]
    #[doc = " * @category: Communication information"]
    #[doc = " * @revision: Created in V2.2.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct IvimReference {
        #[rasn(identifier = "serviceProviderId")]
        pub service_provider_id: Provider,
        #[rasn(identifier = "iviIdentificationNumber")]
        pub ivi_identification_number: IviIdentificationNumber,
    }
    impl IvimReference {
        pub fn new(
            service_provider_id: Provider,
            ivi_identification_number: IviIdentificationNumber,
        ) -> Self {
            Self {
                service_provider_id,
                ivi_identification_number,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DF shall contain a list of @ref IvimReference."]
    #[doc = " *"]
    #[doc = " * @category: Communication information"]
    #[doc = " * @revision: Created in V2.2.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8", extensible))]
    pub struct IvimReferences(pub SequenceOf<IvimReference>);
    #[doc = "*"]
    #[doc = " * This DE indicates a transversal position on the carriageway at a specific longitudinal position, in resolution of lanes of the carriageway. "]
    #[doc = " *"]
    #[doc = " * For right-hand traffic roads, the value shall be set to:"]
    #[doc = " * - `-1` if the position is off, i.e. besides the road,"]
    #[doc = " * - `0` if the position is on the inner hard shoulder, i.e. the hard should adjacent to the leftmost lane,"]
    #[doc = " * - `n` (`n > 0` and `n < 14`), if the position is on the n-th driving lane counted from the leftmost lane to the rightmost lane of a specific traffic direction,"]
    #[doc = " * - `14` if the position is on the outer hard shoulder, i.e. the hard should adjacent to rightmost lane (if present)."]
    #[doc = " *"]
    #[doc = " * For left-hand traffic roads, the value shall be set to:"]
    #[doc = " * - `-1` if the position is off, i.e. besides the road,"]
    #[doc = " * - `0` if the position is on the inner hard shoulder, i.e. the hard should adjacent to the rightmost lane,"]
    #[doc = " * - `n` (`n > 0` and `n < 14`), if the position is on the n-th driving lane counted from the rightmost lane to the leftmost lane of a specific traffic direction,"]
    #[doc = " * - `14` if the position is on the outer hard shoulder, i.e. the hard should adjacent to leftmost lane (if present)."]
    #[doc = " *"]
    #[doc = " *  @note: in practice this means that the position is counted from \"inside\" to \"outside\" no matter which traffic practice is used."]
    #[doc = " *"]
    #[doc = " * If the carriageway allows only traffic in one direction (e.g. in case of dual or multiple carriageway roads), the position is counted from the physical border of the carriageway. "]
    #[doc = " * If the carriageway allows traffic in both directions and there is no physical delimitation between traffic directions (e.g. on a single carrriageway road), "]
    #[doc = " * the position is counted from the legal (i.e. optical) separation between traffic directions (horizontal marking). "]
    #[doc = " *"]
    #[doc = " * If not indicated otherwise (by lane markings or traffic signs), the legal separation on carriageways allowing traffic on both directions is identified as follows:"]
    #[doc = " * - If the total number of lanes N is even, the lanes are divided evenly between the traffic directions starting from the outside of the carriageway on both sides and the "]
    #[doc = " *   imaginary separation between traffic directions is on the border between the even number of lanes N/2."]
    #[doc = " * - If the total number of lanes N is odd, the lanes are divided evenly between traffic direction starting from the outside of the carriageway on both sides. "]
    #[doc = " *   The remaining middle lane is assigned to both traffic directions as innermost lane."]
    #[doc = " *"]
    #[doc = " * @category: Road topology information"]
    #[doc = " * @revision: Description of the legal separation of carriageways added in V2.2.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-1..=14"))]
    pub struct LanePosition(pub i8);
    #[doc = "*"]
    #[doc = " * This DF indicates a transversal position in resolution of lanes and other associated details."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field transversalPosition: the transversal position."]
    #[doc = " * "]
    #[doc = " * @field laneType: the type of the lane identified in the component transversalPosition. By default set to `traffic`."]
    #[doc = " *"]
    #[doc = " * @field direction: the traffic direction for the lane position relative to a defined reference direction. By default set to `sameDirection`, i.e. following the reference direction."]
    #[doc = " *"]
    #[doc = " * @category Road topology information"]
    #[doc = " * @revision: direction added in V2.2.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct LanePositionAndType {
        #[rasn(identifier = "transversalPosition")]
        pub transversal_position: LanePosition,
        #[rasn(
            default = "lane_position_and_type_lane_type_default",
            identifier = "laneType"
        )]
        pub lane_type: LaneType,
        #[rasn(default = "lane_position_and_type_direction_default")]
        pub direction: Direction,
    }
    impl LanePositionAndType {
        pub fn new(
            transversal_position: LanePosition,
            lane_type: LaneType,
            direction: Direction,
        ) -> Self {
            Self {
                transversal_position,
                lane_type,
                direction,
            }
        }
    }
    fn lane_position_and_type_lane_type_default() -> LaneType {
        LaneType(0)
    }
    fn lane_position_and_type_direction_default() -> Direction {
        Direction(0)
    }
    #[doc = "*"]
    #[doc = " * This DF represents a set of options to describe a lane position and is the second level DF to represent a lane position. The top-level DFs are @ref GeneralizedLanePosition or @ref OccupiedLanesWithConfidence. "]
    #[doc = " * A lane position is a transversal position on the carriageway at a specific longitudinal position, in resolution of lanes of the carriageway."]
    #[doc = " *"]
    #[doc = " * The following options are available:"]
    #[doc = " *"]
    #[doc = " * @field simplelanePosition: a single lane position without any additional context information."]
    #[doc = " *"]
    #[doc = " * @field simpleLaneType: a lane type, to be used when the lane position is unknown but the type of lane is known. This can be used in scenarios where a certain confidence about the used lane type is given "]
    #[doc = " * but no or limited knowledge about the absolute lane number is available. For example, a cyclist on a cycle-lane or vehicles on a specific lane that is unique for the part of the road (e.g. a bus lane)."]
    #[doc = " * "]
    #[doc = " * @field detailedlanePosition: a single lane position with additional lane details."]
    #[doc = " * "]
    #[doc = " * @field lanePositionWithLateralDetails: a single lane position with additional details and the lateral position within the lane."]
    #[doc = " *"]
    #[doc = " * @field trafficIslandPosition: a position on a traffic island, i.e. between two lanes. "]
    #[doc = " *"]
    #[doc = " * @category: Road Topology information"]
    #[doc = " * @revision: Created in V2.2.1 from the DF GeneralizedLanePosition of V2.1.1. "]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum LanePositionOptions {
        simplelanePosition(LanePosition),
        simpleLaneType(LaneType),
        detailedlanePosition(LanePositionAndType),
        lanePositionWithLateralDetails(LanePositionWithLateralDetails),
        trafficIslandPosition(TrafficIslandPosition),
    }
    #[doc = "*"]
    #[doc = " * This DF is a third-level DF that represents a lane position and is an extended version of @ref LanePositionAndType that adds the distances to the left and right lane border."]
    #[doc = " *"]
    #[doc = " * It shall additionally include the following components: "]
    #[doc = " *"]
    #[doc = " * @field distanceToLeftBorder: the distance of the transversal position to the left lane border. The real value shall be rounded to the next lower encoding-value."]
    #[doc = " *"]
    #[doc = " * @field distanceToRightBorder: the distance of the transversal position to the right lane border. The real value shall be rounded to the next lower encoding-value."]
    #[doc = " * "]
    #[doc = " * @category: Road Topology information"]
    #[doc = " * @revision: Created in V2.2.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct LanePositionWithLateralDetails {
        #[rasn(identifier = "distanceToLeftBorder")]
        pub distance_to_left_border: StandardLength9b,
        #[rasn(identifier = "distanceToRightBorder")]
        pub distance_to_right_border: StandardLength9b,
        #[rasn(identifier = "transversalPosition")]
        pub transversal_position: LanePosition,
        #[rasn(
            default = "lane_position_with_lateral_details_lane_type_default",
            identifier = "laneType"
        )]
        pub lane_type: LaneType,
        #[rasn(default = "lane_position_with_lateral_details_direction_default")]
        pub direction: Direction,
    }
    impl LanePositionWithLateralDetails {
        pub fn new(
            distance_to_left_border: StandardLength9b,
            distance_to_right_border: StandardLength9b,
            transversal_position: LanePosition,
            lane_type: LaneType,
            direction: Direction,
        ) -> Self {
            Self {
                distance_to_left_border,
                distance_to_right_border,
                transversal_position,
                lane_type,
                direction,
            }
        }
    }
    fn lane_position_with_lateral_details_lane_type_default() -> LaneType {
        LaneType(0)
    }
    fn lane_position_with_lateral_details_direction_default() -> Direction {
        Direction(0)
    }
    #[doc = "*"]
    #[doc = " * This DE represents the type of a lane. "]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0\t- `traffic`            - Lane dedicated to the movement of vehicles,"]
    #[doc = " * - 1\t- `through`            - Lane dedicated to the movement of vehicles travelling ahead and not turning,"]
    #[doc = " * - 2\t- `reversible`         - Lane where the direction of traffic can be changed to match the peak flow,"]
    #[doc = " * - 3\t- `acceleration`\t   - Lane that allows vehicles entering a road to accelerate to the speed of through traffic before merging with it,"]
    #[doc = " * - 4\t- `deceleration`       - Lane that allows vehicles exiting a road to decelerate before leaving it,"]
    #[doc = " * - 5\t- `leftHandTurning`    - Lane reserved for slowing down and making a left turn, so as not to disrupt traffic,"]
    #[doc = " * - 6\t- `rightHandTurning`   - Lane reserved for slowing down and making a right turn so as not to disrupt traffic,"]
    #[doc = " * - 7\t- `dedicatedVehicle`   - Lane dedicated to movement of motor vehicles with specific characteristics, such as heavy goods vehicles, etc., "]
    #[doc = " * - 8\t- `bus`                - Lane dedicated to movement of buses providing public transport,"]
    #[doc = " * - 9\t- `taxi`               - Lane dedicated to movement of taxis,"]
    #[doc = " * - 10\t- `hov`                - Carpooling lane or high occupancy vehicle lane,"]
    #[doc = " * - 11\t- `hot`                - High occupancy vehicle lanes that is allowed to be used without meeting the occupancy criteria by paying a toll,"]
    #[doc = " * - 12\t- `pedestrian`         - Lanes dedicated to pedestrians such as pedestrian sidewalk paths,"]
    #[doc = " * - 13\t- `cycleLane`\t       - Lane dedicated to exclusive or preferred use by bicycles,"]
    #[doc = " * - 14\t- `median`             - Lane not dedicated to movement of vehicles but representing a median / central reservation  such as the central median, "]
    #[doc = "                                 separating the two directional carriageways of the highway,"]
    #[doc = " * - 15\t- `striping`\t       - Lane not dedicated to movement of vehicles but covered with roadway markings,"]
    #[doc = " * - 16\t- `trackedVehicle`     - Lane dedicated to movement of trains, trams and trolleys,"]
    #[doc = " * - 17\t- `parking`            - Lanes dedicated to vehicles parking, stopping and loading lanes,"]
    #[doc = " * - 18\t- `emergency`          - Lane dedicated to vehicles in breakdown or to emergency vehicles also called hard shoulder,"]
    #[doc = " * - 19\t- `verge`              - Lane representing the verge, i.e. a narrow strip of grass or plants and sometimes also trees located between "]
    #[doc = "                                 the road surface edge and the boundary of a road,"]
    #[doc = " * - 20\t`minimumRiskManoeuvre` - Lane dedicated to automated vehicles making a minimum risk manoeuvre,"]
    #[doc = " * - 21\t`separatedCycleLane`   - Lane dedicated to exclusive or preferred use by bicycles that is phyisically separated from the vehicle-traffic lanes, e.g. by a verge."]
    #[doc = " * - values 22 to 30             reserved for future use. "]
    #[doc = " *"]
    #[doc = " * @category: Road topology information"]
    #[doc = " * @revision: Created in V2.1.1, named value 21 added in V2.2.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=31"))]
    pub struct LaneType(pub u8);
    #[doc = "*"]
    #[doc = " * This DE represents the width of a lane measured at a defined position."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `n` (`n > 0` and `n < 1022`) if the lane width information is equal to or less than n x 0,01 metre and more than (n-1) x 0,01 metre,"]
    #[doc = " * - `1022` if the lane width is out of range, i.e. greater than 10,21 m,"]
    #[doc = " * - `1023` if the lane width information is not available."]
    #[doc = " *"]
    #[doc = " * The value 0 shall not be used."]
    #[doc = " *"]
    #[doc = " * @unit: 0,01 metre"]
    #[doc = " * @category: Road topology information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=1023"))]
    pub struct LaneWidth(pub u16);
    #[doc = "*"]
    #[doc = " * This DF indicates the vehicle acceleration at lateral direction and the confidence value of the lateral acceleration."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field lateralAccelerationValue: lateral acceleration value at a point in time."]
    #[doc = " * "]
    #[doc = " * @field lateralAccelerationConfidence: confidence value of the lateral acceleration value."]
    #[doc = " *"]
    #[doc = " * @note: this DF is kept for backwards compatibility reasons only. It is recommended to use @ref AccelerationComponent instead."]
    #[doc = " * @category Vehicle information"]
    #[doc = " * @revision: Description revised in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct LateralAcceleration {
        #[rasn(identifier = "lateralAccelerationValue")]
        pub lateral_acceleration_value: LateralAccelerationValue,
        #[rasn(identifier = "lateralAccelerationConfidence")]
        pub lateral_acceleration_confidence: AccelerationConfidence,
    }
    impl LateralAcceleration {
        pub fn new(
            lateral_acceleration_value: LateralAccelerationValue,
            lateral_acceleration_confidence: AccelerationConfidence,
        ) -> Self {
            Self {
                lateral_acceleration_value,
                lateral_acceleration_confidence,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DE represents the vehicle acceleration at lateral direction in the centre of the mass of the empty vehicle."]
    #[doc = " * It corresponds to the vehicle coordinate system as specified in ISO 8855 [21]."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `-160` for acceleration values equal to or less than -16 m/s^2,"]
    #[doc = " * - `n` (`n > -160` and `n <= 0`) to indicate that the vehicle is accelerating towards the right side with regards to the vehicle orientation "]
    #[doc = " *                            with acceleration equal to or less than n x 0,1 m/s^2 and greater than (n-1) x 0,1 m/s^2,"]
    #[doc = " * - `n` (`n > 0` and `n < 160`) to indicate that the vehicle is accelerating towards the left hand side with regards to the vehicle orientation "]
    #[doc = "\t\t\t\t\t\t     with acceleration equal to or less than n x 0,1 m/s^2 and greater than (n-1) x 0,1 m/s^2,"]
    #[doc = " * - `160` for acceleration values greater than 15,9 m/s^2,"]
    #[doc = " * - `161` when the data is unavailable."]
    #[doc = " *"]
    #[doc = " * @note: the empty load vehicle is defined in ISO 1176 [8], clause 4.6."]
    #[doc = " * @note: this DF is kept for backwards compatibility reasons only. It is recommended to use @ref AccelerationValue instead."]
    #[doc = " *  "]
    #[doc = " * @unit: 0,1 m/s^2"]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: Description updated in V2.1.1 (the meaning of 160 has changed slightly). "]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-160..=161"))]
    pub struct LateralAccelerationValue(pub i16);
    #[doc = "*"]
    #[doc = " * This DE represents the absolute geographical latitude in a WGS84 coordinate system, providing a range of 90 degrees in north or"]
    #[doc = " * in south hemisphere."]
    #[doc = " * The specific WGS84 coordinate system is specified by the corresponding standards applying this DE."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `n` (`n >= -900 000 000` and `n < 0`) x 10^-7 degree, i.e. negative values for latitudes south of the Equator,"]
    #[doc = " * - `0` is used for the latitude of the equator,"]
    #[doc = " * - `n` (`n > 0` and `n < 900 000 001`) x 10^-7 degree, i.e. positive values for latitudes north of the Equator,"]
    #[doc = " * - `900 000 001` when the information is unavailable."]
    #[doc = " *"]
    #[doc = " * @unit: 10^-7 degree"]
    #[doc = " * @category: GeoReference information"]
    #[doc = " * @revision: Editorial update in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-900000000..=900000001"))]
    pub struct Latitude(pub i32);
    #[doc = "*"]
    #[doc = " * This DE indicates the status of light bar and any sort of audible alarm system besides the horn."]
    #[doc = " * This includes various common sirens as well as backup up beepers and other slow speed manoeuvring alerts."]
    #[doc = " *"]
    #[doc = " * The corresponding bit shall be set to 1 under the following conditions:"]
    #[doc = " * - 0 - `lightBarActivated`      - when the light bar is activated,"]
    #[doc = " * - 1 - `sirenActivated`         - when the siren is activated."]
    #[doc = " *"]
    #[doc = " * Otherwise, it shall be set to 0."]
    #[doc = " *"]
    #[doc = " * @category Vehicle information"]
    #[doc = " * @revision: Editorial update in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct LightBarSirenInUse(pub FixedBitString<2usize>);
    #[doc = "*"]
    #[doc = " * This DE represents the absolute geographical longitude in a WGS84 coordinate system, providing a range of 180 degrees"]
    #[doc = " * to the east or to the west of the prime meridian."]
    #[doc = " * The specific WGS84 coordinate system is specified by the corresponding standards applying this DE."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `n` (`n > -1 800 000 000` and `n < 0`) x 10^-7 degree, i.e. negative values for longitudes to the west,"]
    #[doc = " * - `0` to indicate the prime meridian,"]
    #[doc = " * - `n` (`n > 0` and `n < 1 800 000 001`) x 10^-7 degree, i.e. positive values for longitudes to the east,"]
    #[doc = " * - `1 800 000 001` when the information is unavailable."]
    #[doc = " *"]
    #[doc = " * The value -1 800 000 000 shall not be used. "]
    #[doc = " * "]
    #[doc = " * @unit: 10^-7 degree"]
    #[doc = " * @category: GeoReference information"]
    #[doc = " * @revision: Description revised in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-1800000000..=1800000001"))]
    pub struct Longitude(pub i32);
    #[doc = "*"]
    #[doc = " * This DF indicates the vehicle acceleration at longitudinal direction and the confidence value of the longitudinal acceleration."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field longitudinalAccelerationValue: longitudinal acceleration value at a point in time."]
    #[doc = ""]
    #[doc = " * @field longitudinalAccelerationConfidence: confidence value of the longitudinal acceleration value."]
    #[doc = " *"]
    #[doc = " * @note: this DF is kept for backwards compatibility reasons only. It is recommended to use @ref AccelerationComponent instead. "]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: V1.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct LongitudinalAcceleration {
        #[rasn(identifier = "longitudinalAccelerationValue")]
        pub longitudinal_acceleration_value: LongitudinalAccelerationValue,
        #[rasn(identifier = "longitudinalAccelerationConfidence")]
        pub longitudinal_acceleration_confidence: AccelerationConfidence,
    }
    impl LongitudinalAcceleration {
        pub fn new(
            longitudinal_acceleration_value: LongitudinalAccelerationValue,
            longitudinal_acceleration_confidence: AccelerationConfidence,
        ) -> Self {
            Self {
                longitudinal_acceleration_value,
                longitudinal_acceleration_confidence,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DE represents the vehicle acceleration at longitudinal direction in the centre of the mass of the empty vehicle."]
    #[doc = " * The value shall be provided in the vehicle coordinate system as defined in ISO 8855 [21], clause 2.11."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `-160` for acceleration values equal to or less than -16 m/s^2,"]
    #[doc = " * - `n` (`n > -160` and `n <= 0`) to indicate that the vehicle is braking with acceleration equal to or less than n x 0,1 m/s^2, and greater than (n-1) x 0,1 m/s^2"]
    #[doc = " * - `n` (`n > 0` and `n < 160`) to indicate that the vehicle is accelerating with acceleration equal to or less than n x 0,1 m/s^2, and greater than (n-1) x 0,1 m/s^2,"]
    #[doc = " * - `160` for acceleration values greater than 15,9 m/s^2,"]
    #[doc = " * - `161` when the data is unavailable. "]
    #[doc = " * "]
    #[doc = " * This acceleration is along the tangent plane of the road surface and does not include gravity components."]
    #[doc = " * @note: this DF is kept for backwards compatibility reasons only. It is recommended to use @ref AccelerationValue instead."]
    #[doc = " * "]
    #[doc = " * @note: The empty load vehicle is defined in ISO 1176 [8], clause 4.6."]
    #[doc = " * @unit: 0,1 m/s^2"]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: description revised in V2.1.1 (the meaning of 160 has changed slightly). T"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-160..=161"))]
    pub struct LongitudinalAccelerationValue(pub i16);
    #[doc = "* "]
    #[doc = " * This DF represents the estimated position along the longitudinal extension of a carriageway or lane. "]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field  longitudinalLanePositionValue: the mean value of the longitudinal position along the carriageway or lane w.r.t. an externally defined start position."]
    #[doc = " *"]
    #[doc = " * @field  longitudinalLanePositionConfidence: The confidence value associated to the value."]
    #[doc = " *"]
    #[doc = " * @category: Road topology information"]
    #[doc = " * @revision: created in V2.1.1, description revised in V2.2.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct LongitudinalLanePosition {
        #[rasn(identifier = "longitudinalLanePositionValue")]
        pub longitudinal_lane_position_value: LongitudinalLanePositionValue,
        #[rasn(identifier = "longitudinalLanePositionConfidence")]
        pub longitudinal_lane_position_confidence: LongitudinalLanePositionConfidence,
    }
    impl LongitudinalLanePosition {
        pub fn new(
            longitudinal_lane_position_value: LongitudinalLanePositionValue,
            longitudinal_lane_position_confidence: LongitudinalLanePositionConfidence,
        ) -> Self {
            Self {
                longitudinal_lane_position_value,
                longitudinal_lane_position_confidence,
            }
        }
    }
    #[doc = "* "]
    #[doc = " * This DE indicates the longitudinal lane position confidence value which represents the estimated accuracy of longitudinal lane position measurement with a default confidence level of 95 %."]
    #[doc = " * If required, the confidence level can be defined by the corresponding standards applying this DE."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `n` (`n > 0` and `n < 1 022`) if the  confidence value is equal to or less than n x 0,1 m, and more than (n-1) x 0,1 m,"]
    #[doc = " * - `1 022` if the confidence value is out of range i.e. greater than 102,1 m,"]
    #[doc = " * - `1 023` if the confidence value is unavailable."]
    #[doc = " *"]
    #[doc = " * @unit 0,1 metre"]
    #[doc = " * @category: GeoReference information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=1023"))]
    pub struct LongitudinalLanePositionConfidence(pub u16);
    #[doc = "* "]
    #[doc = " * This DE represents the longitudinal offset of a map-matched position along a matched lane, beginning from the lane's starting point."]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `n` (`n >= 0` and `n < 32766`) if the longitudinal offset information is equal to or less than n x 0,1 metre and more than (n-1) x 0,1 metre,"]
    #[doc = " * - `32 766` if the longitudinal offset is out of range, i.e. greater than 3276,5 m,"]
    #[doc = " * - `32 767` if the longitudinal offset information is not available. "]
    #[doc = " *"]
    #[doc = " * @unit 0,1 metre"]
    #[doc = " * @category: GeoReference information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=32767"))]
    pub struct LongitudinalLanePositionValue(pub u16);
    #[doc = "* "]
    #[doc = " * This DF shall contain a list of a lower triangular positive semi-definite matrices."]
    #[doc = " *"]
    #[doc = " * @category: Sensing information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct LowerTriangularPositiveSemidefiniteMatrices(
        pub SequenceOf<LowerTriangularPositiveSemidefiniteMatrix>,
    );
    #[doc = "* "]
    #[doc = " * This DF represents a lower triangular positive semi-definite matrix. "]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field componentsIncludedIntheMatrix: the indication of which components of a @ref PerceivedObject are included in the matrix. "]
    #[doc = " * This component also implicitly indicates the number n of included components which defines the size (n x n) of the full correlation matrix \"A\"."]
    #[doc = " *"]
    #[doc = " * @field matrix: the list of cells of the lower triangular positive semi-definite matrix ordered by columns and by rows. "]
    #[doc = " *"]
    #[doc = " * The number of columns to be included \"k\" is equal to the number of included components \"n\" indicated by componentsIncludedIntheMatrix minus 1: k = n-1."]
    #[doc = " * These components shall be included in the order or their appearance in componentsIncludedIntheMatrix."]
    #[doc = " * Each column \"i\" of the lowerTriangularCorrelationMatrixColumns contains k-(i-1) values."]
    #[doc = " *"]
    #[doc = " * @category: Sensing information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct LowerTriangularPositiveSemidefiniteMatrix {
        #[rasn(identifier = "componentsIncludedIntheMatrix")]
        pub components_included_inthe_matrix: MatrixIncludedComponents,
        pub matrix: LowerTriangularPositiveSemidefiniteMatrixColumns,
    }
    impl LowerTriangularPositiveSemidefiniteMatrix {
        pub fn new(
            components_included_inthe_matrix: MatrixIncludedComponents,
            matrix: LowerTriangularPositiveSemidefiniteMatrixColumns,
        ) -> Self {
            Self {
                components_included_inthe_matrix,
                matrix,
            }
        }
    }
    #[doc = "* "]
    #[doc = " * This DF represents the columns of a lower triangular positive semi-definite matrix, each column not including the main diagonal cell of the matrix."]
    #[doc = " * Given a matrix \"A\" of size n x n, the number of @ref CorrelationColumn to be included in the lower triangular matrix is k=n-1."]
    #[doc = " *"]
    #[doc = " * @category: Sensing information"]
    #[doc = " * @revision: Created in V2.1.1, extension indicator added in V2.2.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=13", extensible))]
    pub struct LowerTriangularPositiveSemidefiniteMatrixColumns(pub SequenceOf<CorrelationColumn>);
    #[doc = "*"]
    #[doc = " * This DF indicates a position on a topology description transmitted in a MAPEM according to ETSI TS 103 301 [15]."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field mapReference: optionally identifies the MAPEM containing the topology information."]
    #[doc = " * It is absent if the MAPEM topology is known from the context."]
    #[doc = " * "]
    #[doc = " * @field laneId: optionally identifies the lane in the road segment or intersection topology on which the position is located."]
    #[doc = " *"]
    #[doc = " * @field connectionId: optionally identifies the connection inside the conflict area of an intersection, i.e. it identifies a trajectory for travelling through the"]
    #[doc = " * conflict area of an intersection which connects e.g an ingress with an egress lane."]
    #[doc = " *"]
    #[doc = " * @field longitudinalLanePosition: optionally indicates the longitudinal offset of the map-matched position of the object along the lane or connection measured from the start of the lane/connection, along the lane."]
    #[doc = " * "]
    #[doc = " * @category: Road topology information"]
    #[doc = " * @revision: Created in V2.1.1, definition of longitudinalLanePosition amended in V2.2.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MapPosition {
        #[rasn(identifier = "mapReference")]
        pub map_reference: Option<MapReference>,
        #[rasn(identifier = "laneId")]
        pub lane_id: Option<Identifier1B>,
        #[rasn(identifier = "connectionId")]
        pub connection_id: Option<Identifier1B>,
        #[rasn(identifier = "longitudinalLanePosition")]
        pub longitudinal_lane_position: Option<LongitudinalLanePosition>,
    }
    impl MapPosition {
        pub fn new(
            map_reference: Option<MapReference>,
            lane_id: Option<Identifier1B>,
            connection_id: Option<Identifier1B>,
            longitudinal_lane_position: Option<LongitudinalLanePosition>,
        ) -> Self {
            Self {
                map_reference,
                lane_id,
                connection_id,
                longitudinal_lane_position,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DF provides the reference to the information contained in a MAPEM according to ETSI TS 103 301 [15]. "]
    #[doc = " *"]
    #[doc = " * The following options are provided:"]
    #[doc = " * "]
    #[doc = " * @field roadsegment: option that identifies the description of a road segment contained in a MAPEM."]
    #[doc = " * "]
    #[doc = " * @field intersection: option that identifies the description of an intersection contained in a MAPEM."]
    #[doc = " *"]
    #[doc = " * @category: Road topology information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum MapReference {
        roadsegment(RoadSegmentReferenceId),
        intersection(IntersectionReferenceId),
    }
    #[doc = "*"]
    #[doc = " * This DF shall contain a list of @ref MapReference."]
    #[doc = " *"]
    #[doc = " * @category: Road topology information"]
    #[doc = " * @revision: Created in V2.2.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8", extensible))]
    pub struct MapReferences(pub SequenceOf<MapReference>);
    #[doc = "* "]
    #[doc = " * This DF provides information about the configuration of a road section in terms of MAPEM lanes or connections using a list of @ref MapemExtractedElementReference. "]
    #[doc = ""]
    #[doc = " * @category: Road topology information"]
    #[doc = " * @revision: Created in V2.2.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16", extensible))]
    pub struct MapemConfiguration(pub SequenceOf<MapemElementReference>);
    #[doc = "* "]
    #[doc = " * This DF provides references to MAPEM connections using a list of @ref Identifier1B."]
    #[doc = " * Note: connections are  allowed Ã¯Â¿Â½maneuversÃ¯Â¿Â½ (e.g. an ingress / egress relation) on an intersection."]
    #[doc = " *"]
    #[doc = " * @category: Road topology information"]
    #[doc = " * @revision: Created in V2.2.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8", extensible))]
    pub struct MapemConnectionList(pub SequenceOf<Identifier1B>);
    #[doc = "* "]
    #[doc = " * This DF provides references to an element described in a MAPEM according to ETSI TS 103 301 [i.15], such as a lane or connection at a specific intersection or road segment. "]
    #[doc = " * "]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field mapReference: the optional reference to a MAPEM that describes the intersection or road segment. It is absent if the MAPEM topology is known from the context."]
    #[doc = " * "]
    #[doc = " * @field laneIds: the optional list of the identifiers of the lanes to be referenced. "]
    #[doc = " * "]
    #[doc = " * @field connectionIds: the optional list of the identifiers of the connections to be referenced. "]
    #[doc = " *"]
    #[doc = " * @category: Road topology information"]
    #[doc = " * @revision: Created in V2.2.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MapemElementReference {
        #[rasn(identifier = "mapReference")]
        pub map_reference: Option<MapReference>,
        #[rasn(identifier = "laneIds")]
        pub lane_ids: Option<MapemLaneList>,
        #[rasn(identifier = "connectionIds")]
        pub connection_ids: Option<MapemConnectionList>,
    }
    impl MapemElementReference {
        pub fn new(
            map_reference: Option<MapReference>,
            lane_ids: Option<MapemLaneList>,
            connection_ids: Option<MapemConnectionList>,
        ) -> Self {
            Self {
                map_reference,
                lane_ids,
                connection_ids,
            }
        }
    }
    #[doc = "* "]
    #[doc = " * This DF provides references to MAPEM lanes using a list of @ref Identifier1B."]
    #[doc = " *"]
    #[doc = " * @category: Road topology information"]
    #[doc = " * @revision: Created in 2.2.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8", extensible))]
    pub struct MapemLaneList(pub SequenceOf<Identifier1B>);
    #[doc = "*"]
    #[doc = " * This DE indicates the components of an @ref PerceivedObject that are included in the @ref LowerTriangularPositiveSemidefiniteMatrix."]
    #[doc = " *"]
    #[doc = " * The corresponding bit shall be set to 1 if the component is included:"]
    #[doc = " * - 0 - `xCoordinate`                   - when the component xCoordinate of the component @ref CartesianPosition3dWithConfidence is included,"]
    #[doc = " * - 1 - `yCoordinate`                   - when the component yCoordinate of the component @ref CartesianPosition3dWithConfidence is included,   "]
    #[doc = " * - 2 - `zCoordinate`                   - when the component zCoordinate of the component @ref CartesianPosition3dWithConfidence is included, "]
    #[doc = " * - 3 - `xVelocityOrVelocityMagnitude`  - when the component xVelocity of the component @ref VelocityCartesian or the component VelocityMagnitude of the component @ref VelocityPolarWithZ is included,   "]
    #[doc = " * - 4 - `yVelocityOrVelocityDirection`  - when the component yVelocity of the component @ref VelocityCartesian or the component VelocityDirection of the component @ref VelocityPolarWithZ is included,   "]
    #[doc = " * - 5 - `zVelocity`                     - when the component zVelocity of the component @ref VelocityCartesian or of the component @ref VelocityPolarWithZ is included,"]
    #[doc = " * - 6 - `xAccelOrAccelMagnitude`        - when the component xAcceleration of the component @ref AccelerationCartesian or the component AccelerationMagnitude of the component @ref AccelerationPolarWithZ is included,  "]
    #[doc = " * - 7 - `yAccelOrAccelDirection`        - when the component yAcceleration of the component @ref AccelerationCartesian or the component AccelerationDirection of the component @ref AccelerationPolarWithZ is included,   "]
    #[doc = " * - 8 - `zAcceleration`                 - when the component zAcceleration of the component @ref AccelerationCartesian or of the component @ref AccelerationPolarWithZ is included,"]
    #[doc = " * - 9 - `zAngle`                        - when the component zAngle is included,"]
    #[doc = " * - 10 - `yAngle`                       - when the component yAngle is included,   "]
    #[doc = " * - 11 - `xAngle`                       - when the component xAngle is included,   "]
    #[doc = " * - 12 - `zAngularVelocity`             - when the component zAngularVelocity is included.   "]
    #[doc = " *"]
    #[doc = " * Otherwise, it shall be set to 0."]
    #[doc = " *"]
    #[doc = " * @category: Sensing information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("13", extensible))]
    pub struct MatrixIncludedComponents(pub BitString);
    #[doc = "* "]
    #[doc = " * This DE represents the type of facility layer message."]
    #[doc = " *"]
    #[doc = " *  The value shall be set to:"]
    #[doc = " *\t- 1  - `denm`              - for Decentralized Environmental Notification Message (DENM) as specified in ETSI EN 302 637-3 [2],"]
    #[doc = " *  - 2  - `cam`               - for Cooperative Awareness Message (CAM) as specified in ETSI EN 302 637-2 [1],"]
    #[doc = " *  - 3  - `poim`              - for Point of Interest Message as specified in ETSI TS 103 916 [9],"]
    #[doc = " *  - 4  - `spatem`            - for Signal Phase And Timing Extended Message (SPATEM) as specified in ETSI TS 103 301 [15],"]
    #[doc = " *  - 5  - `mapem`             - for MAP Extended Message (MAPEM) as specified in ETSI TS 103 301 [15],"]
    #[doc = " *  - 6  - `ivim`              - for in Vehicle Information Message (IVIM) as specified in ETSI TS 103 301 [15],"]
    #[doc = " *  - 7  - `rfu1`              - reserved for future usage,"]
    #[doc = " *  - 8  - `rfu2`              - reserved for future usage,"]
    #[doc = " *  - 9  - `srem`              - for Signal Request Extended Message as specified in ETSI TS 103 301 [15],"]
    #[doc = " *  - 10 - `ssem`              - for Signal request Status Extended Message as specified in ETSI TS 103 301 [15],"]
    #[doc = " *  - 11 - `evcsn`             - for Electrical Vehicle Charging Spot Notification message as specified in ETSI TS 101 556-1 [9],"]
    #[doc = " *  - 12 - `saem`              - for Services Announcement Extended Message as specified in ETSI EN 302 890-1 [17],"]
    #[doc = " *  - 13 - `rtcmem`            - for Radio Technical Commission for Maritime Services Extended Message (RTCMEM) as specified in ETSI TS 103 301 [15],"]
    #[doc = " *  - 14 - `cpm`               - for Collective Perception Message (CPM) as specified in ETSI TS 103 324 [10], "]
    #[doc = " *  - 15 - `imzm`              - for Interference Management Zone Message (IMZM) as specified in ETSI TS 103 724 [13],"]
    #[doc = " *  - 16 - `vam`               - for Vulnerable Road User Awareness Message as specified in ETSI TS 130 300-3 [12], "]
    #[doc = " *  - 17 - `dsm`               - reserved for Diagnosis, logging and Status Message,"]
    #[doc = " *  - 18 - `mim`               - for Marshalling Infrastructure Message as specified in ETSI TS TS 103 882 [11],"]
    #[doc = " *  - 19 - `mvm`               - for Marshalling Vehicle Message as specified in ETSI TS TS 103 882 [11],"]
    #[doc = " *  - 20 - `mcm`               - reserved for Manoeuvre Coordination Message,"]
    #[doc = " *  - 21 - `pim`               - reserved for Parking Information Message, "]
    #[doc = " *  - 22-255                   - reserved for future usage."]
    #[doc = " *"]
    #[doc = " * @category: Communication information"]
    #[doc = " * @revision: Created in V2.1.1 from @ref ItsPduHeader. Value 3 re-assigned to poim and value 7 and 8 reserved in V2.2.1, values 18 and 19 assigned in V2.3.1, "]
    #[doc = "              value 21 assigned in V2.4.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct MessageId(pub u8);
    #[doc = "*"]
    #[doc = " * This DF indicates a message rate."]
    #[doc = " *"]
    #[doc = " * @field mantissa: indicates the mantissa."]
    #[doc = " *"]
    #[doc = " * @field exponent: indicates the exponent."]
    #[doc = " *"]
    #[doc = " * The specified message rate is: mantissa*(10^exponent) "]
    #[doc = " *"]
    #[doc = " * @unit: Hz"]
    #[doc = " * @category: Communication information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct MessageRateHz {
        #[rasn(value("1..=100"))]
        pub mantissa: u8,
        #[rasn(value("-5..=2"))]
        pub exponent: i8,
    }
    impl MessageRateHz {
        pub fn new(mantissa: u8, exponent: i8) -> Self {
            Self { mantissa, exponent }
        }
    }
    #[doc = "*"]
    #[doc = " * This DF provides information about a message with respect to the segmentation process on facility layer at the sender."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field totalMsgNo: indicates the total number of messages that have been assembled on the transmitter side to encode the information "]
    #[doc = " * during the same messsage generation process."]
    #[doc = " *"]
    #[doc = " * @field thisMsgNo: indicates the position of the message within of the total set of messages generated during the same message generation process."]
    #[doc = " *"]
    #[doc = " * @category: Communication information"]
    #[doc = " * @revision: Created in V2.1.1, description revised in V2.2.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct MessageSegmentationInfo {
        #[rasn(identifier = "totalMsgNo")]
        pub total_msg_no: CardinalNumber3b,
        #[rasn(identifier = "thisMsgNo")]
        pub this_msg_no: OrdinalNumber3b,
    }
    impl MessageSegmentationInfo {
        pub fn new(total_msg_no: CardinalNumber3b, this_msg_no: OrdinalNumber3b) -> Self {
            Self {
                total_msg_no,
                this_msg_no,
            }
        }
    }
    #[doc = "* "]
    #[doc = " * This DF provides information about the source of and confidence in information."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field usedDetectionInformation: the type of sensor(s) that is used to provide the detection information."]
    #[doc = " * "]
    #[doc = " * @field usedStoredInformation: the type of source of the stored information. "]
    #[doc = " *"]
    #[doc = " * @field confidenceValue: an optional confidence value associated to the information. "]
    #[doc = " * "]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Created in V2.2.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MetaInformation {
        #[rasn(identifier = "usedDetectionInformation")]
        pub used_detection_information: SensorTypes,
        #[rasn(identifier = "usedStoredInformation")]
        pub used_stored_information: StoredInformationType,
        #[rasn(identifier = "confidenceValue")]
        pub confidence_value: Option<ConfidenceLevel>,
    }
    impl MetaInformation {
        pub fn new(
            used_detection_information: SensorTypes,
            used_stored_information: StoredInformationType,
            confidence_value: Option<ConfidenceLevel>,
        ) -> Self {
            Self {
                used_detection_information,
                used_stored_information,
                confidence_value,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DF shall contain a list of @ref MitigationPerTechnologyClass."]
    #[doc = " *"]
    #[doc = " * @category: Communication information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8"))]
    pub struct MitigationForTechnologies(pub SequenceOf<MitigationPerTechnologyClass>);
    #[doc = "*"]
    #[doc = " * This DF represents a set of mitigation parameters for a specific technology, as specified in ETSI TS 103 724 [24], clause 7."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field accessTechnologyClass:  channel access technology to which this mitigation is intended to be applied."]
    #[doc = " *"]
    #[doc = " * @field lowDutyCycle: duty cycle limit."]
    #[doc = " * @unit: 0,01 % steps"]
    #[doc = " *"]
    #[doc = " * @field powerReduction: the delta value of power to be reduced."]
    #[doc = " * @unit: dB"]
    #[doc = " *"]
    #[doc = " * @field dmcToffLimit: idle time limit as defined in ETSI TS 103 175 [19]."]
    #[doc = " * @unit: ms"]
    #[doc = " *"]
    #[doc = " * @field dmcTonLimit: Transmission duration limit, as defined in ETSI EN 302 571 [20]."]
    #[doc = " * @unit: ms"]
    #[doc = " *"]
    #[doc = " * @note: All parameters are optional, as they may not apply to some of the technologies or"]
    #[doc = " * interference management zone types. Specification details are in ETSI TS 103 724 [24], clause 7. "]
    #[doc = " *"]
    #[doc = " * @category: Communication information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MitigationPerTechnologyClass {
        #[rasn(identifier = "accessTechnologyClass")]
        pub access_technology_class: AccessTechnologyClass,
        #[rasn(value("0..=10000"), identifier = "lowDutyCycle")]
        pub low_duty_cycle: Option<u16>,
        #[rasn(value("0..=30"), identifier = "powerReduction")]
        pub power_reduction: Option<u8>,
        #[rasn(value("0..=1200"), identifier = "dmcToffLimit")]
        pub dmc_toff_limit: Option<u16>,
        #[rasn(value("0..=20"), identifier = "dmcTonLimit")]
        pub dmc_ton_limit: Option<u8>,
    }
    impl MitigationPerTechnologyClass {
        pub fn new(
            access_technology_class: AccessTechnologyClass,
            low_duty_cycle: Option<u16>,
            power_reduction: Option<u8>,
            dmc_toff_limit: Option<u16>,
            dmc_ton_limit: Option<u8>,
        ) -> Self {
            Self {
                access_technology_class,
                low_duty_cycle,
                power_reduction,
                dmc_toff_limit,
                dmc_ton_limit,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DE provides a factor to be multiplied with a DE that represents a measure of something, to extend the range/change the unit. "]
    #[doc = " * The DE that is multiplied is to be specified outside of the context of this DE, e.g. in a facility layer service specification."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `tenth`      - to indicate a factor of 0,1,        "]
    #[doc = " * - `half`       - to indicate a factor of 0,5,        "]
    #[doc = " * - `two`        - to indicate a factor of 2,        "]
    #[doc = " * - `three`      - to indicate a factor of 3,        "]
    #[doc = " * - `five`       - to indicate a factor of 5,        "]
    #[doc = " * - `tenth`      - to indicate a factor of 10,        "]
    #[doc = " * - `fifthy`     - to indicate a factor of 50,        "]
    #[doc = " * - `hundred`    - to indicate a factor of 100,        "]
    #[doc = " *"]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Created in V2.4.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum MultiplicativeFactor {
        tenth = 0,
        half = 1,
        two = 2,
        three = 3,
        five = 4,
        ten = 5,
        fifty = 6,
        hundred = 7,
    }
    #[doc = "*"]
    #[doc = " * This DE represents the number of occupants in a vehicle."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `n` (`n >= 0` and `n < 126`) for the number n of occupants,"]
    #[doc = " * - `126` for values equal to or higher than 125,"]
    #[doc = " * - `127` if information is not available."]
    #[doc = " *"]
    #[doc = " * @unit: 1 person"]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: Editorial update in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=127"))]
    pub struct NumberOfOccupants(pub u8);
    #[doc = "* "]
    #[doc = " * This DF indicates both the class and associated subclass that best describes an object."]
    #[doc = " *"]
    #[doc = " * The following options are available:"]
    #[doc = " *"]
    #[doc = " * @field vehicleSubClass: the object is a road vehicle and the specific subclass is specified."]
    #[doc = " *"]
    #[doc = " * @field vruSubClass: the object is a VRU and the specific subclass is specified."]
    #[doc = " *"]
    #[doc = " * @field groupSubClass: the object is a VRU group or cluster and the cluster information is specified."]
    #[doc = " *"]
    #[doc = " * @field otherSubClass: the object is of a different type than the above and the specific subclass is specified."]
    #[doc = " *"]
    #[doc = " * @category: Sensing information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum ObjectClass {
        #[rasn(value("0..=14"))]
        vehicleSubClass(TrafficParticipantType),
        vruSubClass(VruProfileAndSubprofile),
        #[rasn(value("0.."))]
        groupSubClass(VruClusterInformation),
        otherSubClass(OtherSubClass),
    }
    #[doc = "* "]
    #[doc = " * This DF shall contain a list of object classes."]
    #[doc = " *"]
    #[doc = " * @category: Sensing information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8"))]
    pub struct ObjectClassDescription(pub SequenceOf<ObjectClassWithConfidence>);
    #[doc = "* "]
    #[doc = " * This DF represents the classification of a detected object together with a confidence level."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field objectClass: the class of the object."]
    #[doc = " *"]
    #[doc = " * @field Confidence: the associated confidence level."]
    #[doc = " *"]
    #[doc = " * @category: Sensing information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct ObjectClassWithConfidence {
        #[rasn(identifier = "objectClass")]
        pub object_class: ObjectClass,
        pub confidence: ConfidenceLevel,
    }
    impl ObjectClassWithConfidence {
        pub fn new(object_class: ObjectClass, confidence: ConfidenceLevel) -> Self {
            Self {
                object_class,
                confidence,
            }
        }
    }
    #[doc = "* "]
    #[doc = " * This DF represents a dimension of an object together with a confidence value."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field value: the object dimension value which can be estimated as the mean of the current distribution."]
    #[doc = " *"]
    #[doc = " * @field confidence: the associated confidence value."]
    #[doc = " *"]
    #[doc = " * @category: Sensing information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct ObjectDimension {
        pub value: ObjectDimensionValue,
        pub confidence: ObjectDimensionConfidence,
    }
    impl ObjectDimension {
        pub fn new(value: ObjectDimensionValue, confidence: ObjectDimensionConfidence) -> Self {
            Self { value, confidence }
        }
    }
    #[doc = "* "]
    #[doc = " * This DE indicates the object dimension confidence value which represents the estimated absolute accuracy of an object dimension value with a default confidence level of 95 %."]
    #[doc = " * If required, the confidence level can be defined by the corresponding standards applying this DE."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `n` (`n > 0` and `n < 31`) if the confidence value is equal to or less than n x 0,1 metre, and more than (n-1) x 0,1 metre,"]
    #[doc = " * - `31` if the confidence value is out of range i.e. greater than 3,0 m,"]
    #[doc = " * - `32` if the confidence value is unavailable."]
    #[doc = " *"]
    #[doc = " * @unit 0,1 m"]
    #[doc = " * @category: Sensing information"]
    #[doc = " * @revision: Created in V2.1.1 "]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=32"))]
    pub struct ObjectDimensionConfidence(pub u8);
    #[doc = "* "]
    #[doc = " * This DE represents a single dimension of an object."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `n` (`n > 0` and `n < 255`) if the dimension is equal to or less than n x 0,1 m, and more than (n-1) x 0,1 m,"]
    #[doc = " * - `255` if the dimension is out of range i.e. greater than 25,4 m,"]
    #[doc = " * - `256` if the dimension is unavailable."]
    #[doc = " *"]
    #[doc = " * @unit 0,1 m"]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Created in V2.1.1, corrected the wording in V2.4.1 "]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=256"))]
    pub struct ObjectDimensionValue(pub u16);
    #[doc = "*"]
    #[doc = " * This DE indicates the face or part of a face of a solid object."]
    #[doc = " *"]
    #[doc = " * The object is modelled  as a rectangular prism that has a length that is greater than its width, with the faces of the object being defined as:"]
    #[doc = " * - front: the face defined by the prism's width and height, and which is the first face in direction of longitudinal movement of the object,"]
    #[doc = " * - back: the face defined by the prism's width and height, and which is the last face in direction of longitudinal movement of the object,"]
    #[doc = " * - side: the faces defined by the prism's length and height with \"left\" and \"right\" defined by looking at the front face and \"front\" and \"back\" defined w.r.t to the front and back faces. "]
    #[doc = " *"]
    #[doc = " * Note: It is permissible to derive the required object dimensions and orientation from models to provide a best guess."]
    #[doc = " * "]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum ObjectFace {
        front = 0,
        sideLeftFront = 1,
        sideLeftBack = 2,
        sideRightFront = 3,
        sideRightBack = 4,
        back = 5,
    }
    #[doc = "* "]
    #[doc = " * This DE represents a single-value indication about the overall information quality of a perceived object."]
    #[doc = " * "]
    #[doc = " * The value shall be set to:  "]
    #[doc = " * - `0`                        : if there is no confidence in detected object, e.g. for \"ghost\"-objects or if confidence could not be computed,"]
    #[doc = " * - `n` (`n > 0` and `n < 15`) : for the applicable confidence value,"]
    #[doc = " * - `15`                       : if there is full confidence in the detected Object."]
    #[doc = " * "]
    #[doc = " * @unit n/a"]
    #[doc = " * @category: Sensing information"]
    #[doc = " * @revision: Created in V2.1.1 "]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=15"))]
    pub struct ObjectPerceptionQuality(pub u8);
    #[doc = "*"]
    #[doc = " * This DF represents a set of lanes which are partially or fully occupied by an object or event at an externally defined reference position. "]
    #[doc = " *"]
    #[doc = " * @note: In contrast to @ref GeneralizedLanePosition, the dimension of the object or event area (width and length) is taken into account to determine the occupancy, "]
    #[doc = " * i.e. this DF describes the lanes which are blocked by an object or event and not the position of the object / event itself. A confidence is used to describe the "]
    #[doc = " * probability that exactly all the provided lanes are occupied. "]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field lanePositionBased: a set of up to `4` lanes that are partially or fully occupied by an object or event, ordered by increasing value of @ref LanePosition. "]
    #[doc = " * Lanes that are partially occupied can be described using the component lanePositionWithLateralDetails of @ref  Options, with the following constraints: "]
    #[doc = " * The distance to lane borders which are covered by the object / event shall be set to 0. Only the distances to the leftmost and/or rightmost border which are not covered by "]
    #[doc = " * the object / event shall be provided with values > 0. Those values shall be added to the respective instances of @ref LanePositionOptions, i.e. the first entry shall contain the component distanceToLeftBorder > 0 , "]
    #[doc = " * and/or the last entry shall contain the component distanceToRightBorder > 0; the respective other components of these entries shall be set to 0."]
    #[doc = " * "]
    #[doc = " * @field mapBased: optional lane information described in the context of a MAPEM as specified in ETSI TS 103 301 [15]. "]
    #[doc = " * If present, it shall describe the same lane(s) as listed in the component lanePositionBased, but using the lane identification of the MAPEM. This component can be used only if a "]
    #[doc = " * MAPEM is available for the reference position (e.g. on an intersection): In this case it is used as a synonym to the mandatory component lanePositionBased. "]
    #[doc = " *"]
    #[doc = " * @field confidence: mandatory confidence information for expressing the probability that all the provided lanes are occupied. It also provides information on how the lane "]
    #[doc = " * information were generated. If none of the sensors were used, the lane information is assumed to be derived directly from the absolute reference position and the related dimension."]
    #[doc = " *"]
    #[doc = " * @category: Road Topology information"]
    #[doc = " * @revision: Created in V2.2.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct OccupiedLanesWithConfidence {
        #[rasn(size("1..=4"), identifier = "lanePositionBased")]
        pub lane_position_based: SequenceOf<LanePositionOptions>,
        #[rasn(size("1..=4"), identifier = "mapBased")]
        pub map_based: Option<SequenceOf<MapPosition>>,
        pub confidence: MetaInformation,
    }
    impl OccupiedLanesWithConfidence {
        pub fn new(
            lane_position_based: SequenceOf<LanePositionOptions>,
            map_based: Option<SequenceOf<MapPosition>>,
            confidence: MetaInformation,
        ) -> Self {
            Self {
                lane_position_based,
                map_based,
                confidence,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DE represents a time period to describe the opening days and hours of a Point of Interest."]
    #[doc = " * (for example local commerce)."]
    #[doc = " *"]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: V1.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct OpeningDaysHours(pub Utf8String);
    #[doc = "*"]
    #[doc = " * The DE represents an ordinal number that indicates the position of an element in a set. "]
    #[doc = " * "]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct OrdinalNumber1B(pub u8);
    #[doc = "*"]
    #[doc = " * The DE represents an ordinal number that indicates the position of an element in a set. "]
    #[doc = " * "]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=8"))]
    pub struct OrdinalNumber3b(pub u8);
    #[doc = "* "]
    #[doc = " * This DE indicates the subclass of a detected object for @ref ObjectClass \"otherSubclass\"."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `0` - unknown          - if the subclass is unknown."]
    #[doc = " * - `1` - singleObject     - if the object is a single object."]
    #[doc = " * - `2` - multipleObjects  - if the object is a group of multiple objects."]
    #[doc = " * - `3` - bulkMaterial     - if the object is a bulk material."]
    #[doc = " *"]
    #[doc = " * @category: Sensing information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct OtherSubClass(pub u8);
    #[doc = "* "]
    #[doc = " * This DE indicates the arrangement of parking space in a parking area."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `0` to indicate that the parking spaces are arranged in a line and parallel to a road or curb,"]
    #[doc = " * - `1` to indicate that the parking spaces are arranged side-by-side and diagonally to a curb,"]
    #[doc = " * - `2` to indicate that the parking spaces are arranged side-by-side and perpendicularly to a curb,"]
    #[doc = " * - `3` to indicate that the parking spaces are arranged so that vehicles form a queue,"]
    #[doc = " * - `4` to indicate that the parking spaces are arranged in a mixed fashion, "]
    #[doc = " * - 5-7 - reserved for future usage. "]
    #[doc = " *"]
    #[doc = " * @category: Road topology information"]
    #[doc = " * @revision: Created in V2.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=7"))]
    pub struct ParkingAreaArrangementType(pub u8);
    #[doc = "* "]
    #[doc = " * This DF indicates the allowed type of occupancy of a parking space/area in terms of time and/or usage."]
    #[doc = " *"]
    #[doc = " * The following options are available:"]
    #[doc = " * "]
    #[doc = " * @field unknown: indicates that the allowed type of occupancy is unknown."]
    #[doc = " *"]
    #[doc = " * @field unlimitedOccupancy: indicates that it can be occupied without limits."]
    #[doc = " *"]
    #[doc = " * @field onlyWhileCharging: indicates that it can be occupied only while charging an electric vehicle."]
    #[doc = " *"]
    #[doc = " * @field limitedDuration: indicates that it can be occupied for a limited and indicated duration in minutes."]
    #[doc = " *"]
    #[doc = " * @field onlyWhileChargingLimitedDuration: indicates that it can be occupied only while charging an electric vehicle and only for a limited and indicated duration in minutes."]
    #[doc = " *"]
    #[doc = " * @field parkingAllowedUntil: indicates that it can be occupied only until an indicated moment in time."]
    #[doc = " *"]
    #[doc = " * @field forcedParkingUntil: indicates that it can be occupied and departure is possible only after an indicated moment in time."]
    #[doc = " *"]
    #[doc = " * @category: Road Topology information"]
    #[doc = " * @revision: Created in V2.3.1 "]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum ParkingOccupancyInfo {
        unknown(()),
        unlimitedOccupancy(()),
        onlyWhileCharging(()),
        limitedDuration(Integer),
        onlyWhileChargingLimitedDuration(Integer),
        parkingAllowedUntil(TimestampIts),
        forcedParkingUntil(TimestampIts),
    }
    #[doc = "* "]
    #[doc = " * This DE indicates the type of a reservation of a parking space/area."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `0` to indicate that it is reserved to disabled persons,"]
    #[doc = " * - `1` to indicate that it is reserved to pregnant women,"]
    #[doc = " * - `2` to indicate that it is reserved to women,"]
    #[doc = " * - `3` to indicate that it is reserved to parents with small children,"]
    #[doc = " * - `4` to indicate that it is reserved for loading and unloading of goods,"]
    #[doc = " * - `5` to indicate that it is reserved for manual charging of electric vehicles,"]
    #[doc = " * - `6` to indicate that it is reserved for automated charging of electric vehicles,"]
    #[doc = " * - `7` to indicate that it is reserved for vehicles carrying out refrigerated transport of goods,"]
    #[doc = " * - `8` to indicate that it is reserved for VIPs,"]
    #[doc = " * - `9` to indicate that it is reserved for pre-booked reservations only,"]
    #[doc = " * - `10` to indicate that it is not reserved and can still be reserved,"]
    #[doc = " * - `11` to indicate that a reservation type is not applicable, i.e. that it cannot be reserved,"]
    #[doc = " * - `12` to indicate that it reserved for drop-off and pick-up of vehicles for automated valet parking,"]
    #[doc = " * - `13` to indicate that it is reserved for vehicles with a permit,"]
    #[doc = " * - `14` to indicate that it is an (often unmarked / undesignated, but still not prohibited) space/area which is reserved for use only on occasions "]
    #[doc = "          when all official marked parking spaces to which it blocks the access (if any), are already occupied at the moment of arrival."]
    #[doc = " * - 15-31  - reserved for future usage. "]
    #[doc = " *"]
    #[doc = " * @category: Road topology information"]
    #[doc = " * @revision: Created in V2.3.1, value 14 assigned in V2.4.1."]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=31"))]
    pub struct ParkingReservationType(pub u8);
    #[doc = "*"]
    #[doc = " * This DF provides basic information about the parking capabilities and availabilities of a single parking space. "]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field id: the unqiue identifier of the parking space within the parking area."]
    #[doc = " *"]
    #[doc = " * @field location: the optional location of the geometrical center of the parking space w.r.t. the location of the parking area."]
    #[doc = " *"]
    #[doc = " * @field status: the actual status of the parking space."]
    #[doc = " *"]
    #[doc = " * @category: Road Topology information"]
    #[doc = " * @revision: Created in V2.3.1 "]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct ParkingSpaceBasic {
        pub id: Identifier2B,
        pub location: Option<DeltaReferencePosition>,
        pub status: ParkingSpaceStatus,
    }
    impl ParkingSpaceBasic {
        pub fn new(
            id: Identifier2B,
            location: Option<DeltaReferencePosition>,
            status: ParkingSpaceStatus,
        ) -> Self {
            Self {
                id,
                location,
                status,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DF provides detailed information about the parking capabilities and availabilities of a single parking space. "]
    #[doc = " * "]
    #[doc = " * It is an extension of @ref ParkingSpaceBasic and it shall additionally include the following additional components: "]
    #[doc = " *"]
    #[doc = " * @field arrangementType: the optional arrangement of the parking space w.r.t. other spaces."]
    #[doc = " * This is component, if present, overrides the common arrangementType defined in the @ref ParkingArea."]
    #[doc = " *"]
    #[doc = " * @field boundary: the optional physical boundary of the parking space as a polygon w.r.t. the location of the parking space."]
    #[doc = " *"]
    #[doc = " * @field orientation: the optional orientation of the parking space."]
    #[doc = " * This is component, if present, overrides the common orientation defined in the @ref ParkingArea."]
    #[doc = " *"]
    #[doc = " * @field occupancyRule: the occupancy rule applicable to the parking space."]
    #[doc = " *"]
    #[doc = " * @field chargingStationId: the optional identitfier of a charging station that serves the parking space."]
    #[doc = " *"]
    #[doc = " * @field accessViaLane: the optional identifier of a lane that provides access to the parking space."]
    #[doc = " *"]
    #[doc = " * @field accessViaParkingSpaces: the optional identifier(s) of a parking spaces that provide access to the parking space."]
    #[doc = " * "]
    #[doc = " * @field reservationType: the optional parking reservation type(s) associated to the parking space."]
    #[doc = " * This is component, if present, overrides the common reservationType defined in the @ref ParkingArea."]
    #[doc = " *"]
    #[doc = " * @category: Road Topology information"]
    #[doc = " * @revision: Created in V2.3.1 "]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ParkingSpaceDetailed {
        #[rasn(identifier = "arrangementType")]
        pub arrangement_type: Option<ParkingAreaArrangementType>,
        pub boundary: Option<DeltaPositions>,
        pub orientation: Option<Wgs84Angle>,
        #[rasn(identifier = "occupancyRule")]
        pub occupancy_rule: ParkingOccupancyInfo,
        #[rasn(identifier = "chargingStationId")]
        pub charging_station_id: Option<Identifier2B>,
        #[rasn(identifier = "accessViaLane")]
        pub access_via_lane: Option<Identifier2B>,
        #[rasn(size("0..=7"), identifier = "accessViaParkingSpaces")]
        pub access_via_parking_spaces: Option<SequenceOf<Identifier2B>>,
        #[rasn(size("1..=4", extensible), identifier = "reservationType")]
        pub reservation_type: Option<SequenceOf<ParkingReservationType>>,
        pub id: Identifier2B,
        pub location: Option<DeltaReferencePosition>,
        pub status: ParkingSpaceStatus,
    }
    impl ParkingSpaceDetailed {
        pub fn new(
            arrangement_type: Option<ParkingAreaArrangementType>,
            boundary: Option<DeltaPositions>,
            orientation: Option<Wgs84Angle>,
            occupancy_rule: ParkingOccupancyInfo,
            charging_station_id: Option<Identifier2B>,
            access_via_lane: Option<Identifier2B>,
            access_via_parking_spaces: Option<SequenceOf<Identifier2B>>,
            reservation_type: Option<SequenceOf<ParkingReservationType>>,
            id: Identifier2B,
            location: Option<DeltaReferencePosition>,
            status: ParkingSpaceStatus,
        ) -> Self {
            Self {
                arrangement_type,
                boundary,
                orientation,
                occupancy_rule,
                charging_station_id,
                access_via_lane,
                access_via_parking_spaces,
                reservation_type,
                id,
                location,
                status,
            }
        }
    }
    #[doc = "* "]
    #[doc = " * This DF indicates the status of parking space."]
    #[doc = " *"]
    #[doc = " * The following options are available:"]
    #[doc = " * "]
    #[doc = " * @field unknown: indicates that the status is unknown."]
    #[doc = " *"]
    #[doc = " * @field free: indicates that the parking space is free and hence available to be used."]
    #[doc = " *"]
    #[doc = " * @field freeUntil: indicates that the parking space is free to be used until an indicated moment in time."]
    #[doc = " *"]
    #[doc = " * @field fullyOccupied: indicates that the parking space is interely occupied."]
    #[doc = " *"]
    #[doc = " * @field partiallyOccupied: indicates that the parking space that allows parking of multiple vehicles is occupied by an indicated percentage."]
    #[doc = " *"]
    #[doc = " * @field occupiedUntil: indicates that the parking space is entirely occupied until an indicated moment in time."]
    #[doc = " *"]
    #[doc = " * @field reservedUntil: indicates that the parking space is reserved (but not necessarily occupied) until an indicated moment in time."]
    #[doc = " *"]
    #[doc = " * @field accessBlocked: indicates that the parking space cannot accessed, e.g. due to obstructing vehicles."]
    #[doc = " *"]
    #[doc = " * @field retrictedUsage: indicates that the parking space is available but that it is not an official parking space and that there are some phyiscal restrictions applicable, "]
    #[doc = " * such as parking behind or in front of other vehicles that, depending on the situation, may then have problems entering or leaving their respective parking spaces."]
    #[doc = " *"]
    #[doc = " * @category: Road Topology information"]
    #[doc = " * @revision: Created in V2.3.1 "]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum ParkingSpaceStatus {
        unknown(()),
        free(()),
        freeUntil(TimestampIts),
        fullyOccupied(()),
        #[rasn(value("0..=100"))]
        partiallyOccupied(u8),
        occupiedUntil(TimestampIts),
        reservedUntil(TimestampIts),
        accessBlocked(()),
        retrictedUsage(()),
    }
    #[doc = "*"]
    #[doc = " * This DF represents a path with a set of path points."]
    #[doc = " * It shall contain up to `40` @ref PathPoint. "]
    #[doc = " * "]
    #[doc = " * The first PathPoint presents an offset delta position with regards to an external reference position."]
    #[doc = " * Each other PathPoint presents an offset delta position and optionally an offset travel time with regards to the previous PathPoint. "]
    #[doc = " *"]
    #[doc = " * @category: GeoReference information, Vehicle information"]
    #[doc = " * @revision: created in V2.1.1 based on PathHistory"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=40"))]
    pub struct Path(pub SequenceOf<PathPoint>);
    #[doc = "*"]
    #[doc = " * This DE represents the recorded or estimated travel time between a position and a predefined reference position. "]
    #[doc = " *"]
    #[doc = " * @unit 0,01 second"]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: V1.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=65535", extensible))]
    pub struct PathDeltaTime(pub Integer);
    #[doc = "*"]
    #[doc = " * This DF represents estimated/predicted travel time between a position and a predefined reference position. "]
    #[doc = " *"]
    #[doc = " * the following options are available:"]
    #[doc = " * "]
    #[doc = " * @field deltaTimeHighPrecision: delta time with precision of 0,1 s."]
    #[doc = " *"]
    #[doc = " * @field deltaTimeBigRange: delta time with precision of 10 s."]
    #[doc = " *"]
    #[doc = " * @field deltaTimeMidRange: delta time with precision of 1 s."]
    #[doc = " *"]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Created in V2.2.1, added deltaTimeMidRange extension in V2.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum PathDeltaTimeChoice {
        deltaTimeHighPrecision(DeltaTimeTenthOfSecond),
        deltaTimeBigRange(DeltaTimeTenSeconds),
        #[rasn(extension_addition)]
        deltaTimeMidRange(DeltaTimeSecond),
    }
    #[doc = "* "]
    #[doc = " * This DF represents a path towards a specific point specified in the @ref EventZone."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field pointOfEventZone: the ordinal number of the point within the DF EventZone, i.e. within the list of EventPoints."]
    #[doc = " *"]
    #[doc = " * @field path: the associated path towards the point specified in pointOfEventZone."]
    #[doc = " * The first PathPoint presents an offset delta position with regards to the position of that pointOfEventZone."]
    #[doc = " *"]
    #[doc = " * @category: GeoReference information"]
    #[doc = " * @revision: Created in V2.2.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct PathExtended {
        #[rasn(value("1..=23"), identifier = "pointOfEventZone")]
        pub point_of_event_zone: u8,
        pub path: Path,
    }
    impl PathExtended {
        pub fn new(point_of_event_zone: u8, path: Path) -> Self {
            Self {
                point_of_event_zone,
                path,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DF represents a path history with a set of path points."]
    #[doc = " * It shall contain up to `40` @ref PathPoint. "]
    #[doc = " * "]
    #[doc = " * The first PathPoint presents an offset delta position with regards to an external reference position."]
    #[doc = " * Each other PathPoint presents an offset delta position and optionally an offset travel time with regards to the previous PathPoint. "]
    #[doc = " *"]
    #[doc = " * @note: this DF is kept for backwards compatibility reasons only. It is recommended to use @ref Path instead."]
    #[doc = " * @category: GeoReference information, Vehicle information"]
    #[doc = " * @revision: semantics updated in V2.1.1, size corrected to 0..40 in V2.2.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=40"))]
    pub struct PathHistory(pub SequenceOf<PathPoint>);
    #[doc = "* "]
    #[doc = " * This DE indicates an ordinal number that represents the position of a component in the list of @ref Traces or @ref TracesExtended. "]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `0` - noPath  - if no path is identified"]
    #[doc = " * - `1..7`        - for instances 1..7 of @ref Traces "]
    #[doc = " * - `8..14`       - for instances 1..7 of @ref TracesExtended. "]
    #[doc = " *"]
    #[doc = " * @category: Road topology information"]
    #[doc = " * @revision: Created in V2.2.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=14"))]
    pub struct PathId(pub u8);
    #[doc = "*"]
    #[doc = " * This DF defines an offset waypoint position within a path."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field pathPosition: The waypoint position defined as an offset position with regards to a pre-defined reference position. "]
    #[doc = " *"]
    #[doc = " * @field pathDeltaTime: The optional travel time separated from a waypoint to the predefined reference position."]
    #[doc = " *"]
    #[doc = " * @category GeoReference information"]
    #[doc = " * @revision: semantics updated in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct PathPoint {
        #[rasn(identifier = "pathPosition")]
        pub path_position: DeltaReferencePosition,
        #[rasn(identifier = "pathDeltaTime")]
        pub path_delta_time: Option<PathDeltaTime>,
    }
    impl PathPoint {
        pub fn new(
            path_position: DeltaReferencePosition,
            path_delta_time: Option<PathDeltaTime>,
        ) -> Self {
            Self {
                path_position,
                path_delta_time,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DF defines a predicted offset position that can be used within a predicted path or trajectory, together with optional data to describe a path zone shape."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field deltaLatitude: the offset latitude with regards to a pre-defined reference position. "]
    #[doc = " *"]
    #[doc = " * @field deltaLongitude: the offset longitude with regards to a pre-defined reference position. "]
    #[doc = " * "]
    #[doc = " * @field horizontalPositionConfidence: the optional confidence value associated to the horizontal geographical position."]
    #[doc = " *"]
    #[doc = " * @field deltaAltitude: the optional offset altitude with regards to a pre-defined reference position, with default value unavailable. "]
    #[doc = " *"]
    #[doc = " * @field altitudeConfidence: the optional confidence value associated to the altitude value of the geographical position, with default value unavailable."]
    #[doc = " * "]
    #[doc = " * @field pathDeltaTime: the optional travel time to the waypoint from the predefined reference position."]
    #[doc = ""]
    #[doc = " * @field symmetricAreaOffset: the optional symmetric offset to generate a shape, see Annex D for details."]
    #[doc = " *  "]
    #[doc = " * @field asymmetricAreaOffset: the optional asymmetric offset to generate a shape, see Annex D for details. "]
    #[doc = " *"]
    #[doc = " * @category GeoReference information"]
    #[doc = " * @revision: Created in V2.1.1, type of pathDeltaTime changed and optionality added, fields symmetricAreaOffset and asymmetricAreaOffset added in V2.2.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct PathPointPredicted {
        #[rasn(identifier = "deltaLatitude")]
        pub delta_latitude: DeltaLatitude,
        #[rasn(identifier = "deltaLongitude")]
        pub delta_longitude: DeltaLongitude,
        #[rasn(identifier = "horizontalPositionConfidence")]
        pub horizontal_position_confidence: Option<PosConfidenceEllipse>,
        #[rasn(
            default = "path_point_predicted_delta_altitude_default",
            identifier = "deltaAltitude"
        )]
        pub delta_altitude: DeltaAltitude,
        #[rasn(
            default = "path_point_predicted_altitude_confidence_default",
            identifier = "altitudeConfidence"
        )]
        pub altitude_confidence: AltitudeConfidence,
        #[rasn(identifier = "pathDeltaTime")]
        pub path_delta_time: Option<PathDeltaTimeChoice>,
        #[rasn(identifier = "symmetricAreaOffset")]
        pub symmetric_area_offset: Option<StandardLength9b>,
        #[rasn(identifier = "asymmetricAreaOffset")]
        pub asymmetric_area_offset: Option<StandardLength9b>,
    }
    impl PathPointPredicted {
        pub fn new(
            delta_latitude: DeltaLatitude,
            delta_longitude: DeltaLongitude,
            horizontal_position_confidence: Option<PosConfidenceEllipse>,
            delta_altitude: DeltaAltitude,
            altitude_confidence: AltitudeConfidence,
            path_delta_time: Option<PathDeltaTimeChoice>,
            symmetric_area_offset: Option<StandardLength9b>,
            asymmetric_area_offset: Option<StandardLength9b>,
        ) -> Self {
            Self {
                delta_latitude,
                delta_longitude,
                horizontal_position_confidence,
                delta_altitude,
                altitude_confidence,
                path_delta_time,
                symmetric_area_offset,
                asymmetric_area_offset,
            }
        }
    }
    fn path_point_predicted_delta_altitude_default() -> DeltaAltitude {
        DeltaAltitude(12800)
    }
    fn path_point_predicted_altitude_confidence_default() -> AltitudeConfidence {
        AltitudeConfidence::unavailable
    }
    #[doc = "*"]
    #[doc = " * This DF represents a predicted path or trajectory with a set of predicted points and optional information to generate a shape which is estimated to contain the real path. "]
    #[doc = " * It shall contain up to `16` @ref PathPointPredicted. "]
    #[doc = " * "]
    #[doc = " * The first PathPoint presents an offset delta position with regards to an external reference position."]
    #[doc = " * Each other PathPoint presents an offset delta position and optionally an offset travel time with regards to the previous PathPoint. "]
    #[doc = " *"]
    #[doc = " * @category: GeoReference information"]
    #[doc = " * @revision: created in V2.1.1 , size constraint changed to SIZE(1..16, ...) in V2.2.1, size extended in V2.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16", extensible))]
    pub struct PathPredicted(pub SequenceOf<PathPointPredicted>);
    #[doc = "* "]
    #[doc = " * This DF represents a predicted path, predicted trajectory or predicted path zone together with usage information and a prediction confidence."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field pathPredicted: the predicted path (pathDeltaTime ABSENT) or trajectory (pathDeltaTime PRESENT) and/or the path zone (symmetricAreaOffset PRESENT)."]
    #[doc = " *"]
    #[doc = " * @field usageIndication: an indication of how the predicted path will be used. "]
    #[doc = " *"]
    #[doc = " * @field confidenceLevel: the confidence that the path/trajectory in pathPredicted will occur as predicted."]
    #[doc = " *"]
    #[doc = " * @category: GeoReference information"]
    #[doc = " * @revision: created in V2.2.1 "]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct PathPredicted2 {
        #[rasn(value("0.."), identifier = "pathPredicted")]
        pub path_predicted: PathPredicted,
        #[rasn(identifier = "usageIndication")]
        pub usage_indication: UsageIndication,
        #[rasn(identifier = "confidenceLevel")]
        pub confidence_level: ConfidenceLevel,
    }
    impl PathPredicted2 {
        pub fn new(
            path_predicted: PathPredicted,
            usage_indication: UsageIndication,
            confidence_level: ConfidenceLevel,
        ) -> Self {
            Self {
                path_predicted,
                usage_indication,
                confidence_level,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DF represents one or more predicted paths, or trajectories or path zones (zones that include all possible paths/trajectories within its boundaries) using @ref PathPredicted2."]
    #[doc = " * It shall contain up to `16` @ref PathPredicted2. "]
    #[doc = " * "]
    #[doc = " * @category: GeoReference information"]
    #[doc = " * @revision: V2.2.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16", extensible))]
    pub struct PathPredictedList(pub SequenceOf<PathPredicted2>);
    #[doc = "* "]
    #[doc = " * This DF represents a list of references to the components of a @ref Traces or @ref TracesExtended DF using the @ref PathId. "]
    #[doc = " *"]
    #[doc = " * @category: Road topology information"]
    #[doc = " * @revision: Created in V2.2.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=14"))]
    pub struct PathReferences(pub SequenceOf<PathId>);
    #[doc = "*"]
    #[doc = " * This DE represents the position of a vehicle pedal (e.g. brake or accelerator pedal)."]
    #[doc = " *"]
    #[doc = " * @unit: 10%"]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: Created in V2.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=11"))]
    pub struct PedalPositionValue(pub u8);
    #[doc = "*"]
    #[doc = " * This DE contains information about the status of a vehicle pedal."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field pedalPositionValue: information about the pedal position. "]
    #[doc = " *"]
    #[doc = " * @category: vehicle information"]
    #[doc = " * @revision: created in V2.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct PedalStatus {
        #[rasn(identifier = "pedalPositionValue")]
        pub pedal_position_value: PedalPositionValue,
    }
    impl PedalStatus {
        pub fn new(pedal_position_value: PedalPositionValue) -> Self {
            Self {
                pedal_position_value,
            }
        }
    }
    #[doc = "* "]
    #[doc = " * This DF contains information about a perceived object including its kinematic state and attitude vector in a pre-defined coordinate system and with respect to a reference time."]
    #[doc = " * "]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field objectId: optional identifier assigned to a detected object."]
    #[doc = " *"]
    #[doc = " * @field measurementDeltaTime: the time difference from a reference time to the time of the  measurement of the object. "]
    #[doc = " * Negative values indicate that the provided object state refers to a point in time before the reference time."]
    #[doc = " *"]
    #[doc = " * @field position: the position of the geometric centre of the object's bounding box within the pre-defined coordinate system."]
    #[doc = " *"]
    #[doc = " * @field velocity: the velocity vector of the object within the pre-defined coordinate system."]
    #[doc = " *"]
    #[doc = " * @field acceleration: the acceleration vector of the object within the pre-defined coordinate system."]
    #[doc = " *"]
    #[doc = " * @field angles: optional Euler angles of the object bounding box at the time of measurement. "]
    #[doc = " * "]
    #[doc = " * @field zAngularVelocity: optional angular velocity of the object around the z-axis at the time of measurement."]
    #[doc = " * The angular velocity is measured with positive values considering the object orientation turning around the z-axis using the right-hand rule."]
    #[doc = " *"]
    #[doc = " * @field lowerTriangularCorrelationMatrices: optional set of lower triangular correlation matrices for selected components of the provided kinematic state and attitude vector."]
    #[doc = " *"]
    #[doc = " * @field objectDimensionZ: optional z-dimension of object bounding box. "]
    #[doc = " * This dimension shall be measured along the direction of the z-axis after all the rotations have been applied. "]
    #[doc = " *"]
    #[doc = " * @field objectDimensionY: optional y-dimension of the object bounding box. "]
    #[doc = " * This dimension shall be measured along the direction of the y-axis after all the rotations have been applied. "]
    #[doc = " *"]
    #[doc = " * @field objectDimensionX: optional x-dimension of object bounding box."]
    #[doc = " * This dimension shall be measured along the direction of the x-axis after all the rotations have been applied."]
    #[doc = " * "]
    #[doc = " * @field objectAge: optional age of the detected and described object, i.e. the difference in time between the moment "]
    #[doc = " * it has been first detected and the reference time of the message. Value `1500` indicates that the object has been observed for more than 1.5s."]
    #[doc = " *"]
    #[doc = " * @field objectPerceptionQuality: optional confidence associated to the object. "]
    #[doc = " *"]
    #[doc = " * @field sensorIdList: optional list of sensor-IDs which provided the measurement data. "]
    #[doc = " *"]
    #[doc = " * @field classification: optional classification of the described object"]
    #[doc = " *"]
    #[doc = " * @field matchedPosition: optional map-matched position of an object."]
    #[doc = " *"]
    #[doc = " * @category Sensing information"]
    #[doc = " * @revision: created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct PerceivedObject {
        #[rasn(identifier = "objectId")]
        pub object_id: Option<Identifier2B>,
        #[rasn(identifier = "measurementDeltaTime")]
        pub measurement_delta_time: DeltaTimeMilliSecondSigned,
        pub position: CartesianPosition3dWithConfidence,
        pub velocity: Option<Velocity3dWithConfidence>,
        pub acceleration: Option<Acceleration3dWithConfidence>,
        pub angles: Option<EulerAnglesWithConfidence>,
        #[rasn(identifier = "zAngularVelocity")]
        pub z_angular_velocity: Option<CartesianAngularVelocityComponent>,
        #[rasn(identifier = "lowerTriangularCorrelationMatrices")]
        pub lower_triangular_correlation_matrices:
            Option<LowerTriangularPositiveSemidefiniteMatrices>,
        #[rasn(identifier = "objectDimensionZ")]
        pub object_dimension_z: Option<ObjectDimension>,
        #[rasn(identifier = "objectDimensionY")]
        pub object_dimension_y: Option<ObjectDimension>,
        #[rasn(identifier = "objectDimensionX")]
        pub object_dimension_x: Option<ObjectDimension>,
        #[rasn(value("0..=2047"), identifier = "objectAge")]
        pub object_age: Option<DeltaTimeMilliSecondSigned>,
        #[rasn(identifier = "objectPerceptionQuality")]
        pub object_perception_quality: Option<ObjectPerceptionQuality>,
        #[rasn(identifier = "sensorIdList")]
        pub sensor_id_list: Option<SequenceOfIdentifier1B>,
        pub classification: Option<ObjectClassDescription>,
        #[rasn(identifier = "mapPosition")]
        pub map_position: Option<MapPosition>,
    }
    impl PerceivedObject {
        pub fn new(
            object_id: Option<Identifier2B>,
            measurement_delta_time: DeltaTimeMilliSecondSigned,
            position: CartesianPosition3dWithConfidence,
            velocity: Option<Velocity3dWithConfidence>,
            acceleration: Option<Acceleration3dWithConfidence>,
            angles: Option<EulerAnglesWithConfidence>,
            z_angular_velocity: Option<CartesianAngularVelocityComponent>,
            lower_triangular_correlation_matrices: Option<
                LowerTriangularPositiveSemidefiniteMatrices,
            >,
            object_dimension_z: Option<ObjectDimension>,
            object_dimension_y: Option<ObjectDimension>,
            object_dimension_x: Option<ObjectDimension>,
            object_age: Option<DeltaTimeMilliSecondSigned>,
            object_perception_quality: Option<ObjectPerceptionQuality>,
            sensor_id_list: Option<SequenceOfIdentifier1B>,
            classification: Option<ObjectClassDescription>,
            map_position: Option<MapPosition>,
        ) -> Self {
            Self {
                object_id,
                measurement_delta_time,
                position,
                velocity,
                acceleration,
                angles,
                z_angular_velocity,
                lower_triangular_correlation_matrices,
                object_dimension_z,
                object_dimension_y,
                object_dimension_x,
                object_age,
                object_perception_quality,
                sensor_id_list,
                classification,
                map_position,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DE denotes the ability of an ITS-S to provide information fullfilling additional requirements."]
    #[doc = " * A performance class value is used to describe characteristics of data. The semantic defintion of the values are out of scope of the present document "]
    #[doc = " * and should be subject to profiling."]
    #[doc = " * "]
    #[doc = " *  The value shall be set to:"]
    #[doc = " * - `0` if  the performance class is unknown,"]
    #[doc = " * - `1` for performance class A,"]
    #[doc = " * - `2` for performance class B,"]
    #[doc = " * -  3-7 reserved for future use."]
    #[doc = " *"]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: Editorial update in V2.1.1, description changed in V2.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=7"))]
    pub struct PerformanceClass(pub u8);
    #[doc = "*"]
    #[doc = " * This DE represents a telephone number"]
    #[doc = " * "]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: V1.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16"))]
    pub struct PhoneNumber(pub NumericString);
    #[doc = "*"]
    #[doc = "* The data frame PolygonalLine shall contain the definition of a polygonal line (also known as polygonal chain) w.r.t. an externally defined reference position."]
    #[doc = "*"]
    #[doc = "* The following options are available:"]
    #[doc = "* "]
    #[doc = "* @field deltaPositions: an ordered sequence of delta geographical positions with respect to the previous position, with latitude and longitude,"]
    #[doc = "* with the first instance referring to the reference position and with the order implicitly defining a direction associated with the polygonal line."]
    #[doc = "*"]
    #[doc = "* @field deltaPositionsWithAltitude:  an ordered sequence of delta geographical positions with respect to the previous position, with latitude, longitude and altitude,"]
    #[doc = "* with the first instance referring to the reference position and with the order implicitly defining a direction associated with the polygonal line."]
    #[doc = "*"]
    #[doc = "* @field absolutePositions: a sequence of absolute geographical positions, with latitude and longitude."]
    #[doc = "*"]
    #[doc = "* @field absolutePositionsWithAltitude: a sequence of absolute geographical positions, with latitude, longitude and altitude."]
    #[doc = "*"]
    #[doc = "* @category: GeoReference information"]
    #[doc = "* @revision: created in V2.3.1 based on ISO TS 19321"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum PolygonalLine {
        deltaPositions(DeltaPositions),
        deltaPositionsWithAltitude(DeltaReferencePositions),
        absolutePositions(GeoPositionsWoAltitude),
        absolutePositionsWithAltitude(GeoPositionsWAltitude),
    }
    #[doc = "* "]
    #[doc = " * This DF represents the shape of a polygonal area or of a right prism."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field shapeReferencePoint: the optional reference point used for the definition of the shape, relative to an externally specified reference position. "]
    #[doc = " * If this component is absent, the externally specified reference position represents the shape's reference point. "]
    #[doc = " *"]
    #[doc = " * @field polygon: the polygonal area represented by a list of minimum `3` to maximum `16` @ref CartesianPosition3d."]
    #[doc = " * All nodes of the polygon shall be considered relative to the shape's reference point."]
    #[doc = " *"]
    #[doc = " * @field height: the optional height, present if the shape is a right prism extending in the positive z-axis."]
    #[doc = " * "]
    #[doc = " * @category GeoReference information"]
    #[doc = " * @revision: created in V2.1.1"]
    #[doc = " *"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct PolygonalShape {
        #[rasn(identifier = "shapeReferencePoint")]
        pub shape_reference_point: Option<CartesianPosition3d>,
        #[rasn(size("3..=16", extensible))]
        pub polygon: SequenceOfCartesianPosition3d,
        pub height: Option<StandardLength12b>,
    }
    impl PolygonalShape {
        pub fn new(
            shape_reference_point: Option<CartesianPosition3d>,
            polygon: SequenceOfCartesianPosition3d,
            height: Option<StandardLength12b>,
        ) -> Self {
            Self {
                shape_reference_point,
                polygon,
                height,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DE indicates the perpendicular distance from the centre of mass of an empty load vehicle to the front line of"]
    #[doc = " * the vehicle bounding box of the empty load vehicle."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `n` (`n > 0` and `n < 62`) for any aplicable value n between 0,1 metre and 6,2 metres, "]
    #[doc = " * - `62` for values equal to or higher than 6.1 metres,"]
    #[doc = " * - `63`  if the information is unavailable."]
    #[doc = " * "]
    #[doc = " * @note:\tThe empty load vehicle is defined in ISO 1176 [8], clause 4.6."]
    #[doc = " *"]
    #[doc = " * @unit 0,1 metre "]
    #[doc = " * @category Vehicle information"]
    #[doc = " * @revision: description revised in V2.1.1 (the meaning of 62 has changed slightly) "]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=63"))]
    pub struct PosCentMass(pub u8);
    #[doc = "*"]
    #[doc = " * This DF indicates the horizontal position confidence ellipse which represents the estimated accuracy with a "]
    #[doc = " * confidence level of 95  %. The centre of the ellipse shape corresponds to the reference"]
    #[doc = " * position point for which the position accuracy is evaluated."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field semiMajorConfidence: half of length of the major axis, i.e. distance between the centre point"]
    #[doc = " * and major axis point of the position accuracy ellipse. "]
    #[doc = " *"]
    #[doc = " * @field semiMinorConfidence: half of length of the minor axis, i.e. distance between the centre point"]
    #[doc = " * and minor axis point of the position accuracy ellipse. "]
    #[doc = " *"]
    #[doc = " * @field semiMajorOrientation: orientation direction of the ellipse major axis of the position accuracy"]
    #[doc = " * ellipse with regards to the WGS84 north. "]
    #[doc = " * The specific WGS84 coordinate system is specified by the corresponding standards applying this DE."]
    #[doc = " *"]
    #[doc = " *"]
    #[doc = " * @category GeoReference information"]
    #[doc = " * @revision: V1.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct PosConfidenceEllipse {
        #[rasn(identifier = "semiMajorConfidence")]
        pub semi_major_confidence: SemiAxisLength,
        #[rasn(identifier = "semiMinorConfidence")]
        pub semi_minor_confidence: SemiAxisLength,
        #[rasn(identifier = "semiMajorOrientation")]
        pub semi_major_orientation: HeadingValue,
    }
    impl PosConfidenceEllipse {
        pub fn new(
            semi_major_confidence: SemiAxisLength,
            semi_minor_confidence: SemiAxisLength,
            semi_major_orientation: HeadingValue,
        ) -> Self {
            Self {
                semi_major_confidence,
                semi_minor_confidence,
                semi_major_orientation,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DE indicates the perpendicular distance between the vehicle front line of the bounding box and the front wheel axle in 0,1 metre."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `n` (`n > 0` and `n < 19`) for any aplicable value between 0,1 metre and 1,9 metres,"]
    #[doc = " * - `19` for values equal to or higher than 1.8 metres,"]
    #[doc = " * - `20` if the information is unavailable."]
    #[doc = " *"]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @unit 0,1 metre"]
    #[doc = " * @revision: description revised in V2.1.1 (the meaning of 19 has changed slightly) "]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=20"))]
    pub struct PosFrontAx(pub u8);
    #[doc = "*"]
    #[doc = " * This DE represents the distance from the centre of vehicle front bumper to the right or left longitudinal carrier of vehicle."]
    #[doc = " * The left/right carrier refers to the left/right as seen from a passenger sitting in the vehicle."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `n` (`n > 0` and `n < 126`) for any aplicable value between 0,01 metre and 1,26 metres, "]
    #[doc = " * - `126` for values equal to or higher than 1.25 metres,"]
    #[doc = " * - `127` if the information is unavailable."]
    #[doc = " *"]
    #[doc = " * @unit 0,01 metre "]
    #[doc = " * @category Vehicle information"]
    #[doc = " * @revision: description revised in V2.1.1 (the meaning of 126 has changed slightly) "]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=127"))]
    pub struct PosLonCarr(pub u8);
    #[doc = "*"]
    #[doc = " * This DE represents the perpendicular inter-distance of neighbouring pillar axis of vehicle starting from the"]
    #[doc = " * middle point of the front line of the vehicle bounding box."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `n` (`n > 0` and `n < 29`) for any aplicable value between 0,1 metre and 2,9 metres, "]
    #[doc = " * - `29` for values equal to or greater than 2.8 metres,"]
    #[doc = " * - `30` if the information is unavailable."]
    #[doc = " * "]
    #[doc = " * @unit 0,1 metre "]
    #[doc = " * @category Vehicle information"]
    #[doc = " * @revision: description revised in V2.1.1 (the meaning of 29 has changed slightly) "]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=30"))]
    pub struct PosPillar(pub u8);
    #[doc = "* "]
    #[doc = " * This DE represents a position along a single dimension such as the middle of a road or lane, measured as an offset from an externally defined starting point, "]
    #[doc = " * in direction of an externally defined reference direction."]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `n` (`n >= -8190` and `n < 0`) if the position is equal to or less than n x 1 metre and more than (n-1) x 1 metre, in opposite direction of the reference direction,"]
    #[doc = " * - `0` if the position is at the starting point,"]
    #[doc = " * - `n` (`n > 0` and `n < 8190`) if the position is equal to or less than n x 1 metre and more than (n-1) x 1 metre, in the same direction as the reference direction,"]
    #[doc = " * - `8 190` if the position is out of range, i.e. equal to or greater than 8 189 m,"]
    #[doc = " * - `8 191` if the position information is not available. "]
    #[doc = " *"]
    #[doc = " * @unit 1 metre"]
    #[doc = " * @category: GeoReference information"]
    #[doc = " * @revision: Created in V2.2.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-8190..=8191"))]
    pub struct Position1d(pub i16);
    #[doc = "*"]
    #[doc = " * This DF indicates the horizontal position confidence ellipse which represents the estimated accuracy with a "]
    #[doc = " * confidence level of 95 %. The centre of the ellipse shape corresponds to the reference"]
    #[doc = " * position point for which the position accuracy is evaluated."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field semiMajorAxisLength: half of length of the major axis, i.e. distance between the centre point"]
    #[doc = " * and major axis point of the position accuracy ellipse. "]
    #[doc = " *"]
    #[doc = " * @field semiMinorAxisLength: half of length of the minor axis, i.e. distance between the centre point"]
    #[doc = " * and minor axis point of the position accuracy ellipse. "]
    #[doc = " *"]
    #[doc = " * @field semiMajorAxisOrientation: orientation direction of the ellipse major axis of the position accuracy"]
    #[doc = " * ellipse with regards to the WGS84 north. "]
    #[doc = " * The specific WGS84 coordinate system is specified by the corresponding standards applying this DE."]
    #[doc = " *"]
    #[doc = " * @category GeoReference information"]
    #[doc = " * @revision: created in V2.1.1 based on @ref PosConfidenceEllipse"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct PositionConfidenceEllipse {
        #[rasn(identifier = "semiMajorAxisLength")]
        pub semi_major_axis_length: SemiAxisLength,
        #[rasn(identifier = "semiMinorAxisLength")]
        pub semi_minor_axis_length: SemiAxisLength,
        #[rasn(identifier = "semiMajorAxisOrientation")]
        pub semi_major_axis_orientation: Wgs84AngleValue,
    }
    impl PositionConfidenceEllipse {
        pub fn new(
            semi_major_axis_length: SemiAxisLength,
            semi_minor_axis_length: SemiAxisLength,
            semi_major_axis_orientation: Wgs84AngleValue,
        ) -> Self {
            Self {
                semi_major_axis_length,
                semi_minor_axis_length,
                semi_major_axis_orientation,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DE indicates whether a passenger seat is occupied or whether the occupation status is detectable or not."]
    #[doc = " * "]
    #[doc = " * The number of row in vehicle seats layout is counted in rows from the driver row backwards from front to the rear"]
    #[doc = " * of the vehicle."]
    #[doc = " * The left side seat of a row refers to the left hand side seen from vehicle rear to front."]
    #[doc = " * Additionally, a bit is reserved for each seat row, to indicate if the seat occupation of a row is detectable or not,"]
    #[doc = " * i.e. `row1NotDetectable (3)`, `row2NotDetectable(8)`, `row3NotDetectable(13)` and `row4NotDetectable(18)`."]
    #[doc = " * Finally, a bit is reserved for each row seat to indicate if the seat row is present or not in the vehicle,"]
    #[doc = " * i.e. `row1NotPresent (4)`, `row2NotPresent (9)`, `row3NotPresent(14)`, `row4NotPresent(19)`."]
    #[doc = " * "]
    #[doc = " * When a seat is detected to be occupied, the corresponding seat occupation bit shall be set to `1`."]
    #[doc = " * For example, when the row 1 left seat is occupied, `row1LeftOccupied(0)` bit shall be set to `1`."]
    #[doc = " * When a seat is detected to be not occupied, the corresponding seat occupation bit shall be set to `0`."]
    #[doc = " * Otherwise, the value of seat occupation bit shall be set according to the following conditions:"]
    #[doc = " * - If the seat occupation of a seat row is not detectable, the corresponding bit shall be set to `1`."]
    #[doc = " *   When any seat row not detectable bit is set to `1`, all corresponding seat occupation bits of the same row"]
    #[doc = " *   shall be set to `1`."]
    #[doc = " * - If the seat row is not present, the corresponding not present bit of the same row shall be set to `1`."]
    #[doc = " *   When any of the seat row not present bit is set to `1`, the corresponding not detectable bit for that row"]
    #[doc = " *   shall be set to `1`, and all the corresponding seat occupation bits in that row shall be set to `0`."]
    #[doc = " * "]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: V1.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct PositionOfOccupants(pub FixedBitString<20usize>);
    #[doc = "*"]
    #[doc = " * This DF shall contain a list of distances @ref PosPillar that refer to the perpendicular distance between centre of vehicle front bumper"]
    #[doc = " * and vehicle pillar A, between neighbour pillars until the last pillar of the vehicle."]
    #[doc = " *"]
    #[doc = " * Vehicle pillars refer to the vertical or near vertical support of vehicle,"]
    #[doc = " * designated respectively as the A, B, C or D and other pillars moving in side profile view from the front to rear."]
    #[doc = " * "]
    #[doc = " * The first value of the DF refers to the perpendicular distance from the centre of vehicle front bumper to "]
    #[doc = " * vehicle A pillar. The second value refers to the perpendicular distance from the centre position of A pillar"]
    #[doc = " * to the B pillar of vehicle and so on until the last pillar."]
    #[doc = " *"]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: V1.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=3", extensible))]
    pub struct PositionOfPillars(pub SequenceOf<PosPillar>);
    #[doc = "*"]
    #[doc = " * This DE indicates the positioning technology being used to estimate a geographical position."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 `noPositioningSolution`  - no positioning solution used,"]
    #[doc = " * - 1 `sGNSS`                  - Global Navigation Satellite System used,"]
    #[doc = " * - 2 `dGNSS`                  - Differential GNSS used,"]
    #[doc = " * - 3 `sGNSSplusDR`            - GNSS and dead reckoning used,"]
    #[doc = " * - 4 `dGNSSplusDR`            - Differential GNSS and dead reckoning used,"]
    #[doc = " * - 5 `dR`                     - dead reckoning used,"]
    #[doc = " * - 6 `manuallyByOperator`     - position set manually by a human operator."]
    #[doc = " *"]
    #[doc = " * @category: GeoReference information"]
    #[doc = " * @revision: V1.3.1, extension with value 6 added in V2.2.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum PositioningSolutionType {
        noPositioningSolution = 0,
        sGNSS = 1,
        dGNSS = 2,
        sGNSSplusDR = 3,
        dGNSSplusDR = 4,
        dR = 5,
        #[rasn(extension_addition)]
        manuallyByOperator = 6,
    }
    #[doc = "*"]
    #[doc = " * This DE represents the value of the sub cause codes of the @ref CauseCode `postCrash` ."]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 `unavailable`                                               - in case further detailed information on post crash event is unavailable,"]
    #[doc = " * - 1 `accidentWithoutECallTriggered`                             - in case no eCall has been triggered for an accident,"]
    #[doc = " * - 2 `accidentWithECallManuallyTriggered`                        - in case eCall has been manually triggered and transmitted to eCall back end,"]
    #[doc = " * - 3 `accidentWithECallAutomaticallyTriggered`                   - in case eCall has been automatically triggered and transmitted to eCall back end,"]
    #[doc = " * - 4 `accidentWithECallTriggeredWithoutAccessToCellularNetwork`  - in case eCall has been triggered but cellular network is not accessible from triggering vehicle."]
    #[doc = " * - 5-255                                                         - are reserved for future usage."]
    #[doc = " *"]
    #[doc = " * @category: Traffic information"]
    #[doc = " * @revision: V1.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct PostCrashSubCauseCode(pub u8);
    #[doc = "*"]
    #[doc = "* This DE represent the total amount of rain falling during one hour. It is measured in mm per hour at an area of 1 square metre.  "]
    #[doc = "* "]
    #[doc = "* The following values are specified:"]
    #[doc = "* - `n` (`n > 0` and `n < 2000`) if the amount of rain falling is equal to or less than n x 0,1 mm/h and greater than (n-1) x 0,1 mm/h,"]
    #[doc = "* - `2000` if the amount of rain falling is greater than 199.9 mm/h, "]
    #[doc = "* - `2001` if the information is not available."]
    #[doc = "*"]
    #[doc = "* @unit: 0,1 mm/h "]
    #[doc = "* @category: Basic Information"]
    #[doc = "* @revision: created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=2001"))]
    pub struct PrecipitationIntensity(pub u16);
    #[doc = "*"]
    #[doc = " * This DF describes a zone of protection inside which the ITS communication should be restricted."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field protectedZoneType: type of the protected zone. "]
    #[doc = " * "]
    #[doc = " * @field expiryTime: optional time at which the validity of the protected communication zone will expire."]
    #[doc = " * "]
    #[doc = " * @field protectedZoneLatitude: latitude of the centre point of the protected communication zone."]
    #[doc = " * "]
    #[doc = " * @field protectedZoneLongitude: longitude of the centre point of the protected communication zone."]
    #[doc = " * "]
    #[doc = " * @field protectedZoneRadius: optional radius of the protected communication zone in metres."]
    #[doc = " * "]
    #[doc = " * @field protectedZoneId: the optional ID of the protected communication zone."]
    #[doc = " * "]
    #[doc = " * @note: A protected communication zone may be defined around a CEN DSRC road side equipment."]
    #[doc = " * "]
    #[doc = " * @category: Infrastructure information, Communication information"]
    #[doc = " * @revision: revised in V2.1.1 (changed protectedZoneID to protectedZoneId)"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ProtectedCommunicationZone {
        #[rasn(identifier = "protectedZoneType")]
        pub protected_zone_type: ProtectedZoneType,
        #[rasn(identifier = "expiryTime")]
        pub expiry_time: Option<TimestampIts>,
        #[rasn(identifier = "protectedZoneLatitude")]
        pub protected_zone_latitude: Latitude,
        #[rasn(identifier = "protectedZoneLongitude")]
        pub protected_zone_longitude: Longitude,
        #[rasn(identifier = "protectedZoneRadius")]
        pub protected_zone_radius: Option<ProtectedZoneRadius>,
        #[rasn(identifier = "protectedZoneId")]
        pub protected_zone_id: Option<ProtectedZoneId>,
    }
    impl ProtectedCommunicationZone {
        pub fn new(
            protected_zone_type: ProtectedZoneType,
            expiry_time: Option<TimestampIts>,
            protected_zone_latitude: Latitude,
            protected_zone_longitude: Longitude,
            protected_zone_radius: Option<ProtectedZoneRadius>,
            protected_zone_id: Option<ProtectedZoneId>,
        ) -> Self {
            Self {
                protected_zone_type,
                expiry_time,
                protected_zone_latitude,
                protected_zone_longitude,
                protected_zone_radius,
                protected_zone_id,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DF shall contain a list of @ref ProtectedCommunicationZone provided by a road side ITS-S (Road Side Unit RSU)."]
    #[doc = " *"]
    #[doc = " * It may provide up to 16 protected communication zones information."]
    #[doc = " *"]
    #[doc = " * @category: Infrastructure information, Communication information"]
    #[doc = " * @revision: V1.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16"))]
    pub struct ProtectedCommunicationZonesRSU(pub SequenceOf<ProtectedCommunicationZone>);
    #[doc = "*"]
    #[doc = " * This DE represents the indentifier of a protected communication zone."]
    #[doc = " * "]
    #[doc = " * "]
    #[doc = " * @category: Infrastructure information, Communication information"]
    #[doc = " * @revision: Revision in V2.1.1 (changed name from ProtectedZoneID to ProtectedZoneId)"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=134217727"))]
    pub struct ProtectedZoneId(pub u32);
    #[doc = "*"]
    #[doc = " * This DE represents the radius of a protected communication zone. "]
    #[doc = " * "]
    #[doc = " * "]
    #[doc = " * @unit: metre"]
    #[doc = " * @category: Infrastructure information, Communication information"]
    #[doc = " * @revision: V1.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=255", extensible))]
    pub struct ProtectedZoneRadius(pub Integer);
    #[doc = "*"]
    #[doc = " * This DE indicates the type of a protected communication zone, so that an ITS-S is aware of the actions to do"]
    #[doc = " * while passing by such zone (e.g. reduce the transmit power in case of a DSRC tolling station)."]
    #[doc = " * "]
    #[doc = " * The protected zone type is defined in ETSI TS 102 792 [14]."]
    #[doc = " * "]
    #[doc = " * "]
    #[doc = " * @category: Communication information"]
    #[doc = " * @revision: V1.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum ProtectedZoneType {
        permanentCenDsrcTolling = 0,
        #[rasn(extension_addition)]
        temporaryCenDsrcTolling = 1,
    }
    #[doc = "*"]
    #[doc = " * This DF identifies an organization."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field countryCode: represents the country code that identifies the country of the national registration administrator for issuers according to ISO 14816."]
    #[doc = " *"]
    #[doc = " * @field providerIdentifier: identifies the organization according to the national ISO 14816 register for issuers."]
    #[doc = " *"]
    #[doc = " * @note: See https://www.itsstandards.eu/registries/register-of-nra-i-cs1/ for a list of national registration administrators and their respective registers"]
    #[doc = " * "]
    #[doc = " * @category: Communication information"]
    #[doc = " * @revision: Created in V2.2.1 based on ISO 17573-3 [24]"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct Provider {
        #[rasn(identifier = "countryCode")]
        pub country_code: CountryCode,
        #[rasn(identifier = "providerIdentifier")]
        pub provider_identifier: IssuerIdentifier,
    }
    impl Provider {
        pub fn new(country_code: CountryCode, provider_identifier: IssuerIdentifier) -> Self {
            Self {
                country_code,
                provider_identifier,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DF represents activation data for real-time systems designed for operations control, traffic light priorities, track switches, barriers, etc."]
    #[doc = " * using a range of activation devices equipped in public transport vehicles."]
    #[doc = " *"]
    #[doc = " * The activation of the corresponding equipment is triggered by the approach or passage of a public transport"]
    #[doc = " * vehicle at a certain point (e.g. a beacon)."]
    #[doc = " *"]
    #[doc = " * @field ptActivationType: type of activation. "]
    #[doc = " *"]
    #[doc = " * @field ptActicationData: data of activation. "]
    #[doc = " *"]
    #[doc = " * Today there are different payload variants defined for public transport activation-data. The R09.x is one of"]
    #[doc = " * the industry standard used by public transport vehicles (e.g. buses, trams) in Europe (e.g. Germany Austria)"]
    #[doc = " * for controlling traffic lights, barriers, bollards, etc. This DF shall include information like route, course,"]
    #[doc = " * destination, priority, etc."]
    #[doc = " * "]
    #[doc = " * The R09.x content is defined in VDV recommendation 420 [7]. It includes following information:"]
    #[doc = " * - Priority Request Information (pre-request, request, ready to start)"]
    #[doc = " * - End of Prioritization procedure"]
    #[doc = " * - Priority request direction"]
    #[doc = " * - Public Transport line number"]
    #[doc = " * - Priority of public transport"]
    #[doc = " * - Route line identifier of the public transport"]
    #[doc = " * - Route number identification"]
    #[doc = " * - Destination of public transport vehicle"]
    #[doc = " *"]
    #[doc = " * Other countries may use different message sets defined by the local administration."]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: V1.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct PtActivation {
        #[rasn(identifier = "ptActivationType")]
        pub pt_activation_type: PtActivationType,
        #[rasn(identifier = "ptActivationData")]
        pub pt_activation_data: PtActivationData,
    }
    impl PtActivation {
        pub fn new(
            pt_activation_type: PtActivationType,
            pt_activation_data: PtActivationData,
        ) -> Self {
            Self {
                pt_activation_type,
                pt_activation_data,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DE is used for various tasks in the public transportation environment, especially for controlling traffic"]
    #[doc = " * signal systems to prioritize and speed up public transportation in urban area (e.g. intersection \"_bottlenecks_\")."]
    #[doc = " * The traffic lights may be controlled by an approaching bus or tram automatically. This permits \"_In Time_\" activation"]
    #[doc = " * of the green phase, will enable the individual traffic to clear a potential traffic jam in advance. Thereby the"]
    #[doc = " * approaching bus or tram may pass an intersection with activated green light without slowing down the speed due to"]
    #[doc = " * traffic congestion. Other usage of the DE is the provision of information like the public transport line number"]
    #[doc = " * or the schedule delay of a public transport vehicle."]
    #[doc = " * "]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: V1.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=20"))]
    pub struct PtActivationData(pub OctetString);
    #[doc = "*"]
    #[doc = " * This DE indicates a certain coding type of the PtActivationData data."]
    #[doc = " *"]
    #[doc = " * The folowing value are specified:"]
    #[doc = " * - 0 `undefinedCodingType`  : undefined coding type,"]
    #[doc = " * - 1 `r09-16CodingType`     : coding of PtActivationData conform to VDV recommendation 420 [7],"]
    #[doc = " * - 2 `vdv-50149CodingType`  : coding of PtActivationData based on VDV recommendation 420 [7]."]
    #[doc = " * - 3 - 255                  : reserved for alternative and future use."]
    #[doc = " * "]
    #[doc = " * @category: Vehicle information "]
    #[doc = " * @revision: V1.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct PtActivationType(pub u8);
    #[doc = "*"]
    #[doc = " * This DF describes a radial shape. The circular or spherical sector is constructed by sweeping      "]
    #[doc = " * the provided range about the reference position specified outside of the context of this DF or "]
    #[doc = " * about the optional shapeReferencePoint. The range is swept between a horizontal start and a "]
    #[doc = " * horizontal end angle in the X-Y plane of a cartesian coordinate system specified outside of the "]
    #[doc = " * context of this DF, in a right-hand positive angular direction w.r.t. the x-axis. "]
    #[doc = " * A vertical opening angle in the X-Z plane may optionally be provided in a right-hand positive "]
    #[doc = " * angular direction w.r.t. the x-axis. "]
    #[doc = " *"]
    #[doc = " * It shall include the following components:"]
    #[doc = " * "]
    #[doc = " * @field shapeReferencePoint: the optional reference point used for the definition of the shape, "]
    #[doc = " * relative to an externally specified reference position. If this component is absent, the  "]
    #[doc = " * externally specified reference position represents the shape's reference point. "]
    #[doc = " *"]
    #[doc = " * @field range: the radial range of the shape from the shape's reference point. "]
    #[doc = " *"]
    #[doc = " * @field horizontalOpeningAngleStart: the start of the shape's horizontal opening angle. "]
    #[doc = " *"]
    #[doc = " * @field horizontalOpeningAngleEnd: the end of the shape's horizontal opening angle. "]
    #[doc = " *"]
    #[doc = " * @field verticalOpeningAngleStart: optional start of the shape's vertical opening angle. "]
    #[doc = " *"]
    #[doc = " * @field verticalOpeningAngleEnd: optional end of the shape's vertical opening angle. "]
    #[doc = " *"]
    #[doc = " * @category GeoReference information"]
    #[doc = " * @revision: created in V2.1.1, names and types of the horizontal opening angles changed, constraint added and description revised in V2.2.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct RadialShape {
        #[rasn(identifier = "shapeReferencePoint")]
        pub shape_reference_point: Option<CartesianPosition3d>,
        pub range: StandardLength12b,
        #[rasn(identifier = "horizontalOpeningAngleStart")]
        pub horizontal_opening_angle_start: CartesianAngleValue,
        #[rasn(identifier = "horizontalOpeningAngleEnd")]
        pub horizontal_opening_angle_end: CartesianAngleValue,
        #[rasn(identifier = "verticalOpeningAngleStart")]
        pub vertical_opening_angle_start: Option<CartesianAngleValue>,
        #[rasn(identifier = "verticalOpeningAngleEnd")]
        pub vertical_opening_angle_end: Option<CartesianAngleValue>,
    }
    impl RadialShape {
        pub fn new(
            shape_reference_point: Option<CartesianPosition3d>,
            range: StandardLength12b,
            horizontal_opening_angle_start: CartesianAngleValue,
            horizontal_opening_angle_end: CartesianAngleValue,
            vertical_opening_angle_start: Option<CartesianAngleValue>,
            vertical_opening_angle_end: Option<CartesianAngleValue>,
        ) -> Self {
            Self {
                shape_reference_point,
                range,
                horizontal_opening_angle_start,
                horizontal_opening_angle_end,
                vertical_opening_angle_start,
                vertical_opening_angle_end,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DF describes radial shape details. The circular sector or cone is"]
    #[doc = " * constructed by sweeping the provided range about the position specified outside of the  "]
    #[doc = " * context of this DF. The range is swept between a horizontal start and a horizontal end angle in "]
    #[doc = " * the X-Y plane of a right-hand cartesian coordinate system specified outside of the context of "]
    #[doc = " * this DF, in positive angular direction w.r.t. the x-axis. A vertical opening angle in the X-Z "]
    #[doc = " * plane may optionally be provided in positive angular direction w.r.t. the x-axis."]
    #[doc = " * "]
    #[doc = " * It shall include the following components:"]
    #[doc = " * "]
    #[doc = " * @field range: the radial range of the sensor from the reference point or sensor point offset. "]
    #[doc = " *"]
    #[doc = " * @field horizontalOpeningAngleStart:  the start of the shape's horizontal opening angle."]
    #[doc = " *"]
    #[doc = " * @field horizontalOpeningAngleEnd: the end of the shape's horizontal opening angle. "]
    #[doc = " *"]
    #[doc = " * @field verticalOpeningAngleStart: optional start of the shape's vertical opening angle. "]
    #[doc = " *"]
    #[doc = " * @field verticalOpeningAngleEnd: optional end of the shape's vertical opening angle. "]
    #[doc = " *"]
    #[doc = " * @category: Georeference information"]
    #[doc = " * @revision: created in V2.1.1, description revised and constraint added in V2.2.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct RadialShapeDetails {
        pub range: StandardLength12b,
        #[rasn(identifier = "horizontalOpeningAngleStart")]
        pub horizontal_opening_angle_start: CartesianAngleValue,
        #[rasn(identifier = "horizontalOpeningAngleEnd")]
        pub horizontal_opening_angle_end: CartesianAngleValue,
        #[rasn(identifier = "verticalOpeningAngleStart")]
        pub vertical_opening_angle_start: Option<CartesianAngleValue>,
        #[rasn(identifier = "verticalOpeningAngleEnd")]
        pub vertical_opening_angle_end: Option<CartesianAngleValue>,
    }
    impl RadialShapeDetails {
        pub fn new(
            range: StandardLength12b,
            horizontal_opening_angle_start: CartesianAngleValue,
            horizontal_opening_angle_end: CartesianAngleValue,
            vertical_opening_angle_start: Option<CartesianAngleValue>,
            vertical_opening_angle_end: Option<CartesianAngleValue>,
        ) -> Self {
            Self {
                range,
                horizontal_opening_angle_start,
                horizontal_opening_angle_end,
                vertical_opening_angle_start,
                vertical_opening_angle_end,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DF describes a list of radial shapes positioned w.r.t. to an offset position defined  "]
    #[doc = " * relative to a reference position specified outside of the context of this DF and oriented w.r.t.  "]
    #[doc = " * a cartesian coordinate system specified outside of the context of this DF. "]
    #[doc = " *"]
    #[doc = " * It shall include the following components:"]
    #[doc = " *"]
    #[doc = " * @field refPointId: the identification of the reference point in case of a sensor mounted to trailer. Defaults to ITS ReferencePoint (0)."]
    #[doc = " * "]
    #[doc = " * @field xCoordinate: the x-coordinate of the offset position."]
    #[doc = " *"]
    #[doc = " * @field yCoordinate: the y-coordinate of the offset position."]
    #[doc = " *"]
    #[doc = " * @field zCoordinate: the optional z-coordinate of the offset position."]
    #[doc = " *"]
    #[doc = " * @field radialShapesList: the list of radial shape details."]
    #[doc = " *"]
    #[doc = " * @category: Georeference information"]
    #[doc = " * @revision: created in V2.1.1, description revised in V2.2.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct RadialShapes {
        #[rasn(identifier = "refPointId")]
        pub ref_point_id: Identifier1B,
        #[rasn(identifier = "xCoordinate")]
        pub x_coordinate: CartesianCoordinateSmall,
        #[rasn(identifier = "yCoordinate")]
        pub y_coordinate: CartesianCoordinateSmall,
        #[rasn(identifier = "zCoordinate")]
        pub z_coordinate: Option<CartesianCoordinateSmall>,
        #[rasn(identifier = "radialShapesList")]
        pub radial_shapes_list: RadialShapesList,
    }
    impl RadialShapes {
        pub fn new(
            ref_point_id: Identifier1B,
            x_coordinate: CartesianCoordinateSmall,
            y_coordinate: CartesianCoordinateSmall,
            z_coordinate: Option<CartesianCoordinateSmall>,
            radial_shapes_list: RadialShapesList,
        ) -> Self {
            Self {
                ref_point_id,
                x_coordinate,
                y_coordinate,
                z_coordinate,
                radial_shapes_list,
            }
        }
    }
    #[doc = "* "]
    #[doc = " * The DF contains a list of @ref RadialShapeDetails."]
    #[doc = " *"]
    #[doc = " * @category: Georeference information"]
    #[doc = " * @revision: created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16", extensible))]
    pub struct RadialShapesList(pub SequenceOf<RadialShapeDetails>);
    #[doc = "*"]
    #[doc = " * This DE represents the value of the sub cause codes of the @ref CauseCode `railwayLevelCrossing` ."]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 `unavailable`                   - in case no further detailed information on the railway level crossing status is available,"]
    #[doc = " * - 1 `doNotCrossAbnormalSituation`   - in case when something wrong is detected by equation or sensors of the railway level crossing, "]
    #[doc = "                                         including level crossing is closed for too long (e.g. more than 10 minutes long ; default value),"]
    #[doc = " * - 2 `closed`                        - in case the crossing is closed or closing (barriers down),"]
    #[doc = " * - 3 `unguarded`                     - in case the level crossing is unguarded (i.e a Saint Andrew cross level crossing without detection of train),"]
    #[doc = " * - 4 `nominal`                       - in case the barriers are up and/or the warning systems are off,"]
    #[doc = " * - 5 `trainApproaching`              - in case a train is approaching and the railway level crossing is without barriers."]
    #[doc = " * - 6-255: reserved for future usage."]
    #[doc = " *"]
    #[doc = " * @category: Traffic information"]
    #[doc = " * @revision: V1.3.1, description of value 2 and 4 changed and value 5 added in V2.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct RailwayLevelCrossingSubCauseCode(pub u8);
    #[doc = "* "]
    #[doc = " * This DF represents the shape of a rectangular area or a right rectangular prism that is centred "]
    #[doc = " * on a reference position defined outside of the context of this DF and oriented w.r.t. a cartesian    "]
    #[doc = " * coordinate system defined outside of the context of this DF. "]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field shapeReferencePoint: represents an optional offset point which the rectangle is centred on with "]
    #[doc = " * respect to the reference position. If this component is absent, the externally specified  "]
    #[doc = " * reference position represents the shape's reference point. "]
    #[doc = " *"]
    #[doc = " * @field semiLength: represents half the length of the rectangle located in the X-Y Plane."]
    #[doc = " * "]
    #[doc = " * @field semiBreadth: represents half the breadth of the rectangle located in the X-Y Plane."]
    #[doc = " *"]
    #[doc = " * @field orientation: represents the optional orientation of the longer side of the rectangle, "]
    #[doc = " * measured with positive values turning around the Z-axis using the right-hand rule, starting from"]
    #[doc = " * the X-axis. If absent, the orientation is equal to the value zero."]
    #[doc = " *"]
    #[doc = " * @field height: represents the optional height, present if the shape is a right rectangular prism "]
    #[doc = " * with height extending in the positive Z-axis."]
    #[doc = " *"]
    #[doc = " * @category GeoReference information"]
    #[doc = " * @revision: created in V2.1.1, centerPoint renamed to shapeReferencePoint, the type of the field orientation changed "]
    #[doc = " *            and description revised in V2.2.1, added sentence on absence in V2.4.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct RectangularShape {
        #[rasn(identifier = "shapeReferencePoint")]
        pub shape_reference_point: Option<CartesianPosition3d>,
        #[rasn(identifier = "semiLength")]
        pub semi_length: StandardLength12b,
        #[rasn(identifier = "semiBreadth")]
        pub semi_breadth: StandardLength12b,
        pub orientation: Option<CartesianAngleValue>,
        pub height: Option<StandardLength12b>,
    }
    impl RectangularShape {
        pub fn new(
            shape_reference_point: Option<CartesianPosition3d>,
            semi_length: StandardLength12b,
            semi_breadth: StandardLength12b,
            orientation: Option<CartesianAngleValue>,
            height: Option<StandardLength12b>,
        ) -> Self {
            Self {
                shape_reference_point,
                semi_length,
                semi_breadth,
                orientation,
                height,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * A position within a geographic coordinate system together with a confidence ellipse. "]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field latitude: the latitude of the geographical point."]
    #[doc = " *"]
    #[doc = " * @field longitude: the longitude of the geographical point."]
    #[doc = " *"]
    #[doc = " * @field positionConfidenceEllipse: the confidence ellipse associated to the geographical position."]
    #[doc = " *"]
    #[doc = " * @field altitude: the altitude and an altitude accuracy of the geographical point."]
    #[doc = " *"]
    #[doc = " * @note: this DE is kept for backwards compatibility reasons only. It is recommended to use the @ref ReferencePositionWithConfidence instead. "]
    #[doc = " * @category: GeoReference information"]
    #[doc = " * @revision: description updated in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct ReferencePosition {
        pub latitude: Latitude,
        pub longitude: Longitude,
        #[rasn(identifier = "positionConfidenceEllipse")]
        pub position_confidence_ellipse: PosConfidenceEllipse,
        pub altitude: Altitude,
    }
    impl ReferencePosition {
        pub fn new(
            latitude: Latitude,
            longitude: Longitude,
            position_confidence_ellipse: PosConfidenceEllipse,
            altitude: Altitude,
        ) -> Self {
            Self {
                latitude,
                longitude,
                position_confidence_ellipse,
                altitude,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * A position within a geographic coordinate system together with a confidence ellipse. "]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field latitude: the latitude of the geographical point."]
    #[doc = " *"]
    #[doc = " * @field longitude: the longitude of the geographical point."]
    #[doc = " *"]
    #[doc = " * @field positionConfidenceEllipse: the confidence ellipse associated to the geographical position."]
    #[doc = " *"]
    #[doc = " * @field altitude: the altitude and an altitude accuracy of the geographical point."]
    #[doc = " *"]
    #[doc = " * @category: GeoReference information"]
    #[doc = " * @revision: created in V2.1.1 based on @ref ReferencePosition but using @ref PositionConfidenceEllipse."]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct ReferencePositionWithConfidence {
        pub latitude: Latitude,
        pub longitude: Longitude,
        #[rasn(identifier = "positionConfidenceEllipse")]
        pub position_confidence_ellipse: PositionConfidenceEllipse,
        pub altitude: Altitude,
    }
    impl ReferencePositionWithConfidence {
        pub fn new(
            latitude: Latitude,
            longitude: Longitude,
            position_confidence_ellipse: PositionConfidenceEllipse,
            altitude: Altitude,
        ) -> Self {
            Self {
                latitude,
                longitude,
                position_confidence_ellipse,
                altitude,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DE describes a distance of relevance for information indicated in a message."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 `lessThan50m`   - for distances below 50 m,"]
    #[doc = " * - 1 `lessThan100m`  - for distances below 100 m, "]
    #[doc = " * - 2 `lessThan200m`  - for distances below 200 m, "]
    #[doc = " * - 3 `lessThan500m`  - for distances below 300 m, "]
    #[doc = " * - 4 `lessThan1000m` - for distances below 1 000 m, "]
    #[doc = " * - 5 `lessThan5km`   - for distances below 5 000 m, "]
    #[doc = " * - 6 `lessThan10km`  - for distances below 10 000 m, "]
    #[doc = " * - 7 `over10km`      - for distances over 10 000 m. "]
    #[doc = " * "]
    #[doc = " * @note: this DE is kept for backwards compatibility reasons only. It is recommended to use the @ref StandardLength3b instead. "]
    #[doc = " *"]
    #[doc = " * @category: GeoReference information"]
    #[doc = " * @revision: Editorial update in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum RelevanceDistance {
        lessThan50m = 0,
        lessThan100m = 1,
        lessThan200m = 2,
        lessThan500m = 3,
        lessThan1000m = 4,
        lessThan5km = 5,
        lessThan10km = 6,
        over10km = 7,
    }
    #[doc = "*"]
    #[doc = " * This DE indicates a traffic direction that is relevant to information indicated in a message."]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 `allTrafficDirections` - for all traffic directions, "]
    #[doc = " * - 1 `upstreamTraffic`      - for upstream traffic, "]
    #[doc = " * - 2 `downstreamTraffic`    - for downstream traffic, "]
    #[doc = " * - 3 `oppositeTraffic`      - for traffic in the opposite direction. "]
    #[doc = " *"]
    #[doc = " * The terms `upstream`, `downstream` and `oppositeTraffic` are relative to the event position."]
    #[doc = " *"]
    #[doc = " * @note: Upstream traffic corresponds to the incoming traffic towards the event position,"]
    #[doc = " * and downstream traffic to the departing traffic away from the event position."]
    #[doc = " *"]
    #[doc = " * @note: this DE is kept for backwards compatibility reasons only. It is recommended to use the @ref TrafficDirection instead. "]
    #[doc = " *"]
    #[doc = " * @category: GeoReference information"]
    #[doc = " * @revision: Editorial update in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum RelevanceTrafficDirection {
        allTrafficDirections = 0,
        upstreamTraffic = 1,
        downstreamTraffic = 2,
        oppositeTraffic = 3,
    }
    #[doc = "*"]
    #[doc = " * This DE indicates whether an ITS message is transmitted as request from ITS-S or a response transmitted from"]
    #[doc = " * ITS-S after receiving request from other ITS-Ss."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 `request`  - for a request message, "]
    #[doc = " * - 1 `response` - for a response message.  "]
    #[doc = " *"]
    #[doc = " * @category Communication information"]
    #[doc = " * @revision: Editorial update in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum RequestResponseIndication {
        request = 0,
        response = 1,
    }
    #[doc = "*"]
    #[doc = " * This DE represents the value of the sub cause codes of the @ref CauseCode `RescueRecoveryAndMaintenanceWorkInProgressSubCauseCode` "]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 `unavailable`                         - in case further detailed information on rescue and recovery work is unavailable,"]
    #[doc = " * - 1 `emergencyVehicles`                   - in case rescue and/or safeguarding work is ongoing by emergency vehicles, i.e. by vehicles that have the absolute right of way,"]
    #[doc = " * - 2 `rescueHelicopterLanding`             - in case rescue helicopter is landing,"]
    #[doc = " * - 3 `policeActivityOngoing`               - in case police activity is ongoing (only to be used if a more specific sub cause than (1) is needed),"]
    #[doc = " * - 4 `medicalEmergencyOngoing`             - in case medical emergency recovery is ongoing (only to be used if a more specific sub cause than (1) is needed),"]
    #[doc = " * - 5 `childAbductionInProgress-deprecated` - deprecated,"]
    #[doc = " * - 6 `prioritizedVehicle`                  - in case rescue and/or safeguarding work is ongoing by prioritized vehicles, i.e. by vehicles that have priority but not the absolute right of way,"]
    #[doc = " * - 7 `rescueAndRecoveryVehicle`            - in case technical rescue work is ongoing by rescue and recovery vehicles."]
    #[doc = " * - 8-255: reserved for future usage."]
    #[doc = ""]
    #[doc = " *"]
    #[doc = " * @category: Traffic information"]
    #[doc = " * @revision: V1.3.1, named values 6 and 7 added in V2.2.1, DE renamed and value 5 deprecated in V2.4.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct RescueRecoveryAndMaintenanceWorkInProgressSubCauseCode(pub u8);
    #[doc = "*"]
    #[doc = " * This DF shall contain a list of @ref StationType. to which a certain traffic restriction, e.g. the speed limit, applies."]
    #[doc = " * "]
    #[doc = " * @category: Infrastructure information, Traffic information"]
    #[doc = " * @revision: V1.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=3", extensible))]
    pub struct RestrictedTypes(pub SequenceOf<StationType>);
    #[doc = "* "]
    #[doc = " * This DF provides configuration information about a road section."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field roadSectionDefinition: the topological definition of the road section for which the information in the other components applies throughout its entire length."]
    #[doc = " * "]
    #[doc = " * @field roadType: the optional type of road on which the section is located."]
    #[doc = " * "]
    #[doc = " * @field laneConfiguration: the optional configuration of the road section in terms of basic information per lane."]
    #[doc = " *"]
    #[doc = " * @field mapemConfiguration: the optional configuration of the road section in terms of MAPEM lanes or connections."]
    #[doc = " *"]
    #[doc = " * @category: Road topology information"]
    #[doc = " * @revision: Created in V2.2.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct RoadConfigurationSection {
        #[rasn(identifier = "roadSectionDefinition")]
        pub road_section_definition: RoadSectionDefinition,
        #[rasn(identifier = "roadType")]
        pub road_type: Option<RoadType>,
        #[rasn(identifier = "laneConfiguration")]
        pub lane_configuration: Option<BasicLaneConfiguration>,
        #[rasn(identifier = "mapemConfiguration")]
        pub mapem_configuration: Option<MapemConfiguration>,
    }
    impl RoadConfigurationSection {
        pub fn new(
            road_section_definition: RoadSectionDefinition,
            road_type: Option<RoadType>,
            lane_configuration: Option<BasicLaneConfiguration>,
            mapem_configuration: Option<MapemConfiguration>,
        ) -> Self {
            Self {
                road_section_definition,
                road_type,
                lane_configuration,
                mapem_configuration,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DF shall contain a list of @ref RoadConfigurationSection."]
    #[doc = " * "]
    #[doc = " * @category: Road Topology information"]
    #[doc = " * @revision: Created in V2.2.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8", extensible))]
    pub struct RoadConfigurationSectionList(pub SequenceOf<RoadConfigurationSection>);
    #[doc = "* "]
    #[doc = " * This DF provides the basic topological definition of a road section."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field startingPointSection: the position of the starting point of the section. "]
    #[doc = " * "]
    #[doc = " * @field lengthOfSection: the optional length of the section along the road profile (i.e. including curves)."]
    #[doc = " * "]
    #[doc = " * @field endingPointSection: the optional position of the ending point of the section. "]
    #[doc = " * If this component is absent, the ending position is implicitly defined by other means, e.g. the starting point of the next RoadConfigurationSection, or the sectionÃ¯Â¿Â½s length."]
    #[doc = " *"]
    #[doc = " * @field connectedPaths: the identifier(s) of the path(s) having one or an ordered subset of waypoints located upstream of the RoadConfigurationSectionÃ¯Â¿Â½ starting point. "]
    #[doc = " * "]
    #[doc = " * @field includedPaths: the identifier(s) of the path(s) that covers (either with all its length or with a part of it) a RoadConfigurationSection. "]
    #[doc = " *"]
    #[doc = " * @field isEventZoneIncluded: indicates, if set to TRUE, that the @ref EventZone incl. its reference position covers a RoadConfigurationSection (either with all its length or with a part of it). "]
    #[doc = " * "]
    #[doc = " * @field isEventZoneConnected: indicates, if set to TRUE, that the @ref EventZone incl. its reference position has one or an ordered subset of waypoints located upstream of the RoadConfigurationSectionÃ¯Â¿Â½ starting point."]
    #[doc = " *"]
    #[doc = " * @category: Road topology information"]
    #[doc = " * @revision: Created in V2.2.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct RoadSectionDefinition {
        #[rasn(identifier = "startingPointSection")]
        pub starting_point_section: GeoPosition,
        #[rasn(identifier = "lengthOfSection")]
        pub length_of_section: Option<StandardLength2B>,
        #[rasn(identifier = "endingPointSection")]
        pub ending_point_section: Option<GeoPosition>,
        #[rasn(identifier = "connectedPaths")]
        pub connected_paths: PathReferences,
        #[rasn(identifier = "includedPaths")]
        pub included_paths: PathReferences,
        #[rasn(identifier = "isEventZoneIncluded")]
        pub is_event_zone_included: bool,
        #[rasn(identifier = "isEventZoneConnected")]
        pub is_event_zone_connected: bool,
    }
    impl RoadSectionDefinition {
        pub fn new(
            starting_point_section: GeoPosition,
            length_of_section: Option<StandardLength2B>,
            ending_point_section: Option<GeoPosition>,
            connected_paths: PathReferences,
            included_paths: PathReferences,
            is_event_zone_included: bool,
            is_event_zone_connected: bool,
        ) -> Self {
            Self {
                starting_point_section,
                length_of_section,
                ending_point_section,
                connected_paths,
                included_paths,
                is_event_zone_included,
                is_event_zone_connected,
            }
        }
    }
    #[doc = "* "]
    #[doc = " * This DE indicates an ordinal number that represents the position of a component in the list @ref RoadConfigurationSectionList. "]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `0`     - if no road section is identified"]
    #[doc = " * - `1..8`  - for instances 1..8 of @ref RoadConfigurationSectionList "]
    #[doc = " *"]
    #[doc = " * @category: Road topology information"]
    #[doc = " * @revision: Created in V2.2.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=8", extensible))]
    pub struct RoadSectionId(pub Integer);
    #[doc = "*"]
    #[doc = " * This DF represents a unique id for a road segment"]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field region: the optional identifier of the entity that is responsible for the region in which the road segment is placed."]
    #[doc = " * It is the duty of that entity to guarantee that the @ref Id is unique within the region."]
    #[doc = " *"]
    #[doc = " * @field id: the identifier of the road segment."]
    #[doc = " *"]
    #[doc = " * @note: when the component region is present, the RoadSegmentReferenceId is guaranteed to be globally unique."]
    #[doc = " * @category: GeoReference information"]
    #[doc = " * @revision: created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct RoadSegmentReferenceId {
        pub region: Option<Identifier2B>,
        pub id: Identifier2B,
    }
    impl RoadSegmentReferenceId {
        pub fn new(region: Option<Identifier2B>, id: Identifier2B) -> Self {
            Self { region, id }
        }
    }
    #[doc = "*"]
    #[doc = " * This DE indicates the type of a road segment."]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 `urban-NoStructuralSeparationToOppositeLanes`       - for an urban road with no structural separation between lanes carrying traffic in opposite directions,"]
    #[doc = " * - 1 `urban-WithStructuralSeparationToOppositeLanes`     - for an urban road with structural separation between lanes carrying traffic in opposite directions,"]
    #[doc = " * - 2 `nonUrban-NoStructuralSeparationToOppositeLanes`    - for an non urban road with no structural separation between lanes carrying traffic in opposite directions,"]
    #[doc = " * - 3 `nonUrban-WithStructuralSeparationToOppositeLanes`  - for an non urban road with structural separation between lanes carrying traffic in opposite directions."]
    #[doc = " *"]
    #[doc = " * @category: Road Topology Information"]
    #[doc = " * @revision: Editorial update in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum RoadType {
        #[rasn(identifier = "urban-NoStructuralSeparationToOppositeLanes")]
        urban_NoStructuralSeparationToOppositeLanes = 0,
        #[rasn(identifier = "urban-WithStructuralSeparationToOppositeLanes")]
        urban_WithStructuralSeparationToOppositeLanes = 1,
        #[rasn(identifier = "nonUrban-NoStructuralSeparationToOppositeLanes")]
        nonUrban_NoStructuralSeparationToOppositeLanes = 2,
        #[rasn(identifier = "nonUrban-WithStructuralSeparationToOppositeLanes")]
        nonUrban_WithStructuralSeparationToOppositeLanes = 3,
    }
    #[doc = "*"]
    #[doc = " * This DE represents the value of the sub cause codes of the @ref CauseCode `roadworks`."]
    #[doc = " * "]
    #[doc = "The value shall be set to:"]
    #[doc = " * - 0 `unavailable`                 - in case further detailed information on roadworks is unavailable,"]
    #[doc = " * - 1 `roadOrCarriagewayClosure`    - in case roadworks are ongoing which comprise the closure of the entire road or of one entire carriageway,"]
    #[doc = " * - 2 `roadMarkingWork-deprecated`  - deprecated since covered by 3 or 4,"]
    #[doc = " * - 3 `movingLaneClosure`           - in case moving roadworks are ongoing, which comprise the closure of a lane,"]
    #[doc = " * - 4 `stationaryLaneClosure`       - in case stationary roadworks are ongoing, which comprise the closure of one or multiple lanes but not of the carriageway/road,"]
    #[doc = " * - 5 `streetCleaning-deprecated`   - deprecated since not pertinent to roadworks and already covered by SlowVehicleSubCauseCode,"]
    #[doc = " * - 6 `winterService-deprecated`    - deprecated since not pertinent to roadworks and already covered by SlowVehicleSubCauseCode,"]
    #[doc = " * - 7 `setupPhase`                  - in case the work zone is being setup which, may comprise the closure of one or multiple lanes but the carriageway/road is not closed, "]
    #[doc = " * - 8 `remodellingPhase`            - in case the work zone is being changed, which may comprise the closure of one or multiple lanes but the carriageway/road is not closed, "]
    #[doc = " * - 9 `dismantlingPhase`            - in case the work zone is being dismantled after finished works, which comprised the closure of one or multiple lanes "]
    #[doc = "                                       but the carriageway/road was not closed,"]
    #[doc = " * - 10 `carriagewayCrossover`       - in case the work zone includes lanes that are re-directed to another carriageway. "]
    #[doc = " * - 11-255                          - are reserved for future usage."]
    #[doc = " *"]
    #[doc = " * @category: Traffic information"]
    #[doc = " * @revision: V1.3.1, values 7-9 added in V2.3.1, values 1, 3, 4 renamed in V2.4.1, value 2, 5 und 6 deprecated in V2.4.1, value 10 added in V2.4.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct RoadworksSubCauseCode(pub u8);
    #[doc = "*"]
    #[doc = " * This DE indicates the driving automation level as defined in SAE J3016 [26]."]
    #[doc = " *"]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: created in V2.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=5"))]
    pub struct SaeAutomationLevel(pub u8);
    #[doc = "*"]
    #[doc = " * This DF provides the safe distance indication of a traffic participant with other traffic participant(s)."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field subjectStation: optionally indicates one \"other\" traffic participant identified by its ITS-S."]
    #[doc = " *  "]
    #[doc = " * @field safeDistanceIndicator: indicates whether the distance between the ego ITS-S and the traffic participant(s) is safe."]
    #[doc = " * If subjectStation is present then it indicates whether the distance between the ego ITS-S and the traffic participant indicated in the component subjectStation is safe. "]
    #[doc = " *"]
    #[doc = " * @field timeToCollision: optionally indicated the time-to-collision calculated as sqrt(LaDi^2 + LoDi^2 + VDi^2/relative speed "]
    #[doc = " * and represented in  the  nearest 100  ms. This component may be present only if subjectStation is present. "]
    #[doc = " *"]
    #[doc = " * @note: the abbreviations used are Lateral Distance (LaD),  Longitudinal Distance (LoD) and Vertical Distance (VD) "]
    #[doc = " * and their respective  thresholds, Minimum  Safe  Lateral  Distance (MSLaD), Minimum  Safe  Longitudinal Distance  (MSLoD),  and  Minimum  Safe Vertical Distance (MSVD)."]
    #[doc = " *"]
    #[doc = " * @category: Traffic information, Kinematic information"]
    #[doc = " * @revision: created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SafeDistanceIndication {
        #[rasn(identifier = "subjectStation")]
        pub subject_station: Option<StationId>,
        #[rasn(identifier = "safeDistanceIndicator")]
        pub safe_distance_indicator: SafeDistanceIndicator,
        #[rasn(identifier = "timeToCollision")]
        pub time_to_collision: Option<DeltaTimeTenthOfSecond>,
    }
    impl SafeDistanceIndication {
        pub fn new(
            subject_station: Option<StationId>,
            safe_distance_indicator: SafeDistanceIndicator,
            time_to_collision: Option<DeltaTimeTenthOfSecond>,
        ) -> Self {
            Self {
                subject_station,
                safe_distance_indicator,
                time_to_collision,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DE indicates if a distance is safe. "]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * -  `FALSE`  if  the triple  {LaD,  LoD, VD} < {MSLaD, MSLoD, MSVD} is simultaneously  satisfied with confidence level of  90 % or  more, "]
    #[doc = " * -  `TRUE` otherwise. "]
    #[doc = " *"]
    #[doc = " * @note: the abbreviations used are Lateral Distance (LaD),  Longitudinal Distance (LoD) and Vertical Distance (VD) "]
    #[doc = " * and their respective  thresholds, Minimum  Safe  Lateral  Distance (MSLaD), Minimum  Safe  Longitudinal Distance  (MSLoD),  and  Minimum  Safe Vertical Distance (MSVD)."]
    #[doc = " *"]
    #[doc = " * @category: Traffic information, Kinematic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(delegate)]
    pub struct SafeDistanceIndicator(pub bool);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=4095"))]
    pub struct SemiAxisLength(pub u16);
    #[doc = "* "]
    #[doc = " * This DE indicates the type of sensor."]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0  `undefined`         - in case the sensor type is undefined, "]
    #[doc = " * - 1  `radar`             - in case the sensor is a radar,"]
    #[doc = " * - 2  `lidar`             - in case the sensor is a lidar,"]
    #[doc = " * - 3  `monovideo`         - in case the sensor is mono video,"]
    #[doc = " * - 4  `stereovision`      - in case the sensor is stereo vision,"]
    #[doc = " * - 5  `nightvision`       - in case the sensor supports night vision, using e.g. infrared illumination or thermal imaging,"]
    #[doc = " * - 6  `ultrasonic`        - in case the sensor is ultrasonic,"]
    #[doc = " * - 7  `pmd`               - in case the sensor is photonic mixing device,"]
    #[doc = " * - 8  `inductionLoop`     - in case the sensor is an induction loop,"]
    #[doc = " * - 9  `sphericalCamera`   - in case the sensor is a spherical camera,"]
    #[doc = " * - 10 `uwb`               - in case the sensor is ultra wide band,"]
    #[doc = " * - 11 `acoustic`          - in case the sensor is acoustic,"]
    #[doc = " * - 12 `localAggregation`  - in case the information is provided by a system that aggregates information from different local sensors. Aggregation may include fusion,"]
    #[doc = " * - 13 `itsAggregation`    - in case the information is provided by a system that aggregates information from other received ITS messages,"]
    #[doc = " * - 14 `rfid`              - in case the sensor is radio frequency identification using a passive or active (e.g. Bluetooth, W-LAN) technology. "]
    #[doc = " * - 15-31                  - are reserved for future usage."]
    #[doc = " *"]
    #[doc = " * @category: Sensing Information"]
    #[doc = " * @revision: created in V2.1.1, description of value 5 changed and value 14 added in V2.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=31"))]
    pub struct SensorType(pub u8);
    #[doc = "* "]
    #[doc = " * This DE indicates the type of sensor(s)."]
    #[doc = " * The corresponding bit shall be set to 1 under the following conditions:"]
    #[doc = " * "]
    #[doc = " * - 0  `undefined`         - in case the sensor type is undefined. "]
    #[doc = " * - 1  `radar`             - in case the sensor is a radar,"]
    #[doc = " * - 2  `lidar`             - in case the sensor is a lidar,"]
    #[doc = " * - 3  `monovideo`         - in case the sensor is mono video,"]
    #[doc = " * - 4  `stereovision`      - in case the sensor is stereo vision,"]
    #[doc = " * - 5  `nightvision`       - in case the sensor supports night vision, using e.g. infrared illumination or thermal imaging,"]
    #[doc = " * - 6  `ultrasonic`        - in case the sensor is ultrasonic,"]
    #[doc = " * - 7  `pmd`               - in case the sensor is photonic mixing device,"]
    #[doc = " * - 8  `inductionLoop`     - in case the sensor is an induction loop,"]
    #[doc = " * - 9  `sphericalCamera`   - in case the sensor is a spherical camera,"]
    #[doc = " * - 10 `uwb`               - in case the sensor is ultra wide band,"]
    #[doc = " * - 11 `acoustic`          - in case the sensor is acoustic,"]
    #[doc = " * - 12 `localAggregation`  - in case the information is provided by a system that aggregates information from different local sensors. Aggregation may include fusion,"]
    #[doc = " * - 13 `itsAggregation`    - in case the information is provided by a system that aggregates information from other received ITS messages,"]
    #[doc = " * - 14 `rfid`              - in case the sensor is radio frequency identification using a passive or active (e.g. Bluetooth, W-LAN) technology. "]
    #[doc = " * - 15                     - reserved for future usage."]
    #[doc = " * "]
    #[doc = " * @note: If all bits are set to 0, then no sensor type is used"]
    #[doc = " *"]
    #[doc = " * @category: Sensing Information"]
    #[doc = " * @revision: created in V2.2.1, description of value 5 changed and value 14 added in V2.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("16", extensible))]
    pub struct SensorTypes(pub BitString);
    #[doc = "*"]
    #[doc = " * This DE represents a sequence number."]
    #[doc = " * "]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: V1.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=65535"))]
    pub struct SequenceNumber(pub u16);
    #[doc = "* "]
    #[doc = " * This DF shall contain a list of DF @ref CartesianPosition3d."]
    #[doc = " * "]
    #[doc = " * @category: GeoReference information"]
    #[doc = " * @revision: created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16", extensible))]
    pub struct SequenceOfCartesianPosition3d(pub SequenceOf<CartesianPosition3d>);
    #[doc = "* "]
    #[doc = " * The DF contains a list of DE @ref Identifier1B."]
    #[doc = " *"]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=128", extensible))]
    pub struct SequenceOfIdentifier1B(pub SequenceOf<Identifier1B>);
    #[doc = "*"]
    #[doc = " * The DF contains a list of DF @ref SafeDistanceIndication."]
    #[doc = " *"]
    #[doc = " * @category: Traffic information, Kinematic information"]
    #[doc = " * @revision: created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8", extensible))]
    pub struct SequenceOfSafeDistanceIndication(pub SequenceOf<SafeDistanceIndication>);
    #[doc = "*"]
    #[doc = " * The DF shall contain a list of DF @ref TrajectoryInterceptionIndication."]
    #[doc = " *"]
    #[doc = " * @category: Traffic information, Kinematic information"]
    #[doc = " * @revision: created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8", extensible))]
    pub struct SequenceOfTrajectoryInterceptionIndication(
        pub SequenceOf<TrajectoryInterceptionIndication>,
    );
    #[doc = "*"]
    #[doc = " * This DF provides the definition of a geographical area or volume, based on different options."]
    #[doc = " *"]
    #[doc = " * It is a choice of the following components: "]
    #[doc = " *"]
    #[doc = " * @field rectangular: definition of an rectangular area or a right rectangular prism (with a rectangular base) also called a cuboid, or informally a rectangular box."]
    #[doc = " *"]
    #[doc = " * @field circular: definition of an area of circular shape or a right circular cylinder."]
    #[doc = " *"]
    #[doc = " * @field polygonal: definition of an area of polygonal shape or a right prism."]
    #[doc = " *"]
    #[doc = " * @field elliptical: definition of an area of elliptical shape or a right elliptical cylinder."]
    #[doc = " *"]
    #[doc = " * @field radial: definition of a radial shape."]
    #[doc = " *"]
    #[doc = " * @field radialList: definition of list of radial shapes."]
    #[doc = " *"]
    #[doc = " * @category: GeoReference information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum Shape {
        rectangular(RectangularShape),
        circular(CircularShape),
        polygonal(PolygonalShape),
        elliptical(EllipticalShape),
        radial(RadialShape),
        radialShapes(RadialShapes),
    }
    #[doc = "*"]
    #[doc = " * This DE represents the value of the sub cause codes of the @ref CauseCode `signalViolation`."]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 `unavailable`               - in case further detailed information on signal violation event is unavailable,"]
    #[doc = " * - 1 `stopSignViolation`         - in case a stop sign violation is detected,"]
    #[doc = " * - 2 `trafficLightViolation`     - in case a traffic light violation is detected,"]
    #[doc = " * - 3 `turningRegulationViolation`- in case a turning regulation violation is detected."]
    #[doc = " * - 4-255                         - are reserved for future usage."]
    #[doc = " *"]
    #[doc = " * @category: Traffic information"]
    #[doc = " * @revision: V1.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct SignalViolationSubCauseCode(pub u8);
    #[doc = "*"]
    #[doc = " * This DE represents the sub cause codes of the @ref CauseCode \"slowVehicle\"."]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 `unavailable`                            - in case further detailed information on slow vehicle driving event is unavailable,"]
    #[doc = " * - 1 `maintenanceVehicle`                     - in case of a slow driving maintenance vehicle on the road (incl. winter maintenance without detailed status),"]
    #[doc = " * - 2 `vehiclesSlowingToLookAtAccident`        - in case vehicle is temporally slowing down to look at accident, spot, etc.,"]
    #[doc = " * - 3 `abnormalLoad`                           - in case an abnormal loaded vehicle is driving slowly on the road,"]
    #[doc = " * - 4 `abnormalWideLoad`                       - in case an abnormal wide load vehicle is driving slowly on the road,"]
    #[doc = " * - 5 `convoy`                                 - in case of slow driving convoy on the road,"]
    #[doc = " * - 6 `winterMaintenanceSnowplough             - in case of slow driving snow plough on the road,"]
    #[doc = " * - 7 `deicing-deprecated`                     - deprecated since covered by 8 `winterMaintenanceAdhesionImprovement`,"]
    #[doc = " * - 8 `winterMaintenanceAdhesionImprovement`   - in case of a slow driving winter maintenance vehicle applying measures to improve the driving conditions "]
    #[doc = "                                                  and adhesion on winter roads, including improving grip (by applying sand or grit), de-icing (by applying salt or brine),"]
    #[doc = "                                                  or anti-icing (by applying brine to prevent buildup of ice and snow). "]
    #[doc = " * - 9-255                                      - are reserved for future usage."]
    #[doc = " * "]
    #[doc = " * @category: Traffic information"]
    #[doc = " * @revision: V1.3.1, value 7 deprecated and name of 6 and 8 changed and semantics of 8 refined in V2.4.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct SlowVehicleSubCauseCode(pub u8);
    #[doc = "*"]
    #[doc = " * The DE indicates if a vehicle is carrying goods in the special transport conditions."]
    #[doc = " *"]
    #[doc = " * The corresponding bit shall be set to 1 under the following conditions:"]
    #[doc = " * - 0 `heavyLoad`        - the vehicle is carrying goods with heavy load,"]
    #[doc = " * - 1 `excessWidth`      - the vehicle is carrying goods in excess of width,"]
    #[doc = " * - 2 `excessLength`     - the vehicle is carrying goods in excess of length,"]
    #[doc = " * - 3 `excessHeight`     - the vehicle is carrying goods in excess of height."]
    #[doc = " *"]
    #[doc = " * Otherwise, the corresponding bit shall be set to 0."]
    #[doc = " * @category Vehicle information"]
    #[doc = " * @revision: Description revised in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct SpecialTransportType(pub FixedBitString<4usize>);
    #[doc = "*"]
    #[doc = " * This DF represents the speed and associated confidence value."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field speedValue: the speed value."]
    #[doc = " * "]
    #[doc = " * @field speedConfidence: the confidence value of the speed value."]
    #[doc = " *"]
    #[doc = " * @category: Kinematic information"]
    #[doc = " * @revision: V1.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct Speed {
        #[rasn(identifier = "speedValue")]
        pub speed_value: SpeedValue,
        #[rasn(identifier = "speedConfidence")]
        pub speed_confidence: SpeedConfidence,
    }
    impl Speed {
        pub fn new(speed_value: SpeedValue, speed_confidence: SpeedConfidence) -> Self {
            Self {
                speed_value,
                speed_confidence,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DE indicates the speed confidence value which represents the estimated absolute accuracy of a speed value with a default confidence level of 95 %."]
    #[doc = " * If required, the confidence level can be defined by the corresponding standards applying this DE."]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `n` (`n > 0` and `n < 126`) if the confidence value is equal to or less than n * 0,01 m/s."]
    #[doc = " * - `126` if the confidence value is out of range, i.e. greater than 1,25 m/s,"]
    #[doc = " * - `127` if the confidence value information is not available."]
    #[doc = " *  "]
    #[doc = " * @note: The fact that a speed value is received with confidence value set to `unavailable(127)` can be caused by several reasons, such as:"]
    #[doc = " * - the sensor cannot deliver the accuracy at the defined confidence level because it is a low-end sensor,"]
    #[doc = " * - the sensor cannot calculate the accuracy due to lack of variables, or"]
    #[doc = " * - there has been a vehicle bus (e.g. CAN bus) error."]
    #[doc = " * In all 3 cases above, the speed value may be valid and used by the application."]
    #[doc = " * "]
    #[doc = " * @note: If a speed value is received and its confidence value is set to `outOfRange(126)`, it means that the speed value is not valid "]
    #[doc = " * and therefore cannot be trusted. Such is not useful for the application."]
    #[doc = " *"]
    #[doc = " * @unit: 0,01 m/s"]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: Description revised in V2.1.1 "]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=127"))]
    pub struct SpeedConfidence(pub u8);
    #[doc = "*"]
    #[doc = " * This DE represents a speed limitation applied to a geographical position, a road section or a geographical region."]
    #[doc = " * "]
    #[doc = " * @unit: km/h"]
    #[doc = " * @category: Infrastructure information, Traffic information"]
    #[doc = " * @revision: V1.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=255"))]
    pub struct SpeedLimit(pub u8);
    #[doc = "*"]
    #[doc = " * This DE represents a speed value, i.e. the magnitude of the velocity-vector. "]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `0` in a standstill situation."]
    #[doc = " * - `n` (`n > 0` and `n < 16 382`) if the applicable value is equal to or less than n x 0,01 m/s, and greater than (n-1) x 0,01 m/s,"]
    #[doc = " * - `16 382` for speed values greater than 163,81 m/s,"]
    #[doc = " * - `16 383` if the speed accuracy information is not available."]
    #[doc = " * "]
    #[doc = " * @note: the definition of standstill is out of scope of the present document."]
    #[doc = " *"]
    #[doc = " * @unit: 0,01 m/s"]
    #[doc = " * @category: Kinematic information"]
    #[doc = " * @revision: Description revised in V2.1.1 (the meaning of 16382 has changed slightly) "]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=16383"))]
    pub struct SpeedValue(pub u16);
    #[doc = "*"]
    #[doc = " * This DF  provides the  indication of  change in stability."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field lossProbability: the probability of stability loss. "]
    #[doc = " * "]
    #[doc = " * @field actionDeltaTime: the period over which the the probability of stability loss is estimated. "]
    #[doc = " *"]
    #[doc = " * @category: Kinematic information"]
    #[doc = " * @revision: V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct StabilityChangeIndication {
        #[rasn(identifier = "lossProbability")]
        pub loss_probability: StabilityLossProbability,
        #[rasn(identifier = "actionDeltaTime")]
        pub action_delta_time: DeltaTimeTenthOfSecond,
    }
    impl StabilityChangeIndication {
        pub fn new(
            loss_probability: StabilityLossProbability,
            action_delta_time: DeltaTimeTenthOfSecond,
        ) -> Self {
            Self {
                loss_probability,
                action_delta_time,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DE indicates the estimated probability of a stability level and conversely also the probability of a stability loss."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `0` to indicate an estimated probability of a loss of stability of 0 %, i.e. \"stable\", "]
    #[doc = " * - `n` (`n > 0` and `n < 50`) to indicate the actual stability level,"]
    #[doc = " * - `50` to indicate a estimated probability of a loss of stability of 100 %, i.e. \"total loss of stability\","]
    #[doc = " * - the values between 51 and 62 are reserved for future use,"]
    #[doc = " * - `63`: this value indicates that the information is unavailable."]
    #[doc = " *"]
    #[doc = " * @unit: 2 %"]
    #[doc = " * @category: Kinematic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=63"))]
    pub struct StabilityLossProbability(pub u8);
    #[doc = "*"]
    #[doc = " * The DE represents length as a measure of distance between points or as a dimension of an object or shape. "]
    #[doc = " *"]
    #[doc = " * @unit: 0,1 metre"]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=4095"))]
    pub struct StandardLength12b(pub u16);
    #[doc = "*"]
    #[doc = " * The DE represents length as a measure of distance between points or as a dimension of an object. "]
    #[doc = " *"]
    #[doc = " * @unit: 0,1 metre"]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct StandardLength1B(pub u8);
    #[doc = "*"]
    #[doc = " * The DE represents length as a measure of distance between points or as a dimension of an object.  "]
    #[doc = " *"]
    #[doc = " * @unit: 0,1 metre"]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=65535"))]
    pub struct StandardLength2B(pub u16);
    #[doc = "*"]
    #[doc = " * The DE represents length as a measure of distance between points. "]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 `lessThan50m`   - for distances below 50 m, "]
    #[doc = " * - 1 `lessThan100m`  - for distances below 100 m,"]
    #[doc = " * - 2 `lessThan200m`  - for distances below 200 m, "]
    #[doc = " * - 3 `lessThan500m`  - for distances below 300 m, "]
    #[doc = " * - 4 `lessThan1000m` - for distances below 1 000 m,"]
    #[doc = " * - 5 `lessThan5km`   - for distances below 5 000 m, "]
    #[doc = " * - 6 `lessThan10km`  - for distances below 10 000 m, "]
    #[doc = " * - 7 `over10km`      - for distances over 10 000 m."]
    #[doc = " *"]
    #[doc = " * @category: GeoReference information"]
    #[doc = " * @revision: Created in V2.1.1 from RelevanceDistance"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum StandardLength3b {
        lessThan50m = 0,
        lessThan100m = 1,
        lessThan200m = 2,
        lessThan500m = 3,
        lessThan1000m = 4,
        lessThan5km = 5,
        lessThan10km = 6,
        over10km = 7,
    }
    #[doc = "*"]
    #[doc = " * The DE represents length as a measure of distance between points or as a dimension of an object. "]
    #[doc = " *"]
    #[doc = " * @unit: 0,1 metre"]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=511"))]
    pub struct StandardLength9b(pub u16);
    #[doc = "*"]
    #[doc = " * This DE represents the identifier of an ITS-S."]
    #[doc = " * The ITS-S ID may be a pseudonym. It may change over space and/or over time."]
    #[doc = " *"]
    #[doc = " * @note: this DE is kept for backwards compatibility reasons only. It is recommended to use the @ref StationId instead."]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: V1.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=4294967295"))]
    pub struct StationID(pub u32);
    #[doc = "*"]
    #[doc = " * This DE represents the identifier of an ITS-S."]
    #[doc = " * The ITS-S ID may be a pseudonym. It may change over space and/or over time."]
    #[doc = " *"]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Created in V2.1.1 based on @ref StationID"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=4294967295"))]
    pub struct StationId(pub u32);
    #[doc = "*"]
    #[doc = " * This DE represents the type of technical context the ITS-S is integrated in."]
    #[doc = " * The station type depends on the integration environment of ITS-S into vehicle, mobile devices or at infrastructure."]
    #[doc = " * "]
    #[doc = " * The value shall be set to the corresponding value of the integration environment of DE TrafficParticipantType"]
    #[doc = " * "]
    #[doc = " * @category: Communication information."]
    #[doc = " * @revision: revised in V2.1.1 (named values 12 and 13 added and note to value 9 deleted). Deleted note and type set equal to TrafficParticipantType in V2.4.1."]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct StationType(pub TrafficParticipantType);
    #[doc = "*"]
    #[doc = " * This DE indicates the duration in minutes since which something is stationary."]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 `lessThan1Minute`         - for being stationary since less than 1 minute, "]
    #[doc = " * - 1 `lessThan2Minutes`        - for being stationary since less than 2 minute and for equal to or more than 1 minute,"]
    #[doc = " * - 2 `lessThan15Minutes`       - for being stationary since less than 15 minutes and for equal to or more than 1 minute,"]
    #[doc = " * - 3 `equalOrGreater15Minutes` - for being stationary since equal to or more than 15 minutes."]
    #[doc = " *"]
    #[doc = " * @category: Kinematic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum StationarySince {
        lessThan1Minute = 0,
        lessThan2Minutes = 1,
        lessThan15Minutes = 2,
        equalOrGreater15Minutes = 3,
    }
    #[doc = "*"]
    #[doc = " * This DE provides the value of the sub cause codes of the @ref CauseCode \"stationaryVehicle\". "]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 `unavailable`             - in case further detailed information on stationary vehicle is unavailable,"]
    #[doc = " * - 1 `humanProblem-deprecated` - deprecated since covered by DE HumanProblemSubCauseCode,"]
    #[doc = " * - 2 `vehicleBreakdown`        - in case stationary vehicle is due to vehicle break down,"]
    #[doc = " * - 3 `postCrash`               - in case stationary vehicle is caused by collision,"]
    #[doc = " * - 4 `publicTransportStop`     - in case public transport vehicle is stationary at bus stop,"]
    #[doc = " * - 5 `carryingDangerousGoods`  - in case the stationary vehicle is carrying dangerous goods,"]
    #[doc = " * - 6 `vehicleOnFire`           - in case of vehicle on fire."]
    #[doc = " * - 7-255 reserved for future usage."]
    #[doc = " *"]
    #[doc = " * @category: Traffic information"]
    #[doc = " * @revision: V1.3.1, value 6 added in V2.1.1, value 1 deprecated in V2.4.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct StationaryVehicleSubCauseCode(pub u8);
    #[doc = "*"]
    #[doc = " * This DF represents the steering wheel angle of the vehicle at certain point in time."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field steeringWheelAngleValue: steering wheel angle value."]
    #[doc = " * "]
    #[doc = " * @field steeringWheelAngleConfidence: confidence value of the steering wheel angle value."]
    #[doc = " * "]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct SteeringWheelAngle {
        #[rasn(identifier = "steeringWheelAngleValue")]
        pub steering_wheel_angle_value: SteeringWheelAngleValue,
        #[rasn(identifier = "steeringWheelAngleConfidence")]
        pub steering_wheel_angle_confidence: SteeringWheelAngleConfidence,
    }
    impl SteeringWheelAngle {
        pub fn new(
            steering_wheel_angle_value: SteeringWheelAngleValue,
            steering_wheel_angle_confidence: SteeringWheelAngleConfidence,
        ) -> Self {
            Self {
                steering_wheel_angle_value,
                steering_wheel_angle_confidence,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DE indicates the steering wheel angle confidence value which represents the estimated absolute accuracy for a  steering wheel angle value with a confidence level of 95 %."]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `n` (`n > 0` and `n < 126`) if the confidence value is equal to or less than n x 1,5 degrees,"]
    #[doc = " * - `126` if the confidence value is out of range, i.e. greater than 187,5 degrees,"]
    #[doc = " * - `127` if the confidence value is not available."]
    #[doc = " * "]
    #[doc = " * @note: The fact that a steering wheel angle value is received with confidence value set to `unavailable(127)`"]
    #[doc = " * can be caused by several reasons, such as:"]
    #[doc = " * - the sensor cannot deliver the accuracy at the defined confidence level because it is a low-end sensor,"]
    #[doc = " * - the sensor cannot calculate the accuracy due to lack of variables, or"]
    #[doc = " * - there has been a vehicle bus (e.g. CAN bus) error."]
    #[doc = " * In all 3 cases above, the steering wheel angle value may be valid and used by the application."]
    #[doc = " * "]
    #[doc = " * If a steering wheel angle value is received and its confidence value is set to `outOfRange(126)`,"]
    #[doc = " * it means that the steering wheel angle value is not valid and therefore cannot be trusted."]
    #[doc = " * Such value is not useful for the application."]
    #[doc = " * "]
    #[doc = " * @unit: 1,5 degree"]
    #[doc = " * @category: Vehicle Information"]
    #[doc = " * @revision: Description revised in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=127"))]
    pub struct SteeringWheelAngleConfidence(pub u8);
    #[doc = "*"]
    #[doc = " * This DE represents the steering wheel angle of the vehicle at certain point in time."]
    #[doc = " * The value shall be provided in the vehicle coordinate system as defined in ISO 8855 [21], clause 2.11."]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `-511` if the steering wheel angle is equal to or greater than 511 x 1,5 degrees = 766,5 degrees to the right,"]
    #[doc = " * - `n` (`n > -511` and `n <= 0`) if  the steering wheel angle is equal to or less than n x 1,5 degrees, and greater than (n-1) x 1,5 degrees, "]
    #[doc = "      turning clockwise (i.e. to the right),"]
    #[doc = " * - `n` (`n >= 1` and `n < 511`) if the steering wheel angle is equal to or less than n x 0,1 degrees, and greater than (n-1) x 0,1 degrees, "]
    #[doc = "      turning counter-clockwise (i.e. to the left),"]
    #[doc = " * - `511` if the steering wheel angle is greater than 510 x 1,5 degrees = 765 degrees to the left,"]
    #[doc = " * - `512` if information is not available."]
    #[doc = " *"]
    #[doc = " * @unit: 1,5 degree"]
    #[doc = " * @revision: Description revised in V2.1.1 (meaning of value 511 has changed slightly)."]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-511..=512"))]
    pub struct SteeringWheelAngleValue(pub i16);
    #[doc = "* "]
    #[doc = " * This DE indicates the type of stored information."]
    #[doc = " *"]
    #[doc = " * The corresponding bit shall be set to 1 under the following conditions:"]
    #[doc = " * "]
    #[doc = " * - `0` undefined        - in case the stored information type is undefined. "]
    #[doc = " * - `1` staticDb         - in case the stored information type is a static database."]
    #[doc = " * - `2` dynamicDb        - in case the stored information type is a dynamic database"]
    #[doc = " * - `3` realTimeDb       - in case the stored information type is a real time updated database."]
    #[doc = " * - `4` map              - in case the stored information type is a road topology map."]
    #[doc = " * - Bits 5 to 7          - are reserved for future use."]
    #[doc = " *"]
    #[doc = " * @note: If all bits are set to 0, then no stored information type is used"]
    #[doc = " *"]
    #[doc = " * @category: Basic Information"]
    #[doc = " * @revision: created in V2.2.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("8", extensible))]
    pub struct StoredInformationType(pub BitString);
    #[doc = "*"]
    #[doc = " * This DE indicates the generic sub cause of a detected event."]
    #[doc = " * "]
    #[doc = " * @note: The sub cause code value assignment varies based on value of @ref CauseCode."]
    #[doc = " *"]
    #[doc = " * @category: Traffic information"]
    #[doc = " * @revision: Description revised in V2.1.1 (this is  the generic sub cause type)"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct SubCauseCodeType(pub u8);
    #[doc = "*"]
    #[doc = " * This DE indicates a temperature value."]
    #[doc = ""]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `-60` for temperature equal to or less than -60 degrees C,"]
    #[doc = " * - `n` (`n > -60` and `n < 67`) for the actual temperature n in degrees C,"]
    #[doc = " * - `67` for temperature equal to or greater than 67 degrees C."]
    #[doc = " * "]
    #[doc = " * @unit: degrees Celsius"]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Editorial update in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-60..=67"))]
    pub struct Temperature(pub i8);
    #[doc = "*"]
    #[doc = " * This DE represents the number of elapsed (TAI) milliseconds since the ITS Epoch. "]
    #[doc = " * The ITS epoch is `00:00:00.000 UTC, 1 January 2004`."]
    #[doc = " * \"Elapsed\" means that the true number of milliseconds is continuously counted without interruption,"]
    #[doc = " *  i.e. it is not altered by leap seconds, which occur in UTC."]
    #[doc = " * "]
    #[doc = " * @note: International Atomic Time (TAI) is the time reference coordinate on the basis of the readings of atomic clocks, "]
    #[doc = " * operated in accordance with the definition of the second, the unit of time of the International System of Units. "]
    #[doc = " * TAI is a continuous time scale. UTC has discontinuities, as it is occasionally adjusted by leap seconds. "]
    #[doc = " * As of 1 January, 2022, TimestampIts is 5 seconds ahead of UTC, because since the ITS epoch on 1 January 2004 00:00:00.000 UTC, "]
    #[doc = " * further 5 leap seconds have been inserted in UTC."]
    #[doc = " * "]
    #[doc = " * EXAMPLE: The value for TimestampIts for 1 January 2007 00:00:00.000 UTC is `94 694 401 000` milliseconds,"]
    #[doc = " * which includes one leap second insertion since the ITS epoch."]
    #[doc = " * @unit: 0,001 s"]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Description revised in in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=4398046511103"))]
    pub struct TimestampIts(pub u64);
    #[doc = "*"]
    #[doc = " * This DF represents one or more paths using @ref Path."]
    #[doc = " * "]
    #[doc = " * @category: GeoReference information"]
    #[doc = " * @revision: Description revised in V2.1.1. Is is now based on Path and not on PathHistory"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=7"))]
    pub struct Traces(pub SequenceOf<Path>);
    #[doc = "*"]
    #[doc = " * This DF represents one or more paths using @ref PathExtended."]
    #[doc = " * "]
    #[doc = " * @category: GeoReference information"]
    #[doc = " * @revision: Created in V2.2.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=7"))]
    pub struct TracesExtended(pub SequenceOf<PathExtended>);
    #[doc = "*"]
    #[doc = " * This DE represents the value of the sub cause codes of the @ref CauseCode `trafficCondition`. "]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 `unavailable`                  - in case further detailed information on the traffic condition is unavailable,"]
    #[doc = " * - 1 `increasedVolumeOfTraffic`     - in case the type of traffic condition is increased traffic volume,"]
    #[doc = " * - 2 `trafficJamSlowlyIncreasing`   - in case the type of traffic condition is a traffic jam which volume is increasing slowly,"]
    #[doc = " * - 3 `trafficJamIncreasing`         - in case the type of traffic condition is a traffic jam which volume is increasing,"]
    #[doc = " * - 4 `trafficJamStronglyIncreasing` - in case the type of traffic condition is a traffic jam which volume is strongly increasing,"]
    #[doc = " * - 5 `trafficJam`         `         - in case the type of traffic condition is a traffic jam and no further detailed information about its volume is available,"]
    #[doc = " * - 6 `trafficJamSlightlyDecreasing` - in case the type of traffic condition is a traffic jam which volume is decreasing slowly,"]
    #[doc = " * - 7 `trafficJamDecreasing`         - in case the type of traffic condition is a traffic jam which volume is decreasing,"]
    #[doc = " * - 8 `trafficJamStronglyDecreasing` - in case the type of traffic condition is a traffic jam which volume is decreasing rapidly,"]
    #[doc = " * - 9 `trafficJamStable`             - in case the traffic condition is a traffic jam with stable volume,"]
    #[doc = " * - 10-255: reserved for future usage."]
    #[doc = " *"]
    #[doc = " * @category: Traffic information"]
    #[doc = " * @revision: V1.3.1, definition of value 0 and 1 changed in V2.2.1, name and definition of value 5 changed in V2.2.1, value 9 added in V2.2.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct TrafficConditionSubCauseCode(pub u8);
    #[doc = "*"]
    #[doc = " * This DE indicates a direction of traffic with respect to a reference direction, and a portion of that traffic with respect to a reference position."]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 `allTrafficDirections`                                    - for all directions of traffic, "]
    #[doc = " * - 1 `sameAsReferenceDirection-upstreamOfReferencePosition`    - for the direction of traffic according to the reference direction, and the portion of traffic upstream of the reference position, "]
    #[doc = " * - 2 `sameAsReferenceDirection-downstreamOfReferencePosition`  - for the direction of traffic according to the reference direction, and the portion of traffic downstream of the reference position, "]
    #[doc = " * - 3 `oppositeToReferenceDirection`                            - for the direction of traffic opposite to the reference direction. "]
    #[doc = " *"]
    #[doc = " * @note: Upstream traffic corresponds to the incoming traffic towards the event position, and downstream traffic to the departing traffic away from the event position."]
    #[doc = " * @category: GeoReference information"]
    #[doc = " * @revision: Created in V2.1.1 from RelevanceTrafficDirection, description and naming of values changed in V2.2.1"]
    #[doc = " *"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum TrafficDirection {
        allTrafficDirections = 0,
        #[rasn(identifier = "sameAsReferenceDirection-upstreamOfReferencePosition")]
        sameAsReferenceDirection_upstreamOfReferencePosition = 1,
        #[rasn(identifier = "sameAsReferenceDirection-downstreamOfReferencePosition")]
        sameAsReferenceDirection_downstreamOfReferencePosition = 2,
        oppositeToReferenceDirection = 3,
    }
    #[doc = "*"]
    #[doc = " * Ths DF represents the a position on a traffic island between two lanes. "]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field oneSide: represents one lane."]
    #[doc = " * "]
    #[doc = " * @field otherSide: represents the other lane."]
    #[doc = " * "]
    #[doc = " * @category: Road Topology information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct TrafficIslandPosition {
        #[rasn(identifier = "oneSide")]
        pub one_side: LanePositionAndType,
        #[rasn(identifier = "otherSide")]
        pub other_side: LanePositionAndType,
    }
    impl TrafficIslandPosition {
        pub fn new(one_side: LanePositionAndType, other_side: LanePositionAndType) -> Self {
            Self {
                one_side,
                other_side,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DE represents the type of a traffic participant."]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 `unknown`          - information about traffic participant is not provided,"]
    #[doc = " * - 1 `pedestrian`       - human being not using a mechanical device for their trip (VRU profile 1),"]
    #[doc = " * - 2 `cyclist`          - non-motorized unicycles, bicycles , tricycles, quadracycles (VRU profile 2),"]
    #[doc = " * - 3 `moped`            - light motor vehicles with less than four wheels as defined in UNECE/TRANS/WP.29/78/Rev.4 [16] class L1, L2 (VRU Profile 3),"]
    #[doc = " * - 4 `motorcycles`      - motor vehicles with less than four wheels as defined in UNECE/TRANS/WP.29/78/Rev.4 [16] class L3, L4, L5, L6, L7 (VRU Profile 3),"]
    #[doc = " * - 5 `passengerCar`     - small passenger vehicles as defined in UNECE/TRANS/WP.29/78/Rev.4 [16] class M1,"]
    #[doc = " * - 6 `bus`              - large passenger vehicles as defined in UNECE/TRANS/WP.29/78/Rev.4 [16] class M2, M3,"]
    #[doc = " * - 7 `lightTruck`       - light Goods Vehicles as defined in UNECE/TRANS/WP.29/78/Rev.4 [16] class N1,"]
    #[doc = " * - 8 `heavyTruck`       - Heavy Goods Vehicles as defined in UNECE/TRANS/WP.29/78/Rev.4 [16] class N2 and N3,"]
    #[doc = " * - 9 `trailer`          - unpowered vehicle that is intended to be towed by a powered vehicle as defined in UNECE/TRANS/WP.29/78/Rev.4 [16] class O,"]
    #[doc = " * - 10 `specialVehicles` - vehicles which have special purposes other than the above (e.g. moving road works vehicle),"]
    #[doc = " * - 11 `tram`            - vehicle which runs on tracks along public streets,"]
    #[doc = " * - 12 `lightVruVehicle` - human being traveling on light vehicle, incl. possible use of roller skates or skateboards (VRU profile 2),"]
    #[doc = " * - 13 `animal`          - animal presenting a safety risk to other road users e.g. domesticated dog in a city or horse (VRU Profile 4),"]
    #[doc = " * - 14 `agricultural`    - agricultural and forestry vehicles as defined in UNECE/TRANS/WP.29/78/Rev.4 [16] class T,"]
    #[doc = " * - 15 `infrastructure`  - infrastructure typically positioned outside of the drivable roadway (e.g. on a gantry, on a pole, on a stationary road works trailer, "]
    #[doc = "                            or in a traffic control center); the infrastructure is static during the entire operation period of the ITS-S (e.g. no stop and go activity),"]
    #[doc = " * - 16-255               - are reserved for future usage."]
    #[doc = " * "]
    #[doc = " * @category: Communication information."]
    #[doc = " * @revision: Created in V2.1.1 based on StationType, Changed name and definition of value 15 in V2.4.1 "]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct TrafficParticipantType(pub u8);
    #[doc = "*"]
    #[doc = " * This DE indicates traffic rules that apply to vehicles at a certain position."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `0` - if overtaking is prohibited for all vehicles,"]
    #[doc = " * - `1` - if overtaking is prohibited for trucks,"]
    #[doc = " * - `2` - if vehicles should pass to the right lane,"]
    #[doc = " * - `3` - if vehicles should pass to the left lane."]
    #[doc = " * - `4` - if vehicles should pass to the left or right lane."]
    #[doc = " *"]
    #[doc = " * @category: Infrastructure information, Traffic information"]
    #[doc = " * @revision: Editorial update in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum TrafficRule {
        noPassing = 0,
        noPassingForTrucks = 1,
        passToRight = 2,
        passToLeft = 3,
        #[rasn(extension_addition)]
        passToLeftOrRight = 4,
    }
    #[doc = "* "]
    #[doc = " * This DF provides detailed information about an attached trailer."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field refPointId: identifier of the reference point of the trailer."]
    #[doc = " *"]
    #[doc = " * @field hitchPointOffset: optional position of the hitch point in negative x-direction (according to ISO 8855) from the"]
    #[doc = " * vehicle Reference Point."]
    #[doc = " *"]
    #[doc = " * @field frontOverhang: optional length of the trailer overhang in the positive x direction (according to ISO 8855) from the"]
    #[doc = " * trailer Reference Point indicated by the refPointID. The value defaults to 0 in case the trailer"]
    #[doc = " * is not overhanging to the front with respect to the trailer reference point."]
    #[doc = " *"]
    #[doc = " * @field rearOverhang: optional length of the trailer overhang in the negative x direction (according to ISO 8855) from the"]
    #[doc = " * trailer Reference Point indicated by the refPointID."]
    #[doc = " *"]
    #[doc = " * @field trailerWidth: optional width of the trailer."]
    #[doc = " *"]
    #[doc = " * @field hitchAngle: optional Value and confidence value of the angle between the trailer orientation (corresponding to the x"]
    #[doc = " * direction of the ISO 8855 [21] coordinate system centered on the trailer) and the direction of"]
    #[doc = " * the segment having as end points the reference point of the trailer and the reference point of"]
    #[doc = " * the pulling vehicle, which can be another trailer or a vehicle looking on the horizontal plane"]
    #[doc = " * xy, described in the local Cartesian coordinate system of the trailer. The"]
    #[doc = " * angle is measured with negative values considering the trailer orientation turning clockwise"]
    #[doc = " * starting from the segment direction. The angle value accuracy is provided with the"]
    #[doc = " * confidence level of 95 %."]
    #[doc = " *"]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct TrailerData {
        #[rasn(identifier = "refPointId")]
        pub ref_point_id: Identifier1B,
        #[rasn(identifier = "hitchPointOffset")]
        pub hitch_point_offset: StandardLength1B,
        #[rasn(identifier = "frontOverhang")]
        pub front_overhang: Option<StandardLength1B>,
        #[rasn(identifier = "rearOverhang")]
        pub rear_overhang: Option<StandardLength1B>,
        #[rasn(identifier = "trailerWidth")]
        pub trailer_width: Option<VehicleWidth>,
        #[rasn(identifier = "hitchAngle")]
        pub hitch_angle: CartesianAngle,
    }
    impl TrailerData {
        pub fn new(
            ref_point_id: Identifier1B,
            hitch_point_offset: StandardLength1B,
            front_overhang: Option<StandardLength1B>,
            rear_overhang: Option<StandardLength1B>,
            trailer_width: Option<VehicleWidth>,
            hitch_angle: CartesianAngle,
        ) -> Self {
            Self {
                ref_point_id,
                hitch_point_offset,
                front_overhang,
                rear_overhang,
                trailer_width,
                hitch_angle,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DE provides information about the presence of a trailer. "]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 `noTrailerPresent`                - to indicate that no trailer is present, i.e. either the vehicle is physically not enabled to tow a trailer or it has been detected that no trailer is present."]
    #[doc = " * - 1 `trailerPresentWithKnownLength`   - to indicate that a trailer has been detected as present and the length is included in the vehicle length value."]
    #[doc = " * - 2 `trailerPresentWithUnknownLength` - to indicate that a trailer has been detected as present and the length is not included in the vehicle length value."]
    #[doc = " * - 3 `trailerPresenceIsUnknown`        - to indicate that information about the trailer presence is unknown, i.e. the vehicle is physically enabled to tow a trailer but the detection of trailer presence/absence is not possible."]
    #[doc = " * - 4 `unavailable`                     - to indicate that the information about the presence of a trailer is not available, i.e. it is neither known whether the vehicle is able to tow a trailer "]
    #[doc = " *                                         nor the detection of trailer presence/absence is possible."]
    #[doc = " * "]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: Created in V2.1.1 based on VehicleLengthConfidenceIndication"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum TrailerPresenceInformation {
        noTrailerPresent = 0,
        trailerPresentWithKnownLength = 1,
        trailerPresentWithUnknownLength = 2,
        trailerPresenceIsUnknown = 3,
        unavailable = 4,
    }
    #[doc = "*"]
    #[doc = " * This DE  defines  the  confidence level of the trajectoryInterceptionProbability."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `0` - to indicate confidence level less than 50 %,"]
    #[doc = " * - `1` - to indicate confidence level greater than or equal to 50 % and less than 70 %,"]
    #[doc = " * - `2` - to indicate confidence level greater than or equal to 70 % and less than 90 %,"]
    #[doc = " * - `3` - to indicate confidence level greater than or equal to 90%."]
    #[doc = " * "]
    #[doc = " * @category: Kinematic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=3"))]
    pub struct TrajectoryInterceptionConfidence(pub u8);
    #[doc = "*"]
    #[doc = " * This DF  provides the trajectory  interception  indication  of  ego-VRU  ITS-S  with another ITS-Ss. "]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field subjectStation: indicates the subject station."]
    #[doc = " * "]
    #[doc = " * @field trajectoryInterceptionProbability: indicates the propbability of the interception of the subject station trajectory "]
    #[doc = " * with the trajectory of the station indicated in the component subjectStation."]
    #[doc = " *"]
    #[doc = " * @field trajectoryInterceptionConfidence: indicates the confidence of interception of the subject station trajectory "]
    #[doc = " * with the trajectory of the station indicated in the component subjectStation."]
    #[doc = " * "]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct TrajectoryInterceptionIndication {
        #[rasn(identifier = "subjectStation")]
        pub subject_station: Option<StationId>,
        #[rasn(identifier = "trajectoryInterceptionProbability")]
        pub trajectory_interception_probability: TrajectoryInterceptionProbability,
        #[rasn(identifier = "trajectoryInterceptionConfidence")]
        pub trajectory_interception_confidence: Option<TrajectoryInterceptionConfidence>,
    }
    impl TrajectoryInterceptionIndication {
        pub fn new(
            subject_station: Option<StationId>,
            trajectory_interception_probability: TrajectoryInterceptionProbability,
            trajectory_interception_confidence: Option<TrajectoryInterceptionConfidence>,
        ) -> Self {
            Self {
                subject_station,
                trajectory_interception_probability,
                trajectory_interception_confidence,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This  DE  defines  the  probability  that the ego trajectory  intercepts  with any  other object's  trajectory  on the  road. "]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `n` (`n >= 0` and `n <= 50`) to indicate the actual probability,"]
    #[doc = " * - the values between 51 and 62 are reserved,"]
    #[doc = " * - `63`: to indicate that the information is unavailable. "]
    #[doc = " *"]
    #[doc = " * @unit: 2 %"]
    #[doc = " * @category: Kinematic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=63"))]
    pub struct TrajectoryInterceptionProbability(pub u8);
    #[doc = "*"]
    #[doc = " * This DE represents the time interval between two consecutive message transmissions."]
    #[doc = " * "]
    #[doc = " * @note: this DE is kept for backwards compatibility reasons only. It is recommended to use the @ref DeltaTimeMilliSecondPos instead."]
    #[doc = " * @unit: 0,001 s"]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: V1.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=10000"))]
    pub struct TransmissionInterval(pub u16);
    #[doc = "*"]
    #[doc = " * This DE provides the turning direction. "]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `left`  for turning to te left."]
    #[doc = " * - `right` for turing to the right."]
    #[doc = " *"]
    #[doc = " * @category: Kinematic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum TurningDirection {
        left = 0,
        right = 1,
    }
    #[doc = "*"]
    #[doc = " * This DE represents the smallest circular turn (i.e. U-turn) that the vehicle is capable of making."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `n` (`n > 0` and `n < 254`) to indicate the applicable value is equal to or less than n x 0,4 metre, and greater than (n-1) x 0,4 metre,"]
    #[doc = " * - `254` to indicate that the turning radius is  greater than 253 x 0,4 metre = 101.2 metres,"]
    #[doc = " * - `255` to indicate that the information is unavailable."]
    #[doc = " * "]
    #[doc = " * For vehicle with tracker, the turning radius applies to the vehicle only."]
    #[doc = " *"]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @unit 0,4 metre"]
    #[doc = " * @revision: Description revised V2.1.1 (the meaning of 254 has changed slightly)"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=255"))]
    pub struct TurningRadius(pub u8);
    #[doc = "*"]
    #[doc = " * This DE represents an indication of how a certain path or area will be used. "]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0  - ` noIndication `     - in case it will remain free to be used,"]
    #[doc = " * - 1  - ` specialUse `       - in case it will be physically blocked by special use,"]
    #[doc = " * - 2  - ` rescueOperation`   - in case it is intended to be used for rescue operations,"]
    #[doc = " * - 3  - ` railroad `         - in case it is intended to be used for rail traffic,"]
    #[doc = " * - 4  - ` fixedRoute `       - in case it is intended to be used for fixed route traffic (e.g. bus line in service, delivery services, garbage truck in service etc.),"]
    #[doc = " * - 5  - ` restrictedRoute `  - in case it is intended to be used for driving on restricted routes (e.g dedicated lanes/routes for heavy trucks or buses),"]
    #[doc = " * - 6  - ` adasAd `           - in case it is intended to be used for driving with an active ADAS or AD system (see DE AccelerationControl or AutomationControl),"]
    #[doc = " * - 7  - ` navigation `       - in case it is intended to be used for driving with an active navigation system."]
    #[doc = " *"]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Created in V2.2.1, extension 3-7 added in V2.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum UsageIndication {
        noIndication = 0,
        specialUse = 1,
        rescueOperation = 2,
        #[rasn(extension_addition)]
        railroad = 3,
        #[rasn(extension_addition)]
        fixedRoute = 4,
        #[rasn(extension_addition)]
        restrictedRoute = 5,
        #[rasn(extension_addition)]
        adasAd = 6,
        #[rasn(extension_addition)]
        navigation = 7,
    }
    #[doc = "*"]
    #[doc = " * This DE represents the Vehicle Descriptor Section (VDS). The values are assigned according to ISO 3779 [6]."]
    #[doc = " * "]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: V1.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("6"))]
    pub struct VDS(pub Ia5String);
    #[doc = "* "]
    #[doc = " * This DE represents the duration of a traffic event validity. "]
    #[doc = " *"]
    #[doc = " * @note: this DE is kept for backwards compatibility reasons only. It is recommended to use the @ref DeltaTimeSecond instead."]
    #[doc = " * @unit: 1 s"]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=86400"))]
    pub struct ValidityDuration(pub u32);
    #[doc = "*"]
    #[doc = " * This DF together with its sub DFs Ext1, Ext2 and the DE Ext3 provides the custom (i.e. not ASN.1 standard) definition of an integer with variable lenght, that can be used for example to encode the ITS-AID. "]
    #[doc = " *"]
    #[doc = " * @category: Basic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice)]
    pub enum VarLengthNumber {
        #[rasn(value("0..=127"), tag(context, 0))]
        content(u8),
        #[rasn(tag(context, 1))]
        extension(Ext1),
    }
    #[doc = "*"]
    #[doc = " * This DE represents the value of the sub cause codes of the @ref CauseCode `vehicleBreakdown`. "]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 `unavailable`              - in case further detailed information on cause of vehicle break down is unavailable,"]
    #[doc = " * - 1 `lackOfFuel`               - in case vehicle break down is due to lack of fuel,"]
    #[doc = " * - 2 `lackOfBatteryPower`       - in case vehicle break down is caused by lack of battery power,"]
    #[doc = " * - 3 `engineProblem`            - in case vehicle break down is caused by an engine problem,"]
    #[doc = " * - 4 `transmissionProblem`      - in case vehicle break down is caused by transmission problem,"]
    #[doc = " * - 5 `engineCoolingProblem`     - in case vehicle break down is caused by an engine cooling problem,"]
    #[doc = " * - 6 `brakingSystemProblem`     - in case vehicle break down is caused by a braking system problem,"]
    #[doc = " * - 7 `steeringProblem`          - in case vehicle break down is caused by a steering problem,"]
    #[doc = " * - 8 `tyrePuncture-deprecated`  - deprecated since covered by `tyrePressureProblem`,"]
    #[doc = " * - 9 `tyrePressureProblem`      - in case low tyre pressure in detected,"]
    #[doc = " * - 10 `vehicleOnFire`           - in case the vehicle is on fire."]
    #[doc = " * - 11-255                       - are reserved for future usage."]
    #[doc = " *"]
    #[doc = " * @category: Traffic information"]
    #[doc = " * @revision: V1.3.1, value 10 assigned in V2.1.1, value 8 deprecated in V2.4.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct VehicleBreakdownSubCauseCode(pub u8);
    #[doc = "* "]
    #[doc = " * This DE represents the height of the vehicle, measured from the ground to the highest point, excluding any antennas."]
    #[doc = " * In case vehicles are equipped with adjustable ride heights, camper shells, and any other"]
    #[doc = " * equipment which may result in varying height, the largest possible height shall be used."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `n` (`n >0` and `n < 127`) indicates the applicable value is equal to or less than n x 0,05 metre, and greater than (n-1) x 0,05 metre,"]
    #[doc = " * - `127` indicates that the vehicle width is greater than 6,3 metres,"]
    #[doc = " * - `128` indicates that the information in unavailable."]
    #[doc = " *"]
    #[doc = " * @unit: 0,05 metre "]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=128"))]
    pub struct VehicleHeight(pub u8);
    #[doc = "* "]
    #[doc = " * This DE represents the height of the vehicle, measured from the ground to the highest point, excluding any antennas."]
    #[doc = " * In case vehicles are equipped with adjustable ride heights, camper shells, and any other"]
    #[doc = " * equipment which may result in varying height, the largest possible height shall be used."]
    #[doc = ""]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `n` (`n > 0` and `n < 61`) indicates the applicable value is equal to or less than n x 0,1 metre, and greater than (n-1) x 0,1 metre,"]
    #[doc = " * - `61` indicates that the vehicle height is greater than 6,0 metres,"]
    #[doc = " * - `62` indicates that the information in unavailable."]
    #[doc = " * "]
    #[doc = " * @unit: 0,1 metre"]
    #[doc = " * @category: Vehicle information "]
    #[doc = " * @revision: created in V2.3.1 based on VehicleHeight but better aligned in unit and range with VehicleWidth."]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=62"))]
    pub struct VehicleHeight2(pub u8);
    #[doc = "*"]
    #[doc = " * This DF provides information related to the identification of a vehicle."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field wMInumber: World Manufacturer Identifier (WMI) code."]
    #[doc = " * "]
    #[doc = " * @field vDS: Vehicle Descriptor Section (VDS). "]
    #[doc = " * "]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: V1.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct VehicleIdentification {
        #[rasn(identifier = "wMInumber")]
        pub w_minumber: Option<WMInumber>,
        #[rasn(identifier = "vDS")]
        pub v_ds: Option<VDS>,
    }
    impl VehicleIdentification {
        pub fn new(w_minumber: Option<WMInumber>, v_ds: Option<VDS>) -> Self {
            Self { w_minumber, v_ds }
        }
    }
    #[doc = "*"]
    #[doc = " * This DF represents the length of vehicle and accuracy indication information."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field vehicleLengthValue: length of vehicle. "]
    #[doc = " * "]
    #[doc = " * @field vehicleLengthConfidenceIndication: indication of the length value confidence."]
    #[doc = " * "]
    #[doc = " * @note: this DF is kept for backwards compatibility reasons only. It is recommended to use @ref VehicleLengthV2 instead."]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: V1.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct VehicleLength {
        #[rasn(identifier = "vehicleLengthValue")]
        pub vehicle_length_value: VehicleLengthValue,
        #[rasn(identifier = "vehicleLengthConfidenceIndication")]
        pub vehicle_length_confidence_indication: VehicleLengthConfidenceIndication,
    }
    impl VehicleLength {
        pub fn new(
            vehicle_length_value: VehicleLengthValue,
            vehicle_length_confidence_indication: VehicleLengthConfidenceIndication,
        ) -> Self {
            Self {
                vehicle_length_value,
                vehicle_length_confidence_indication,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DE provides information about the presence of a trailer. "]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 `noTrailerPresent`                - to indicate that no trailer is present, i.e. either the vehicle is physically not enabled to tow a trailer or it has been detected that no trailer is present,"]
    #[doc = " * - 1 `trailerPresentWithKnownLength`   - to indicate that a trailer has been detected as present and the length is  included in the vehicle length value,"]
    #[doc = " * - 2 `trailerPresentWithUnknownLength` - to indicate that a trailer has been detected as present and the length is not included in the vehicle length value,"]
    #[doc = " * - 3 `trailerPresenceIsUnknown`        - to indicate that information about the trailer presence is unknown, i.e. the vehicle is physically enabled to tow a trailer but the detection of trailer presence/absence is not possible,"]
    #[doc = " * - 4 `unavailable`                     - to indicate that the information about the presence of a trailer is not available, i.e. it is neither known whether the vehicle is able to tow a trailer, "]
    #[doc = " *                                        nor the detection of trailer presence/absence is possible."]
    #[doc = " * "]
    #[doc = " * @note: this DE is kept for backwards compatibility reasons only. It is recommended to use the @ref TrailerPresenceInformation instead. "]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: Description revised in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum VehicleLengthConfidenceIndication {
        noTrailerPresent = 0,
        trailerPresentWithKnownLength = 1,
        trailerPresentWithUnknownLength = 2,
        trailerPresenceIsUnknown = 3,
        unavailable = 4,
    }
    #[doc = "*"]
    #[doc = " * This DF represents the length of vehicle and accuracy indication information."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field vehicleLengthValue: length of vehicle. "]
    #[doc = " * "]
    #[doc = " * @field trailerPresenceInformation: information about the trailer presence."]
    #[doc = " * "]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: created in V2.1.1 based on @ref VehicleLength but using @ref TrailerPresenceInformation."]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct VehicleLengthV2 {
        #[rasn(identifier = "vehicleLengthValue")]
        pub vehicle_length_value: VehicleLengthValue,
        #[rasn(identifier = "trailerPresenceInformation")]
        pub trailer_presence_information: TrailerPresenceInformation,
    }
    impl VehicleLengthV2 {
        pub fn new(
            vehicle_length_value: VehicleLengthValue,
            trailer_presence_information: TrailerPresenceInformation,
        ) -> Self {
            Self {
                vehicle_length_value,
                trailer_presence_information,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DE represents the length of a vehicle."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `n` (`n > 0` and `n < 1022`) to indicate the applicable value n is equal to or less than n x 0,1 metre, and greater than (n-1) x 0,1 metre,"]
    #[doc = " * - `1 022` to indicate that the vehicle length is greater than 102.1 metres,"]
    #[doc = " * - `1 023` to indicate that the information in unavailable."]
    #[doc = " * "]
    #[doc = " * "]
    #[doc = " * @unit: 0,1 metre"]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: Description updated in V2.1.1 (the meaning of 1 022 has changed slightly)."]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=1023"))]
    pub struct VehicleLengthValue(pub u16);
    #[doc = "*"]
    #[doc = " * This DE represents the mass of an empty loaded vehicle."]
    #[doc = " *"]
    #[doc = " * The value shall be set to: "]
    #[doc = " * - `n` (`n > 0` and `n < 1023`) to indicate that the applicable value is equal to or less than n x 10^5 gramm, and greater than (n-1) x 10^5 gramm,"]
    #[doc = " * - `1 023` indicates that the vehicle mass is greater than 102 200 000 g,"]
    #[doc = " * - `1 024` indicates  the vehicle mass information is unavailable."]
    #[doc = " * "]
    #[doc = " * @note:\tThe empty load vehicle is defined in ISO 1176 [8], clause 4.6."]
    #[doc = " * "]
    #[doc = " * @unit: 10^5 gramm"]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: Description updated in V2.1.1 (the meaning of 1 023 has changed slightly)."]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=1024"))]
    pub struct VehicleMass(pub u16);
    #[doc = "*"]
    #[doc = " * This DF provides information about the status of the vehicleÂ´s movement control mechanisms. "]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field accelerationPedalStatus: information about the status of the acceleration pedal. "]
    #[doc = " * "]
    #[doc = " * @field brakePedalPostionStatus information about the status of the brake pedal. "]
    #[doc = " *"]
    #[doc = " * @field saeAutomationLevel: optional information about the level of driving automation. "]
    #[doc = " *"]
    #[doc = " * @field automationControl: optional information about the controlling mechanism for lateral, or combined lateral and longitudinal movement."]
    #[doc = " *"]
    #[doc = " * @field accelerationControl: optional information about the controlling mechanism for longitudinal movement. "]
    #[doc = " *"]
    #[doc = " * @field accelerationControlExtension: optional extended information about the controlling mechanism for longitudinal movement. "]
    #[doc = " *"]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: created in V2.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct VehicleMovementControl {
        #[rasn(identifier = "accelerationPedalStatus")]
        pub acceleration_pedal_status: PedalStatus,
        #[rasn(identifier = "brakePedalStatus")]
        pub brake_pedal_status: PedalStatus,
        #[rasn(identifier = "saeAutomationLevel")]
        pub sae_automation_level: Option<SaeAutomationLevel>,
        #[rasn(identifier = "automationControl")]
        pub automation_control: Option<AutomationControl>,
        #[rasn(identifier = "accelerationControl")]
        pub acceleration_control: Option<AccelerationControl>,
        #[rasn(identifier = "accelerationControlExtension")]
        pub acceleration_control_extension: Option<AccelerationControlExtension>,
    }
    impl VehicleMovementControl {
        pub fn new(
            acceleration_pedal_status: PedalStatus,
            brake_pedal_status: PedalStatus,
            sae_automation_level: Option<SaeAutomationLevel>,
            automation_control: Option<AutomationControl>,
            acceleration_control: Option<AccelerationControl>,
            acceleration_control_extension: Option<AccelerationControlExtension>,
        ) -> Self {
            Self {
                acceleration_pedal_status,
                brake_pedal_status,
                sae_automation_level,
                automation_control,
                acceleration_control,
                acceleration_control_extension,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DE indicates the role played by a vehicle at a point in time."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 `default`          - to indicate the default vehicle role as indicated by the vehicle type,"]
    #[doc = " * - 1 `publicTransport`  - to indicate that the vehicle is used to operate public transport service,"]
    #[doc = " * - 2 `specialTransport` - to indicate that the vehicle is used for special transport purpose, e.g. oversized trucks,"]
    #[doc = " * - 3 `dangerousGoods`   - to indicate that the vehicle is used for dangerous goods transportation,"]
    #[doc = " * - 4 `roadWork`         - to indicate that the vehicle is used to realize roadwork or road maintenance mission,"]
    #[doc = " * - 5 `rescue`           - to indicate that the vehicle is used for rescue purpose in case of an accident, e.g. as a towing service,"]
    #[doc = " * - 6 `emergency`        - to indicate that the vehicle is used for emergency mission, e.g. ambulance, fire brigade,"]
    #[doc = " * - 7 `safetyCar`        - to indicate that the vehicle is used for public safety, e.g. patrol,"]
    #[doc = " * - 8 `agriculture`      - to indicate that the vehicle is used for agriculture, e.g. farm tractor, "]
    #[doc = " * - 9 `commercial`       - to indicate that the vehicle is used for transportation of commercial goods,"]
    #[doc = " * - 10 `military`        - to indicate that the vehicle is used for military purpose, "]
    #[doc = " * - 11 `roadOperator`    - to indicate that the vehicle is used in road operator missions,"]
    #[doc = " * - 12 `taxi`            - to indicate that the vehicle is used to provide an authorized taxi service,"]
    #[doc = " * - 13 `uvar`            - to indicate that the vehicle is authorized to enter a zone according to the applicable Urban Vehicle Access Restrictions."]
    #[doc = " * - 14 `rfu1`            - is reserved for future usage."]
    #[doc = " * - 15 `rfu2`            - is reserved for future usage."]
    #[doc = " * "]
    #[doc = " * @category: Vehicle Information"]
    #[doc = " * @revision: Description updated in V2.1.1 (removed reference to CEN/TS 16157-3), value 13 assigned in V2.2.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum VehicleRole {
        default = 0,
        publicTransport = 1,
        specialTransport = 2,
        dangerousGoods = 3,
        roadWork = 4,
        rescue = 5,
        emergency = 6,
        safetyCar = 7,
        agriculture = 8,
        commercial = 9,
        military = 10,
        roadOperator = 11,
        taxi = 12,
        uvar = 13,
        rfu1 = 14,
        rfu2 = 15,
    }
    #[doc = "*"]
    #[doc = " * This DE represents the width of a vehicle, excluding side mirrors and possible similar extensions."]
    #[doc = ""]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `n` (`n > 0` and `n < 61`) indicates the applicable value is equal to or less than n x 0,1 metre, and greater than (n-1) x 0,1 metre,"]
    #[doc = " * - `61` indicates that the vehicle width is greater than 6,0 metres,"]
    #[doc = " * - `62` indicates that the information in unavailable."]
    #[doc = " * "]
    #[doc = " * @unit: 0,1 metre"]
    #[doc = " * @category: Vehicle information "]
    #[doc = " * @revision: Description updated in V2.1.1 (the meaning of 61 has changed slightly)."]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=62"))]
    pub struct VehicleWidth(pub u8);
    #[doc = "*"]
    #[doc = " * This DF represents a velocity vector with associated confidence value."]
    #[doc = " *"]
    #[doc = " * The following options are available:"]
    #[doc = " * "]
    #[doc = " * @field polarVelocity: the representation of the velocity vector in a polar or cylindrical coordinate system. "]
    #[doc = " * "]
    #[doc = " * @field cartesianVelocity: the representation of the velocity vector in a cartesian coordinate system."]
    #[doc = " * "]
    #[doc = " * @category: Kinematic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum Velocity3dWithConfidence {
        polarVelocity(VelocityPolarWithZ),
        cartesianVelocity(VelocityCartesian),
    }
    #[doc = "*"]
    #[doc = " * This DF represents a velocity vector in a cartesian coordinate system."]
    #[doc = " "]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field xVelocity: the x component of the velocity vector with the associated confidence value."]
    #[doc = " * "]
    #[doc = " * @field yVelocity: the y component of the velocity vector with the associated confidence value."]
    #[doc = " *"]
    #[doc = " * @field zVelocity: the optional z component of the velocity vector with the associated confidence value."]
    #[doc = " * "]
    #[doc = " * @category: Kinematic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct VelocityCartesian {
        #[rasn(identifier = "xVelocity")]
        pub x_velocity: VelocityComponent,
        #[rasn(identifier = "yVelocity")]
        pub y_velocity: VelocityComponent,
        #[rasn(identifier = "zVelocity")]
        pub z_velocity: Option<VelocityComponent>,
    }
    impl VelocityCartesian {
        pub fn new(
            x_velocity: VelocityComponent,
            y_velocity: VelocityComponent,
            z_velocity: Option<VelocityComponent>,
        ) -> Self {
            Self {
                x_velocity,
                y_velocity,
                z_velocity,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DF represents a component of the velocity vector and the associated confidence value."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field value: the value of the component."]
    #[doc = " * "]
    #[doc = " * @field confidence: the confidence value of the value."]
    #[doc = " *"]
    #[doc = " * @category: Kinematic information"]
    #[doc = " * @revision: V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct VelocityComponent {
        pub value: VelocityComponentValue,
        pub confidence: SpeedConfidence,
    }
    impl VelocityComponent {
        pub fn new(value: VelocityComponentValue, confidence: SpeedConfidence) -> Self {
            Self { value, confidence }
        }
    }
    #[doc = "* "]
    #[doc = " * This DE represents the value of a velocity component in a defined coordinate system."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `-16 383` if the velocity is equal to or smaller than -163,83 m/s,"]
    #[doc = " * - `n` (`n > -16 383` and `n < 16 382`) if the applicable value is equal to or less than n x 0,01 m/s, and greater than (n-1) x 0,01 m/s,"]
    #[doc = " * - `16 382` for velocity values equal to or greater than 163,81 m/s,"]
    #[doc = " * - `16 383` if the velocity information is not available."]
    #[doc = " * "]
    #[doc = " * @unit: 0,01 m/s"]
    #[doc = " * @category: Kinematic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-16383..=16383"))]
    pub struct VelocityComponentValue(pub i16);
    #[doc = "*"]
    #[doc = " * This DF represents a velocity vector in a polar or cylindrical coordinate system."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field velocityMagnitude: magnitude of the velocity vector on the reference plane, with the associated confidence value."]
    #[doc = " * "]
    #[doc = " * @field velocityDirection: polar angle of the velocity vector on the reference plane, with the associated confidence value."]
    #[doc = " *"]
    #[doc = " * @field zVelocity: the optional z component of the velocity vector along the reference axis of the cylindrical coordinate system, with the associated confidence value."]
    #[doc = " * "]
    #[doc = " * @category: Kinematic information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct VelocityPolarWithZ {
        #[rasn(identifier = "velocityMagnitude")]
        pub velocity_magnitude: Speed,
        #[rasn(identifier = "velocityDirection")]
        pub velocity_direction: CartesianAngle,
        #[rasn(identifier = "zVelocity")]
        pub z_velocity: Option<VelocityComponent>,
    }
    impl VelocityPolarWithZ {
        pub fn new(
            velocity_magnitude: Speed,
            velocity_direction: CartesianAngle,
            z_velocity: Option<VelocityComponent>,
        ) -> Self {
            Self {
                velocity_magnitude,
                velocity_direction,
                z_velocity,
            }
        }
    }
    #[doc = " four and more octets length"]
    #[doc = "*"]
    #[doc = " * This DF indicates the vehicle acceleration at vertical direction and the associated confidence value."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field verticalAccelerationValue: vertical acceleration value at a point in time."]
    #[doc = " * "]
    #[doc = " * @field verticalAccelerationConfidence: confidence value of the vertical acceleration value with a predefined confidence level."]
    #[doc = " * "]
    #[doc = " * @note: this DF is kept for backwards compatibility reasons only. It is recommended to use @ref AccelerationComponent instead."]
    #[doc = " * @category Vehicle information"]
    #[doc = " * @revision: Description revised in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct VerticalAcceleration {
        #[rasn(identifier = "verticalAccelerationValue")]
        pub vertical_acceleration_value: VerticalAccelerationValue,
        #[rasn(identifier = "verticalAccelerationConfidence")]
        pub vertical_acceleration_confidence: AccelerationConfidence,
    }
    impl VerticalAcceleration {
        pub fn new(
            vertical_acceleration_value: VerticalAccelerationValue,
            vertical_acceleration_confidence: AccelerationConfidence,
        ) -> Self {
            Self {
                vertical_acceleration_value,
                vertical_acceleration_confidence,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DE represents the vehicle acceleration at vertical direction in the centre of the mass of the empty vehicle."]
    #[doc = " * The value shall be provided in the vehicle coordinate system as defined in ISO 8855 [21], clause 2.11."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `-160` for acceleration values equal to or less than -16 m/s^2,"]
    #[doc = " * - `n` (`n > -160` and `n <= 0`) to indicate downwards acceleration equal to or less than n x 0,1 m/s^2, and greater than (n-1) x 0,1 m/s^2,"]
    #[doc = " * - `n` (`n > 0` and `n < 160`) to indicate upwards acceleration equal to or less than n x 0,1 m/s^2, and greater than (n-1) x 0,1 m/s^2,"]
    #[doc = " * - `160` for acceleration values greater than 15,9 m/s^2,"]
    #[doc = " * - `161` when the data is unavailable."]
    #[doc = " * "]
    #[doc = " * @note: The empty load vehicle is defined in ISO 1176 [8], clause 4.6."]
    #[doc = " *"]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @unit: 0,1 m/s^2"]
    #[doc = " * @revision: Desciption updated in V2.1.1 (the meaning of 160 has changed slightly)."]
    #[doc = " *  "]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-160..=161"))]
    pub struct VerticalAccelerationValue(pub i16);
    #[doc = "* "]
    #[doc = " * This DF provides information about a VRU cluster."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field clusterId: optional identifier of a VRU cluster."]
    #[doc = " *"]
    #[doc = " * @field clusterBoundingBoxShape: optionally indicates the shape of the cluster bounding box, per default inside an East-North-Up coordinate system "]
    #[doc = " * centered around a reference point defined outside of the context of this DF."]
    #[doc = " *"]
    #[doc = " * @field clusterCardinalitySize: indicates an estimation of the number of VRUs in the group, e.g. the known members in the cluster + 1 (for the cluster leader) ."]
    #[doc = " *"]
    #[doc = " * @field clusterProfiles: optionally identifies all the VRU profile types that are estimated to be within the cluster."]
    #[doc = " * if this component is absent it means that the information is unavailable. "]
    #[doc = " *"]
    #[doc = " * @category: VRU information"]
    #[doc = " * @revision: Created in V2.1.1, description revised in V2.2.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct VruClusterInformation {
        #[rasn(identifier = "clusterId")]
        pub cluster_id: Option<Identifier1B>,
        #[rasn(value("0.."), identifier = "clusterBoundingBoxShape")]
        pub cluster_bounding_box_shape: Option<Shape>,
        #[rasn(identifier = "clusterCardinalitySize")]
        pub cluster_cardinality_size: CardinalNumber1B,
        #[rasn(identifier = "clusterProfiles")]
        pub cluster_profiles: Option<VruClusterProfiles>,
    }
    impl VruClusterInformation {
        pub fn new(
            cluster_id: Option<Identifier1B>,
            cluster_bounding_box_shape: Option<Shape>,
            cluster_cardinality_size: CardinalNumber1B,
            cluster_profiles: Option<VruClusterProfiles>,
        ) -> Self {
            Self {
                cluster_id,
                cluster_bounding_box_shape,
                cluster_cardinality_size,
                cluster_profiles,
            }
        }
    }
    #[doc = "* "]
    #[doc = " * This DE Identifies all the VRU profile types within a cluster."]
    #[doc = " * It consist of a Bitmap encoding VRU profiles, to allow multiple profiles to be indicated in a single cluster (heterogeneous cluster if more than one profile)."]
    #[doc = " * "]
    #[doc = " * The corresponding bit shall be set to 1 under the following conditions:"]
    #[doc = " * - 0 `pedestrian`  - indicates that the VRU cluster contains at least one pedestrian VRU,"]
    #[doc = " * - 1 `bicycle`     - indicates that the VRU cluster contains at least one bicycle VRU member,"]
    #[doc = " * - 2 `motorcyclist`- indicates that the VRU cluster contains at least one motorcycle VRU member,"]
    #[doc = " * - 3 `animal`      - indicates that the VRU cluster contains at least one animal VRU member."]
    #[doc = " * "]
    #[doc = " * Otherwise, the corresponding bit shall be set to 0."]
    #[doc = " *"]
    #[doc = " * @category: VRU information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct VruClusterProfiles(pub FixedBitString<4usize>);
    #[doc = "*"]
    #[doc = " * This DE represents the possible usage conditions of the VRU device."]
    #[doc = ""]
    #[doc = " * - The value shall be set to:"]
    #[doc = " * - 0 `unavailable`      - to indicate that the usage conditions are unavailable,"]
    #[doc = " * - 1 `other`            - to indicate that the VRU device is in a state not defined below,"]
    #[doc = " * - 2 `idle`             - to indicate that the human is currently not interacting with the device,"]
    #[doc = " * - 3 `listeningToAudio` - to indicate that any audio source other than calling is in use,"]
    #[doc = " * - 4 `typing`           - to indicate that the human is texting or performaing any other manual input activity,"]
    #[doc = " * - 5 `calling`          - to indicate that the VRU device is currently receiving a call,"]
    #[doc = " * - 6 `playingGames`     - to indicate that the human is playing games,"]
    #[doc = " * - 7 `reading`          - to indicate that the human is reading on the VRU device,"]
    #[doc = " * - 8 `viewing`          - to indicate that the human is watching dynamic content, including following navigation prompts, viewing videos or other visual contents that are not static."]
    #[doc = " * - value 9 to 15        - are reserved for future usage. "]
    #[doc = " *"]
    #[doc = " * @category: VRU information"]
    #[doc = " * @revision: Created in V2.1.1, type changed from ENUMERATED to INTEGER in V2.2.1 and range changed from 0..255 to 0..15"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=15"))]
    pub struct VruDeviceUsage(pub u8);
    #[doc = "*"]
    #[doc = " * This DE represents the possible VRU environment conditions."]
    #[doc = " *"]
    #[doc = " * - The value shall be set to:"]
    #[doc = " * - 0 `unavailable`            - to indicate that the information on the type of environment is unavailable,"]
    #[doc = " * - 1 `intersectionCrossing`   - to indicate that the VRU is on an intersection or crossing,"]
    #[doc = " * - 2 `zebraCrossing`          - to indicate that the VRU is on a  zebra crossing (crosswalk),"]
    #[doc = " * - 3 `sidewalk`               - to indicate that the VRU is on a sidewalk,"]
    #[doc = " * - 4 `onVehicleRoad`          - to indicate that the VRU is on a traffic lane,"]
    #[doc = " * - 5 `protectedGeographicArea`- to indicate that the VRU is in a protected area."]
    #[doc = " * - value 6 to 15              - are reserved for future usage. "]
    #[doc = " *"]
    #[doc = " * @category: VRU information"]
    #[doc = " * @revision: Created in V2.1.1, type changed from ENUMERATED to INTEGER in V2.2.1 and range changed from 0..255 to 0..15"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=15"))]
    pub struct VruEnvironment(pub u8);
    #[doc = "*"]
    #[doc = " * This DF represents the status of the exterior light switches of a VRU."]
    #[doc = " * This DF is an extension of the vehicular DE @ref ExteriorLights."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field vehicular:  represents the status of the exterior light switches of a road vehicle."]
    #[doc = " * "]
    #[doc = " * @field vruSpecific: represents the status of the exterior light switches of a VRU."]
    #[doc = " *"]
    #[doc = " * @category: VRU information"]
    #[doc = " * @revision: created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct VruExteriorLights {
        pub vehicular: ExteriorLights,
        #[rasn(identifier = "vruSpecific")]
        pub vru_specific: VruSpecificExteriorLights,
    }
    impl VruExteriorLights {
        pub fn new(vehicular: ExteriorLights, vru_specific: VruSpecificExteriorLights) -> Self {
            Self {
                vehicular,
                vru_specific,
            }
        }
    }
    #[doc = "*"]
    #[doc = " *  This DE indicates the status of the possible human control over a VRU vehicle."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 `unavailable`                 - to indicate that the information is unavailable,"]
    #[doc = " * - 1 `braking`                     - to indicate that the VRU is braking,"]
    #[doc = " * - 2 `hardBraking`                 - to indicate that the VRU is braking hard,"]
    #[doc = " * - 3 `stopPedaling`                - to indicate that the VRU stopped pedaling,"]
    #[doc = " * - 4 `brakingAndStopPedaling`      - to indicate that the VRU stopped pedaling an is braking,"]
    #[doc = " * - 5 `hardBrakingAndStopPedaling`  - to indicate that the VRU stopped pedaling an is braking hard,"]
    #[doc = " * - 6 `noReaction`                  - to indicate that the VRU is not changing its behavior."]
    #[doc = " * - 7 to 15                         - are reserved for future usage. "]
    #[doc = " *"]
    #[doc = " * @category: VRU information"]
    #[doc = " * @revision: Created in V2.1.1, type changed from ENUMERATED to INTEGER in V2.2.1 and range changed from 0..255 to 0..15"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=15"))]
    pub struct VruMovementControl(pub u8);
    #[doc = "*"]
    #[doc = " * This DF indicates the profile of a VRU including sub-profile information"]
    #[doc = " * It identifies four options corresponding to the four types of VRU profiles specified in ETSI TS 103 300-2 [18]:"]
    #[doc = " *"]
    #[doc = " * @field pedestrian: VRU Profile 1 - Pedestrian."]
    #[doc = " *"]
    #[doc = " * @field bicyclistAndLightVruVehicle: VRU Profile  2 - Bicyclist."]
    #[doc = " *"]
    #[doc = " * @field motorcyclist: VRU Profile 3  - Motorcyclist."]
    #[doc = " *"]
    #[doc = " * @field animal: VRU Profile  4 -  Animal."]
    #[doc = " *"]
    #[doc = " * @category: VRU information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum VruProfileAndSubprofile {
        pedestrian(VruSubProfilePedestrian),
        bicyclistAndLightVruVehicle(VruSubProfileBicyclist),
        motorcyclist(VruSubProfileMotorcyclist),
        animal(VruSubProfileAnimal),
    }
    #[doc = "*"]
    #[doc = " * This DE indicates the approximate size of a VRU including the VRU vehicle used."]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 `unavailable`    - to indicate that there is no matched size class or due to privacy reasons in profile 1, "]
    #[doc = " * - 1 `low`            - to indicate that the VRU size class is low depending on the VRU profile,"]
    #[doc = " * - 2 `medium`         - to indicate that the VRU size class is medium depending on the VRU profile,"]
    #[doc = " * - 3 `high`           - to indicate that the VRU size class is high depending on the VRU profile."]
    #[doc = " * - 4 to 15            - are reserved for future usage. "]
    #[doc = " *"]
    #[doc = " * @category: VRU information"]
    #[doc = " * @revision: Created in V2.1.1, type changed from ENUMERATED to INTEGER in V2.2.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=15"))]
    pub struct VruSizeClass(pub u8);
    #[doc = "*"]
    #[doc = " * This DE describes the status of the exterior light switches of a VRU."]
    #[doc = " *"]
    #[doc = " * The value of each bit indicates the state of the switch, which commands the corresponding light. "]
    #[doc = " * The bit corresponding to a specific light shall be set to 1, when the corresponding switch is turned on, either manually by the driver or VRU "]
    #[doc = " * or automatically by a vehicle or VRU system: "]
    #[doc = " * - 0 `unavailable`     - indicates no information available, "]
    #[doc = " * - 1 `backFlashLight ` - indicates the status of the back flash light,"]
    #[doc = " * - 2 `helmetLight`     - indicates the status of the helmet light,"]
    #[doc = " * - 3 `armLight`        - indicates the status of the arm light,"]
    #[doc = " * - 4 `legLight`        - indicates the status of the leg light,"]
    #[doc = " * - 5 `wheelLight`      - indicates the status of the wheel light. "]
    #[doc = " * - Bits 6 to 8         - are reserved for future use. "]
    #[doc = " * The bit values do not indicate if the corresponding lamps are alight or not."]
    #[doc = " * If  VRU is not equipped with a certain light or if the light switch status information is not available, the corresponding bit shall be set to 0."]
    #[doc = " *"]
    #[doc = " * @category: VRU information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct VruSpecificExteriorLights(pub FixedBitString<8usize>);
    #[doc = "*"]
    #[doc = " * This DE indicates the profile of an animal"]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 `unavailable`     - to indicate that the information  is unavailable,"]
    #[doc = " * - 1 `wild-animal`     - to indicate a animal living in the wildness, "]
    #[doc = " * - 2 `farm-animal`     - to indicate an animal beloning to a farm,"]
    #[doc = " * - 3 `service-animal`  - to indicate an animal that supports a human being."]
    #[doc = " * - 4 to 15             - are reserved for future usage. "]
    #[doc = " *"]
    #[doc = " * @category: VRU information"]
    #[doc = " * @revision: Created in V2.1.1, type changed from ENUMERATED to INTEGER in V2.2.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=15"))]
    pub struct VruSubProfileAnimal(pub u8);
    #[doc = "*"]
    #[doc = " * This DE indicates the profile of a VRU and its light VRU vehicle / mounted animal. "]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 `unavailable`           - to indicate that the information  is unavailable,"]
    #[doc = " * - 1 `bicyclist `            - to indicate a cycle and bicyclist to which no more-specific profile applies, "]
    #[doc = " * - 2 `wheelchair-user`       - to indicate a wheelchair and its user,"]
    #[doc = " * - 3 `horse-and-rider`       - to indicate a horse and rider,"]
    #[doc = " * - 4 `rollerskater`          - to indicate a roller-skater and skater,"]
    #[doc = " * - 5 `e-scooter`             - to indicate an e-scooter and rider,"]
    #[doc = " * - 6 `personal-transporter`  - to indicate a personal-transporter and rider,"]
    #[doc = " * - 7 `pedelec`               - to indicate a pedelec and rider to which no more-specific profile applies,"]
    #[doc = " * - 8 `speed-pedelec`         - to indicate a speed-pedelec and rider."]
    #[doc = " * - 9 `roadbike`              - to indicate a road bicycle (or road pedelec) and rider,"]
    #[doc = " * - 10 `childrensbike`        - to indicate a childrenÂ´s bicycle (or childrenÂ´s pedelec) and rider,"]
    #[doc = " * - 11 `racebike`             - to indicate a race bicycle (according to local applicable regulations) and rider. "]
    #[doc = " * - 12 to 15                  - are reserved for future usage. "]
    #[doc = " *"]
    #[doc = " * @category: VRU information"]
    #[doc = " * @revision: Created in V2.1.1, values 9 and 10 assigned in V2.2.1, value 11 assigned in V2.4.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=15"))]
    pub struct VruSubProfileBicyclist(pub u8);
    #[doc = "*"]
    #[doc = " * This DE indicates the profile of a motorcyclist and corresponding vehicle."]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 `unavailable `                  - to indicate that the information  is unavailable,"]
    #[doc = " * - 1 `moped`                         - to indicate a moped and rider,"]
    #[doc = " * - 2 `motorcycle`                    - to indicate a motorcycle and rider,"]
    #[doc = " * - 3 `motorcycle-and-sidecar-right`  - to indicate a motorcycle with sidecar on the right and rider,"]
    #[doc = " * - 4 `motorcycle-and-sidecar-left`   - to indicate  a motorcycle with sidecar on the left and rider."]
    #[doc = " * - 5 to 15                           - are reserved for future usage. "]
    #[doc = " *"]
    #[doc = " * @category: VRU information"]
    #[doc = " * @revision: Created in V2.1.1, type changed from ENUMERATED to INTEGER in V2.2.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=15"))]
    pub struct VruSubProfileMotorcyclist(pub u8);
    #[doc = "*"]
    #[doc = " * This DE indicates the profile of a pedestrian."]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 `unavailable`             - to indicate that the information on is unavailable,"]
    #[doc = " * - 1 `ordinary-pedestrian`     - to indicate a pedestrian to which no more-specific profile applies,"]
    #[doc = " * - 2 `road-worker`             - to indicate a pedestrian with the role of a road worker,"]
    #[doc = " * - 3 `first-responder`         - to indicate a pedestrian with the role of a first responder."]
    #[doc = " * - value 4 to 15               - are reserved for future usage. "]
    #[doc = " *"]
    #[doc = " * @category: VRU information"]
    #[doc = " * @revision: Created in V2.1.1, type changed from ENUMERATED to INTEGER in V2.2.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=15"))]
    pub struct VruSubProfilePedestrian(pub u8);
    #[doc = "*"]
    #[doc = " * This DE represents the World Manufacturer Identifier (WMI). The values are assigned according to ISO 3779 [6]."]
    #[doc = " * "]
    #[doc = " *"]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: V1.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=3"))]
    pub struct WMInumber(pub Ia5String);
    #[doc = "* "]
    #[doc = " * This DF represents an angular component along with a confidence value in the WGS84 coordinate system."]
    #[doc = " * The specific WGS84 coordinate system is specified by the corresponding standards applying this DE."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field value: the angle value, which can be estimated as the mean of the current distribution."]
    #[doc = " *"]
    #[doc = " * @field confidence: the confidence value associated to the angle value."]
    #[doc = " *"]
    #[doc = " * @category: GeoReference information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct Wgs84Angle {
        pub value: Wgs84AngleValue,
        pub confidence: Wgs84AngleConfidence,
    }
    impl Wgs84Angle {
        pub fn new(value: Wgs84AngleValue, confidence: Wgs84AngleConfidence) -> Self {
            Self { value, confidence }
        }
    }
    #[doc = "* "]
    #[doc = " * This DE indicates the angle confidence value which represents the estimated accuracy of an angle value with a default confidence level of 95 %."]
    #[doc = " * If required, the confidence level can be defined by the corresponding standards applying this DE."]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `n` (`n >= 1` and `n < 126`) if the confidence value is equal to or less than n x 0,1 degrees and more than (n-1) x 0,1 degrees,"]
    #[doc = " * - `126` if the confidence value is out of range, i.e. greater than 12,5 degrees,"]
    #[doc = " * - `127` if the confidence value is not available."]
    #[doc = " *"]
    #[doc = " *"]
    #[doc = " * @unit 0,1 degrees"]
    #[doc = " * @category: GeoReference Information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=127"))]
    pub struct Wgs84AngleConfidence(pub u8);
    #[doc = "* "]
    #[doc = " * This DE represents an angle value in degrees described in the WGS84 reference system with respect to the WGS84 north."]
    #[doc = " * The specific WGS84 coordinate system is specified by the corresponding standards applying this DE."]
    #[doc = " * When the information is not available, the DE shall be set to 3 601. The value 3600 shall not be used."]
    #[doc = " *"]
    #[doc = " * @unit 0,1 degrees"]
    #[doc = " * @category: GeoReference Information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=3601"))]
    pub struct Wgs84AngleValue(pub u16);
    #[doc = "*"]
    #[doc = " * This DE indicates the perpendicular distance between front and rear axle of the wheel base of vehicle."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `n` (`n >= 1` and `n < 126`) if the value is equal to or less than n x 0,1 metre  and more than (n-1) x 0,1 metre,"]
    #[doc = " * - `126` indicates that the wheel base distance is equal to or greater than 12,5 metres,"]
    #[doc = " * - `127` indicates that the information is unavailable."]
    #[doc = " *"]
    #[doc = " * @unit 0,1 metre"]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: Created in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=127"))]
    pub struct WheelBaseVehicle(pub u8);
    #[doc = "*"]
    #[doc = " * This DE indicates the actual status of the front wipers of the vehicle. "]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 `unavailable`   - to indicate that the information is unavailable,"]
    #[doc = " * - 1 `off`           - to indicate that the wipers are switched off,"]
    #[doc = " * - 2 `intermittent`  - to indicate that the wipers are moving intermittently,"]
    #[doc = " * - 3 `low`           - to indicate that the wipers are moving at low speed,"]
    #[doc = " * - 4 `high`          - to indicate that the wipers are moving at high speed,"]
    #[doc = " * - values 5 to 7      - are reserved for future usage. "]
    #[doc = " *"]
    #[doc = " * @note:  the status can be either set manually by the driver or automatically by e.g. a rain sensor. The way the status is set does not affect the status itself. "]
    #[doc = " * @category: Vehicle information "]
    #[doc = " * @revision: created in V2.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=7"))]
    pub struct WiperStatus(pub u8);
    #[doc = "*"]
    #[doc = " * This DE represents the sub cause codes of the @ref CauseCode `wrongWayDriving` ."]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - 0 `unavailable`           - in case further detailed information on wrong way driving event is unavailable,"]
    #[doc = " * - 1 `wrongLane-deprecated`   - deprecated since not pertinent to wrong way driving,"]
    #[doc = " * - 2 `wrongDirection`        - in case vehicle is driving in a direction that it is not allowed,"]
    #[doc = " * - 3-255                     - reserved for future usage."]
    #[doc = " * "]
    #[doc = " * @category: Traffic information"]
    #[doc = " * @revision: V1.3.1, value 1 deprecated in V2.4.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct WrongWayDrivingSubCauseCode(pub u8);
    #[doc = "*"]
    #[doc = " * This DF represents a yaw rate of vehicle at a point in time."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " * "]
    #[doc = " * @field yawRateValue: yaw rate value at a point in time."]
    #[doc = " * "]
    #[doc = " * @field yawRateConfidence: confidence value associated to the yaw rate value."]
    #[doc = " * "]
    #[doc = " * @category: Vehicle Information"]
    #[doc = " * @revision: V1.3.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct YawRate {
        #[rasn(identifier = "yawRateValue")]
        pub yaw_rate_value: YawRateValue,
        #[rasn(identifier = "yawRateConfidence")]
        pub yaw_rate_confidence: YawRateConfidence,
    }
    impl YawRate {
        pub fn new(yaw_rate_value: YawRateValue, yaw_rate_confidence: YawRateConfidence) -> Self {
            Self {
                yaw_rate_value,
                yaw_rate_confidence,
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DE indicates the yaw rate confidence value which represents the estimated accuracy for a yaw rate value with a default confidence level of 95 %."]
    #[doc = " * If required, the confidence level can be defined by the corresponding standards applying this DE."]
    #[doc = " * "]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `0` if the confidence value is equal to or less than 0,01 degree/second,"]
    #[doc = " * - `1` if the confidence value is equal to or less than 0,05 degrees/second or greater than 0,01 degree/second,"]
    #[doc = " * - `2` if the confidence value is equal to or less than 0,1 degree/second or greater than 0,05 degree/second,"]
    #[doc = " * - `3` if the confidence value is equal to or less than 1 degree/second or greater than 0,1 degree/second,"]
    #[doc = " * - `4` if the confidence value is equal to or less than 5 degrees/second or greater than 1 degrees/second,"]
    #[doc = " * - `5` if the confidence value is equal to or less than 10 degrees/second or greater than 5 degrees/second,"]
    #[doc = " * - `6` if the confidence value is equal to or less than 100 degrees/second or greater than 10 degrees/second,"]
    #[doc = " * - `7` if the confidence value is out of range, i.e. greater than 100 degrees/second,"]
    #[doc = " * - `8` if the confidence value is unavailable."]
    #[doc = " * "]
    #[doc = " * NOTE: The fact that a yaw rate value is received with confidence value set to `unavailable(8)` can be caused by"]
    #[doc = " * several reasons, such as:"]
    #[doc = " * - the sensor cannot deliver the accuracy at the defined confidence level because it is a low-end sensor,"]
    #[doc = " * - the sensor cannot calculate the accuracy due to lack of variables, or"]
    #[doc = " * - there has been a vehicle bus (e.g. CAN bus) error."]
    #[doc = " * In all 3 cases above, the yaw rate value may be valid and used by the application."]
    #[doc = " * "]
    #[doc = " * If a yaw rate value is received and its confidence value is set to `outOfRange(7)`, it means that the "]
    #[doc = " * yaw rate value is not valid and therefore cannot be trusted. Such value is not useful the application."]
    #[doc = " * "]
    #[doc = " * @category: Vehicle information"]
    #[doc = " * @revision: Description revised in V2.1.1"]
    #[doc = " "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum YawRateConfidence {
        #[rasn(identifier = "degSec-000-01")]
        degSec_000_01 = 0,
        #[rasn(identifier = "degSec-000-05")]
        degSec_000_05 = 1,
        #[rasn(identifier = "degSec-000-10")]
        degSec_000_10 = 2,
        #[rasn(identifier = "degSec-001-00")]
        degSec_001_00 = 3,
        #[rasn(identifier = "degSec-005-00")]
        degSec_005_00 = 4,
        #[rasn(identifier = "degSec-010-00")]
        degSec_010_00 = 5,
        #[rasn(identifier = "degSec-100-00")]
        degSec_100_00 = 6,
        outOfRange = 7,
        unavailable = 8,
    }
    #[doc = "*"]
    #[doc = " * This DE represents the vehicle rotation around z-axis of the coordinate system centred on the centre of mass of the empty-loaded"]
    #[doc = " * vehicle. The leading sign denotes the direction of rotation."]
    #[doc = " * "]
    #[doc = " * The value shall be provided in the vehicle coordinate system as defined in ISO 8855 [21], clause 2.11."]
    #[doc = " *"]
    #[doc = " * The value shall be set to:"]
    #[doc = " * - `-32 766` to indicate that the yaw rate is equal to or greater than 327,66 degrees/second to the right,"]
    #[doc = " * - `n` (`n > -32 766` and `n <= 0`) to indicate that the rotation is clockwise (i.e. to the right) and is equal to or less than n x 0,01 degrees/s, "]
    #[doc = "      and greater than (n-1) x 0,01 degrees/s,"]
    #[doc = " * - `n` (`n > 0` and `n < 32 766`) to indicate that the rotation is anti-clockwise (i.e. to the left) and is equal to or less than n x 0,01 degrees/s, "]
    #[doc = "      and greater than (n-1) x 0,01 degrees/s,"]
    #[doc = " * - `32 766` to indicate that the yaw rate is greater than 327.65 degrees/second to the left,"]
    #[doc = " * - `32 767` to indicate that the information is not available."]
    #[doc = " * "]
    #[doc = " * The reading instant should be the same as for the vehicle acceleration."]
    #[doc = " * "]
    #[doc = " * @note: The empty load vehicle is defined in ISO 1176 [8], clause 4.6."]
    #[doc = " * "]
    #[doc = " * @unit: 0,01 degree per second. "]
    #[doc = " * @category: Vehicle Information"]
    #[doc = " * @revision: Description revised in V2.1.1 (the meaning of 32766 has changed slightly). Requirement on raw data deleted in V2.4.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-32766..=32767"))]
    pub struct YawRateValue(pub i16);
}
