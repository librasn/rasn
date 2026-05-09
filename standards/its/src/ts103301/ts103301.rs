#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod etsi_its_dsrc {
    extern crate alloc;
    use super::etsi_its_cdd::{Iso3833VehicleType, Latitude, Longitude, StationID};
    use super::etsi_its_dsrc_region::{
        RegAdvisorySpeed, RegComputedLane, RegConnectionManeuverAssist, RegGenericLane,
        RegIntersectionGeometry, RegIntersectionState, RegLaneAttributes, RegLaneDataAttribute,
        RegMapData, RegMovementEvent, RegMovementState, RegNodeAttributeSetXY,
        RegNodeOffsetPointXY, RegPosition3D, RegRTCMcorrections, RegRequestorDescription,
        RegRequestorType, RegRestrictionUserType, RegRoadSegment, RegSPAT, RegSignalControlZone,
        RegSignalRequest, RegSignalRequestMessage, RegSignalRequestPackage, RegSignalStatus,
        RegSignalStatusMessage, RegSignalStatusPackage,
    };
    use core::borrow::Borrow;
    use lazy_static::lazy_static;
    use rasn::prelude::*;
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousAdvisorySpeedRegional {
        #[rasn(value("0..=255"), identifier = "regionId")]
        pub region_id: u8,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousAdvisorySpeedRegional {
        pub fn new(region_id: u8, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    impl AnonymousAdvisorySpeedRegional {
        pub fn decode_reg_ext_value<D: Decoder>(
            &self,
            decoder: &mut D,
        ) -> Result<RegAdvisorySpeed_Type, D::Error> {
            RegAdvisorySpeed_Type::decode(decoder, Some(&self.reg_ext_value), &self.region_id)
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct AdvisorySpeedRegional(pub SequenceOf<AnonymousAdvisorySpeedRegional>);
    #[doc = "*"]
    #[doc = "* This DF is used to convey a recommended traveling approach speed to an intersection"]
    #[doc = "* from the message issuer to various travelers and vehicle types. Besides support for various eco-driving applications, this"]
    #[doc = "* allows transmitting recommended speeds for specialty vehicles such as transit buses."]
    #[doc = "*"]
    #[doc = "* @field type: the type of advisory which this is."]
    #[doc = "* @field speed: See @ref SpeedAdvice for converting and translating speed expressed in mph into units of m/s."]
    #[doc = "*               This element is optional ONLY when superceded by the presence of a regional speed element found in Reg-AdvisorySpeed entry"]
    #[doc = "* @field confidence: A confidence value for the above speed"]
    #[doc = "* @field distance: The distance indicates the region for which the advised speed is recommended, it is specified upstream from the stop bar"]
    #[doc = "*                  along the connected egressing lane. Unit = 1 meter "]
    #[doc = "* @field class: the vehicle types to which it applies when absent, the AdvisorySpeed applies to all motor vehicle types"]
    #[doc = "* @field regional: optional region specific data."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct AdvisorySpeed {
        #[rasn(identifier = "type")]
        pub r_type: AdvisorySpeedType,
        pub speed: Option<SpeedAdvice>,
        pub confidence: Option<SpeedConfidenceDSRC>,
        pub distance: Option<ZoneLength>,
        pub class: Option<RestrictionClassID>,
        pub regional: Option<AdvisorySpeedRegional>,
    }
    impl AdvisorySpeed {
        pub fn new(
            r_type: AdvisorySpeedType,
            speed: Option<SpeedAdvice>,
            confidence: Option<SpeedConfidenceDSRC>,
            distance: Option<ZoneLength>,
            class: Option<RestrictionClassID>,
            regional: Option<AdvisorySpeedRegional>,
        ) -> Self {
            Self {
                r_type,
                speed,
                confidence,
                distance,
                class,
                regional,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* This DF consists of a list of @ref AdvisorySpeed entries."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16"))]
    pub struct AdvisorySpeedList(pub SequenceOf<AdvisorySpeed>);
    #[doc = "*"]
    #[doc = "* This DE relates the type of travel to which a given speed refers. This element is"]
    #[doc = "* typically used as part of an @ref AdvisorySpeed data frame for signal phase and timing data."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum AdvisorySpeedType {
        none = 0,
        greenwave = 1,
        ecoDrive = 2,
        transit = 3,
    }
    #[doc = "*"]
    #[doc = "* This DE relates the allowed (possible) maneuvers from a lane, typically a"]
    #[doc = "* motorized vehicle lane. It should be noted that in practice these values may be further restricted by vehicle class, local"]
    #[doc = "* regulatory environment and other changing conditions."]
    #[doc = "*"]
    #[doc = "* @note: When used by data frames, the AllowedManeuvers data concept is used in two places: optionally in the"]
    #[doc = "*    generic lane structure to list all possible maneuvers (as in what that lane can do at its stop line point); and within each"]
    #[doc = "*    ConnectsTo structure. Each ConnectsTo structure contains a list used to provide a single valid maneuver in the context of"]
    #[doc = "*    one lane connecting to another in the context of a signal phase that applies to that maneuver. It should be noted that, in"]
    #[doc = "*    some intersections, multiple outbound lanes can be reached by the same maneuver (for example two independent left"]
    #[doc = "*    turns might be found in a 5-legged intersection) but that to reach any given lane from the stop line of another lane is"]
    #[doc = "*    always a single maneuver item (hence the use of a list). Not all intersection descriptions may contain an exhaustive set of"]
    #[doc = "*    ConnectsTo information (unsignalized intersections for example) and in such cases the AllowedManeuvers in the generic"]
    #[doc = "*    lane structure can be used. If present in both places, the data expressed in the generic lane shall not conflict with the data"]
    #[doc = "*    found in the collection of ConnectsTo entries."]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct AllowedManeuvers(pub FixedBitString<12usize>);
    #[doc = "*"]
    #[doc = "* This DE is used to describe an angular measurement in units of degrees. This data"]
    #[doc = "* element is often used as a heading direction when in motion. In this use, the current heading of the sending device is"]
    #[doc = "* expressed in unsigned units of 0.0125 degrees from North, such that 28799 such degrees represent 359.9875 degrees."]
    #[doc = "* North shall be defined as the axis defined by the WGS-84 coordinate system and its reference ellipsoid. Any angle \"to the"]
    #[doc = "* east\" is defined as the positive direction. A value of 28800 shall be used when Angle is unavailable."]
    #[doc = "*"]
    #[doc = "* @note: Note that other heading and angle data elements of various sizes and precisions are found in other parts of this standard and in ITS."]
    #[doc = "* @unit: 0.0125 degrees"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=28800"))]
    pub struct Angle(pub u16);
    #[doc = "*"]
    #[doc = "* This DF is a collection of three offset values in an orthogonal coordinate system"]
    #[doc = "* which describe how far the electrical phase center of an antenna is in each axis from a nearby known anchor point in units of 1 cm."]
    #[doc = "*"]
    #[doc = "* When the antenna being described is on a vehicle, the signed offset shall be in the coordinate system defined in section 11.4."]
    #[doc = "*"]
    #[doc = "* @field antOffsetX: a range of +- 20.47 meters."]
    #[doc = "* @field antOffsetY: a range of +- 2.55 meters."]
    #[doc = "* @field antOffsetZ: a range of +- 5.11 meters."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct AntennaOffsetSet {
        #[rasn(identifier = "antOffsetX")]
        pub ant_offset_x: OffsetB12,
        #[rasn(identifier = "antOffsetY")]
        pub ant_offset_y: OffsetB09,
        #[rasn(identifier = "antOffsetZ")]
        pub ant_offset_z: OffsetB10,
    }
    impl AntennaOffsetSet {
        pub fn new(
            ant_offset_x: OffsetB12,
            ant_offset_y: OffsetB09,
            ant_offset_z: OffsetB10,
        ) -> Self {
            Self {
                ant_offset_x,
                ant_offset_y,
                ant_offset_z,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* This DE is used to relate the index of an approach, either ingress or egress within the"]
    #[doc = "* subject lane. In general, an approach index in the context of a timing movement is not of value in the MAP and SPAT"]
    #[doc = "* process because the lane ID and signal group ID concepts handle this with more precision. This value can also be useful"]
    #[doc = "* as an aid as it can be used to indicate the gross position of a moving object (vehicle) when its lane level accuracy is"]
    #[doc = "* unknown. This value can also be used when a deployment represents sets of lanes as groups without further details (as is"]
    #[doc = "* done in Japan)."]
    #[doc = "*"]
    #[doc = "* @note: zero to be used when valid value is unknown"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=15"))]
    pub struct ApproachID(pub u8);
    #[doc = "*"]
    #[doc = "* This DE provides a means to indicate the current role that a DSRC device is playing"]
    #[doc = "* This is most commonly employed when a vehicle needs to take on another role in order to send certain DSRC message"]
    #[doc = "* types. As an example, when a public safety vehicle such as a police car wishes to send a signal request message (SRM)"]
    #[doc = "* to an intersection to request a preemption service, the vehicle takes on the role \"police\" from the below list in both the"]
    #[doc = "* SRM message itself and also in the type of security CERT which is sent (the SSP in the CERT it used to identify the"]
    #[doc = "* requester as being of type \"police\" and that they are allowed to send this message in this way). The BasicVehicleRole"]
    #[doc = "* entry is often used and combined with other information about the requester as well, such as details of why the request is"]
    #[doc = "* being made."]
    #[doc = "*"]
    #[doc = "* - 0 - `basicVehicle`     - Light duty passenger vehicle type"]
    #[doc = "* - 1 - `publicTransport`  - Used in EU for Transit us"]
    #[doc = "* - 2 - `specialTransport` - Used in EU (e.g. heavy load)"]
    #[doc = "* - 3 - `dangerousGoods`   - Used in EU for any HAZMAT"]
    #[doc = "* - 4 - `roadWork`         - Used in EU for State and Local DOT uses"]
    #[doc = "* - 5 - `roadRescue`       - Used in EU and in the US to include tow trucks."]
    #[doc = "* - 6 - `emergency`        - Used in EU for Police, Fire and Ambulance units"]
    #[doc = "* - 7 - `safetyCar`        - Used in EU for Escort vehicles"]
    #[doc = "* - 8 - `none-unknown`     - added to follow current SAE style guidelines"]
    #[doc = "* - 9 - `truck`            - Heavy trucks with additional BSM rights and obligations"]
    #[doc = "* - 10 - `motorcycle`      - Motorcycle"]
    #[doc = "* - 11 - `roadSideSource`  - For infrastructure generated calls such as fire house, rail infrastructure, roadwork site, etc."]
    #[doc = "* - 12 - `police`          - Police vehicle"]
    #[doc = "* - 13 - `fire`            - Firebrigade"]
    #[doc = "* - 14 - `ambulance`       - (does not include private para-transit etc.)"]
    #[doc = "* - 15 - `dot`             - all roadwork vehicles"]
    #[doc = "* - 16 - `transit`         - all transit vehicles"]
    #[doc = "* - 17 - `slowMoving`      - to also include oversize etc."]
    #[doc = "* - 18 - `stopNgo`         - to include trash trucks, school buses and others"]
    #[doc = "* - 19 - `cyclist`         - bicycles"]
    #[doc = "* - 20 - `pedestrian`      - also includes those with mobility limitations"]
    #[doc = "* - 21 - `nonMotorized`    - other, horse drawn, etc."]
    #[doc = "* - 22 - `military`        - military vehicles"]
    #[doc = "*"]
    #[doc = "* @note: It should be observed that devices can at times change their roles (i.e. a fire operated by a volunteer"]
    #[doc = "*    fireman can assume a fire role for a period of time when in service, or a pedestrian may assume a cyclist role when using"]
    #[doc = "*    a bicycle). It should be observed that not all devices (or vehicles) can assume all roles, nor that a given"]
    #[doc = "*    device in a given role will be provided with a security certificate (CERT) that has suitable SSP credentials to provide the"]
    #[doc = "*    ability to send a particular message or message content. The ultimate responsibility to determine what role is to be used,"]
    #[doc = "*    and what CERTs would be provided for that role (which in turn controls the messages and message content that can be"]
    #[doc = "*    sent within SAE-defined PSIDs) rests with the regional deployment."]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum BasicVehicleRole {
        basicVehicle = 0,
        publicTransport = 1,
        specialTransport = 2,
        dangerousGoods = 3,
        roadWork = 4,
        roadRescue = 5,
        emergency = 6,
        safetyCar = 7,
        #[rasn(identifier = "none-unknown")]
        none_unknown = 8,
        truck = 9,
        motorcycle = 10,
        roadSideSource = 11,
        police = 12,
        fire = 13,
        ambulance = 14,
        dot = 15,
        transit = 16,
        slowMoving = 17,
        stopNgo = 18,
        cyclist = 19,
        pedestrian = 20,
        nonMotorized = 21,
        military = 22,
        #[rasn(extension_addition)]
        tram = 23,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum ComputedLaneOffsetXaxis {
        small(DrivenLineOffsetSm),
        large(DrivenLineOffsetLg),
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum ComputedLaneOffsetYaxis {
        small(DrivenLineOffsetSm),
        large(DrivenLineOffsetLg),
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousComputedLaneRegional {
        #[rasn(value("0..=255"), identifier = "regionId")]
        pub region_id: u8,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousComputedLaneRegional {
        pub fn new(region_id: u8, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    impl AnonymousComputedLaneRegional {
        pub fn decode_reg_ext_value<D: Decoder>(
            &self,
            decoder: &mut D,
        ) -> Result<RegComputedLane_Type, D::Error> {
            RegComputedLane_Type::decode(decoder, Some(&self.reg_ext_value), &self.region_id)
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct ComputedLaneRegional(pub SequenceOf<AnonymousComputedLaneRegional>);
    #[doc = "*"]
    #[doc = "* This DE is used to contain information needed to compute one lane from another"]
    #[doc = "* (hence the name). This concept is used purely as a means of saving size in the message payload. The new lane is"]
    #[doc = "* expressed as an X,Y offset from the first point of the source lane. It can be optionally rotated and scaled. Any attribute"]
    #[doc = "* information found within the node of the source lane list cannot be changed and must be reused."]
    #[doc = "*"]
    #[doc = "* @field referenceLaneId: the lane ID upon which this computed lane will be based Lane Offset in X and Y direction"]
    #[doc = "* @field offsetXaxis: A path X offset value for translations of the path's points when creating translated lanes."]
    #[doc = "* @field offsetYaxis: The values found in the reference lane are all offset based on the X and Y values from"]
    #[doc = "*                     the coordinates of the reference lane's initial path point."]
    #[doc = "* @field rotateXY: A path rotation value for the entire lane"]
    #[doc = "*                  Observe that this rotates the existing orientation"]
    #[doc = "*                  of the referenced lane, it does not replace it."]
    #[doc = "*                  Rotation occurs about the initial path point."]
    #[doc = "* @field scaleXaxis: value for translations or zooming of the path's points. The values found in the reference lane"]
    #[doc = "* @field scaleYaxis: are all expanded or contracted based on the X and Y and width values from the coordinates of  the reference lane's initial path point."]
    #[doc = "*                    The Z axis remains untouched."]
    #[doc = "* @field regional: optional region specific data."]
    #[doc = "*"]
    #[doc = "* @note: The specified transformation shall be applied to the reference lane without any intermediary loss of precision"]
    #[doc = "*        (truncation). The order of the transformations shall be: the East-West and North-South offsets, the scaling factors, and"]
    #[doc = "*        finally the rotation."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ComputedLane {
        #[rasn(identifier = "referenceLaneId")]
        pub reference_lane_id: LaneID,
        #[rasn(identifier = "offsetXaxis")]
        pub offset_xaxis: ComputedLaneOffsetXaxis,
        #[rasn(identifier = "offsetYaxis")]
        pub offset_yaxis: ComputedLaneOffsetYaxis,
        #[rasn(identifier = "rotateXY")]
        pub rotate_xy: Option<Angle>,
        #[rasn(identifier = "scaleXaxis")]
        pub scale_xaxis: Option<ScaleB12>,
        #[rasn(identifier = "scaleYaxis")]
        pub scale_yaxis: Option<ScaleB12>,
        pub regional: Option<ComputedLaneRegional>,
    }
    impl ComputedLane {
        pub fn new(
            reference_lane_id: LaneID,
            offset_xaxis: ComputedLaneOffsetXaxis,
            offset_yaxis: ComputedLaneOffsetYaxis,
            rotate_xy: Option<Angle>,
            scale_xaxis: Option<ScaleB12>,
            scale_yaxis: Option<ScaleB12>,
            regional: Option<ComputedLaneRegional>,
        ) -> Self {
            Self {
                reference_lane_id,
                offset_xaxis,
                offset_yaxis,
                rotate_xy,
                scale_xaxis,
                scale_yaxis,
                regional,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* The data concept ties a single lane to a single maneuver needed to reach it from another lane."]
    #[doc = "* It is typically used to connect the allowed maneuver from the end of a lane to the outbound lane so that these can be"]
    #[doc = "* mapped to the SPAT message to which both lanes apply."]
    #[doc = "*"]
    #[doc = "* @field lane: Index of the connecting lane."]
    #[doc = "*"]
    #[doc = "* @field maneuver: This data element allows only the description of a subset of possible manoeuvres and therefore"]
    #[doc = "*    represents an incomplete list of possible travel directions. The connecting **lane** data element gives the"]
    #[doc = "*    exact information about the manoeuvre relation from ingress to egress lane. Therefore the \"maneuver\""]
    #[doc = "*    data element may be used only additionally if the travel direction of the manoeuvre is unanmbigoulsy"]
    #[doc = "*    represented (e.g. left, right, straight, etc.)."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct ConnectingLane {
        pub lane: LaneID,
        pub maneuver: Option<AllowedManeuvers>,
    }
    impl ConnectingLane {
        pub fn new(lane: LaneID, maneuver: Option<AllowedManeuvers>) -> Self {
            Self { lane, maneuver }
        }
    }
    #[doc = "*"]
    #[doc = "* This DF is used to combine/connect multiple physical lanes (i.e. within intersections or road"]
    #[doc = "* segments). For signalized movements, the **connectsTo** data frame defines e.g. the relation between"]
    #[doc = "* ingress and egress lanes within an intersection. It describes the allowed manoeuvres and includes the"]
    #[doc = "* link (**signalGroup** identifier) between the @ref MapData and the @ref SPAT message. The data frame is also used"]
    #[doc = "* to describe the relation of lanes within a non signalized intersection (e.g. ingress lanes which are"]
    #[doc = "* bypassing the conflict area and ending in an egress lane without signalization). Within a road segment,"]
    #[doc = "* it is used to combine two or multiple physical lanes into a single lane object."]
    #[doc = "*"]
    #[doc = "* @field connectingLane: "]
    #[doc = "* @field remoteIntersection: When the data element **remoteIntersection** is used, it indicates"]
    #[doc = "*                            that the connecting lane belongs to another intersection. "]
    #[doc = "*                            (see clause [ISO TS 19091] G.9.1 for further explainations)."]
    #[doc = "* @field signalGroup: "]
    #[doc = "* @field userClass: "]
    #[doc = "* @field connectionID: "]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct Connection {
        #[rasn(identifier = "connectingLane")]
        pub connecting_lane: ConnectingLane,
        #[rasn(identifier = "remoteIntersection")]
        pub remote_intersection: Option<IntersectionReferenceID>,
        #[rasn(identifier = "signalGroup")]
        pub signal_group: Option<SignalGroupID>,
        #[rasn(identifier = "userClass")]
        pub user_class: Option<RestrictionClassID>,
        #[rasn(identifier = "connectionID")]
        pub connection_id: Option<LaneConnectionID>,
    }
    impl Connection {
        pub fn new(
            connecting_lane: ConnectingLane,
            remote_intersection: Option<IntersectionReferenceID>,
            signal_group: Option<SignalGroupID>,
            user_class: Option<RestrictionClassID>,
            connection_id: Option<LaneConnectionID>,
        ) -> Self {
            Self {
                connecting_lane,
                remote_intersection,
                signal_group,
                user_class,
                connection_id,
            }
        }
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousConnectionManeuverAssistRegional {
        #[rasn(value("0..=255"), identifier = "regionId")]
        pub region_id: u8,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousConnectionManeuverAssistRegional {
        pub fn new(region_id: u8, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    impl AnonymousConnectionManeuverAssistRegional {
        pub fn decode_reg_ext_value<D: Decoder>(
            &self,
            decoder: &mut D,
        ) -> Result<RegConnectionManeuverAssist_Type, D::Error> {
            RegConnectionManeuverAssist_Type::decode(
                decoder,
                Some(&self.reg_ext_value),
                &self.region_id,
            )
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct ConnectionManeuverAssistRegional(
        pub SequenceOf<AnonymousConnectionManeuverAssistRegional>,
    );
    #[doc = "*"]
    #[doc = "* This DF contains information about the the dynamic flow of traffic for the lane(s)"]
    #[doc = "* and maneuvers in question (as determined by the @ref LaneConnectionID). Note that this information can be sent regarding"]
    #[doc = "* any lane-to-lane movement; it need not be limited to the lanes with active (non-red) phases when sent."]
    #[doc = "*"]
    #[doc = "* @field connectionID: the common connectionID used by all lanes to which this data applies"]
    #[doc = "*                     (this value traces to ConnectsTo entries in lanes)"]
    #[doc = "*"]
    #[doc = "* @field queueLength: The distance from the stop line to the back edge of the last vehicle in the queue,"]
    #[doc = "*                     as measured along the lane center line. (Unit = 1 meter, 0 = no queue)"]
    #[doc = "*"]
    #[doc = "* @field availableStorageLength: Distance (e.g. beginning from the downstream stop-line up to a given distance) with a high"]
    #[doc = "*                     probability for successfully executing the connecting maneuver between the two lanes"]
    #[doc = "*                     during the current cycle."]
    #[doc = "*                     Used for enhancing the awareness of vehicles to anticipate if they can pass the stop line"]
    #[doc = "*                     of the lane. Used for optimizing the green wave, due to knowledge of vehicles waiting in front"]
    #[doc = "*                     of a red light (downstream)."]
    #[doc = "*                     The element nextTime in @ref TimeChangeDetails in the containing data frame contains the next"]
    #[doc = "*                     timemark at which an active phase is expected, a form of storage flush interval."]
    #[doc = "*                     (Unit = 1 meter, 0 = no space remains)"]
    #[doc = "*"]
    #[doc = "* @field waitOnStop:  If true, the vehicles on this specific connecting"]
    #[doc = "*                     maneuver have to stop on the stop-line and not to enter the collision area"]
    #[doc = "*"]
    #[doc = "* @field pedBicycleDetect: true if ANY ped or bicycles are detected crossing the above lanes. Set to false ONLY if there is a"]
    #[doc = "*                     high certainty that there are none present, otherwise element is not sent."]
    #[doc = "*"]
    #[doc = "* @field regional:    This data element includes additional data content @ref ConnectionManeuverAssist-addGrpC defined in"]
    #[doc = "*                     this profile (see [ISO TS 19091] G.5.1.1). The content is included using the regional extension framework as defined in"]
    #[doc = "*                     @ref ConnectionManeuverAssist-addGrpC is used for position feedback to moving ITS stations for executing"]
    #[doc = "*                     safe manoeuvres and is included for this purpose in the data element \"intersectionState\""]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ConnectionManeuverAssist {
        #[rasn(identifier = "connectionID")]
        pub connection_id: LaneConnectionID,
        #[rasn(identifier = "queueLength")]
        pub queue_length: Option<ZoneLength>,
        #[rasn(identifier = "availableStorageLength")]
        pub available_storage_length: Option<ZoneLength>,
        #[rasn(identifier = "waitOnStop")]
        pub wait_on_stop: Option<WaitOnStopline>,
        #[rasn(identifier = "pedBicycleDetect")]
        pub ped_bicycle_detect: Option<PedestrianBicycleDetect>,
        pub regional: Option<ConnectionManeuverAssistRegional>,
    }
    impl ConnectionManeuverAssist {
        pub fn new(
            connection_id: LaneConnectionID,
            queue_length: Option<ZoneLength>,
            available_storage_length: Option<ZoneLength>,
            wait_on_stop: Option<WaitOnStopline>,
            ped_bicycle_detect: Option<PedestrianBicycleDetect>,
            regional: Option<ConnectionManeuverAssistRegional>,
        ) -> Self {
            Self {
                connection_id,
                queue_length,
                available_storage_length,
                wait_on_stop,
                ped_bicycle_detect,
                regional,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* This DF is used in the generic lane descriptions to provide a sequence of other defined"]
    #[doc = "* lanes to which each lane connects beyond its stop point. See the Connection data frame entry for details. Note that this"]
    #[doc = "* data frame is not used in some lane object types."]
    #[doc = "*"]
    #[doc = "* @note: The assignment of lanes in the Connection structure shall start with the leftmost lane from the vehicle"]
    #[doc = "*   perspective (the u-turn lane in some cases) followed by subsequent lanes in a clockwise assignment order. Therefore, the"]
    #[doc = "*   rightmost lane to which this lane connects would always be listed last. Note that this order is observed regardless of which"]
    #[doc = "*   side of the road vehicles use. If this structure is used in the lane description, then all valid lanes to which the subject lane"]
    #[doc = "*   connects shall be listed."]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16"))]
    pub struct ConnectsToList(pub SequenceOf<Connection>);
    #[doc = "*"]
    #[doc = "* The DSRC style date is a compound value consisting of finite-length sequences of integers (not characters) of the"]
    #[doc = "* form: \"yyyy, mm, dd, hh, mm, ss (sss+)\" - as defined below."]
    #[doc = "*"]
    #[doc = "* @note: Note that some elements of this structure may not be sent when not needed. At least one element shall be present."]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct DDateTime {
        pub year: Option<DYear>,
        pub month: Option<DMonth>,
        pub day: Option<DDay>,
        pub hour: Option<DHour>,
        pub minute: Option<DMinute>,
        pub second: Option<DSecond>,
        pub offset: Option<DOffset>,
    }
    impl DDateTime {
        pub fn new(
            year: Option<DYear>,
            month: Option<DMonth>,
            day: Option<DDay>,
            hour: Option<DHour>,
            minute: Option<DMinute>,
            second: Option<DSecond>,
            offset: Option<DOffset>,
        ) -> Self {
            Self {
                year,
                month,
                day,
                hour,
                minute,
                second,
                offset,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* The DSRC style day is a simple value consisting of integer values from zero to 31. The value of zero shall represent an unknown value."]
    #[doc = "*"]
    #[doc = "* @unit: days"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=31"))]
    pub struct DDay(pub u8);
    #[doc = "*"]
    #[doc = "* The DSRC hour consists of integer values from zero to 23 representing the hours within a day. The value of 31 shall"]
    #[doc = "* represent an unknown value. The range 24 to 30 is used in some transit applications to represent schedule adherence."]
    #[doc = "*"]
    #[doc = "* @unit: hours"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=31"))]
    pub struct DHour(pub u8);
    #[doc = "*"]
    #[doc = "* The DSRC style minute is a simple value consisting of integer values from zero to 59 representing the minutes"]
    #[doc = "* within an hour. The value of 60 shall represent an unknown value."]
    #[doc = "*"]
    #[doc = "* @unit: minutes"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=60"))]
    pub struct DMinute(pub u8);
    #[doc = "*"]
    #[doc = "* The DSRC month consists of integer values from one to 12, representing the month within a year. The value of 0"]
    #[doc = "* shall represent an unknown value."]
    #[doc = "*"]
    #[doc = "* @unit: months"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=12"))]
    pub struct DMonth(pub u8);
    #[doc = "*"]
    #[doc = "* The DSRC (time zone) offset consists of a signed integer representing an hour and minute value set from -14:00 to"]
    #[doc = "* +14:00, representing all the world’s local time zones in units of minutes. The value of zero (00:00) may also represent an"]
    #[doc = "* unknown value. Note some time zones are do not align to hourly boundaries."]
    #[doc = "*"]
    #[doc = "* @unit: minutes from UTC time"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-840..=840"))]
    pub struct DOffset(pub i16);
    #[doc = "*"]
    #[doc = "* The DSRC second expressed in this DE consists of integer values from zero to 60999, representing the"]
    #[doc = "* milliseconds within a minute. A leap second is represented by the value range 60000 to 60999. The value of 65535 shall"]
    #[doc = "* represent an unavailable value in the range of the minute. The values from 61000 to 65534 are reserved."]
    #[doc = "*"]
    #[doc = "* @unit: milliseconds"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=65535"))]
    pub struct DSecond(pub u16);
    #[doc = "*"]
    #[doc = "* The DSRC year consists of integer values from zero to 4095 representing the year according to the Gregorian"]
    #[doc = "* calendar date system. The value of zero shall represent an unknown value."]
    #[doc = "*"]
    #[doc = "* @unit: years"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=4095"))]
    pub struct DYear(pub u16);
    #[doc = "*"]
    #[doc = "* This DF is used to provide basic (static) information on how a map fragment was processed or determined."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct DataParameters {
        #[rasn(size("1..=255"), identifier = "processMethod")]
        pub process_method: Option<Ia5String>,
        #[rasn(size("1..=255"), identifier = "processAgency")]
        pub process_agency: Option<Ia5String>,
        #[rasn(size("1..=255"), identifier = "lastCheckedDate")]
        pub last_checked_date: Option<Ia5String>,
        #[rasn(size("1..=255"), identifier = "geoidUsed")]
        pub geoid_used: Option<Ia5String>,
    }
    impl DataParameters {
        pub fn new(
            process_method: Option<Ia5String>,
            process_agency: Option<Ia5String>,
            last_checked_date: Option<Ia5String>,
            geoid_used: Option<Ia5String>,
        ) -> Self {
            Self {
                process_method,
                process_agency,
                last_checked_date,
                geoid_used,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* This DE provides the final angle used in the last point of the lane path. Used to \"cant\" the stop line of the lane."]
    #[doc = "*"]
    #[doc = "* With an angle range from negative 150 to positive 150 in one degree steps where zero is directly"]
    #[doc = "* along the axis or the lane center line as defined by the two closest points."]
    #[doc = "*"]
    #[doc = "* @unit: degree"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-150..=150"))]
    pub struct DeltaAngle(pub i16);
    #[doc = "*"]
    #[doc = "* This DE provides a time definition for an object's schedule adherence (typically a transit"]
    #[doc = "* vehicle) within a limited range of time. When the reporting object is ahead of schedule, a positive value is used; when"]
    #[doc = "* behind, a negative value is used. A value of zero indicates schedule adherence. This value is typically sent from a vehicle"]
    #[doc = "* to the traffic signal controller's RSU to indicate the urgency of a signal request in the context of being within schedule or"]
    #[doc = "* not. In another use case, the traffic signal controller may advise the transit vehicle to speed up (DeltaTime > 0) or to slow"]
    #[doc = "* down (DeltaTime < 0) to optimize the transit vehicle distribution driving along a specific route (e.g. a Bus route)."]
    #[doc = "*"]
    #[doc = "* Supporting a range of +/- 20 minute in steps of 10 seconds:"]
    #[doc = "* - the value of `-121` shall be used when more than -20 minutes"]
    #[doc = "* - the value of `+120` shall be used when more than +20 minutes"]
    #[doc = "* - the value `-122` shall be used when the value is unavailable"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-122..=121"))]
    pub struct DeltaTime(pub i8);
    #[doc = "*"]
    #[doc = "* This DE is used in maps and intersections to provide a human readable and"]
    #[doc = "* recognizable name for the feature that follows. It is typically used when debugging a data flow and not in production use."]
    #[doc = "* One key exception to this general rule is to provide a human-readable string for disabled travelers in the case of"]
    #[doc = "* crosswalks and sidewalk lane objects."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=63"))]
    pub struct DescriptiveName(pub Ia5String);
    #[doc = "*"]
    #[doc = "* This DE is an integer value expressing the offset in a defined axis from a"]
    #[doc = "* reference lane number from which a computed lane is offset. The measurement is taken from the reference lane center"]
    #[doc = "* line to the new center line, independent of any width values. The units are a signed value with an LSB of 1 cm."]
    #[doc = "*"]
    #[doc = "* @unit: cm"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-32767..=32767"))]
    pub struct DrivenLineOffsetLg(pub i16);
    #[doc = "*"]
    #[doc = "* The DrivenLineOffsetSmall data element is an integer value expressing the offset in a defined axis from a reference"]
    #[doc = "* lane number from which a computed lane is offset. The measurement is taken from the reference lane center line to the"]
    #[doc = "* new center line, independent of any width values. The units are a signed value with an LSB of 1 cm."]
    #[doc = "*"]
    #[doc = "* @unit: cm"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-2047..=2047"))]
    pub struct DrivenLineOffsetSm(pub i16);
    #[doc = "*"]
    #[doc = "* This DE represents the geographic position above or below the reference ellipsoid (typically WGS-84)."]
    #[doc = "* The number has a resolution of 1 decimeter and represents an asymmetric range of positive and negative"]
    #[doc = "* values. Any elevation higher than +6143.9 meters is represented as +61439."]
    #[doc = "*"]
    #[doc = "* Any elevation lower than -409.5 meters is represented as -4095."]
    #[doc = "*"]
    #[doc = "* If the sending device does not know its elevation, it shall encode the Elevation data element with -4096."]
    #[doc = "*"]
    #[doc = "* @note: When a vehicle is being measured, the elevation is taken from the horizontal spatial center of the vehicle"]
    #[doc = "*        projected downward, regardless of vehicle tilt, to the point where the vehicle meets the road surface."]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-4096..=61439"))]
    pub struct Elevation(pub i32);
    #[doc = "*"]
    #[doc = "* This DE is used to provide the 95% confidence level for the currently reported value of @ref Elevation,"]
    #[doc = "* taking into account the current calibration and precision of the sensor(s) used to measure and/or"]
    #[doc = "* calculate the value. This data element is only to provide the listener with information on the limitations of the sensing"]
    #[doc = "* system, not to support any type of automatic error correction or to imply a guaranteed maximum error. This data element"]
    #[doc = "* should not be used for fault detection or diagnosis, but if a vehicle is able to detect a fault, the confidence interval should"]
    #[doc = "* be increased accordingly. The frame of reference and axis of rotation used shall be in accordance with that defined in Section 11."]
    #[doc = "*"]
    #[doc = "* - `unavailable` - 0:   B'0000 Not Equipped or unavailable"]
    #[doc = "* - `elev-500-00` - 1:   B'0001 (500 m)"]
    #[doc = "* - `elev-200-00` - 2:   B'0010 (200 m)"]
    #[doc = "* - `elev-100-00` - 3:   B'0011 (100 m)"]
    #[doc = "* - `elev-050-00` - 4:   B'0100 (50 m)"]
    #[doc = "* - `elev-020-00` - 5:   B'0101 (20 m)"]
    #[doc = "* - `elev-010-00` - 6:   B'0110 (10 m)"]
    #[doc = "* - `elev-005-00` - 7:   B'0111 (5 m)"]
    #[doc = "* - `elev-002-00` - 8:   B'1000 (2 m)"]
    #[doc = "* - `elev-001-00` - 9:   B'1001 (1 m)"]
    #[doc = "* - `elev-000-50` - 10:  B'1010 (50 cm)"]
    #[doc = "* - `elev-000-20` - 11:  B'1011 (20 cm)"]
    #[doc = "* - `elev-000-10` - 12:  B'1100 (10 cm)"]
    #[doc = "* - `elev-000-05` - 13:  B'1101 (5 cm)"]
    #[doc = "* - `elev-000-02` - 14:  B'1110 (2 cm)"]
    #[doc = "* - `elev-000-01` - 15:  B'1111 (1 cm)"]
    #[doc = "*"]
    #[doc = "* @note: Encoded as a 4 bit value"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum ElevationConfidence {
        unavailable = 0,
        #[rasn(identifier = "elev-500-00")]
        elev_500_00 = 1,
        #[rasn(identifier = "elev-200-00")]
        elev_200_00 = 2,
        #[rasn(identifier = "elev-100-00")]
        elev_100_00 = 3,
        #[rasn(identifier = "elev-050-00")]
        elev_050_00 = 4,
        #[rasn(identifier = "elev-020-00")]
        elev_020_00 = 5,
        #[rasn(identifier = "elev-010-00")]
        elev_010_00 = 6,
        #[rasn(identifier = "elev-005-00")]
        elev_005_00 = 7,
        #[rasn(identifier = "elev-002-00")]
        elev_002_00 = 8,
        #[rasn(identifier = "elev-001-00")]
        elev_001_00 = 9,
        #[rasn(identifier = "elev-000-50")]
        elev_000_50 = 10,
        #[rasn(identifier = "elev-000-20")]
        elev_000_20 = 11,
        #[rasn(identifier = "elev-000-10")]
        elev_000_10 = 12,
        #[rasn(identifier = "elev-000-05")]
        elev_000_05 = 13,
        #[rasn(identifier = "elev-000-02")]
        elev_000_02 = 14,
        #[rasn(identifier = "elev-000-01")]
        elev_000_01 = 15,
    }
    #[doc = "*"]
    #[doc = "* This DF is a sequence of lane IDs for lane objects that are activated in the current map"]
    #[doc = "* configuration. These lanes, unlike most lanes, have their RevocableLane bit set to one (asserted). Such lanes are not"]
    #[doc = "* considered to be part of the current map unless they are in the Enabled Lane List. This concept is used to describe all the"]
    #[doc = "* possible regulatory states for a given physical lane. For example, it is not uncommon to enable or disable the ability to"]
    #[doc = "* make a right hand turn on red during different periods of a day. Another similar example would be a lane which is used for"]
    #[doc = "* driving during one period and where parking is allowed at another. Traditionally, this information is conveyed to the vehicle"]
    #[doc = "* driver by local signage. By using the Enabled Lane List data frame in conjunction with the RevocableLane bit and"]
    #[doc = "* constructing a separate lane object in the intersection map for each different configuration, a single unified map can be"]
    #[doc = "* developed and used."]
    #[doc = "*"]
    #[doc = "* Contents are the unique ID numbers for each lane object which is **active** as part of the dynamic map contents."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16"))]
    pub struct EnabledLaneList(pub SequenceOf<LaneID>);
    #[doc = "*"]
    #[doc = "* This DE provides the type of fuel used by a vehicle."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=15"))]
    pub struct FuelType(pub u8);
    #[doc = "*"]
    #[doc = "* A complete report of the vehicle's position, speed, and heading at an instant in time. Used in the probe vehicle"]
    #[doc = "* message (and elsewhere) as the initial position information. Often followed by other data frames that may provide offset"]
    #[doc = "* path data."]
    #[doc = "*"]
    #[doc = "* @field utcTime:   time with mSec precision"]
    #[doc = "* @field long:      Longitude in 1/10th microdegree"]
    #[doc = "* @field lat:       Latitude in 1/10th microdegree"]
    #[doc = "* @field elevation: Elevation in units of 0.1 m"]
    #[doc = "* @field heading:   Heading value "]
    #[doc = "* @field speed:     Speed value"]
    #[doc = "* @field posAccuracy:      position accuracy"]
    #[doc = "* @field timeConfidence:   time confidence"]
    #[doc = "* @field posConfidence:    position confidence"]
    #[doc = "* @field speedConfidence:  speed confidence "]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct FullPositionVector {
        #[rasn(identifier = "utcTime")]
        pub utc_time: Option<DDateTime>,
        pub long: Longitude,
        pub lat: Latitude,
        pub elevation: Option<Elevation>,
        pub heading: Option<HeadingDSRC>,
        pub speed: Option<TransmissionAndSpeed>,
        #[rasn(identifier = "posAccuracy")]
        pub pos_accuracy: Option<PositionalAccuracy>,
        #[rasn(identifier = "timeConfidence")]
        pub time_confidence: Option<TimeConfidence>,
        #[rasn(identifier = "posConfidence")]
        pub pos_confidence: Option<PositionConfidenceSet>,
        #[rasn(identifier = "speedConfidence")]
        pub speed_confidence: Option<SpeedandHeadingandThrottleConfidence>,
    }
    impl FullPositionVector {
        pub fn new(
            utc_time: Option<DDateTime>,
            long: Longitude,
            lat: Latitude,
            elevation: Option<Elevation>,
            heading: Option<HeadingDSRC>,
            speed: Option<TransmissionAndSpeed>,
            pos_accuracy: Option<PositionalAccuracy>,
            time_confidence: Option<TimeConfidence>,
            pos_confidence: Option<PositionConfidenceSet>,
            speed_confidence: Option<SpeedandHeadingandThrottleConfidence>,
        ) -> Self {
            Self {
                utc_time,
                long,
                lat,
                elevation,
                heading,
                speed,
                pos_accuracy,
                time_confidence,
                pos_confidence,
                speed_confidence,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* This DE is used to relate the current state of a GPS/GNSS rover or base system in terms"]
    #[doc = "* of its general health, lock on satellites in view, and use of any correction information. Various bits can be asserted (made"]
    #[doc = "* to a value of one) to reflect these values. A GNSS set with unknown health and no tracking or corrections would be"]
    #[doc = "* represented by setting the unavailable bit to one. A value of zero shall be used when a defined data element is"]
    #[doc = "* unavailable. The term \"GPS\" in any data element name in this standard does not imply that it is only to be used for GPS-"]
    #[doc = "* type GNSS systems."]
    #[doc = "*"]
    #[doc = "* - `unavailable`              - 0: Not Equipped or unavailable"]
    #[doc = "* - `isHealthy`                - 1:"]
    #[doc = "* - `isMonitored`              - 2:"]
    #[doc = "* - `baseStationType`          - 3: Set to zero if a moving base station, or if a rover device (an OBU), Set to one if it is a fixed base station"]
    #[doc = "* - `aPDOPofUnder5`            - 4: A dilution of precision greater than 5"]
    #[doc = "* - `inViewOfUnder5`           - 5: Less than 5 satellites in view"]
    #[doc = "* - `localCorrectionsPresent`  - 6: DGPS type corrections used"]
    #[doc = "* - `networkCorrectionsPresen` - 7: RTK type corrections used"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct GNSSstatus(pub FixedBitString<8usize>);
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousGenericLaneRegional {
        #[rasn(value("0..=255"), identifier = "regionId")]
        pub region_id: u8,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousGenericLaneRegional {
        pub fn new(region_id: u8, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    impl AnonymousGenericLaneRegional {
        pub fn decode_reg_ext_value<D: Decoder>(
            &self,
            decoder: &mut D,
        ) -> Result<RegGenericLane_Type, D::Error> {
            RegGenericLane_Type::decode(decoder, Some(&self.reg_ext_value), &self.region_id)
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct GenericLaneRegional(pub SequenceOf<AnonymousGenericLaneRegional>);
    #[doc = "*"]
    #[doc = "* This DF is used for all types of lanes, e.g. motorized vehicle lanes, crosswalks, medians. The"]
    #[doc = "* GenericLane describes the basic attribute information of the lane. The LaneID value for each lane is unique within an"]
    #[doc = "* intersection. One use for the LaneID is in the SPAT message, where a given signal or movement phase is mapped to a"]
    #[doc = "* set of applicable lanes using their respective LaneIDs. The NodeList2 data frame includes a sequence of offset points (or"]
    #[doc = "* node points) representing the center line path of the lane. As described in this standard, node points are sets of variable"]
    #[doc = "* sized delta orthogonal offsets from the prior point in the node path. (The initial point is offset from the LLH anchor point"]
    #[doc = "* used in the intersection.) Each node point may convey optional attribute data as well. The use of attributes is described"]
    #[doc = "* further in the Node definition, and in a later clause, but an example use would be to indicate a node point where the lane"]
    #[doc = "* width changes."]
    #[doc = "*"]
    #[doc = "* It should be noted that a \"lane\" is an abstract concept that can describe objects other than motorized vehicle lanes, and"]
    #[doc = "* that the generic lane structure (using features drawn from Japanese usage) also allows combining multiple physical lanes"]
    #[doc = "* into a single lane object. In addition, such lanes can describe connectivity points with other lanes beyond a single"]
    #[doc = "* intersection, extending such a lane description over multiple nearby physical intersections and side streets which"]
    #[doc = "* themselves may not be equipped or assigned an index number in the regional intersection numbering system. (See the"]
    #[doc = "* ConnectsTo entry for details) This has value when describing a broader service area in terms of the roadway network,"]
    #[doc = "* probably with less precision and detail."]
    #[doc = "*"]
    #[doc = "* @field laneID:  The unique ID number assigned to this lane object"]
    #[doc = "* @field name:    often for debug use only but at times used to name ped crossings"]
    #[doc = "* @field ingressApproach:  inbound Approach ID to which this lane belongs"]
    #[doc = "* @field egressApproach: outbound Approach ID to which this lane belongs"]
    #[doc = "* @field laneAttributes: All Attribute information about the basic selected lane type"]
    #[doc = "*                        Directions of use, Geometric co-sharing and Type Specific Attributes"]
    #[doc = "*                        These Attributes are **lane - global** that is, they are true for the entire length of the lane"]
    #[doc = "* @field maneuvers: This data element allows only the description of a subset of possible manoeuvres and therefore"]
    #[doc = "*                    reperesents an incomplete list of possible travel directions. The connecting **lane** data element gives"]
    #[doc = "*                    the exact information about the manoeuvre relation from ingress to egress lane. Therefore the"]
    #[doc = "*                    \"maneuver\" data element is used only additionally if the travel direction of the manoeuvre is"]
    #[doc = "* @field nodeList: Lane spatial path information as well as various Attribute information along the node path"]
    #[doc = "*                    Attributes found here are more general and may come and go over the length of the lane."]
    #[doc = "* @field connectsTo: a list of other lanes and their signal group IDs each connecting lane and its signal group ID"]
    #[doc = "*                    is given, therefore this element provides the information formerly in \"signalGroups\" in prior editions."]
    #[doc = "* @field overlays: A list of any lanes which have spatial paths that overlay (run on top of, and not simply cross)"]
    #[doc = "*                    the path of this lane when used"]
    #[doc = "* @field regional: optional region specific data."]
    #[doc = "*"]
    #[doc = "* @note: The data elements **ingressApproach** and **egressApproach** are used for grouping lanes whin an"]
    #[doc = "*       approach (e.g. lanes defined in travel direction towards the intersection, lanes in exiting direction and"]
    #[doc = "*       cross walks). For a bidirectrional lane (e.g. bike lane) both dataelements are used for the same lane. The"]
    #[doc = "*       integer value used for identifying the **ingressApproach** and the **egressAproach**, based on the"]
    #[doc = "*       topology, may be e.g. the same for all lanes within an approach of an intersection."]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct GenericLane {
        #[rasn(identifier = "laneID")]
        pub lane_id: LaneID,
        pub name: Option<DescriptiveName>,
        #[rasn(identifier = "ingressApproach")]
        pub ingress_approach: Option<ApproachID>,
        #[rasn(identifier = "egressApproach")]
        pub egress_approach: Option<ApproachID>,
        #[rasn(identifier = "laneAttributes")]
        pub lane_attributes: LaneAttributes,
        pub maneuvers: Option<AllowedManeuvers>,
        #[rasn(identifier = "nodeList")]
        pub node_list: NodeListXY,
        #[rasn(identifier = "connectsTo")]
        pub connects_to: Option<ConnectsToList>,
        pub overlays: Option<OverlayLaneList>,
        pub regional: Option<GenericLaneRegional>,
    }
    impl GenericLane {
        pub fn new(
            lane_id: LaneID,
            name: Option<DescriptiveName>,
            ingress_approach: Option<ApproachID>,
            egress_approach: Option<ApproachID>,
            lane_attributes: LaneAttributes,
            maneuvers: Option<AllowedManeuvers>,
            node_list: NodeListXY,
            connects_to: Option<ConnectsToList>,
            overlays: Option<OverlayLaneList>,
            regional: Option<GenericLaneRegional>,
        ) -> Self {
            Self {
                lane_id,
                name,
                ingress_approach,
                egress_approach,
                lane_attributes,
                maneuvers,
                node_list,
                connects_to,
                overlays,
                regional,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* The DE_HeadingConfidence data element is used to provide the 95% confidence level for the currently reported"]
    #[doc = "* calculate the value. This data element is only to provide the listener with information on the limitations of the sensing"]
    #[doc = "* value of DE_Heading, taking into account the current calibration and precision of the sensor(s) used to measure and/or"]
    #[doc = "* system, not to support any type of automatic error correction or to imply a guaranteed maximum error. This data element"]
    #[doc = "* should not be used for fault detection or diagnosis, but if a vehicle is able to detect a fault, the confidence interval should"]
    #[doc = "* be increased accordingly. The frame of reference and axis of rotation used shall be in accordance with that defined Section 11."]
    #[doc = "*"]
    #[doc = "* - `unavailable`   - 0: B'000 Not Equipped or unavailable"]
    #[doc = "* - `prec10deg`     - 1: B'010 10 degrees"]
    #[doc = "* - `prec05deg`     - 2: B'011 5 degrees"]
    #[doc = "* - `prec01deg`     - 3: B'100 1 degrees"]
    #[doc = "* - `prec0-1deg`    - 4: B'101 0.1 degrees"]
    #[doc = "* - `prec0-05deg`   - 5: B'110 0.05 degrees"]
    #[doc = "* - `prec0-01deg`   - 6: B'110 0.01 degrees"]
    #[doc = "* - `prec0-0125deg` - 7: B'111 0.0125 degrees, aligned with heading LSB"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum HeadingConfidenceDSRC {
        unavailable = 0,
        prec10deg = 1,
        prec05deg = 2,
        prec01deg = 3,
        #[rasn(identifier = "prec0-1deg")]
        prec0_1deg = 4,
        #[rasn(identifier = "prec0-05deg")]
        prec0_05deg = 5,
        #[rasn(identifier = "prec0-01deg")]
        prec0_01deg = 6,
        #[rasn(identifier = "prec0-0125deg")]
        prec0_0125deg = 7,
    }
    #[doc = "*"]
    #[doc = "* This DE provides the current heading of the sending device, expressed in unsigned units of"]
    #[doc = "* 0.0125 degrees from North such that 28799 such degrees represent 359.9875 degrees. North shall be defined as the axis"]
    #[doc = "* prescribed by the WGS-84 coordinate system and its reference ellipsoid. Headings \"to the east\" are defined as the"]
    #[doc = "* positive direction. A value of 28800 shall be used when unavailable. This element indicates the direction of motion of the"]
    #[doc = "* device. When the sending device is stopped and the trajectory (path) over which it traveled to reach that location is well"]
    #[doc = "* known, the past heading may be used."]
    #[doc = "*"]
    #[doc = "* Value provides a range of 0 to 359.9875 degrees"]
    #[doc = "*"]
    #[doc = "* @unit: Note that other heading data elements of various sizes and precisions are found in other parts of this standard"]
    #[doc = "*        and in ITS. This element should no longer be used for new work: the @ref Angle entry is preferred."]
    #[doc = "*"]
    #[doc = "* @unit: 0.0125 degrees"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=28800"))]
    pub struct HeadingDSRC(pub u16);
    #[doc = "*"]
    #[doc = "* This DF is used to specify the index of either a single approach or a single lane at"]
    #[doc = "* which a service is needed. This is used, for example, with the Signal Request Message (SRM) to indicate the inbound"]
    #[doc = "* and outbound points by which the requestor (such as a public safety vehicle) can traverse an intersection."]
    #[doc = "*"]
    #[doc = "* @field lane: the representation of the point as lane identifier."]
    #[doc = "* @field approach: the representation of the point as approach identifier."]
    #[doc = "* @field connection: the representation of the point as connection identifier."]
    #[doc = "*"]
    #[doc = "* @note: Note that the value of zero has a reserved meaning for these two indexing systems. In both cases, this value"]
    #[doc = "*    is used to indicate the concept of \"none\" in use. When the value is of zero is used here, it implies the center of the"]
    #[doc = "*    intersection itself. For example, requesting an outbound point of zero implies the requestor wishes to have the intersection"]
    #[doc = "*    itself be the destination. Alternatively, an inbound value of zero implies the requestor is within the intersection itself and"]
    #[doc = "*    wishes to depart for the outbound value provided. This special meaning for the value zero can be used in either the lane"]
    #[doc = "*    or approach with the same results."]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum IntersectionAccessPoint {
        lane(LaneID),
        approach(ApproachID),
        connection(LaneConnectionID),
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousIntersectionGeometryRegional {
        #[rasn(value("0..=255"), identifier = "regionId")]
        pub region_id: u8,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousIntersectionGeometryRegional {
        pub fn new(region_id: u8, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    impl AnonymousIntersectionGeometryRegional {
        pub fn decode_reg_ext_value<D: Decoder>(
            &self,
            decoder: &mut D,
        ) -> Result<RegIntersectionGeometry_Type, D::Error> {
            RegIntersectionGeometry_Type::decode(
                decoder,
                Some(&self.reg_ext_value),
                &self.region_id,
            )
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct IntersectionGeometryRegional(pub SequenceOf<AnonymousIntersectionGeometryRegional>);
    #[doc = "*"]
    #[doc = "* A complete description of an intersection's roadway geometry and its allowed navigational paths (independent of"]
    #[doc = "* any additional regulatory restrictions that may apply over time or from user classification)."]
    #[doc = "*"]
    #[doc = "* @field name: For debug use only"]
    #[doc = "* @field id: A globally unique value set, consisting of a regionID and intersection ID assignment"]
    #[doc = "* @field revision: This profile extends the purpose of the **revision** data element as defined in SAE J2735 as follows."]
    #[doc = "*           The revision data element is used to communicate the valid release of the intersection geometry"]
    #[doc = "*           description. If there are no changes in the deployed intersection description, the same revision counter"]
    #[doc = "*           is transmitted. Due to a revised deployment of the intersection description (e.g. new lane added, ID's"]
    #[doc = "*           changed, etc.), the revision is increased by one. After revision equal to 127, the increment restarts by 0."]
    #[doc = "*           The intersection geometry and the signal phase and timing information is related each other. Therefore,"]
    #[doc = "*           the revision of the intersection geometry of the MapData message shall be the same as the revision of"]
    #[doc = "*           the intersection state of the SPAT (see data element **revision** of **DF_IntersectionState** in [ISO TS 19091] G.8.2.9)"]
    #[doc = "* @field refPoint: The reference from which subsequent data points are offset until a new point is used."]
    #[doc = "* @field laneWidth: Reference width used by all subsequent lanes unless a new width is given"]
    #[doc = "* @field speedLimits: Reference regulatory speed limits used by all subsequent lanes unless a new speed is given"]
    #[doc = "* @field laneSet: Data about one or more lanes (all lane data is found here) Data describing how to use and request preemption and"]
    #[doc = "*           priority services from this intersection (if supported)"]
    #[doc = "* @field preemptPriorityData: This DF is not used."]
    #[doc = "* @field regional: optional region specific data."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct IntersectionGeometry {
        pub name: Option<DescriptiveName>,
        pub id: IntersectionReferenceID,
        pub revision: MsgCount,
        #[rasn(identifier = "refPoint")]
        pub ref_point: Position3D,
        #[rasn(identifier = "laneWidth")]
        pub lane_width: Option<LaneWidth>,
        #[rasn(identifier = "speedLimits")]
        pub speed_limits: Option<SpeedLimitList>,
        #[rasn(identifier = "laneSet")]
        pub lane_set: LaneList,
        #[rasn(identifier = "preemptPriorityData")]
        pub preempt_priority_data: Option<PreemptPriorityList>,
        pub regional: Option<IntersectionGeometryRegional>,
    }
    impl IntersectionGeometry {
        pub fn new(
            name: Option<DescriptiveName>,
            id: IntersectionReferenceID,
            revision: MsgCount,
            ref_point: Position3D,
            lane_width: Option<LaneWidth>,
            speed_limits: Option<SpeedLimitList>,
            lane_set: LaneList,
            preempt_priority_data: Option<PreemptPriorityList>,
            regional: Option<IntersectionGeometryRegional>,
        ) -> Self {
            Self {
                name,
                id,
                revision,
                ref_point,
                lane_width,
                speed_limits,
                lane_set,
                preempt_priority_data,
                regional,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* This DF consists of a list of IntersectionGeometry entries."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"))]
    pub struct IntersectionGeometryList(pub SequenceOf<IntersectionGeometry>);
    #[doc = "*"]
    #[doc = "* This DE is used within a region to uniquely define an intersection within that country or region in a 16-bit"]
    #[doc = "* field. Assignment rules are established by the regional authority associated with the RoadRegulatorID under which this"]
    #[doc = "* IntersectionID is assigned. Within the region the policies used to ensure an assigned value’s uniqueness before that value"]
    #[doc = "* is reused (if ever) is the responsibility of that region. Any such reuse would be expected to occur over a long epoch (many years)."]
    #[doc = "* The values zero through 255 are allocated for testing purposes"]
    #[doc = "*"]
    #[doc = "* @note:  Note that the value assigned to an intersection will be unique within a given regional ID only"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=65535"))]
    pub struct IntersectionID(pub u16);
    #[doc = "*"]
    #[doc = "* This DF conveys the combination of an optional RoadRegulatorID and of an"]
    #[doc = "* IntersectionID that is unique within that region. When the RoadRegulatorID is present the IntersectionReferenceID is"]
    #[doc = "* guaranteed to be globally unique."]
    #[doc = "*"]
    #[doc = "* @field region: a globally unique regional assignment value typical assigned to a regional DOT authority"]
    #[doc = "*                the value zero shall be used for testing needs"]
    #[doc = "* @field id: a unique mapping to the intersection in question within the above region of use"]
    #[doc = "*"]
    #[doc = "* @note: A fully qualified intersection consists of its regionally unique ID (the IntersectionID) and its region ID (the"]
    #[doc = "*        RoadRegulatorID). Taken together these form a unique value which is never repeated."]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct IntersectionReferenceID {
        pub region: Option<RoadRegulatorID>,
        pub id: IntersectionID,
    }
    impl IntersectionReferenceID {
        pub fn new(region: Option<RoadRegulatorID>, id: IntersectionID) -> Self {
            Self { region, id }
        }
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousIntersectionStateRegional {
        #[rasn(value("0..=255"), identifier = "regionId")]
        pub region_id: u8,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousIntersectionStateRegional {
        pub fn new(region_id: u8, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    impl AnonymousIntersectionStateRegional {
        pub fn decode_reg_ext_value<D: Decoder>(
            &self,
            decoder: &mut D,
        ) -> Result<RegIntersectionState_Type, D::Error> {
            RegIntersectionState_Type::decode(decoder, Some(&self.reg_ext_value), &self.region_id)
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct IntersectionStateRegional(pub SequenceOf<AnonymousIntersectionStateRegional>);
    #[doc = "*"]
    #[doc = "* This DF is used to convey all the SPAT information for a single intersection. Both current"]
    #[doc = "* and future data can be sent."]
    #[doc = "*"]
    #[doc = "* @field name: human readable name for intersection to be used only in debug mode"]
    #[doc = "* @field id: A globally unique value set, consisting of a regionID and intersection ID assignment"]
    #[doc = "*            provides a unique mapping to the intersection MAP in question which provides complete location"]
    #[doc = "*            and approach/move/lane data"]
    #[doc = "* @field revision: The data element **revision** is used to communicate the actual valid release of the intersection"]
    #[doc = "*                  description. If there are no changes in the deployed intersection description, almost the same revision"]
    #[doc = "*                  counter is transmitted. Due to a revised deployment of the intersection description (e.g. introduction of"]
    #[doc = "*                  additional signal state element), the revision is increased by one. After revision equal to 127, the"]
    #[doc = "*                  increment leads to 0 (due to the element range)."]
    #[doc = "*                  The intersection state and the intersection geometry is related to each other. Therefore, the revision of"]
    #[doc = "*                  the intersection state shall be the same as the revision of the intersection geometry (see the data"]
    #[doc = "*                  element **revision** of **DF_IntersectionGeometry** in [ISO TS 19091] G.8.2.6)."]
    #[doc = "* @field status: general status of the controller(s)"]
    #[doc = "* @field moy: Minute of current UTC year, used only with messages to be archived."]
    #[doc = "* @field timeStamp: the mSec point in the current UTC minute that this message was constructed."]
    #[doc = "* @field enabledLanes: a list of lanes where the RevocableLane bit has been set which are now active and"]
    #[doc = "*                      therefore part of the current intersection"]
    #[doc = "* @field states: Each Movement is given in turn and contains its signal phase state,"]
    #[doc = "*                mapping to the lanes it applies to, and point in time it will end, and it"]
    #[doc = "*                may contain both active and future states"]
    #[doc = "* @field maneuverAssistList: Assist data"]
    #[doc = "* @field regional: optional region specific data."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct IntersectionState {
        pub name: Option<DescriptiveName>,
        pub id: IntersectionReferenceID,
        pub revision: MsgCount,
        pub status: IntersectionStatusObject,
        pub moy: Option<MinuteOfTheYear>,
        #[rasn(identifier = "timeStamp")]
        pub time_stamp: Option<DSecond>,
        #[rasn(identifier = "enabledLanes")]
        pub enabled_lanes: Option<EnabledLaneList>,
        pub states: MovementList,
        #[rasn(identifier = "maneuverAssistList")]
        pub maneuver_assist_list: Option<ManeuverAssistList>,
        pub regional: Option<IntersectionStateRegional>,
    }
    impl IntersectionState {
        pub fn new(
            name: Option<DescriptiveName>,
            id: IntersectionReferenceID,
            revision: MsgCount,
            status: IntersectionStatusObject,
            moy: Option<MinuteOfTheYear>,
            time_stamp: Option<DSecond>,
            enabled_lanes: Option<EnabledLaneList>,
            states: MovementList,
            maneuver_assist_list: Option<ManeuverAssistList>,
            regional: Option<IntersectionStateRegional>,
        ) -> Self {
            Self {
                name,
                id,
                revision,
                status,
                moy,
                time_stamp,
                enabled_lanes,
                states,
                maneuver_assist_list,
                regional,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* This DF consists of a list of IntersectionState entries."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"))]
    pub struct IntersectionStateList(pub SequenceOf<IntersectionState>);
    #[doc = "*"]
    #[doc = "* The Intersection Status Object contains Advanced Traffic Controller (ATC) status information that may be sent to"]
    #[doc = "* local OBUs as part of the SPAT process."]
    #[doc = "*"]
    #[doc = "* With bits as defined:"]
    #[doc = "* - `manualControlIsEnabled`                - 0: Timing reported is per programmed values, etc. but person at cabinet can manually request that certain intervals are terminated early (e.g. green)."]
    #[doc = "* - `stopTimeIsActivated`                   - 1: And all counting/timing has stopped."]
    #[doc = "* - `failureFlash`                          - 2: Above to be used for any detected hardware failures, e.g. conflict monitor as well as for police flash"]
    #[doc = "* - `fixedTimeOperation`                    - 5: Schedule of signals is based on time only (i.e. the state can be calculated)"]
    #[doc = "* - `trafficDependentOperation`             - 6: Operation is based on different levels of traffic parameters (requests, duration of gaps or more complex parameters)"]
    #[doc = "* - `standbyOperation`                      - 7: Controller: partially switched off or partially amber flashing"]
    #[doc = "* - `failureMode`                           - 8: Controller has a problem or failure in operation"]
    #[doc = "* - `off`                                   - 9: Controller is switched off"]
    #[doc = "* - `recentMAPmessageUpdate`                - 10: Map revision with content changes"]
    #[doc = "* - `recentChangeInMAPassignedLanesIDsUsed` - 11: Change in MAP's assigned lanes used (lane changes) Changes in the active lane list description"]
    #[doc = "* - `noValidMAPisAvailableAtThisTime`       - 12: MAP (and various lanes indexes) not available"]
    #[doc = "* - `noValidSPATisAvailableAtThisTime`      - 13: SPAT system is not working at this time"]
    #[doc = "* - Bits 14,15 reserved at this time and shall be zero"]
    #[doc = "*"]
    #[doc = "* @note: All zeros indicate normal operating mode with no recent changes. The duration of the term **recent** is defined by the system performance requirement in use."]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct IntersectionStatusObject(pub FixedBitString<16usize>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct LaneAttributesRegional {
        #[rasn(value("0..=255"), identifier = "regionId")]
        pub region_id: u8,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl LaneAttributesRegional {
        pub fn new(region_id: u8, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    impl LaneAttributesRegional {
        pub fn decode_reg_ext_value<D: Decoder>(
            &self,
            decoder: &mut D,
        ) -> Result<RegLaneAttributes_Type, D::Error> {
            RegLaneAttributes_Type::decode(decoder, Some(&self.reg_ext_value), &self.region_id)
        }
    }
    #[doc = "*"]
    #[doc = "* This DF holds all of the constant attribute information of any lane object (as well as"]
    #[doc = "* denoting the basic lane type itself) within a single structure. Constant attribute information are those values which do not"]
    #[doc = "* change over the path of the lane, such as the direction of allowed travel. Other lane attribute information can change at or"]
    #[doc = "* between each node."]
    #[doc = "* The structure consists of three element parts as follows: LaneDirection specifies the allowed directions of travel, if any."]
    #[doc = "* LaneSharing indicates whether this lane type is shared with other types of travel modes or users. The lane type is defined"]
    #[doc = "* in LaneTypeAttributes, along with additional attributes specific to that type."]
    #[doc = "* The fundamental type of lane object is described by the element selected in the LaneTypeAttributes data concept."]
    #[doc = "* Additional information specific or unique to a given lane type can be found there as well. A regional extension is provided"]
    #[doc = "* as well."]
    #[doc = "* Note that combinations of regulatory maneuver information such as \"both a left turn and straight ahead movement are"]
    #[doc = "* allowed, but never a u-turn,\" are expressed by the AllowedManeuvers data concept which typically follows after this"]
    #[doc = "* element and in the same structure. Note that not all lane objects require this information (for example a median). The"]
    #[doc = "* various values are set via bit flags to indicate the assertion of a value. Each defined lane type contains the bit flags"]
    #[doc = "* suitable for its application area."]
    #[doc = "* Note that the concept of LaneSharing is used to indicate that there are other users of this lane with equal regulatory rights"]
    #[doc = "* to occupy the lane (which is a term this standard does not formally define since it varies by world region). A typical case is"]
    #[doc = "* a light rail vehicle running along the same lane path as motorized traffic. In such a case, motor traffic may be allowed"]
    #[doc = "* equal access to the lane when a train is not present. Another case would be those intersection lanes (at the time of writing"]
    #[doc = "* rather unusual) where bicycle traffic is given full and equal right of way to an entire width of motorized vehicle lane. This"]
    #[doc = "* example would not be a bike lane or bike box in the traditional sense."]
    #[doc = "*"]
    #[doc = "* @field directionalUse: directions of lane use"]
    #[doc = "* @field sharedWith: co-users of the lane path"]
    #[doc = "* @field laneType: specific lane type data"]
    #[doc = "* @field regional: optional region specific data."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct LaneAttributes {
        #[rasn(identifier = "directionalUse")]
        pub directional_use: LaneDirection,
        #[rasn(identifier = "sharedWith")]
        pub shared_with: LaneSharing,
        #[rasn(identifier = "laneType")]
        pub lane_type: LaneTypeAttributes,
        pub regional: Option<LaneAttributesRegional>,
    }
    impl LaneAttributes {
        pub fn new(
            directional_use: LaneDirection,
            shared_with: LaneSharing,
            lane_type: LaneTypeAttributes,
            regional: Option<LaneAttributesRegional>,
        ) -> Self {
            Self {
                directional_use,
                shared_with,
                lane_type,
                regional,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* This DE relates specific properties found in a Barrier or Median lane type (a type of lane object used to separate traffic lanes)."]
    #[doc = "* It should be noted that various common lane attribute properties (such as travel directions and allowed movements or maneuvers) can be found in other entries."]
    #[doc = "*"]
    #[doc = "* With bits as defined:"]
    #[doc = "* - `median-RevocableLane` - 0: this lane may be activated or not based on the current SPAT message contents if not asserted, the lane is ALWAYS present"]
    #[doc = "* - Bits 10-15 reserved and set to zero"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "LaneAttributes-Barrier")]
    pub struct LaneAttributesBarrier(pub FixedBitString<16usize>);
    #[doc = "*"]
    #[doc = "* This DE relates specific properties found in a bicycle lane type. It should be noted that various common lane attribute properties"]
    #[doc = "* (such as travel directions and allowed movements or maneuvers) can be found in other entries."]
    #[doc = "*"]
    #[doc = "* With bits as defined:"]
    #[doc = "* - `bikeRevocableLane`       - 0: this lane may be activated or not based on the current SPAT message contents if not asserted, the lane is ALWAYS present"]
    #[doc = "* - `pedestrianUseAllowed`    - 1: The path allows pedestrian traffic, if not set, this mode is prohibited"]
    #[doc = "* - `isBikeFlyOverLane`       - 2: path of lane is not at grade"]
    #[doc = "* - `fixedCycleTime`          - 3: the phases use preset times, i.e. there is not a **push to cross** button"]
    #[doc = "* - `biDirectionalCycleTimes` - 4: ped walk phases use different SignalGroupID for each direction. The first SignalGroupID in the first Connection"]
    #[doc = "*                                  represents **inbound** flow (the direction of travel towards the first node point) while second SignalGroupID in the"]
    #[doc = "*                                  next Connection entry represents the `outbound` flow. And use of RestrictionClassID entries in the Connect follow this same pattern in pairs."]
    #[doc = "* - `isolatedByBarrier`           - 5: The lane path is isolated by a fixed barrier"]
    #[doc = "* - `unsignalizedSegmentsPresent` - 6: The lane path consists of one of more segments which are not part of a signal group ID"]
    #[doc = "* - Bits 7-15 reserved and set to zero"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "LaneAttributes-Bike")]
    pub struct LaneAttributesBike(pub FixedBitString<16usize>);
    #[doc = "*"]
    #[doc = "* This DE relates specific properties found in a crosswalk lane type. It should be noted that various common lane attribute properties"]
    #[doc = "* (such as travel directions and allowed movements or maneuvers) can be found in other entries."]
    #[doc = "*"]
    #[doc = "* With bits as defined:"]
    #[doc = "* - `crosswalkRevocableLane`  - 0:  this lane may be activated or not based on the current SPAT message contents if not asserted, the lane is ALWAYS present"]
    #[doc = "* - `bicyleUseAllowed`        - 1: The path allows bicycle traffic, if not set, this mode is prohibited"]
    #[doc = "* - `isXwalkFlyOverLane`      - 2: path of lane is not at grade"]
    #[doc = "* - `fixedCycleTime`          - 3: ped walk phases use preset times. i.e. there is not a **push to cross** button"]
    #[doc = "* - `biDirectionalCycleTimes` - 4:  ped walk phases use different SignalGroupID for each direction. The first SignalGroupID"]
    #[doc = "*                                   in the first Connection represents **inbound** flow (the direction of travel towards the first"]
    #[doc = "*                                   node point) while second SignalGroupID in the next Connection entry represents the **outbound**"]
    #[doc = "*                                   flow. And use of RestrictionClassID entries in the Connect follow this same pattern in pairs."]
    #[doc = "* - `hasPushToWalkButton`     - 5: Has a demand input"]
    #[doc = "* - `audioSupport`            - 6:  audio crossing cues present"]
    #[doc = "* - `rfSignalRequestPresent`  - 7: Supports RF push to walk technologies"]
    #[doc = "* - `unsignalizedSegmentsPresent` - 8: The lane path consists of one of more segments which are not part of a signal group ID"]
    #[doc = "* - Bits 9-15 reserved and set to zero"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "LaneAttributes-Crosswalk")]
    pub struct LaneAttributesCrosswalk(pub FixedBitString<16usize>);
    #[doc = "*"]
    #[doc = "* This DE relates specific properties found in a vehicle parking lane type. It should be noted that various common lane attribute"]
    #[doc = "* properties can be found in other entries."]
    #[doc = "*"]
    #[doc = "* With bits as defined:"]
    #[doc = "* - `parkingRevocableLane` - 0: this lane may be activated or not based on the current SPAT message contents if not asserted, the lane is ALWAYS present"]
    #[doc = "* - `doNotParkZone`        - 3: used to denote fire hydrants as well as short disruptions in a parking zone"]
    #[doc = "* - `noPublicParkingUse`   - 6: private parking, as in front of private property"]
    #[doc = "* - Bits 7-15 reserved and set to zero*"]
    #[doc = "*"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "LaneAttributes-Parking")]
    pub struct LaneAttributesParking(pub FixedBitString<16usize>);
    #[doc = "*"]
    #[doc = "* This DE relates specific properties found in a sidewalk lane type. It should be noted that various common lane attribute properties"]
    #[doc = "* (such as travel directions and allowed movements or maneuvers) can be found in other entries."]
    #[doc = "*"]
    #[doc = "* With bits as defined:"]
    #[doc = "* - `sidewalk-RevocableLane`- 0: this lane may be activated or not based on the current SPAT message contents if not asserted, the lane is ALWAYS present."]
    #[doc = "* - `bicyleUseAllowed`      - 1: The path allows bicycle traffic, if not set, this mode is prohibited"]
    #[doc = "* - `isSidewalkFlyOverLane` - 2: path of lane is not at grade"]
    #[doc = "* - `walkBikes`             - 3: bike traffic must dismount and walk"]
    #[doc = "* - Bits 4-15 reserved and set to zero"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "LaneAttributes-Sidewalk")]
    pub struct LaneAttributesSidewalk(pub FixedBitString<16usize>);
    #[doc = "*"]
    #[doc = "* This DE relates specific properties found in various types of ground striping lane"]
    #[doc = "* types. This includes various types of painted lane ground striping and iconic information needs to convey information in a"]
    #[doc = "* complex intersection. Typically, this consists of visual guidance for drivers to assist them to connect across the"]
    #[doc = "* intersection to the correct lane. Such markings are typically used with restraint and only under conditions when the"]
    #[doc = "* geometry of the intersection makes them more beneficial than distracting. It should be noted that various common lane"]
    #[doc = "* attribute properties (such as travel directions and allowed movements or maneuvers) can be found in other entries."]
    #[doc = "*"]
    #[doc = "* With bits as defined:"]
    #[doc = "* - `stripeToConnectingLanesRevocableLane` - 0: this lane may be activated or not activated based on the current SPAT message contents if not asserted, the lane is ALWAYS present"]
    #[doc = "* - `stripeToConnectingLanesAhead` - 5: the stripe type should be presented to the user visually to reflect stripes in the intersection for the type of movement indicated."]
    #[doc = "* - Bits 6-15 reserved and set to zero"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "LaneAttributes-Striping")]
    pub struct LaneAttributesStriping(pub FixedBitString<16usize>);
    #[doc = "*"]
    #[doc = "* This DE relates specific properties found in a tracked vehicle lane types (trolley"]
    #[doc = "* and train lanes). The term “rail vehicle” can be considered synonymous. In this case, the term does not relate to vehicle"]
    #[doc = "* types with tracks or treads. It should be noted that various common lane attribute properties (such as travel directions and"]
    #[doc = "* allowed movements or maneuvers) can be found in other entries. It should also be noted that often this type of lane object"]
    #[doc = "* does not clearly relate to an approach in the traditional traffic engineering sense, although the message set allows"]
    #[doc = "* assigning a value when desired."]
    #[doc = "*"]
    #[doc = "* With bits as defined:"]
    #[doc = "* - `spec-RevocableLane` - 0: this lane may be activated or not based on the current SPAT message contents if not asserted, the lane is ALWAYS present."]
    #[doc = "* - Bits 5-15 reserved and set to zero"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "LaneAttributes-TrackedVehicle")]
    pub struct LaneAttributesTrackedVehicle(pub FixedBitString<16usize>);
    #[doc = "*"]
    #[doc = "* This DE relates specific properties found in a vehicle lane type. This data element provides a means to denote that the use of a lane"]
    #[doc = "* is restricted to certain vehicle types. Various common lane attribute properties (such as travel directions and allowed movements or maneuvers)"]
    #[doc = "* can be found in other entries."]
    #[doc = "*"]
    #[doc = "* With bits as defined:"]
    #[doc = "* - `isVehicleRevocableLane` - 0: this lane may be activated or not based on the current SPAT message contents if not asserted, the lane is ALWAYS present"]
    #[doc = "* - `isVehicleFlyOverLane`   - 1: path of lane is not at grade"]
    #[doc = "* - `permissionOnRequest`    - 7: e.g. to inform about a lane for e-cars"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "LaneAttributes-Vehicle", size("8", extensible))]
    pub struct LaneAttributesVehicle(pub BitString);
    #[doc = "*"]
    #[doc = "* This DE is used to state a connection index for a lane to lane connection. It is used to"]
    #[doc = "* relate this connection between the lane (defined in the MAP) and any dynamic clearance data sent in the SPAT. It should"]
    #[doc = "* be noted that the index may be shared with other lanes (for example, two left turn lanes may share the same dynamic"]
    #[doc = "* clearance data). It should also be noted that a given lane to lane connection may be part of more than one GroupID due"]
    #[doc = "* to signal phase considerations, but will only have one ConnectionID. The ConnectionID concept is not used (is not"]
    #[doc = "* present) when dynamic clearance data is not provided in the SPAT."]
    #[doc = "*"]
    #[doc = "* @note: It should be noted that the LaneConnectionID is used as a means to index to a connection description"]
    #[doc = "*        between two lanes. It is not the same as the laneID, which is the unique index to each lane itself."]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct LaneConnectionID(pub u8);
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousLaneDataAttributeRegional {
        #[rasn(value("0..=255"), identifier = "regionId")]
        pub region_id: u8,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousLaneDataAttributeRegional {
        pub fn new(region_id: u8, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    impl AnonymousLaneDataAttributeRegional {
        pub fn decode_reg_ext_value<D: Decoder>(
            &self,
            decoder: &mut D,
        ) -> Result<RegLaneDataAttribute_Type, D::Error> {
            RegLaneDataAttribute_Type::decode(decoder, Some(&self.reg_ext_value), &self.region_id)
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct LaneDataAttributeRegional(pub SequenceOf<AnonymousLaneDataAttributeRegional>);
    #[doc = "*"]
    #[doc = "* This DF is used to relate an attribute and a control value at a node point or along a"]
    #[doc = "* lane segment from an enumerated list of defined choices. It is then followed by a defined data value associated with it and"]
    #[doc = "* which is defined elsewhere in this standard."]
    #[doc = "*"]
    #[doc = "* @field pathEndPointAngle: adjusts final point/width slant of the lane to align with the stop line"]
    #[doc = "* @field laneCrownPointCenter: sets the canter of the road bed from centerline point"]
    #[doc = "* @field laneCrownPointLeft: sets the canter of the road bed from left edge"]
    #[doc = "* @field laneCrownPointRight: sets the canter of the road bed from right edge"]
    #[doc = "* @field laneAngle: the angle or direction of another lane this is required when a merge point angle is required"]
    #[doc = "* @field speedLimits: Reference regulatory speed limits used by all segments"]
    #[doc = "* @field regional: optional region specific data."]
    #[doc = "*"]
    #[doc = "* @note: This data concept handles a variety of use case needs with a common and consistent message pattern. The"]
    #[doc = "*     typical use of this data concept (and several similar others) is to inject the selected Attribute into the spatial description of"]
    #[doc = "*     a lane's center line path (the segment list). In this way, attribute information which is true for a portion of the overall lane"]
    #[doc = "*     can be described when needed. This attribute information applies from the node point in the stream of segment data until"]
    #[doc = "*     changed again. Denoting the porous aspects of a lane along its path as it merges with another lane would be an example"]
    #[doc = "*     of this use case. In this case the start and end node points would be followed by suitable segment attributes. Re-using a"]
    #[doc = "*     lane path (previously called a computed lane) is another example. In this case the reference lane to be re-used appears"]
    #[doc = "*     as a segment attribute followed by the lane value. It is then followed by one or more segment attributes which relate the"]
    #[doc = "*     positional translation factors to be used (offset, rotate, scale) and any further segment attribute changes."]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum LaneDataAttribute {
        pathEndPointAngle(DeltaAngle),
        laneCrownPointCenter(RoadwayCrownAngle),
        laneCrownPointLeft(RoadwayCrownAngle),
        laneCrownPointRight(RoadwayCrownAngle),
        laneAngle(MergeDivergeNodeAngle),
        speedLimits(SpeedLimitList),
        regional(LaneDataAttributeRegional),
    }
    #[doc = "*"]
    #[doc = "* This DF consists of a list of LaneDataAttribute entries."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8"))]
    pub struct LaneDataAttributeList(pub SequenceOf<LaneDataAttribute>);
    #[doc = "*"]
    #[doc = "* This DE is used to denote the allowed direction of travel over a lane object. By convention, the lane object is always described"]
    #[doc = "* from the stop line outwards away from the intersection. Therefore, the ingress direction is from the end of the path to the stop"]
    #[doc = "* line and the egress direction is from the stop line outwards."]
    #[doc = "*"]
    #[doc = "* It should be noted that some lane objects are not used for travel and that some lane objects allow bi-directional travel."]
    #[doc = "*"]
    #[doc = "* With bits as defined:"]
    #[doc = "* - Allowed directions of travel in the lane object"]
    #[doc = "* - All lanes are described from the stop line outwards"]
    #[doc = "*"]
    #[doc = "* @field ingressPath: travel from rear of path to front is allowed"]
    #[doc = "*"]
    #[doc = "* @field egressPath: travel from front of path to rear is allowed"]
    #[doc = "*"]
    #[doc = "* @note: No Travel, i.e. the lane object type does not support travel (medians, curbs, etc.) is indicated by not"]
    #[doc = "*        asserting any bit value Bi-Directional Travel (such as a ped crosswalk) is indicated by asserting both of the bits."]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct LaneDirection(pub FixedBitString<2usize>);
    #[doc = "*"]
    #[doc = "* This DE conveys an assigned index that is unique within an intersection. It is used to refer to"]
    #[doc = "* that lane by other objects in the intersection map data structure. Lanes may be ingress (inbound traffic) or egress"]
    #[doc = "* (outbound traffic) in nature, as well as barriers and other types of specialty lanes. Each lane (each lane object) is"]
    #[doc = "* assigned a unique ID. The Lane ID, in conjunction with the intersection ID, forms a regionally unique way to address a"]
    #[doc = "* specific lane in that region."]
    #[doc = "*"]
    #[doc = "* - the value 0 shall be used when the lane ID is not available or not known"]
    #[doc = "* - the value 255 is reserved for future use"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct LaneID(pub u8);
    #[doc = "*"]
    #[doc = "* This DF consists of a list of GenericLane entries."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=255"))]
    pub struct LaneList(pub SequenceOf<GenericLane>);
    #[doc = "*"]
    #[doc = "* This DE is used to denote the presence of other user types (travel modes) who have an"]
    #[doc = "* equal right to access and use the lane. There may also be another lane object describing their use of a lane. This data"]
    #[doc = "* concept is used to indicate lanes and/or users that travel along the same path, and not those that simply cross over the"]
    #[doc = "* lane's segments path (such as a pedestrian crosswalk crossing a lane for motor vehicle use). The typical use is to alert"]
    #[doc = "* the user of the MAP data that additional traffic of another mode may be present in the same spatial lane."]
    #[doc = "*"]
    #[doc = "* Bits used:"]
    #[doc = "* - 0 - overlappingLaneDescriptionProvided: Assert when another lane object is present to describe the"]
    #[doc = "*                                           path of the overlapping shared lane this construct is not used for lane objects which simply cross"]
    #[doc = "* - 1 - multipleLanesTreatedAsOneLane: Assert if the lane object path and width details represents multiple lanes within it"]
    #[doc = "*                                      that are not further described Various modes and type of traffic that may share this lane:"]
    #[doc = "* - 2 - otherNonMotorizedTrafficTypes: horse drawn etc."]
    #[doc = "* - 3 - individualMotorizedVehicleTraffic:"]
    #[doc = "* - 4 - busVehicleTraffic:"]
    #[doc = "* - 5 - taxiVehicleTraffic:"]
    #[doc = "* - 6 - pedestriansTraffic:"]
    #[doc = "* - 7 - cyclistVehicleTraffic:"]
    #[doc = "* - 8 - trackedVehicleTraffic:"]
    #[doc = "* - 9 - pedestrianTraffic:"]
    #[doc = "*"]
    #[doc = "* @note: All zeros would indicate **not shared** and **not overlapping**"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct LaneSharing(pub FixedBitString<10usize>);
    #[doc = "*"]
    #[doc = "* This DF is used to hold attribute information specific to a given lane type. It is typically"]
    #[doc = "* used in the DE_LaneAttributes data frame as part of an overall description of a lane object. Information unique to the"]
    #[doc = "* specific type of lane is found here. Information common to lanes is expressed in other entries. The various values are set"]
    #[doc = "* by bit flags to indicate the assertion of a value. Each defined lane type contains bit flags suitable for its application area."]
    #[doc = "*"]
    #[doc = "* @field vehicle:         motor vehicle lanes"]
    #[doc = "*"]
    #[doc = "* @field crosswalk:       pedestrian crosswalks"]
    #[doc = "*"]
    #[doc = "* @field bikeLane:        bike lanes"]
    #[doc = "*"]
    #[doc = "* @field sidewalk:        pedestrian sidewalk paths"]
    #[doc = "*"]
    #[doc = "* @field median:          medians & channelization"]
    #[doc = "*"]
    #[doc = "* @field striping:        roadway markings"]
    #[doc = "*"]
    #[doc = "* @field trackedVehicle:  trains and trolleys"]
    #[doc = "*"]
    #[doc = "* @field parking:         parking and stopping lanes"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum LaneTypeAttributes {
        vehicle(LaneAttributesVehicle),
        crosswalk(LaneAttributesCrosswalk),
        bikeLane(LaneAttributesBike),
        sidewalk(LaneAttributesSidewalk),
        median(LaneAttributesBarrier),
        striping(LaneAttributesStriping),
        trackedVehicle(LaneAttributesTrackedVehicle),
        parking(LaneAttributesParking),
    }
    #[doc = "*"]
    #[doc = "* This DE conveys the width of a lane in LSB units of 1 cm. Maximum value for a lane is 327.67 meters in width"]
    #[doc = "*"]
    #[doc = "* @units: cm"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=32767"))]
    pub struct LaneWidth(pub u16);
    #[doc = "*"]
    #[doc = "* Large @ref MapData descriptions are not possible to be broadcast with a single message and have to be"]
    #[doc = "* fragmented using two or more messages over the air. Therefore, the LayerID allows defining an"]
    #[doc = "* index for fragmentation of large @ref MapData descriptions. The fragmentation of the messages shall be"]
    #[doc = "* executed on application layer. The fragmentation occurs on an approach base. This means that almost a"]
    #[doc = "* complete approach (e.g. lanes, connectsTo, etc.) has to be included within a fragment."]
    #[doc = "* The decimal value of the **layerID** is used to define the amount of maximum @ref MapData fragments. The"]
    #[doc = "* lower value defines the actual fragment."]
    #[doc = "*"]
    #[doc = "* Example:"]
    #[doc = "* If a MapData consists of three fragments (e.g. three approaches), the fragments are identified as follows:"]
    #[doc = "* - `31` - first fragment of three (e.g. approach south);"]
    #[doc = "* - `33` - third fragment of three (e.g. approach north)."]
    #[doc = "* - `32` - second fragment of three (e.g. approach west);"]
    #[doc = "*"]
    #[doc = "* If there are only two fragments, the fragment identification will be 21, 22."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=100"))]
    pub struct LayerID(pub u8);
    #[doc = "*"]
    #[doc = "* This DE is used to uniquely identify the type of information to be found in a layer of a geographic map fragment such as an intersection."]
    #[doc = "*"]
    #[doc = "* @field `mixedContent`: two or more of the below types"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum LayerType {
        none = 0,
        mixedContent = 1,
        generalMapData = 2,
        intersectionData = 3,
        curveData = 4,
        roadwaySectionData = 5,
        parkingAreaData = 6,
        sharedLaneData = 7,
    }
    #[doc = "*"]
    #[doc = "* This DE is used to provide the R09 line information."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V2.2.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=4294967295"))]
    pub struct LineNumber(pub u32);
    #[doc = "*"]
    #[doc = "* This DF consists of a list of @ref ConnectionManeuverAssist entries."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16"))]
    pub struct ManeuverAssistList(pub SequenceOf<ConnectionManeuverAssist>);
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMapDataRegional {
        #[rasn(value("0..=255"), identifier = "regionId")]
        pub region_id: u8,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousMapDataRegional {
        pub fn new(region_id: u8, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    impl AnonymousMapDataRegional {
        pub fn decode_reg_ext_value<D: Decoder>(
            &self,
            decoder: &mut D,
        ) -> Result<RegMapData_Type, D::Error> {
            RegMapData_Type::decode(decoder, Some(&self.reg_ext_value), &self.region_id)
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct MapDataRegional(pub SequenceOf<AnonymousMapDataRegional>);
    #[doc = "*"]
    #[doc = "* This DF is used to convey many types of geographic road information. At the current time its primary"]
    #[doc = "* use is to convey one or more intersection lane geometry maps within a single message. The map message content"]
    #[doc = "* includes such items as complex intersection descriptions, road segment descriptions, high speed curve outlines (used in"]
    #[doc = "* curve safety messages), and segments of roadway (used in some safety applications). A given single MapData message"]
    #[doc = "* may convey descriptions of one or more geographic areas or intersections. The contents of this message involve defining"]
    #[doc = "* the details of indexing systems that are in turn used by other messages to relate additional information (for example, the"]
    #[doc = "* signal phase and timing via the SPAT message) to events at specific geographic locations on the roadway."]
    #[doc = "*"]
    #[doc = "* @field timeStamp: time reference"]
    #[doc = "* @field msgIssueRevision: The MapData revision is defined by the data element **revision** for each intersection"]
    #[doc = "*                          geometry (see [ISO TS 19091] G.8.2.4.1). Therefore, an additional revision indication of the overall"]
    #[doc = "*                          MapData message is not used in this profile. It shall be set to \"0\" for this profile."]
    #[doc = "* @field layerType: There is no need to additionally identify the topological content by an additional identifier. The ASN.1"]
    #[doc = "*                   definition of the data frames **intersections** and **roadSegments** are clearly defined and need no"]
    #[doc = "*                   additional identifier. Therefore, this optional data element shall not be used in this profile."]
    #[doc = "* @field layerID: This profile extends the purpose of the **layerID** data element as defined in SAE J2735 as follows: For"]
    #[doc = "*                 large intersections, the length of a MapData description may exceed the maximum data length of the"]
    #[doc = "*                 communication message. Therefore, a fragmentation of the MapData message (at application layer) in"]
    #[doc = "*                 two or more MapData fragments may be executed. If no MapData fragmentation is needed, the **layerID**"]
    #[doc = "*                 shall not be used. For more details, see the definition of the data element @ref LayerID."]
    #[doc = "* @field intersections: All Intersection definitions."]
    #[doc = "* @field roadSegments: All roadway descriptions."]
    #[doc = "* @field dataParameters: Any meta data regarding the map contents."]
    #[doc = "* @field restrictionList: Any restriction ID tables which have established for these map entries"]
    #[doc = "* @field regional: This profile extends the MapData message with the regional data element @ref MapData-addGrpC"]
    #[doc = "*"]
    #[doc = "* @category: Road topology information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MapData {
        #[rasn(identifier = "timeStamp")]
        pub time_stamp: Option<MinuteOfTheYear>,
        #[rasn(identifier = "msgIssueRevision")]
        pub msg_issue_revision: MsgCount,
        #[rasn(identifier = "layerType")]
        pub layer_type: Option<LayerType>,
        #[rasn(identifier = "layerID")]
        pub layer_id: Option<LayerID>,
        pub intersections: Option<IntersectionGeometryList>,
        #[rasn(identifier = "roadSegments")]
        pub road_segments: Option<RoadSegmentList>,
        #[rasn(identifier = "dataParameters")]
        pub data_parameters: Option<DataParameters>,
        #[rasn(identifier = "restrictionList")]
        pub restriction_list: Option<RestrictionClassList>,
        pub regional: Option<MapDataRegional>,
    }
    impl MapData {
        pub fn new(
            time_stamp: Option<MinuteOfTheYear>,
            msg_issue_revision: MsgCount,
            layer_type: Option<LayerType>,
            layer_id: Option<LayerID>,
            intersections: Option<IntersectionGeometryList>,
            road_segments: Option<RoadSegmentList>,
            data_parameters: Option<DataParameters>,
            restriction_list: Option<RestrictionClassList>,
            regional: Option<MapDataRegional>,
        ) -> Self {
            Self {
                time_stamp,
                msg_issue_revision,
                layer_type,
                layer_id,
                intersections,
                road_segments,
                data_parameters,
                restriction_list,
                regional,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* The angle at which another lane path meets the current lanes at the node point. Typically found in the node"]
    #[doc = "* attributes and used to describe the angle of the departing or merging lane. Note that oblique and obtuse angles are allowed."]
    #[doc = "*"]
    #[doc = "* The value `-180` shall be used to represent data is not available or unknown"]
    #[doc = "*"]
    #[doc = "* @unit: 1.5 degrees from north"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-180..=180"))]
    pub struct MergeDivergeNodeAngle(pub i16);
    #[doc = "*"]
    #[doc = "* This DE expresses the number of elapsed minutes of the current year in the time system being used (typically UTC time)."]
    #[doc = "*"]
    #[doc = "* It is typically used to provide a longer range time stamp indicating when a message was created."]
    #[doc = "* Taken together with the DSecond data element, it provides a range of one full year with a resolution of 1 millisecond."]
    #[doc = "*"]
    #[doc = "* The value 527040 shall be used for invalid."]
    #[doc = "*"]
    #[doc = "* @note: It should be noted that at the yearly roll-over point there is no \"zero\" minute, in the same way that there was"]
    #[doc = "*        never a \"year zero\" at the very start of the common era (BC -> AD). By using the number of elapsed whole minutes here"]
    #[doc = "*        this issue is avoided and the first valid value of every new year is zero, followed by one, etc. Leap years are"]
    #[doc = "*        accommodated, as are leap seconds in the DSecond data concept."]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=527040"))]
    pub struct MinuteOfTheYear(pub u32);
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMovementEventRegional {
        #[rasn(value("0..=255"), identifier = "regionId")]
        pub region_id: u8,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousMovementEventRegional {
        pub fn new(region_id: u8, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    impl AnonymousMovementEventRegional {
        pub fn decode_reg_ext_value<D: Decoder>(
            &self,
            decoder: &mut D,
        ) -> Result<RegMovementEvent_Type, D::Error> {
            RegMovementEvent_Type::decode(decoder, Some(&self.reg_ext_value), &self.region_id)
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct MovementEventRegional(pub SequenceOf<AnonymousMovementEventRegional>);
    #[doc = "*"]
    #[doc = "* This DF contains details about a single movement. It is used by the movement state to"]
    #[doc = "* convey one of number of movements (typically occurring over a sequence of times) for a SignalGroupID."]
    #[doc = "*"]
    #[doc = "* @field eventState: Consisting of: Phase state (the basic 11 states), Directional, protected, or permissive state"]
    #[doc = "* @field timing: Timing Data in UTC time stamps for event includes start and min/max end times of phase confidence and estimated next occurrence"]
    #[doc = "* @field speeds: various speed advisories for use by general and specific types of vehicles supporting green-wave and other flow needs"]
    #[doc = "* @field regional: optional region specific data."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MovementEvent {
        #[rasn(identifier = "eventState")]
        pub event_state: MovementPhaseState,
        pub timing: Option<TimeChangeDetails>,
        pub speeds: Option<AdvisorySpeedList>,
        pub regional: Option<MovementEventRegional>,
    }
    impl MovementEvent {
        pub fn new(
            event_state: MovementPhaseState,
            timing: Option<TimeChangeDetails>,
            speeds: Option<AdvisorySpeedList>,
            regional: Option<MovementEventRegional>,
        ) -> Self {
            Self {
                event_state,
                timing,
                speeds,
                regional,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* This DF consists of a list of @ref MovementEvent entries."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16"))]
    pub struct MovementEventList(pub SequenceOf<MovementEvent>);
    #[doc = "*"]
    #[doc = "* This DF consists of a list of @ref MovementState entries."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=255"))]
    pub struct MovementList(pub SequenceOf<MovementState>);
    #[doc = "*"]
    #[doc = "* This DE provides the overall current state of the movement (in many cases a signal state), including its core phase state"]
    #[doc = "*  and an indication of whether this state is permissive or protected."]
    #[doc = "*"]
    #[doc = "* It is expected that the allowed transitions from one state to another will be defined by regional deployments. Not all"]
    #[doc = "* regions will use all states; however, no new states are to be defined. In most regions a regulatory body provides precise"]
    #[doc = "* legal definitions of these state changes. For example, in the US the MUTCD is used, as is indicated in the US regional"]
    #[doc = "* variant of the above image. In various regions and modes of transportation, the visual expression of these states varies"]
    #[doc = "* (the precise meaning of various color combinations, shapes, and/or flashing etc.). The below definition is designed to to"]
    #[doc = "* be independent of these regional conventions."]
    #[doc = "*"]
    #[doc = "* Values:"]
    #[doc = "* - `unavailable` - 0:         This state is used for unknown or error"]
    #[doc = "* - `dark` - 1:                The signal head is dark (unlit)"]
    #[doc = "* - `stop-Then-Proceed` - 2:   Often called **flashing red**"]
    #[doc = "*                              Driver Action:"]
    #[doc = "*                              - Stop vehicle at stop line."]
    #[doc = "*                              - Do not proceed unless it is safe."]
    #[doc = "*                              Note that the right to proceed either right or left when it is safe may be contained in the lane description to"]
    #[doc = "*                              handle what is called a **right on red**"]
    #[doc = "* - `stop-And-Remain` - 3:     e.g. called **red light**"]
    #[doc = "*                              Driver Action:"]
    #[doc = "*                              - Stop vehicle at stop line."]
    #[doc = "*                              - Do not proceed."]
    #[doc = "*                              Note that the right to proceed either right or left when it is safe may be contained in the lane description to"]
    #[doc = "*                              handle what is called a **right on red**"]
    #[doc = "* - `pre-Movement` - 4:        Not used in the US, red+yellow partly in EU"]
    #[doc = "*                              Driver Action:"]
    #[doc = "*                              - Stop vehicle."]
    #[doc = "*                              - Prepare to proceed (pending green)"]
    #[doc = "*                              - (Prepare for transition to green/go)"]
    #[doc = "* - `permissive-Movement-Allowed` - 5: Often called **permissive green**"]
    #[doc = "*                              Driver Action:"]
    #[doc = "*                              - Proceed with caution,"]
    #[doc = "*                              - must yield to all conflicting traffic"]
    #[doc = "*                              Conflicting traffic may be present in the intersection conflict area"]
    #[doc = "* - `protected-Movement-Allowed` - 6: Often called **protected green**"]
    #[doc = "*                              Driver Action:"]
    #[doc = "*                              - Proceed, tossing caution to the wind, in indicated (allowed) direction."]
    #[doc = "* - `permissive-clearance` - 7: Often called **permissive yellow**."]
    #[doc = "*                              The vehicle is not allowed to cross the stop bar if it is possible"]
    #[doc = "*                              to stop without danger."]
    #[doc = "*                              Driver Action:"]
    #[doc = "*                              - Prepare to stop."]
    #[doc = "*                              - Proceed if unable to stop,"]
    #[doc = "*                              - Clear Intersection."]
    #[doc = "*                              Conflicting traffic may be present in the intersection conflict area"]
    #[doc = "* - `protected-clearance` - 8:  Often called **protected yellow**"]
    #[doc = "*                              Driver Action:"]
    #[doc = "*                              - Prepare to stop."]
    #[doc = "*                              - Proceed if unable to stop, in indicated direction (to connected lane)"]
    #[doc = "*                              - Clear Intersection."]
    #[doc = "* - `caution-Conflicting-Traffic` - 9: Often called **flashing yellow**"]
    #[doc = "*                              Often used for extended periods of time"]
    #[doc = "*                              Driver Action:"]
    #[doc = "*                              - Proceed with caution,"]
    #[doc = "*                              Conflicting traffic may be present in the intersection conflict area"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum MovementPhaseState {
        unavailable = 0,
        dark = 1,
        #[rasn(identifier = "stop-Then-Proceed")]
        stop_Then_Proceed = 2,
        #[rasn(identifier = "stop-And-Remain")]
        stop_And_Remain = 3,
        #[rasn(identifier = "pre-Movement")]
        pre_Movement = 4,
        #[rasn(identifier = "permissive-Movement-Allowed")]
        permissive_Movement_Allowed = 5,
        #[rasn(identifier = "protected-Movement-Allowed")]
        protected_Movement_Allowed = 6,
        #[rasn(identifier = "permissive-clearance")]
        permissive_clearance = 7,
        #[rasn(identifier = "protected-clearance")]
        protected_clearance = 8,
        #[rasn(identifier = "caution-Conflicting-Traffic")]
        caution_Conflicting_Traffic = 9,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMovementStateRegional {
        #[rasn(value("0..=255"), identifier = "regionId")]
        pub region_id: u8,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousMovementStateRegional {
        pub fn new(region_id: u8, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    impl AnonymousMovementStateRegional {
        pub fn decode_reg_ext_value<D: Decoder>(
            &self,
            decoder: &mut D,
        ) -> Result<RegMovementState_Type, D::Error> {
            RegMovementState_Type::decode(decoder, Some(&self.reg_ext_value), &self.region_id)
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct MovementStateRegional(pub SequenceOf<AnonymousMovementStateRegional>);
    #[doc = "*"]
    #[doc = "* This DF is used to convey various information about the current or future movement state of"]
    #[doc = "* a designated collection of one or more lanes of a common type. This is referred to as the GroupID. Note that lane object"]
    #[doc = "* types supported include both motorized vehicle lanes as well as pedestrian lanes and dedicated rail and transit lanes. Of"]
    #[doc = "* the reported data elements, the time to change (the time remaining in the current state) is often of the most value. Lanes"]
    #[doc = "* with a common state (typically adjacent sets of lanes in an approach) in a signalized intersection will have individual lane"]
    #[doc = "* values such as total vehicle counts, summed. It is used in the SPAT message to convey every active movement in a"]
    #[doc = "* given intersection so that vehicles, when combined with certain map information, can determine the state of the signal phases."]
    #[doc = "*"]
    #[doc = "* @field movementName: uniquely defines movement by name human readable name for intersection to be used only in debug mode."]
    #[doc = "* @field signalGroup: is used to map to lists of lanes (and their descriptions) which this MovementState data applies to."]
    #[doc = "* @field state-time-speed: Consisting of sets of movement data with @ref SignalPhaseState, @ref TimeChangeDetail and @ref AdvisorySpeed"]
    #[doc = "*                          *Note:* one or more of the movement events may be for a future time and that this allows conveying multiple"]
    #[doc = "*                          predictive phase and movement timing for various uses for the current signal group."]
    #[doc = "* @field maneuverAssistList: This information may also be placed in the @ref IntersectionState when common information applies to different lanes in the same way"]
    #[doc = "* @field regional: optional region specific data."]
    #[doc = "*"]
    #[doc = "* @note: Note that the value given for the time to change will vary in many actuated signalized intersections based on"]
    #[doc = "*      the sensor data received during the phase. The data transmitted always reflects the then most current timemark value"]
    #[doc = "*      (which is the point in UTC time when the change will occur). As an example, in a phase which may vary from 15 to 25"]
    #[doc = "*      seconds of duration based on observed traffic flows, a time to change value of 15 seconds in the future might be"]
    #[doc = "*      transmitted for many consecutive seconds (and the time mark value extended for as much as 10 seconds depending on"]
    #[doc = "*      the extension time logic used by the controller before it either times out or gaps out), followed by a final time mark value"]
    #[doc = "*      reflecting the decreasing values as the time runs out, presuming the value was not again extended to a new time mark"]
    #[doc = "*      due to other detection events. The time to change element can therefore generally be regarded as a guaranteed minimum"]
    #[doc = "*      value of the time that will elapse unless a preemption event occurs."]
    #[doc = "*"]
    #[doc = "*      In use, the @ref SignalGroupID element is matched to lanes that are members of that ID. The type of lane (vehicle, crosswalk,"]
    #[doc = "*      etc.) is known by the lane description as well as its allowed maneuvers and any vehicle class restrictions. Every lane type"]
    #[doc = "*      is treated the same way (cross walks map to suitable meanings, etc.). Lane objects which are not part of the sequence of"]
    #[doc = "*      signalized lanes do not appear in any GroupID. The visual details of how a given signal phase is presented to a mobile"]
    #[doc = "*      user will vary based on lane type and with regional conventions. Not all signal states will be used in all regional"]
    #[doc = "*      deployments. For example, a pre-green visual indication is not generally found in US deployments. Under such operating"]
    #[doc = "*      conditions, the unused phase states are simply skipped."]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MovementState {
        #[rasn(identifier = "movementName")]
        pub movement_name: Option<DescriptiveName>,
        #[rasn(identifier = "signalGroup")]
        pub signal_group: SignalGroupID,
        #[rasn(identifier = "state-time-speed")]
        pub state_time_speed: MovementEventList,
        #[rasn(identifier = "maneuverAssistList")]
        pub maneuver_assist_list: Option<ManeuverAssistList>,
        pub regional: Option<MovementStateRegional>,
    }
    impl MovementState {
        pub fn new(
            movement_name: Option<DescriptiveName>,
            signal_group: SignalGroupID,
            state_time_speed: MovementEventList,
            maneuver_assist_list: Option<ManeuverAssistList>,
            regional: Option<MovementStateRegional>,
        ) -> Self {
            Self {
                movement_name,
                signal_group,
                state_time_speed,
                maneuver_assist_list,
                regional,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* This DE is used to provide a sequence number within a stream of messages with the same DSRCmsgID and from the same sender."]
    #[doc = "* A sender may initialize this element to any value in the range 0-127 when sending the first message with a given DSRCmsgID,"]
    #[doc = "* or if the sender has changed identity (e.g. by changing its TemporaryID) since sending the most recent message with that DSRCmsgID."]
    #[doc = "*"]
    #[doc = "* Depending on the application the sequence number may change with every message or may remain fixed during a stream of messages when the content within each"]
    #[doc = "* message has not changed from the prior message sent. For this element, the value after 127 is zero."]
    #[doc = "*"]
    #[doc = "* The receipt of a non-sequential MsgCount value (from the same sending device and message type) implies that one or"]
    #[doc = "* more messages from that sending device may have been lost, unless MsgCount has been re-initialized due to an identity"]
    #[doc = "* change."]
    #[doc = "*"]
    #[doc = "* @note: In the absence of additional requirements defined in a standard using this data element, the follow guidelines shall be used."]
    #[doc = "*"]
    #[doc = "* In usage, some devices change their Temporary ID frequently, to prevent identity tracking, while others do not. A change"]
    #[doc = "* in Temporary ID data element value (which also changes the message contents in which it appears) implies that the"]
    #[doc = "* MsgCount may also change value."]
    #[doc = "*"]
    #[doc = "* If a sender is composing a message with new content with a given DSRCmsgID, and the TemporaryID has not changed"]
    #[doc = "* since it sent the previous message, the sender shall increment the previous value."]
    #[doc = "* If a sender is composing a message with new content with a given DSRCmsgID, and the TemporaryID has changed since"]
    #[doc = "* it sent the previous message, the sender may set the MsgCount element to any valid value in the range (including"]
    #[doc = "* incrementing the previous value)."]
    #[doc = "*"]
    #[doc = "* If a sender is composing a message with the same content as the most recent message with the same DSRCmsgID, and"]
    #[doc = "* less than 10 seconds have elapsed since it sent the previous message with that DSRCmsgID, the sender will use the"]
    #[doc = "* same MsgCount as sent in the previous message."]
    #[doc = "*"]
    #[doc = "* If a sender is composing a message with the same content as the most recent message with the same DSRCmsgID, and"]
    #[doc = "* at least 10 seconds have elapsed since it sent the previous message with that DSRCmsgID, the sender may set the"]
    #[doc = "* MsgCount element to any valid value in the range; this includes the re-use of the previous value."]
    #[doc = "*"]
    #[doc = "* If a sending device sends more than one stream of messages from message types that utilize the MsgCount element, it"]
    #[doc = "* shall maintain a separate MsgCount state for each message type so that the MsgCount value in a given message"]
    #[doc = "* identifies its place in the stream of that message type. The MsgCount element is a function only of the message type in a"]
    #[doc = "* given sending device, not of the one or more applications in that device which may be sending the same type of message."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=127"))]
    pub struct MsgCount(pub u8);
    #[doc = "*"]
    #[doc = "* A 64-bit node type with lat-long values expressed in one tenth of a micro degree."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "Node-LLmD-64b")]
    pub struct NodeLLmD64b {
        pub lon: Longitude,
        pub lat: Latitude,
    }
    impl NodeLLmD64b {
        pub fn new(lon: Longitude, lat: Latitude) -> Self {
            Self { lon, lat }
        }
    }
    #[doc = "*"]
    #[doc = "* A 20-bit node type with offset values from the last point in X and Y."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "Node-XY-20b")]
    pub struct NodeXY20b {
        pub x: OffsetB10,
        pub y: OffsetB10,
    }
    impl NodeXY20b {
        pub fn new(x: OffsetB10, y: OffsetB10) -> Self {
            Self { x, y }
        }
    }
    #[doc = "*"]
    #[doc = "* A 22-bit node type with offset values from the last point in X and Y."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "Node-XY-22b")]
    pub struct NodeXY22b {
        pub x: OffsetB11,
        pub y: OffsetB11,
    }
    impl NodeXY22b {
        pub fn new(x: OffsetB11, y: OffsetB11) -> Self {
            Self { x, y }
        }
    }
    #[doc = "*"]
    #[doc = "* A 24-bit node type with offset values from the last point in X and Y."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "Node-XY-24b")]
    pub struct NodeXY24b {
        pub x: OffsetB12,
        pub y: OffsetB12,
    }
    impl NodeXY24b {
        pub fn new(x: OffsetB12, y: OffsetB12) -> Self {
            Self { x, y }
        }
    }
    #[doc = "*"]
    #[doc = "* A 26-bit node type with offset values from the last point in X and Y."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "Node-XY-26b")]
    pub struct NodeXY26b {
        pub x: OffsetB13,
        pub y: OffsetB13,
    }
    impl NodeXY26b {
        pub fn new(x: OffsetB13, y: OffsetB13) -> Self {
            Self { x, y }
        }
    }
    #[doc = "*"]
    #[doc = "* A 28-bit node type with offset values from the last point in X and Y."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "Node-XY-28b")]
    pub struct NodeXY28b {
        pub x: OffsetB14,
        pub y: OffsetB14,
    }
    impl NodeXY28b {
        pub fn new(x: OffsetB14, y: OffsetB14) -> Self {
            Self { x, y }
        }
    }
    #[doc = "*"]
    #[doc = "* A 32-bit node type with offset values from the last point in X and Y."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "Node-XY-32b")]
    pub struct NodeXY32b {
        pub x: OffsetB16,
        pub y: OffsetB16,
    }
    impl NodeXY32b {
        pub fn new(x: OffsetB16, y: OffsetB16) -> Self {
            Self { x, y }
        }
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousNodeAttributeSetXYRegional {
        #[rasn(value("0..=255"), identifier = "regionId")]
        pub region_id: u8,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousNodeAttributeSetXYRegional {
        pub fn new(region_id: u8, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    impl AnonymousNodeAttributeSetXYRegional {
        pub fn decode_reg_ext_value<D: Decoder>(
            &self,
            decoder: &mut D,
        ) -> Result<RegNodeAttributeSetXY_Type, D::Error> {
            RegNodeAttributeSetXY_Type::decode(decoder, Some(&self.reg_ext_value), &self.region_id)
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct NodeAttributeSetXYRegional(pub SequenceOf<AnonymousNodeAttributeSetXYRegional>);
    #[doc = "*"]
    #[doc = "* All the node attributes defined in this DF are valid in the direction of"]
    #[doc = "* node declaration and not in driving direction (i.e. along the sequence of the declared nodes). E.g. node"]
    #[doc = "* attributes of an **ingress** or an **egress** lane are defined from the conflict area (first node) to the"]
    #[doc = "* outside of the intersection (last node). Node attributes with **left** and **right** in their name are also"]
    #[doc = "* defined in the direction of the node declaration. This allows using attributes in a unambigious way also"]
    #[doc = "* for lanes with biderctional driving. See the following attribuets examples for additianl explanations."]
    #[doc = "*"]
    #[doc = "* @field localNode: Attribute states which pertain to this node point"]
    #[doc = "* @field disabled: Attribute states which are disabled at this node point"]
    #[doc = "* @field enabled: Attribute states which are enabled at this node point and which remain enabled until disabled or the lane ends"]
    #[doc = "* @field data: Attributes which require an additional data values some of these are local to the node point, while others"]
    #[doc = "*              persist with the provided values until changed and this is indicated in each entry"]
    #[doc = "* @field dWidth: A value added to the current lane width at this node and from this node onwards, in 1cm steps"]
    #[doc = "*               lane width between nodes are a linear taper between pts the value of zero shall not be sent here."]
    #[doc = "* @field dElevation: A value added to the current Elevation at this node from this node onwards, in 10cm steps"]
    #[doc = "*                    elevations between nodes are a linear taper between pts the value of zero shall not be sent here"]
    #[doc = "* @field regional: optional region specific data."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct NodeAttributeSetXY {
        #[rasn(identifier = "localNode")]
        pub local_node: Option<NodeAttributeXYList>,
        pub disabled: Option<SegmentAttributeXYList>,
        pub enabled: Option<SegmentAttributeXYList>,
        pub data: Option<LaneDataAttributeList>,
        #[rasn(identifier = "dWidth")]
        pub d_width: Option<OffsetB10>,
        #[rasn(identifier = "dElevation")]
        pub d_elevation: Option<OffsetB10>,
        pub regional: Option<NodeAttributeSetXYRegional>,
    }
    impl NodeAttributeSetXY {
        pub fn new(
            local_node: Option<NodeAttributeXYList>,
            disabled: Option<SegmentAttributeXYList>,
            enabled: Option<SegmentAttributeXYList>,
            data: Option<LaneDataAttributeList>,
            d_width: Option<OffsetB10>,
            d_elevation: Option<OffsetB10>,
            regional: Option<NodeAttributeSetXYRegional>,
        ) -> Self {
            Self {
                local_node,
                disabled,
                enabled,
                data,
                d_width,
                d_elevation,
                regional,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* This DE is an enumerated list of attributes which can pertain to the current node"]
    #[doc = "* point. The **scope** of these values is limited to the node itself. That is, unlike other types of attributes which can be"]
    #[doc = "* switched on or off at any given node (and hence pertains to one or more segments), the DE_NodeAttribute is local to the"]
    #[doc = "* node in which it is found. These attributes are all binary flags in that they do not need to convey any additional data. Other"]
    #[doc = "* attributes allow sending short data values to reflect a setting which is set and persists in a similar fashion."]
    #[doc = "*"]
    #[doc = "*  - reserved:             do not use"]
    #[doc = "*  - stopLine:             point where a mid-path stop line exists. See also **do not block** for segments"]
    #[doc = "*  - roundedCapStyleA:     Used to control final path rounded end shape with edge of curve at final point in a circle"]
    #[doc = "*  - roundedCapStyleB:     Used to control final path rounded end shape with edge of curve extending 50% of width past final point in a circle"]
    #[doc = "*  - mergePoint:           merge with 1 or more lanes"]
    #[doc = "*  - divergePoint:         diverge with 1 or more lanes"]
    #[doc = "*  - downstreamStopLine:   downstream intersection (a 2nd intersection) stop line"]
    #[doc = "*  - downstreamStartNode:  downstream intersection (a 2nd intersection) start node"]
    #[doc = "*  - closedToTraffic:      where a pedestrian may NOT go to be used during construction events"]
    #[doc = "*  - safeIsland:           a pedestrian safe stopping point also called a traffic island"]
    #[doc = "*                          This usage described a point feature on a path, other entries can describe a path"]
    #[doc = "*  - curbPresentAtStepOff: the sidewalk to street curb is NOT angled where it meets the edge of the roadway (user must step up/down)"]
    #[doc = "*  - hydrantPresent:       Or other services access"]
    #[doc = "*"]
    #[doc = "* @note: See usage examples in [ISO TS 19091] G.8.2.8"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum NodeAttributeXY {
        reserved = 0,
        stopLine = 1,
        roundedCapStyleA = 2,
        roundedCapStyleB = 3,
        mergePoint = 4,
        divergePoint = 5,
        downstreamStopLine = 6,
        downstreamStartNode = 7,
        closedToTraffic = 8,
        safeIsland = 9,
        curbPresentAtStepOff = 10,
        hydrantPresent = 11,
    }
    #[doc = "*"]
    #[doc = "* This DF consists of a list of @ref NodeAttributeXY entries."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8"))]
    pub struct NodeAttributeXYList(pub SequenceOf<NodeAttributeXY>);
    #[doc = "*"]
    #[doc = "* This DF provides the sequence of signed offset node point values for determining the Xs and Ys"]
    #[doc = "* (and possibly Width or Zs when present), using the then current Position3D object to build a path for the centerline of"]
    #[doc = "* the subject lane type. Each X,Y point is referred to as a Node Point. The straight line paths between these points are"]
    #[doc = "* referred to as Segments."]
    #[doc = "* All nodes may have various optional attributes the state of which can vary along the path and which are enabled and"]
    #[doc = "* disabled by the sequence of objects found in the list of node structures. Refer to the explanatory text in Section 11 for a"]
    #[doc = "* description of how to correctly encode and decode this type of the data element. As a simple example, a motor vehicle"]
    #[doc = "* lane may have a section of the overall lane path marked \"do not block\", indicating that vehicles should not come to a stop"]
    #[doc = "* and remain in that region. This is encoded in the Node data structures by an element in one node to indicate the start of"]
    #[doc = "* the \"do not block\" lane attributes at a given offset, and then by a termination element when this attribute is set false. Other"]
    #[doc = "* types of elements in the segment choice allow inserting attributes containing data values affecting the segment or the"]
    #[doc = "* node."]
    #[doc = "*"]
    #[doc = "* @field nodes: a lane made up of two or more XY node points and any attributes defined in those nodes"]
    #[doc = "* @field computed: a lane path computed by translating the data defined by another lane"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum NodeListXY {
        nodes(NodeSetXY),
        computed(ComputedLane),
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct NodeOffsetPointXYRegional {
        #[rasn(value("0..=255"), identifier = "regionId")]
        pub region_id: u8,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl NodeOffsetPointXYRegional {
        pub fn new(region_id: u8, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    impl NodeOffsetPointXYRegional {
        pub fn decode_reg_ext_value<D: Decoder>(
            &self,
            decoder: &mut D,
        ) -> Result<RegNodeOffsetPointXY_Type, D::Error> {
            RegNodeOffsetPointXY_Type::decode(decoder, Some(&self.reg_ext_value), &self.region_id)
        }
    }
    #[doc = "*"]
    #[doc = "* This DF presents a structure to hold different sized data frames for a single node"]
    #[doc = "* point in a lane. Nodes are described in terms of X and Y offsets in units of 1 centimeter (when zoom is 1:1). Changes in"]
    #[doc = "* elevation and in the lane width can be expressed in a similar way with the optional Attributes data entry which appears"]
    #[doc = "* alongside the NodeOffsetPoint in use."]
    #[doc = "*"]
    #[doc = "* The choice of which node type is driven by the magnitude (size) of the offset data to be encoded. When the distance from"]
    #[doc = "* the last node point is smaller, the smaller entries can (and should) be chosen"]
    #[doc = "* Each single selected node is computed as an X and Y offset from the prior node point unless one of the entries reflecting"]
    #[doc = "* a complete lat-long representation is selected. In this case, subsequent entries become offsets from that point. This ability"]
    #[doc = "* was added for assistance with the development, storage, and back office exchange of messages where message size is"]
    #[doc = "* not a concern and should not be sent over the air due to its additional message payload size."]
    #[doc = "*"]
    #[doc = "* The general usage guidance is to construct the content of each lane node point with the smallest possible element to"]
    #[doc = "* conserve message size. However, using an element which is larger than needed is not a violation of the ASN.1 rules."]
    #[doc = "*"]
    #[doc = "* @field node-XY1:    node is within 5.11m of last node"]
    #[doc = "* @field node-XY2:    node is within 10.23m of last node"]
    #[doc = "* @field node-XY3:    node is within 20.47m of last node"]
    #[doc = "* @field node-XY4:    node is within 40.96m of last node"]
    #[doc = "* @field node-XY5:    node is within 81.91m of last node"]
    #[doc = "* @field node-XY6:    node is within 327.67m of last node"]
    #[doc = "* @field node-LatLon: node is a full 32b Lat/Lon range"]
    #[doc = "* @field regional: optional region specific data."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum NodeOffsetPointXY {
        #[rasn(identifier = "node-XY1")]
        node_XY1(NodeXY20b),
        #[rasn(identifier = "node-XY2")]
        node_XY2(NodeXY22b),
        #[rasn(identifier = "node-XY3")]
        node_XY3(NodeXY24b),
        #[rasn(identifier = "node-XY4")]
        node_XY4(NodeXY26b),
        #[rasn(identifier = "node-XY5")]
        node_XY5(NodeXY28b),
        #[rasn(identifier = "node-XY6")]
        node_XY6(NodeXY32b),
        #[rasn(identifier = "node-LatLon")]
        node_LatLon(NodeLLmD64b),
        regional(NodeOffsetPointXYRegional),
    }
    #[doc = "*"]
    #[doc = "* This DF consists of a list of Node entries using XY offsets."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("2..=63"))]
    pub struct NodeSetXY(pub SequenceOf<NodeXY>);
    #[doc = "*"]
    #[doc = "* This DF presents a structure to hold data for a single node point in a path. Each selected node"]
    #[doc = "* has an X and Y offset from the prior node point (or a complete lat-long representation in some cases) as well as optional"]
    #[doc = "* attribute information. The node list for a lane (or other object) is made up of a sequence of these to describe the desired"]
    #[doc = "* path. The X,Y points are selected to reflect the centerline of the path with sufficient accuracy for the intended applications."]
    #[doc = "* Simple lanes can be adequately described with only two node points, while lanes with curvature may require more points."]
    #[doc = "* Changes to the lane width and elevation can be expressed in the NodeAttributes entry, as well as various attributes that"]
    #[doc = "* pertain to either the current node point or to one of more subsequent segments along the list of lane node points. As a"]
    #[doc = "* broad concept, NodeAttributes are used to describe aspects of the lane that persist for only a portion of the overall lane"]
    #[doc = "* path (either at a node or over a set of segments)."]
    #[doc = "* A further description of the use of the NodeOffsetPoint and the Attributes data concepts can be found in the data"]
    #[doc = "* dictionary entries for each one. Note that each allows regional variants to be supported as well."]
    #[doc = "*"]
    #[doc = "* @field delta:      A choice of which X,Y offset value to use this includes various delta values as well a regional choices."]
    #[doc = "* @field attributes: Any optional Attributes which are needed. This includes changes to the current lane width and elevation."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct NodeXY {
        pub delta: NodeOffsetPointXY,
        pub attributes: Option<NodeAttributeSetXY>,
    }
    impl NodeXY {
        pub fn new(delta: NodeOffsetPointXY, attributes: Option<NodeAttributeSetXY>) -> Self {
            Self { delta, attributes }
        }
    }
    #[doc = "* This DF is the ODG Addition for Legancy R09 telegrams."]
    #[doc = "* "]
    #[doc = "* @field reportingPoint: reporting point as of R09 (maps to R09 field M Meldepunktnummer)"]
    #[doc = "* @field priorityLevel:  priority level as of R09 (maps to R09 field P Prioritaet)"]
    #[doc = "* @field length:         train length point as of R09 (maps to R09 field A Zuglaenge)"]
    #[doc = "* @field route:          route as of R09 (maps to R09 field K Kursnummer)"]
    #[doc = "* @field line:           line as of R09 (maps to R09 field L Liniennummer)"]
    #[doc = "* @field direction:      direction as of R09 (maps to R09 field H Richtung von Hand)"]
    #[doc = "* @field tour:           tour as of R09 (maps to R09 field Z Zielnummer)"]
    #[doc = "* @field version:        version of R09"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V2.2.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct OcitRequestorDescriptionContainer {
        #[rasn(identifier = "reportingPoint")]
        pub reporting_point: Option<ReportingPoint>,
        #[rasn(identifier = "priorityLevel")]
        pub priority_level: Option<PriorityLevel>,
        pub length: Option<TrainLength>,
        pub route: Option<RouteNumber>,
        pub line: Option<LineNumber>,
        pub direction: Option<TransitDirection>,
        pub tour: Option<TourNumber>,
        pub version: Option<VersionId>,
    }
    impl OcitRequestorDescriptionContainer {
        pub fn new(
            reporting_point: Option<ReportingPoint>,
            priority_level: Option<PriorityLevel>,
            length: Option<TrainLength>,
            route: Option<RouteNumber>,
            line: Option<LineNumber>,
            direction: Option<TransitDirection>,
            tour: Option<TourNumber>,
            version: Option<VersionId>,
        ) -> Self {
            Self {
                reporting_point,
                priority_level,
                length,
                route,
                line,
                direction,
                tour,
                version,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* A 9-bit delta offset in X, Y or Z direction from some known point. For non-vehicle centric coordinate frames of"]
    #[doc = "* reference, offset is positive to the East (X) and to the North (Y) directions. The most negative value shall be used to"]
    #[doc = "* indicate an unknown value."]
    #[doc = "* a range of +- 2.55 meters"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "Offset-B09", value("-256..=255"))]
    pub struct OffsetB09(pub i16);
    #[doc = "*"]
    #[doc = "* A 10-bit delta offset in X, Y or Z direction from some known point. For non-vehicle centric coordinate frames of"]
    #[doc = "* reference, offset is positive to the East (X) and to the North (Y) directions. The most negative value shall be used to"]
    #[doc = "* indicate an unknown value."]
    #[doc = "* a range of +- 5.11 meters"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "Offset-B10", value("-512..=511"))]
    pub struct OffsetB10(pub i16);
    #[doc = "*"]
    #[doc = "* An 11-bit delta offset in X or Y direction from some known point. For non-vehicle centric coordinate frames of"]
    #[doc = "* reference, offset is positive to the East (X) and to the North (Y) directions. The most negative value shall be used to"]
    #[doc = "* indicate an unknown value."]
    #[doc = "* a range of +- 10.23 meters"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "Offset-B11", value("-1024..=1023"))]
    pub struct OffsetB11(pub i16);
    #[doc = "*"]
    #[doc = "* A 12-bit delta offset in X, Y or Z direction from some known point. For non-vehicle centric coordinate frames of"]
    #[doc = "* reference, non-vehicle centric coordinate frames of reference, offset is positive to the East (X) and to the North (Y)"]
    #[doc = "* directions. The most negative value shall be used to indicate an unknown value."]
    #[doc = "* a range of +- 20.47 meters"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "Offset-B12", value("-2048..=2047"))]
    pub struct OffsetB12(pub i16);
    #[doc = "*"]
    #[doc = "* A 13-bit delta offset in X or Y direction from some known point. For non-vehicle centric coordinate frames of"]
    #[doc = "* reference, offset is positive to the East (X) and to the North (Y) directions. The most negative value shall be used to"]
    #[doc = "* indicate an unknown value."]
    #[doc = "* a range of +- 40.95 meters"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "Offset-B13", value("-4096..=4095"))]
    pub struct OffsetB13(pub i16);
    #[doc = "*"]
    #[doc = "* A 14-bit delta offset in X or Y direction from some known point. For non-vehicle centric coordinate frames of"]
    #[doc = "* reference, offset is positive to the East (X) and to the North (Y) directions."]
    #[doc = "* a range of +- 81.91 meters"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "Offset-B14", value("-8192..=8191"))]
    pub struct OffsetB14(pub i16);
    #[doc = "*"]
    #[doc = "* A 16-bit delta offset in X, Y or Z direction from some known point. For non-vehicle centric coordinate frames of"]
    #[doc = "* reference, offset is positive to the East (X) and to the North (Y) directions. The most negative value shall be used to"]
    #[doc = "* indicate an unknown value."]
    #[doc = "* a range of +- 327.68 meters"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "Offset-B16", value("-32768..=32767"))]
    pub struct OffsetB16(pub i16);
    #[doc = "*"]
    #[doc = "* This DF is a sequence of lane IDs which refers to lane objects that overlap or overlay the current lane's spatial path."]
    #[doc = "*"]
    #[doc = "* Contains the unique ID numbers for any lane object which have spatial paths that overlay (run on top of, and not"]
    #[doc = "* simply cross with) the current lane."]
    #[doc = "* Such as a train path that overlays a motor vehicle lane object for a roadway segment."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=5"))]
    pub struct OverlayLaneList(pub SequenceOf<LaneID>);
    #[doc = "*"]
    #[doc = "* This DE is used to provide an indication of whether Pedestrians and/or Bicyclists have been detected in the crossing lane."]
    #[doc = "* true if ANY Pedestrians or Bicyclists are detected crossing the target lane or lanes"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(delegate)]
    pub struct PedestrianBicycleDetect(pub bool);
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousPosition3DRegional {
        #[rasn(value("0..=255"), identifier = "regionId")]
        pub region_id: u8,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousPosition3DRegional {
        pub fn new(region_id: u8, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    impl AnonymousPosition3DRegional {
        pub fn decode_reg_ext_value<D: Decoder>(
            &self,
            decoder: &mut D,
        ) -> Result<RegPosition3D_Type, D::Error> {
            RegPosition3D_Type::decode(decoder, Some(&self.reg_ext_value), &self.region_id)
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct Position3DRegional(pub SequenceOf<AnonymousPosition3DRegional>);
    #[doc = "*"]
    #[doc = "* This DF provides a precise location in the WGS-84 coordinate system, from which short"]
    #[doc = "* offsets may be used to create additional data using a flat earth projection centered on this location. Position3D is typically"]
    #[doc = "* used in the description of maps and intersections, as well as signs and traveler data."]
    #[doc = "*"]
    #[doc = "* @field lat: Latitude in 1/10th microdegrees"]
    #[doc = "* @field long: Longitude in 1/10th microdegrees"]
    #[doc = "* @field elevation: The elevation information is defined by the regional extension (see module ETSI-ITS-DSRC-AddGrpC). "]
    #[doc = "*                   Therefore, the **elevation** data element of **DF_Position3D** is not used."]
    #[doc = "* @field regional: optional region specific data."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct Position3D {
        pub lat: Latitude,
        pub long: Longitude,
        pub elevation: Option<Elevation>,
        pub regional: Option<Position3DRegional>,
    }
    impl Position3D {
        pub fn new(
            lat: Latitude,
            long: Longitude,
            elevation: Option<Elevation>,
            regional: Option<Position3DRegional>,
        ) -> Self {
            Self {
                lat,
                long,
                elevation,
                regional,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum PositionConfidence {
        unavailable = 0,
        a500m = 1,
        a200m = 2,
        a100m = 3,
        a50m = 4,
        a20m = 5,
        a10m = 6,
        a5m = 7,
        a2m = 8,
        a1m = 9,
        a50cm = 10,
        a20cm = 11,
        a10cm = 12,
        a5cm = 13,
        a2cm = 14,
        a1cm = 15,
    }
    #[doc = "*"]
    #[doc = "* This DF combines multiple related bit fields into a single concept."]
    #[doc = "*"]
    #[doc = "* @field pos:       confidence for both horizontal directions"]
    #[doc = "* @field elevation: confidence for vertical direction"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct PositionConfidenceSet {
        pub pos: PositionConfidence,
        pub elevation: ElevationConfidence,
    }
    impl PositionConfidenceSet {
        pub fn new(pos: PositionConfidence, elevation: ElevationConfidence) -> Self {
            Self { pos, elevation }
        }
    }
    #[doc = "*"]
    #[doc = "* This DF consists of various parameters of quality used to model the accuracy of the"]
    #[doc = "* positional determination with respect to each given axis."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct PositionalAccuracy {
        #[rasn(identifier = "semiMajor")]
        pub semi_major: SemiMajorAxisAccuracy,
        #[rasn(identifier = "semiMinor")]
        pub semi_minor: SemiMinorAxisAccuracy,
        pub orientation: SemiMajorAxisOrientation,
    }
    impl PositionalAccuracy {
        pub fn new(
            semi_major: SemiMajorAxisAccuracy,
            semi_minor: SemiMinorAxisAccuracy,
            orientation: SemiMajorAxisOrientation,
        ) -> Self {
            Self {
                semi_major,
                semi_minor,
                orientation,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* This DF consists of a list of RegionalSignalControlZone entries."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"))]
    pub struct PreemptPriorityList(pub SequenceOf<SignalControlZone>);
    #[doc = "*"]
    #[doc = "* This DE is used in the @ref PrioritizationResponse data frame to indicate the"]
    #[doc = "* general status of a prior prioritization request."]
    #[doc = "*"]
    #[doc = "* - `unknown`           - 0: Unknown state"]
    #[doc = "* - `requested`         - 1: This prioritization request was detected by the traffic controller"]
    #[doc = "* - `processing`        - 2: Checking request (request is in queue, other requests are prior)"]
    #[doc = "* - `watchOtherTraffic` - 3: Cannot give full permission, therefore watch for other traffic. Note that other requests may be present"]
    #[doc = "* - `granted`           - 4: Intervention was successful and now prioritization is active"]
    #[doc = "* - `rejected`          - 5: The prioritization or preemption request was rejected by the traffic controller"]
    #[doc = "* - `maxPresence`       - 6: The Request has exceeded maxPresence time. Used when the controller has determined that the requester should then back off and request an alternative."]
    #[doc = "* - `reserviceLocked`   - 7: Prior conditions have resulted in a reservice"]
    #[doc = "*                            locked event: the controller requires the passage of time before another similar request will be accepted"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum PrioritizationResponseStatus {
        unknown = 0,
        requested = 1,
        processing = 2,
        watchOtherTraffic = 3,
        granted = 4,
        rejected = 5,
        maxPresence = 6,
        reserviceLocked = 7,
    }
    #[doc = "*"]
    #[doc = "* This DE is used to provide the R09 priority."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V2.2.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct PriorityLevel(pub u8);
    #[doc = "*"]
    #[doc = "* This DE provides a means to indicate if a request (found in the Signal RequestMessage) represents"]
    #[doc = "* a new service request, a request update, or a request cancellation for either preemption or priority services."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum PriorityRequestType {
        priorityRequestTypeReserved = 0,
        priorityRequest = 1,
        priorityRequestUpdate = 2,
        priorityCancellation = 3,
    }
    #[doc = "*"]
    #[doc = "* This DE provides the specific revision of the RTCM standard which is being used. This is"]
    #[doc = "* helpful to know precisely the mapping of the message types to their definitions, as well as some minor transport layer"]
    #[doc = "* ordering details when received in the mobile unit. All RTCM SC-104 messages follow a common message numbering"]
    #[doc = "* method (wherein all defined messages are given unique values) which can be decoded from the initial octets of the"]
    #[doc = "* message. This operation is typically performed by the GNSS rover that consumes the messages, so it is transparent at"]
    #[doc = "* the DSRC message set level."]
    #[doc = "*"]
    #[doc = "* Values:"]
    #[doc = "* - `rtcmRev2`:  Std 10402.x et al"]
    #[doc = "* - `rtcmRev3`:  Std 10403.x et al"]
    #[doc = "*"]
    #[doc = "* @note:: In order to fully support the use of networked transport of RTCM corrections (so-called Ntrip systems), the"]
    #[doc = "*         enumerated list of protocol types provides for all the common types outlined in RTCM Standard 10410.0, Appendix B. It is"]
    #[doc = "*         anticipated that revisions 3.x and 2.3 will predominate in practice as they do today. It should also be noted that RTCM"]
    #[doc = "*         standards use the term `byte` for an 8-bit value, while in this standard the term `octet` is used."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "RTCM-Revision")]
    #[non_exhaustive]
    pub enum RTCMRevision {
        unknown = 0,
        rtcmRev2 = 1,
        rtcmRev3 = 2,
        reserved = 3,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousRTCMcorrectionsRegional {
        #[rasn(value("0..=255"), identifier = "regionId")]
        pub region_id: u8,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousRTCMcorrectionsRegional {
        pub fn new(region_id: u8, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    impl AnonymousRTCMcorrectionsRegional {
        pub fn decode_reg_ext_value<D: Decoder>(
            &self,
            decoder: &mut D,
        ) -> Result<RegRTCMcorrections_Type, D::Error> {
            RegRTCMcorrections_Type::decode(decoder, Some(&self.reg_ext_value), &self.region_id)
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct RTCMcorrectionsRegional(pub SequenceOf<AnonymousRTCMcorrectionsRegional>);
    #[doc = "*"]
    #[doc = "* This DF is used to encapsulate RTCM differential corrections for GPS and other radio"]
    #[doc = "* navigation signals as defined by the RTCM (Radio Technical Commission For Maritime Services) special committee"]
    #[doc = "* number 104 in its various standards. Here, in the work of DSRC, these messages are \"wrapped\" for transport on the"]
    #[doc = "* DSRC media, and then can be re-constructed back into the final expected formats defined by the RTCM standard and"]
    #[doc = "* used directly by various positioning systems to increase the absolute and relative accuracy estimates produced."]
    #[doc = "*"]
    #[doc = "* @field msgCnt: monotonic incrementing identifier."]
    #[doc = "* @field rev: the specific edition of the standard that is being sent."]
    #[doc = "* @field timeStamp: time reference"]
    #[doc = "* @field anchorPoint: Observer position, if needed."]
    #[doc = "* @field rtcmHeader: Precise antenna position and noise data for a rover"]
    #[doc = "* @field msgs: one or more RTCM messages."]
    #[doc = "* @field regional: optional region specific data."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct RTCMcorrections {
        #[rasn(identifier = "msgCnt")]
        pub msg_cnt: MsgCount,
        pub rev: RTCMRevision,
        #[rasn(identifier = "timeStamp")]
        pub time_stamp: Option<MinuteOfTheYear>,
        #[rasn(identifier = "anchorPoint")]
        pub anchor_point: Option<FullPositionVector>,
        #[rasn(identifier = "rtcmHeader")]
        pub rtcm_header: Option<RTCMheader>,
        pub msgs: RTCMmessageList,
        pub regional: Option<RTCMcorrectionsRegional>,
    }
    impl RTCMcorrections {
        pub fn new(
            msg_cnt: MsgCount,
            rev: RTCMRevision,
            time_stamp: Option<MinuteOfTheYear>,
            anchor_point: Option<FullPositionVector>,
            rtcm_header: Option<RTCMheader>,
            msgs: RTCMmessageList,
            regional: Option<RTCMcorrectionsRegional>,
        ) -> Self {
            Self {
                msg_cnt,
                rev,
                time_stamp,
                anchor_point,
                rtcm_header,
                msgs,
                regional,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* This DF is a collection of data values used to convey RTCM information between users. It"]
    #[doc = "* is not required or used when sending RTCM data from a corrections source to end users (from a base station to devices"]
    #[doc = "* deployed in the field which are called rovers)."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct RTCMheader {
        pub status: GNSSstatus,
        #[rasn(identifier = "offsetSet")]
        pub offset_set: AntennaOffsetSet,
    }
    impl RTCMheader {
        pub fn new(status: GNSSstatus, offset_set: AntennaOffsetSet) -> Self {
            Self { status, offset_set }
        }
    }
    #[doc = "*"]
    #[doc = "* This DE contains the stream of octets of the actual RTCM message that is being sent."]
    #[doc = "* The message’s contents are defined in RTCM Standard 10403.1 and in RTCM Standard 10402.1 and its successors."]
    #[doc = "* Note that most RTCM messages are considerably smaller than the size limit defined here, but that some messages may"]
    #[doc = "* need to be broken into smaller messages (as per the rules defined in the RTCM work) in order to be transmitted over DSRC."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=1023"))]
    pub struct RTCMmessage(pub OctetString);
    #[doc = "*"]
    #[doc = "* This DF consists of a list of @ref RTCMmessage entries."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=5"))]
    pub struct RTCMmessageList(pub SequenceOf<RTCMmessage>);
    #[doc = "*"]
    #[doc = "* This DE is used to define regions where unique additional content may be added and"]
    #[doc = "* used in the message set. The index values defined below represent various regions known at the time of publication. This"]
    #[doc = "* list is expected to grow over time. The index values assigned here can be augmented by local (uncoordinated)"]
    #[doc = "* assignments in the allowed range. It should be noted that such a local value is specified in the \"REGION\" ASN module, so"]
    #[doc = "* there is no need to edit the DSRC ASN specification of the standard. This process is further described in Section 11.1."]
    #[doc = "*"]
    #[doc = "* - `noRegion` - 0: Use default supplied stubs"]
    #[doc = "* - `addGrpA`  - 1: USA"]
    #[doc = "* - `addGrpB`  - 2: Japan"]
    #[doc = "* - `addGrpC`  - 3: EU"]
    #[doc = "*"]
    #[doc = "* @note: new registered regional IDs will be added here"]
    #[doc = "*        The values 128 and above are for local region use"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct RegionId(pub u8);
    #[doc = "*"]
    #[doc = "* This DF is used to convey a regulatory speed about a lane, lanes, or roadway segment."]
    #[doc = "*"]
    #[doc = "* @field type: The type of regulatory speed which follows"]
    #[doc = "* @field speed: The speed in units of 0.02 m/s"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct RegulatorySpeedLimit {
        #[rasn(identifier = "type")]
        pub r_type: SpeedLimitType,
        pub speed: Velocity,
    }
    impl RegulatorySpeedLimit {
        pub fn new(r_type: SpeedLimitType, speed: Velocity) -> Self {
            Self { r_type, speed }
        }
    }
    #[doc = "*"]
    #[doc = "* This DE is used to provide the R09 reporting point."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V2.2.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=65535"))]
    pub struct ReportingPoint(pub u16);
    #[doc = "*"]
    #[doc = "* This DE is used to provide a unique ID between two parties for various dialog exchanges."]
    #[doc = "* Combined with the sender's VehicleID (consisting of a TempID or a Station ID), this provides a unique string for some"]
    #[doc = "* mutually defined period of time. A typical example of use would be a signal preemption or priority request dialog"]
    #[doc = "* containing multiple requests from one sender (denoted by the unique RequestID with each). When such a request is"]
    #[doc = "* processed and reflected in the signal status messages, the original sender and the specific request can both be determined."]
    #[doc = "*"]
    #[doc = "* @note: In typical use, this value is simply incremented in a modulo fashion to ensure a unique stream of values for the"]
    #[doc = "*        device creating it. Any needs for uniqueness across multiple dialogs to one or more parties shall be the responsibility of"]
    #[doc = "*        the device to manage. There are often normative restrictions on the device changing its TempID during various dialogs"]
    #[doc = "*        when this data element is used. Further details of these operational concepts can be found in the relevant standards."]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct RequestID(pub u8);
    #[doc = "*"]
    #[doc = "* This DE is used to state what type of signal request is being made to a signal"]
    #[doc = "* controller by a DSRC device in a defined role (such as a police vehicle). The levels of the request typically convey a"]
    #[doc = "* sense of urgency or importance with respect to other demands to allow the controller to use predefined business rules to"]
    #[doc = "* determine how to respond. These rules will vary in terms of how details of overall importance and urgency are to be"]
    #[doc = "* ranked, so they are to be implemented locally. As a result of this regional process, the list below should be assigned well-"]
    #[doc = "* defined meanings by the local deployment. These meaning will typically result in assigning a set of values to list for each"]
    #[doc = "* vehicle role type that is to be supported."]
    #[doc = "*"]
    #[doc = "* - `requestImportanceLevel1`     1: The least important request"]
    #[doc = "* - `requestImportanceLevel14`   14: The most important request"]
    #[doc = "* - `requestImportanceReserved`  15: Reserved for future use"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum RequestImportanceLevel {
        requestImportanceLevelUnKnown = 0,
        requestImportanceLevel1 = 1,
        requestImportanceLevel2 = 2,
        requestImportanceLevel3 = 3,
        requestImportanceLevel4 = 4,
        requestImportanceLevel5 = 5,
        requestImportanceLevel6 = 6,
        requestImportanceLevel7 = 7,
        requestImportanceLevel8 = 8,
        requestImportanceLevel9 = 9,
        requestImportanceLevel10 = 10,
        requestImportanceLevel11 = 11,
        requestImportanceLevel12 = 12,
        requestImportanceLevel13 = 13,
        requestImportanceLevel14 = 14,
        requestImportanceReserved = 15,
    }
    #[doc = "*"]
    #[doc = "* This DE is used to further define the details of the role which any DSRC device might"]
    #[doc = "* play when making a request to a signal controller. This value is not always needed. For example, perhaps in a"]
    #[doc = "* deployment all police vehicles are to be treated equally. The taxonomy of what details are selected to be entered into the"]
    #[doc = "* list is a regional choice but should be devised to allow the controller to use predefined business rules to respond using the"]
    #[doc = "* data. As another example, perhaps in a regional deployment a cross-city express type of transit vehicle is given a different"]
    #[doc = "* service response for the same request than another type of transit vehicle making an otherwise similar request. As a"]
    #[doc = "* result of this regional process, the list below should be assigned well-defined meanings by the local deployment. These"]
    #[doc = "* meanings will typically result in assigning a set of values to list for each vehicle role type that is to be supported."]
    #[doc = "*"]
    #[doc = "* - `requestSubRole1`        - 1:  The first type of sub role"]
    #[doc = "* - `requestSubRole14`       - 14: The last type of sub role"]
    #[doc = "* - `requestSubRoleReserved` - 15: Reserved for future use"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum RequestSubRole {
        requestSubRoleUnKnown = 0,
        requestSubRole1 = 1,
        requestSubRole2 = 2,
        requestSubRole3 = 3,
        requestSubRole4 = 4,
        requestSubRole5 = 5,
        requestSubRole6 = 6,
        requestSubRole7 = 7,
        requestSubRole8 = 8,
        requestSubRole9 = 9,
        requestSubRole10 = 10,
        requestSubRole11 = 11,
        requestSubRole12 = 12,
        requestSubRole13 = 13,
        requestSubRole14 = 14,
        requestSubRoleReserved = 15,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousRequestorDescriptionRegional {
        #[rasn(identifier = "regionId")]
        pub region_id: RegionId,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousRequestorDescriptionRegional {
        pub fn new(region_id: RegionId, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    impl AnonymousRequestorDescriptionRegional {
        pub fn decode_reg_ext_value<D: Decoder>(
            &self,
            decoder: &mut D,
        ) -> Result<RegRequestorDescription_Type, D::Error> {
            RegRequestorDescription_Type::decode(
                decoder,
                Some(&self.reg_ext_value),
                &self.region_id,
            )
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct RequestorDescriptionRegional(pub SequenceOf<AnonymousRequestorDescriptionRegional>);
    #[doc = "*"]
    #[doc = "* This DF is used to provide identity information about a selected vehicle or users."]
    #[doc = "* This data frame is typically used with fleet type vehicles which can (or which must) safely release such information for use"]
    #[doc = "* with probe measurements or with other interactions (such as a signal request)."]
    #[doc = "*"]
    #[doc = "* @field id:               The ID used in the CAM of the requestor. This ID is presumed not to change during the exchange."]
    #[doc = "* @field type:             Information regarding all type and class data about the requesting vehicle"]
    #[doc = "* @field position:         The location of the requesting vehicle"]
    #[doc = "* @field name:             A human readable name for debugging use"]
    #[doc = "* @field routeName:        A string for transit operations use"]
    #[doc = "* @field transitStatus:    current vehicle state (loading, etc.)"]
    #[doc = "* @field transitOccupancy: current vehicle occupancy"]
    #[doc = "* @field transitSchedule:  current vehicle schedule adherence"]
    #[doc = "* @field regional:         optional region specific data."]
    #[doc = "* @field ocit:             Extension container for Legacy R09 data (as defined by [OCIT])."]
    #[doc = "*"]
    #[doc = "* @note: Note that the requestor description elements which are used when the request (the req) is made differ from"]
    #[doc = "*        those used when the status of an active or pending request is reported (the ack). Typically, when reporting the status to"]
    #[doc = "*        other parties, less information is required and only the temporaryID (contained in the VehicleID) and request number (a"]
    #[doc = "*        unique ID used in the orginal request) are used."]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct RequestorDescription {
        pub id: VehicleID,
        #[rasn(identifier = "type")]
        pub r_type: Option<RequestorType>,
        pub position: Option<RequestorPositionVector>,
        pub name: Option<DescriptiveName>,
        #[rasn(identifier = "routeName")]
        pub route_name: Option<DescriptiveName>,
        #[rasn(identifier = "transitStatus")]
        pub transit_status: Option<TransitVehicleStatus>,
        #[rasn(identifier = "transitOccupancy")]
        pub transit_occupancy: Option<TransitVehicleOccupancy>,
        #[rasn(identifier = "transitSchedule")]
        pub transit_schedule: Option<DeltaTime>,
        pub regional: Option<RequestorDescriptionRegional>,
        #[rasn(extension_addition)]
        pub ocit: OcitRequestorDescriptionContainer,
    }
    impl RequestorDescription {
        pub fn new(
            id: VehicleID,
            r_type: Option<RequestorType>,
            position: Option<RequestorPositionVector>,
            name: Option<DescriptiveName>,
            route_name: Option<DescriptiveName>,
            transit_status: Option<TransitVehicleStatus>,
            transit_occupancy: Option<TransitVehicleOccupancy>,
            transit_schedule: Option<DeltaTime>,
            regional: Option<RequestorDescriptionRegional>,
            ocit: OcitRequestorDescriptionContainer,
        ) -> Self {
            Self {
                id,
                r_type,
                position,
                name,
                route_name,
                transit_status,
                transit_occupancy,
                transit_schedule,
                regional,
                ocit,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* This DF provides a report of the requestor's position, speed, and heading."]
    #[doc = "* Used by a vehicle or other type of user to request services and at other times when the larger FullPositionVector is not required."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct RequestorPositionVector {
        pub position: Position3D,
        pub heading: Option<Angle>,
        pub speed: Option<TransmissionAndSpeed>,
    }
    impl RequestorPositionVector {
        pub fn new(
            position: Position3D,
            heading: Option<Angle>,
            speed: Option<TransmissionAndSpeed>,
        ) -> Self {
            Self {
                position,
                heading,
                speed,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct RequestorTypeRegional {
        #[rasn(identifier = "regionId")]
        pub region_id: RegionId,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl RequestorTypeRegional {
        pub fn new(region_id: RegionId, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    impl RequestorTypeRegional {
        pub fn decode_reg_ext_value<D: Decoder>(
            &self,
            decoder: &mut D,
        ) -> Result<RegRequestorType_Type, D::Error> {
            RegRequestorType_Type::decode(decoder, Some(&self.reg_ext_value), &self.region_id)
        }
    }
    #[doc = "*"]
    #[doc = "* This DF is used when a DSRC-equipped device is requesting service from another"]
    #[doc = "* device. The most common use case is when a vehicle is requesting a signal preemption or priority service call from the"]
    #[doc = "* signal controller in an intersection. This data frame provides the details of the requestor class taxonomy required to"]
    #[doc = "* support the request. Depending on the precise use case and the local implementation, these details can vary"]
    #[doc = "* considerably. As a result, besides the basic role of the vehicle, the other classification systems supported are optional. It"]
    #[doc = "* should also be observed that often only a subset of the information in the RequestorType data frame is used to report the"]
    #[doc = "* \"results\" of such a request to others. As an example, a police vehicle might request service based on being in a police"]
    #[doc = "* vehicle role (and any further sub-type if required) and on the type of service call to which the vehicle is then responding"]
    #[doc = "* (perhaps a greater degree of emergency than another type of call), placing these information elements in the"]
    #[doc = "* RequestorType, which is then part of the Signal Request Message (SRM). This allows the roadway operator to define"]
    #[doc = "* suitable business rules regarding how to reply. When informing the requestor and other nearby drivers of the outcome,"]
    #[doc = "* using the Signal Status Message (SSM) message, only the fact that the preemption was granted or denied to some"]
    #[doc = "* vehicle with a unique request ID is conveyed."]
    #[doc = "*"]
    #[doc = "* @field role:     Basic role of this user at this time."]
    #[doc = "* @field subrole:  A local list with role based items."]
    #[doc = "* @field request:  A local list with request items"]
    #[doc = "* @field iso3883:  Additional classification details"]
    #[doc = "* @field hpmsType: HPMS classification types"]
    #[doc = "* @field regional: optional region specific data."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct RequestorType {
        pub role: BasicVehicleRole,
        pub subrole: Option<RequestSubRole>,
        pub request: Option<RequestImportanceLevel>,
        pub iso3883: Option<Iso3833VehicleType>,
        #[rasn(identifier = "hpmsType")]
        pub hpms_type: Option<VehicleType>,
        pub regional: Option<RequestorTypeRegional>,
    }
    impl RequestorType {
        pub fn new(
            role: BasicVehicleRole,
            subrole: Option<RequestSubRole>,
            request: Option<RequestImportanceLevel>,
            iso3883: Option<Iso3833VehicleType>,
            hpms_type: Option<VehicleType>,
            regional: Option<RequestorTypeRegional>,
        ) -> Self {
            Self {
                role,
                subrole,
                request,
                iso3883,
                hpms_type,
                regional,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* The RestrictionAppliesTo data element provides a short list of common vehicle types which may have one or more"]
    #[doc = "* special movements at an intersection. In general, these movements are not visible to other traffic with signal heads, but"]
    #[doc = "* the SPAT data reflects the state of the movement. Various restricted movements at an intersection can be expressed"]
    #[doc = "* using this element to indicate where the movement applies."]
    #[doc = "*"]
    #[doc = "* - `none` :              applies to nothing"]
    #[doc = "* - `equippedTransit`:    buses etc."]
    #[doc = "* - `equippedTaxis`:"]
    #[doc = "* - `equippedOther`:      other vehicle types with necessary signal phase state reception equipment"]
    #[doc = "* - `emissionCompliant`:  regional variants with more definitive items also exist"]
    #[doc = "* - `equippedBicycle`:"]
    #[doc = "* - `weightCompliant`:"]
    #[doc = "* - `heightCompliant`:    Items dealing with traveler needs serviced by the infrastructure. These end users (which are not vehicles) are presumed to be suitably equipped"]
    #[doc = "* - `pedestrians`:"]
    #[doc = "* - `slowMovingPersons`:"]
    #[doc = "* - `wheelchairUsers`:"]
    #[doc = "* - `visualDisabilities`:"]
    #[doc = "* - `audioDisabilities`:  hearing"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum RestrictionAppliesTo {
        none = 0,
        equippedTransit = 1,
        equippedTaxis = 2,
        equippedOther = 3,
        emissionCompliant = 4,
        equippedBicycle = 5,
        weightCompliant = 6,
        heightCompliant = 7,
        pedestrians = 8,
        slowMovingPersons = 9,
        wheelchairUsers = 10,
        visualDisabilities = 11,
        audioDisabilities = 12,
        otherUnknownDisabilities = 13,
    }
    #[doc = "*"]
    #[doc = "* This DF is used to assign (or bind) a single RestrictionClassID data"]
    #[doc = "* element to a list of all user classes to which it applies. A collection of these bindings is conveyed in the"]
    #[doc = "* RestrictionClassList data frame in the MAP message to travelers. The established index is then used in the lane object of"]
    #[doc = "* the MAP message, in the ConnectTo data frame, to qualify to whom a signal group ID applies when it is sent by the SPAT"]
    #[doc = "* message about a movement."]
    #[doc = "*"]
    #[doc = "* @field id: the unique value (within an intersection or local region) that is assigned to this group of users."]
    #[doc = "* @field users: The list of user types/classes to which this restriction ID applies."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct RestrictionClassAssignment {
        pub id: RestrictionClassID,
        pub users: RestrictionUserTypeList,
    }
    impl RestrictionClassAssignment {
        pub fn new(id: RestrictionClassID, users: RestrictionUserTypeList) -> Self {
            Self { id, users }
        }
    }
    #[doc = "*"]
    #[doc = "* This DE defines an intersection-unique value to convey data about classes of users."]
    #[doc = "* The mapping used varies with each intersection and is defined in the MAP message if needed. The defined mappings"]
    #[doc = "* found there are used to determine when a given class is meant. The typical use of this element is to map additional"]
    #[doc = "* movement restrictions or rights (in both the MAP and SPAT messages) to special classes of users (trucks, high sided"]
    #[doc = "* vehicles, special vehicles etc.). There is the general presumption that in the absence of this data, any allowed movement"]
    #[doc = "* extends to all users."]
    #[doc = "*"]
    #[doc = "* An index value to identify data about classes of users the value used varies with each intersection's"]
    #[doc = "* needs and is defined in the map to the assigned classes of supported users."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct RestrictionClassID(pub u8);
    #[doc = "*"]
    #[doc = "* This DF is used to enumerate a list of user classes which belong to a given"]
    #[doc = "* assigned index. The resulting collection is treated as a group by the signal controller when it issues movement data"]
    #[doc = "* (signal phase information) with the GroupID for this group. This data frame is typically static for long periods of time"]
    #[doc = "* (months) and conveyed to the user by means of the MAP message."]
    #[doc = "*"]
    #[doc = "* @note: The overall restriction class assignment process allows dynamic support within the framework of the common"]
    #[doc = "*        message set for the various special cases that some signalized intersections must support. While the assigned value"]
    #[doc = "*        needs to be unique only within the scope of the intersection that uses it, the resulting assignment lists will tend to be static"]
    #[doc = "*        and stable for regional deployment areas such as a metropolitan area based on their operational practices and needs."]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=254"))]
    pub struct RestrictionClassList(pub SequenceOf<RestrictionClassAssignment>);
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousRestrictionUserTypeRegional {
        #[rasn(identifier = "regionId")]
        pub region_id: RegionId,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousRestrictionUserTypeRegional {
        pub fn new(region_id: RegionId, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    impl AnonymousRestrictionUserTypeRegional {
        pub fn decode_reg_ext_value<D: Decoder>(
            &self,
            decoder: &mut D,
        ) -> Result<RegRestrictionUserType_Type, D::Error> {
            RegRestrictionUserType_Type::decode(decoder, Some(&self.reg_ext_value), &self.region_id)
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct RestrictionUserTypeRegional(pub SequenceOf<AnonymousRestrictionUserTypeRegional>);
    #[doc = "*"]
    #[doc = "* This DF is used to provide a means to select one, and only one, user type or class"]
    #[doc = "* from a number of well-known lists. The selected entry is then used in the overall Restriction Class assignment process to"]
    #[doc = "* indicate that a given GroupID (a way of expressing a movement in the SPAT/MAP system) applies to (is restricted to) this"]
    #[doc = "* class of user."]
    #[doc = "*"]
    #[doc = "* @field basicType: a set of the most commonly used types."]
    #[doc = "* @field regional:  optional region specific data."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum RestrictionUserType {
        basicType(RestrictionAppliesTo),
        regional(RestrictionUserTypeRegional),
    }
    #[doc = "*"]
    #[doc = "* This DF consists of a list of @ref RestrictionUserType entries."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16"))]
    pub struct RestrictionUserTypeList(pub SequenceOf<RestrictionUserType>);
    #[doc = "*"]
    #[doc = "* This DF consists of a list of GenericLane entries used to describe a segment of roadway."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=255"))]
    pub struct RoadLaneSetList(pub SequenceOf<GenericLane>);
    #[doc = "*"]
    #[doc = "* This DE is a 16-bit globally unique identifier assigned to an entity responsible for assigning"]
    #[doc = "* Intersection IDs in the region over which it has such authority. The value zero shall be used for testing, and should only be"]
    #[doc = "* used in the absence of a suitable assignment. A single entity which assigns intersection IDs may be assigned several"]
    #[doc = "* RoadRegulatorIDs. These assignments are presumed to be permanent."]
    #[doc = "*"]
    #[doc = "* The value zero shall be used for testing only"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=65535"))]
    pub struct RoadRegulatorID(pub u16);
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousRoadSegmentRegional {
        #[rasn(identifier = "regionId")]
        pub region_id: RegionId,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousRoadSegmentRegional {
        pub fn new(region_id: RegionId, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    impl AnonymousRoadSegmentRegional {
        pub fn decode_reg_ext_value<D: Decoder>(
            &self,
            decoder: &mut D,
        ) -> Result<RegRoadSegment_Type, D::Error> {
            RegRoadSegment_Type::decode(decoder, Some(&self.reg_ext_value), &self.region_id)
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct RoadSegmentRegional(pub SequenceOf<AnonymousRoadSegmentRegional>);
    #[doc = "*"]
    #[doc = "* This DF is a complete description of a RoadSegment including its geometry and its"]
    #[doc = "* allowed navigational paths (independent of any additional regulatory restrictions that may apply over time or from user"]
    #[doc = "* classification) and any current disruptions such as a work zone or incident event."]
    #[doc = "*"]
    #[doc = "* @field name: some descriptive text."]
    #[doc = "* @field id: a globally unique value for the segment."]
    #[doc = "* @field revision: ."]
    #[doc = "* @field refPoint: the reference from which subsequent data points are offset until a new point is used."]
    #[doc = "* @field laneWidth: Reference width used by all subsequent lanes unless a new width is given."]
    #[doc = "* @field speedLimits: Reference regulatory speed limits used by all subsequent lanes unless a new speed is given."]
    #[doc = "* @field roadLaneSet: Data describing disruptions in the RoadSegment such as work zones etc will be added here."]
    #[doc = "* @field regional: optional region specific data."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct RoadSegment {
        pub name: Option<DescriptiveName>,
        pub id: RoadSegmentReferenceID,
        pub revision: MsgCount,
        #[rasn(identifier = "refPoint")]
        pub ref_point: Position3D,
        #[rasn(identifier = "laneWidth")]
        pub lane_width: Option<LaneWidth>,
        #[rasn(identifier = "speedLimits")]
        pub speed_limits: Option<SpeedLimitList>,
        #[rasn(identifier = "roadLaneSet")]
        pub road_lane_set: RoadLaneSetList,
        pub regional: Option<RoadSegmentRegional>,
    }
    impl RoadSegment {
        pub fn new(
            name: Option<DescriptiveName>,
            id: RoadSegmentReferenceID,
            revision: MsgCount,
            ref_point: Position3D,
            lane_width: Option<LaneWidth>,
            speed_limits: Option<SpeedLimitList>,
            road_lane_set: RoadLaneSetList,
            regional: Option<RoadSegmentRegional>,
        ) -> Self {
            Self {
                name,
                id,
                revision,
                ref_point,
                lane_width,
                speed_limits,
                road_lane_set,
                regional,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* This DE is used to uniquely define a section of roadway within a country or region in a 16-bit field."]
    #[doc = "* Assignment rules for this value are established elsewhere and may use regional assignment schemas that vary. Within"]
    #[doc = "* the region the policies used to ensure an assigned value’s uniqueness before that value is reused is the responsibility of"]
    #[doc = "* that region. Such reuse is expected to occur, but over somewhat lengthy epoch (months)."]
    #[doc = "*"]
    #[doc = "* The values zero to 255 shall be used for testing only"]
    #[doc = "* Note that the value assigned to an RoadSegment will be"]
    #[doc = "* unique within a given regional ID only during its use"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=65535"))]
    pub struct RoadSegmentID(pub u16);
    #[doc = "*"]
    #[doc = "* This DF consists of a list of @ref RoadSegment entries."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"))]
    pub struct RoadSegmentList(pub SequenceOf<RoadSegment>);
    #[doc = "*"]
    #[doc = "* This DF is used to convey theRoadSegmentID which is unique to a given road segment of interest,"]
    #[doc = "* and also the RoadRegulatorID assigned to the region in which it is operating (when required)."]
    #[doc = "*"]
    #[doc = "* @field region: a globally unique regional assignment value typically assigned to a regional DOT authority the value zero shall be used for testing needs."]
    #[doc = "* @field id:     a unique mapping to the road segment in question within the above region of use during its period of assignment and use"]
    #[doc = "*                note that unlike intersectionID values, this value can be reused by the region."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct RoadSegmentReferenceID {
        pub region: Option<RoadRegulatorID>,
        pub id: RoadSegmentID,
    }
    impl RoadSegmentReferenceID {
        pub fn new(region: Option<RoadRegulatorID>, id: RoadSegmentID) -> Self {
            Self { region, id }
        }
    }
    #[doc = "*"]
    #[doc = "* The RoadwayCrownAngle data element relates the gross tangential angle of the roadway surface with respect to"]
    #[doc = "* the local horizontal axis and is measured at the indicated part of the lane. This measurement is typically made at the"]
    #[doc = "* crown (centerline) or at an edge of the lane path. Its typical use is to relate data used in speed warning and traction"]
    #[doc = "* calculations for the lane segment or roadway segment in which the measurement is taken."]
    #[doc = "*"]
    #[doc = "* - The value -128 shall be used for unknown"]
    #[doc = "* - The value zero shall be used for angles which are between -0.15 and +0.15"]
    #[doc = "*"]
    #[doc = "* @unit: 0.3 degrees of angle over a range of -38.1 to + 38.1 degrees"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-128..=127"))]
    pub struct RoadwayCrownAngle(pub i8);
    #[doc = "*"]
    #[doc = "* This DE is used to provide the R09 route information."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V2.2.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=4294967295"))]
    pub struct RouteNumber(pub u32);
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousSPATRegional {
        #[rasn(identifier = "regionId")]
        pub region_id: RegionId,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousSPATRegional {
        pub fn new(region_id: RegionId, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    impl AnonymousSPATRegional {
        pub fn decode_reg_ext_value<D: Decoder>(
            &self,
            decoder: &mut D,
        ) -> Result<RegSPAT_Type, D::Error> {
            RegSPAT_Type::decode(decoder, Some(&self.reg_ext_value), &self.region_id)
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct SPATRegional(pub SequenceOf<AnonymousSPATRegional>);
    #[doc = "*"]
    #[doc = "* This DF is used to convey the current status of one or more signalized"]
    #[doc = "* intersections. Along with the MapData message (which describes a full geometric layout of an intersection) the"]
    #[doc = "* receiver of this message can determine the state of the signal phasing and when the next expected phase will occur."]
    #[doc = "* The SPAT message sends the current movement state of each active phase in the system as needed (such as values of"]
    #[doc = "* what states are active and values at what time a state has begun/does begin earliest, is expected to begin most likely and"]
    #[doc = "* will end latest). The state of inactive movements is not normally transmitted. Movements are mapped to specific"]
    #[doc = "* approaches and connections of ingress to egress lanes and by use of the SignalGroupID in the MapData message"]
    #[doc = "*"]
    #[doc = "* The current signal preemption and priority status values (when present or active) are also sent. A more complete"]
    #[doc = "* summary of any pending priority or preemption events can be found in the Signal Status message."]
    #[doc = "*"]
    #[doc = "* @field timeStamp: time reference"]
    #[doc = "* @field name: human readable name for this collection. to be used only in debug mode."]
    #[doc = "* @field intersections: sets of SPAT data (one per intersection)"]
    #[doc = "* @field regional: optional region specific data."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SPAT {
        #[rasn(identifier = "timeStamp")]
        pub time_stamp: Option<MinuteOfTheYear>,
        pub name: Option<DescriptiveName>,
        pub intersections: IntersectionStateList,
        pub regional: Option<SPATRegional>,
    }
    impl SPAT {
        pub fn new(
            time_stamp: Option<MinuteOfTheYear>,
            name: Option<DescriptiveName>,
            intersections: IntersectionStateList,
            regional: Option<SPATRegional>,
        ) -> Self {
            Self {
                time_stamp,
                name,
                intersections,
                regional,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* A 12-bit signed scaling factor supporting scales from zero (which is not used) to >200%. In this data element, the"]
    #[doc = "* value zero is taken to represent a value of one (scale 1:1). Values above and below this add or remove exactly 0.05%"]
    #[doc = "* from the initial value of 100%. Hence, a value of 2047 adds 102.35% to 100%, resulting in a scale of 202.35% exactly (the"]
    #[doc = "* largest valid scale value). Negative values which would result in an effective final value below zero are not supported. The"]
    #[doc = "* smallest valid value allowed is -1999 and the remaining negative values are reserved for future definition."]
    #[doc = "*"]
    #[doc = "* @unit: in steps of 0.05 percent"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "Scale-B12", value("-2048..=2047"))]
    pub struct ScaleB12(pub i16);
    #[doc = "*"]
    #[doc = "* This DE is an enumerated list of attributes about the current lane segment which"]
    #[doc = "* may be enabled or disabled to indicate the presence or absence of the selected attribute on the segment. A segment is"]
    #[doc = "* one or more of the straight lines formed between each set of node points. It is common for a segment attribute to persist"]
    #[doc = "* for more than one set of node points if there is any curvature in the lane itself. The described attributes are all binary flags"]
    #[doc = "* in that they do not need to convey any additional data. Other attributes allow sending short data values to reflect a setting"]
    #[doc = "* which is set and persists in a similar fashion."]
    #[doc = "*"]
    #[doc = "* Various values which can be Enabled and Disabled for a lane segment"]
    #[doc = "* - reserved:"]
    #[doc = "* - doNotBlock: segment where a vehicle may not come to a stop"]
    #[doc = "* - whiteLine:  segment where lane crossing not allowed such as the final few meters of a lane "]
    #[doc = "* - mergingLaneLeft: indicates porous lanes"]
    #[doc = "* - mergingLaneRight: indicates porous lanes"]
    #[doc = "* - curbOnLeft: indicates presence of curbs"]
    #[doc = "* - curbOnRight: indicates presence of curbs"]
    #[doc = "* - loadingzoneOnLeft:  loading or drop off zones"]
    #[doc = "* - loadingzoneOnRight: loading or drop off zones"]
    #[doc = "* - turnOutPointOnLeft: opening to adjacent street/alley/road"]
    #[doc = "* - turnOutPointOnRight: opening to adjacent street/alley/road"]
    #[doc = "* - adjacentParkingOnLeft: side of road parking"]
    #[doc = "* - adjacentParkingOnRight: side of road parking"]
    #[doc = "* - adjacentBikeLaneOnLeft: presence of marked bike lanes"]
    #[doc = "* - adjacentBikeLaneOnRight: presence of marked bike lanes"]
    #[doc = "* - sharedBikeLane: right of way is shared with bikes who may occupy entire lane width"]
    #[doc = "* - bikeBoxInFront:"]
    #[doc = "* - transitStopOnLeft: any form of bus/transit loading, with pull in-out access to lane on left"]
    #[doc = "* - transitStopOnRight: any form of bus/transit loading, with pull in-out access to lane on right"]
    #[doc = "* - transitStopInLane: any form of bus/transit loading, in mid path of the lane"]
    #[doc = "* - sharedWithTrackedVehicle: lane is shared with train or trolley, not used for crossing tracks "]
    #[doc = "* - safeIsland: begin/end a safety island in path"]
    #[doc = "* - lowCurbsPresent: for ADA support"]
    #[doc = "* - rumbleStripPresent: for ADA support"]
    #[doc = "* - audibleSignalingPresent: for ADA support"]
    #[doc = "* - adaptiveTimingPresent: for ADA support"]
    #[doc = "* - rfSignalRequestPresent: Supports RF push to walk technologies"]
    #[doc = "* - partialCurbIntrusion: path is blocked by a median or curb but at least 1 meter remains open for use"]
    #[doc = "*                         and at-grade passage Lane geometry details"]
    #[doc = "* - taperToLeft: Used to control final path shape (see standard for defined shapes)"]
    #[doc = "* - taperToRight: Used to control final path shape (see standard for defined shapes)"]
    #[doc = "* - taperToCenterLine: Used to control final path shape (see standard for defined shapes)"]
    #[doc = "* - parallelParking: Parking at an angle with the street"]
    #[doc = "* - headInParking:   Parking at an angle with the street"]
    #[doc = "* - freeParking:     No restriction on use of parking"]
    #[doc = "* - timeRestrictionsOnParking: Parking is not permitted at all times"]
    #[doc = "*                              typically used when the **parking** lane becomes a driving lane at times"]
    #[doc = "* - costToPark: Used where parking has a cost"]
    #[doc = "* - midBlockCurbPresent: a protruding curb near lane edge"]
    #[doc = "* - unEvenPavementPresent: a disjoint height at lane edge"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum SegmentAttributeXY {
        reserved = 0,
        doNotBlock = 1,
        whiteLine = 2,
        mergingLaneLeft = 3,
        mergingLaneRight = 4,
        curbOnLeft = 5,
        curbOnRight = 6,
        loadingzoneOnLeft = 7,
        loadingzoneOnRight = 8,
        turnOutPointOnLeft = 9,
        turnOutPointOnRight = 10,
        adjacentParkingOnLeft = 11,
        adjacentParkingOnRight = 12,
        adjacentBikeLaneOnLeft = 13,
        adjacentBikeLaneOnRight = 14,
        sharedBikeLane = 15,
        bikeBoxInFront = 16,
        transitStopOnLeft = 17,
        transitStopOnRight = 18,
        transitStopInLane = 19,
        sharedWithTrackedVehicle = 20,
        safeIsland = 21,
        lowCurbsPresent = 22,
        rumbleStripPresent = 23,
        audibleSignalingPresent = 24,
        adaptiveTimingPresent = 25,
        rfSignalRequestPresent = 26,
        partialCurbIntrusion = 27,
        taperToLeft = 28,
        taperToRight = 29,
        taperToCenterLine = 30,
        parallelParking = 31,
        headInParking = 32,
        freeParking = 33,
        timeRestrictionsOnParking = 34,
        costToPark = 35,
        midBlockCurbPresent = 36,
        unEvenPavementPresent = 37,
    }
    #[doc = "*"]
    #[doc = "* This DF consists of a list of @ref SegmentAttributeXY entries."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8"))]
    pub struct SegmentAttributeXYList(pub SequenceOf<SegmentAttributeXY>);
    #[doc = "*"]
    #[doc = "* This DE is used to express the radius (length) of the semi-major axis of an"]
    #[doc = "* ellipsoid representing the accuracy which can be expected from a GNSS system in 5cm steps,"]
    #[doc = "* typically at a one sigma level of confidence."]
    #[doc = "*"]
    #[doc = "* Value is semi-major axis accuracy at one standard dev."]
    #[doc = "* - Range 0-12.7 meter, LSB = .05m"]
    #[doc = "* - 254 = any value equal or greater than 12.70 meter"]
    #[doc = "* - 255 = unavailable semi-major axis value"]
    #[doc = "*"]
    #[doc = "* @unit: 0.05m"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct SemiMajorAxisAccuracy(pub u8);
    #[doc = "*"]
    #[doc = "* This DE is used to orientate the angle of the semi-major axis of an"]
    #[doc = "* ellipsoid representing the accuracy which can be expected from a GNSS system with respect to the coordinate system."]
    #[doc = "*"]
    #[doc = "* Value is orientation of semi-major axis"]
    #[doc = "* - relative to true north (0-359.9945078786 degrees)"]
    #[doc = "* - LSB units of 360/65535 deg = 0.0054932479"]
    #[doc = "* - a value of 0 shall be 0 degrees"]
    #[doc = "* - a value of 1 shall be 0.0054932479 degrees"]
    #[doc = "* - a value of 65534 shall be 359.9945078786 deg"]
    #[doc = "* - a value of 65535 shall be used for orientation unavailable"]
    #[doc = "*"]
    #[doc = "* @unit: 360/65535 degree"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=65535"))]
    pub struct SemiMajorAxisOrientation(pub u16);
    #[doc = "*"]
    #[doc = "* This DE is used to express the radius of the semi-minor axis of an ellipsoid"]
    #[doc = "* representing the accuracy which can be expected from a GNSS system in 5cm steps, typically at a one sigma level of"]
    #[doc = "* confidence."]
    #[doc = "*"]
    #[doc = "* Value is semi-minor axis accuracy at one standard dev"]
    #[doc = "* - range 0-12.7 meter, LSB = .05m"]
    #[doc = "* - 254 = any value equal or greater than 12.70 meter"]
    #[doc = "* - 255 = unavailable semi-minor axis value"]
    #[doc = "*"]
    #[doc = "* @unit: 0.05m"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct SemiMinorAxisAccuracy(pub u8);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct SignalControlZoneZone {
        #[rasn(identifier = "regionId")]
        pub region_id: RegionId,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl SignalControlZoneZone {
        pub fn new(region_id: RegionId, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    impl SignalControlZoneZone {
        pub fn decode_reg_ext_value<D: Decoder>(
            &self,
            decoder: &mut D,
        ) -> Result<RegSignalControlZone_Type, D::Error> {
            RegSignalControlZone_Type::decode(decoder, Some(&self.reg_ext_value), &self.region_id)
        }
    }
    #[doc = "*"]
    #[doc = "* This DF is a dummy placeholder to contain a regional SignalControlZone DF."]
    #[doc = "* It is not used, yet here for backwards compatibility."]
    #[doc = "*"]
    #[doc = "* @field regional: optional region specific data."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SignalControlZone {
        pub zone: SignalControlZoneZone,
    }
    impl SignalControlZone {
        pub fn new(zone: SignalControlZoneZone) -> Self {
            Self { zone }
        }
    }
    #[doc = "*"]
    #[doc = "* This DE is an index used to map between the internal state machine of one or more signal controllers (or"]
    #[doc = "* other types of traffic flow devices) and a common numbering system that can represent all possible combinations of active"]
    #[doc = "* states (movements and phases in US traffic terminology). All possible movement variations are assigned a unique value"]
    #[doc = "* within the intersection. Conceptually, the ID represents a means to provide a list of lanes in a set which would otherwise"]
    #[doc = "* need to be enumerated in the message. The values zero and 255 are reserved, so there may up to 254 different signal"]
    #[doc = "* group IDs within one single intersection. The value 255 represents a protected-Movement-Allowed or permissive-"]
    #[doc = "* Movement-Allowed condition that exists at all times. This value is applied to lanes, with or without traffic control devices,"]
    #[doc = "* that operate as free-flow lanes. Typically referred to as Channelized Right/Left Turn Lanes (in right/left-hand drive"]
    #[doc = "* countries)."]
    #[doc = "*"]
    #[doc = "* Values:"]
    #[doc = "* - the value `0` shall be used when the ID is not available or not known"]
    #[doc = "* - the value `255` is reserved to indicate a permanent green movement state"]
    #[doc = "* - therefore a simple 8 phase signal controller device might use 1..9 as its groupIDs"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct SignalGroupID(pub u8);
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousSignalRequestRegional {
        #[rasn(identifier = "regionId")]
        pub region_id: RegionId,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousSignalRequestRegional {
        pub fn new(region_id: RegionId, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    impl AnonymousSignalRequestRegional {
        pub fn decode_reg_ext_value<D: Decoder>(
            &self,
            decoder: &mut D,
        ) -> Result<RegSignalRequest_Type, D::Error> {
            RegSignalRequest_Type::decode(decoder, Some(&self.reg_ext_value), &self.region_id)
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct SignalRequestRegional(pub SequenceOf<AnonymousSignalRequestRegional>);
    #[doc = "*"]
    #[doc = "* This DF is used (as part of a request message) to request either a priority or a preemption service"]
    #[doc = "* from a signalized intersection. It relates the intersection ID as well as the specific request information. Additional"]
    #[doc = "* information includes the approach and egress values or lanes to be used."]
    #[doc = "*"]
    #[doc = "* @field id: the unique ID of the target intersection"]
    #[doc = "* @field requestID: The unique requestID used by the requestor"]
    #[doc = "* @field requestType: The type of request or cancel for priority or preempt use when a prior request is canceled, only the requestID is needed."]
    #[doc = "* @field inBoundLane: desired entry approach or lane."]
    #[doc = "* @field outBoundLane: desired exit approach or lane. the value zero is used to indicate intent to stop within the intersection."]
    #[doc = "* @field regional: optional region specific data."]
    #[doc = "*"]
    #[doc = "* @note: In typical use either an approach or a lane number would be given, this indicates the requested"]
    #[doc = "*        path through the intersection to the degree it is known."]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SignalRequest {
        pub id: IntersectionReferenceID,
        #[rasn(identifier = "requestID")]
        pub request_id: RequestID,
        #[rasn(identifier = "requestType")]
        pub request_type: PriorityRequestType,
        #[rasn(identifier = "inBoundLane")]
        pub in_bound_lane: IntersectionAccessPoint,
        #[rasn(identifier = "outBoundLane")]
        pub out_bound_lane: Option<IntersectionAccessPoint>,
        pub regional: Option<SignalRequestRegional>,
    }
    impl SignalRequest {
        pub fn new(
            id: IntersectionReferenceID,
            request_id: RequestID,
            request_type: PriorityRequestType,
            in_bound_lane: IntersectionAccessPoint,
            out_bound_lane: Option<IntersectionAccessPoint>,
            regional: Option<SignalRequestRegional>,
        ) -> Self {
            Self {
                id,
                request_id,
                request_type,
                in_bound_lane,
                out_bound_lane,
                regional,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* This DF consists of a list of @ref SignalRequest entries."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"))]
    pub struct SignalRequestList(pub SequenceOf<SignalRequestPackage>);
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousSignalRequestMessageRegional {
        #[rasn(identifier = "regionId")]
        pub region_id: RegionId,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousSignalRequestMessageRegional {
        pub fn new(region_id: RegionId, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    impl AnonymousSignalRequestMessageRegional {
        pub fn decode_reg_ext_value<D: Decoder>(
            &self,
            decoder: &mut D,
        ) -> Result<RegSignalRequestMessage_Type, D::Error> {
            RegSignalRequestMessage_Type::decode(
                decoder,
                Some(&self.reg_ext_value),
                &self.region_id,
            )
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct SignalRequestMessageRegional(pub SequenceOf<AnonymousSignalRequestMessageRegional>);
    #[doc = "*"]
    #[doc = "* This DF is a message sent by a DSRC equipped entity (such as a vehicle) to the RSU in a"]
    #[doc = "* signalized intersection. It is used for either a priority signal request or a preemption signal request depending on the way"]
    #[doc = "* each request is set. Each request defines a path through the intersection which is desired in terms of lanes and"]
    #[doc = "* approaches to be used. Each request can also contain the time of arrival and the expected duration of the service."]
    #[doc = "* Multiple requests to multiple intersections are supported. The requestor identifies itself in various ways (using methods"]
    #[doc = "* supported by the @refRequestorDescription data frame), and its current speed, heading and location can be placed in this"]
    #[doc = "* structure as well. The specific request for service is typically based on previously decoding and examining the list of lanes"]
    #[doc = "* and approaches for that intersection (sent in MAP messages). The outcome of all of the pending requests to a signal can"]
    #[doc = "* be found in the Signal Status Message (SSM), and may be reflected in the SPAT message contents if successful."]
    #[doc = "*"]
    #[doc = "* @field timeStamp: time reference"]
    #[doc = "* @field second: time reference"]
    #[doc = "* @field sequenceNumber: monotonic incrementing identifier"]
    #[doc = "* @field requests: Request Data for one or more signalized intersections that support SRM dialogs"]
    #[doc = "* @field requestor: Requesting Device and other User Data contains vehicle ID (if from a vehicle) as well as type data and current"]
    #[doc = "*                   position and may contain additional transit data"]
    #[doc = "* @field regional: optional region specific data."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SignalRequestMessage {
        #[rasn(identifier = "timeStamp")]
        pub time_stamp: Option<MinuteOfTheYear>,
        pub second: DSecond,
        #[rasn(identifier = "sequenceNumber")]
        pub sequence_number: Option<MsgCount>,
        pub requests: Option<SignalRequestList>,
        pub requestor: RequestorDescription,
        pub regional: Option<SignalRequestMessageRegional>,
    }
    impl SignalRequestMessage {
        pub fn new(
            time_stamp: Option<MinuteOfTheYear>,
            second: DSecond,
            sequence_number: Option<MsgCount>,
            requests: Option<SignalRequestList>,
            requestor: RequestorDescription,
            regional: Option<SignalRequestMessageRegional>,
        ) -> Self {
            Self {
                time_stamp,
                second,
                sequence_number,
                requests,
                requestor,
                regional,
            }
        }
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousSignalRequestPackageRegional {
        #[rasn(identifier = "regionId")]
        pub region_id: RegionId,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousSignalRequestPackageRegional {
        pub fn new(region_id: RegionId, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    impl AnonymousSignalRequestPackageRegional {
        pub fn decode_reg_ext_value<D: Decoder>(
            &self,
            decoder: &mut D,
        ) -> Result<RegSignalRequestPackage_Type, D::Error> {
            RegSignalRequestPackage_Type::decode(
                decoder,
                Some(&self.reg_ext_value),
                &self.region_id,
            )
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct SignalRequestPackageRegional(pub SequenceOf<AnonymousSignalRequestPackageRegional>);
    #[doc = "*"]
    #[doc = "* This DF contains both the service request itself (the preemption and priority"]
    #[doc = "* details and the inbound-outbound path details for an intersection) and the time period (start and end time) over which this"]
    #[doc = "* service is sought from one single intersection. One or more of these packages are contained in a list in the Signal"]
    #[doc = "* Request Message (SREM)."]
    #[doc = "*"]
    #[doc = "* @field request:  The specific request to the intersection contains IntersectionID, request type, requested action (approach/lane request)."]
    #[doc = "* @field minute:   Time period start."]
    #[doc = "* @field second:   Time period start."]
    #[doc = "* @field duration: The duration value is used to provide a short interval that extends the ETA so that the requesting vehicle can arrive at"]
    #[doc = "*                  the point of service with uncertainty or with some desired duration of service. This concept can be used to avoid needing"]
    #[doc = "*                  to frequently update the request. The requester must update the ETA and duration values if the"]
    #[doc = "*                  period of services extends beyond the duration time. It should be assumed that if the vehicle does not clear the"]
    #[doc = "*                  intersection when the duration is reached, the request will be cancelled and the intersection will revert to normal operation."]
    #[doc = "* @field regional: optional region specific data."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SignalRequestPackage {
        pub request: SignalRequest,
        pub minute: Option<MinuteOfTheYear>,
        pub second: Option<DSecond>,
        pub duration: Option<DSecond>,
        pub regional: Option<SignalRequestPackageRegional>,
    }
    impl SignalRequestPackage {
        pub fn new(
            request: SignalRequest,
            minute: Option<MinuteOfTheYear>,
            second: Option<DSecond>,
            duration: Option<DSecond>,
            regional: Option<SignalRequestPackageRegional>,
        ) -> Self {
            Self {
                request,
                minute,
                second,
                duration,
                regional,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* This DF is used to contain information regarding the entity that requested a given"]
    #[doc = "* signal behavior. In addition to the VehicleID, the data frame also contains a request reference number used to uniquely"]
    #[doc = "* refer to the request and some basic type information about the request maker which may be used by other parties."]
    #[doc = "*"]
    #[doc = "* @field id: to uniquely identify the requester and the specific request to all parties."]
    #[doc = "* @field request: to uniquely identify the requester and the specific request to all parties."]
    #[doc = "* @field sequenceNumber: to uniquely identify the requester and the specific request to all parties."]
    #[doc = "* @field role: vehicle role"]
    #[doc = "* @field typeData: Used when addition data besides the role is needed, at which point the role entry above is not sent."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SignalRequesterInfo {
        pub id: VehicleID,
        pub request: RequestID,
        #[rasn(identifier = "sequenceNumber")]
        pub sequence_number: MsgCount,
        pub role: Option<BasicVehicleRole>,
        #[rasn(identifier = "typeData")]
        pub type_data: Option<RequestorType>,
    }
    impl SignalRequesterInfo {
        pub fn new(
            id: VehicleID,
            request: RequestID,
            sequence_number: MsgCount,
            role: Option<BasicVehicleRole>,
            type_data: Option<RequestorType>,
        ) -> Self {
            Self {
                id,
                request,
                sequence_number,
                role,
                type_data,
            }
        }
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousSignalStatusRegional {
        #[rasn(identifier = "regionId")]
        pub region_id: RegionId,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousSignalStatusRegional {
        pub fn new(region_id: RegionId, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    impl AnonymousSignalStatusRegional {
        pub fn decode_reg_ext_value<D: Decoder>(
            &self,
            decoder: &mut D,
        ) -> Result<RegSignalStatus_Type, D::Error> {
            RegSignalStatus_Type::decode(decoder, Some(&self.reg_ext_value), &self.region_id)
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct SignalStatusRegional(pub SequenceOf<AnonymousSignalStatusRegional>);
    #[doc = "*"]
    #[doc = "* This DF is used to provide the status of a single intersection to others, including any active"]
    #[doc = "* preemption or priority state in effect."]
    #[doc = "*"]
    #[doc = "* @field sequenceNumber: changed whenever the below contents have change"]
    #[doc = "* @field id:             this provides a unique mapping to the intersection map in question which provides complete location"]
    #[doc = "*                        and approach/movement/lane data as well as zones for priority/preemption."]
    #[doc = "* @field sigStatus:      a list of detailed status containing all priority or preemption state data, both active and pending,"]
    #[doc = "*                        and who requested it requests which are denied are also listed here for a short period of time."]
    #[doc = "* @field regional: optional region specific data."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SignalStatus {
        #[rasn(identifier = "sequenceNumber")]
        pub sequence_number: MsgCount,
        pub id: IntersectionReferenceID,
        #[rasn(identifier = "sigStatus")]
        pub sig_status: SignalStatusPackageList,
        pub regional: Option<SignalStatusRegional>,
    }
    impl SignalStatus {
        pub fn new(
            sequence_number: MsgCount,
            id: IntersectionReferenceID,
            sig_status: SignalStatusPackageList,
            regional: Option<SignalStatusRegional>,
        ) -> Self {
            Self {
                sequence_number,
                id,
                sig_status,
                regional,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* This DF consists of a list of @ref SignalStatus entries."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"))]
    pub struct SignalStatusList(pub SequenceOf<SignalStatus>);
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousSignalStatusMessageRegional {
        #[rasn(identifier = "regionId")]
        pub region_id: RegionId,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousSignalStatusMessageRegional {
        pub fn new(region_id: RegionId, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    impl AnonymousSignalStatusMessageRegional {
        pub fn decode_reg_ext_value<D: Decoder>(
            &self,
            decoder: &mut D,
        ) -> Result<RegSignalStatusMessage_Type, D::Error> {
            RegSignalStatusMessage_Type::decode(decoder, Some(&self.reg_ext_value), &self.region_id)
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct SignalStatusMessageRegional(pub SequenceOf<AnonymousSignalStatusMessageRegional>);
    #[doc = "*"]
    #[doc = "* This DF is a message sent by an RSU in a signalized intersection. It is used to relate the current"]
    #[doc = "* status of the signal and the collection of pending or active preemption or priority requests acknowledged by the controller."]
    #[doc = "* It is also used to send information about preemption or priority requests which were denied. This in turn allows a dialog"]
    #[doc = "* acknowledgment mechanism between any requester and the signal controller. The data contained in this message allows"]
    #[doc = "* other users to determine their \"ranking\" for any request they have made as well as to see the currently active events."]
    #[doc = "* When there have been no recently received requests for service messages, this message may not be sent. While the"]
    #[doc = "* outcome of all pending requests to a signal can be found in the Signal Status Message, the current active event (if any)"]
    #[doc = "* will be reflected in the SPAT message contents."]
    #[doc = "*"]
    #[doc = "* @field timeStamp: time reference"]
    #[doc = "* @field second: time reference"]
    #[doc = "* @field sequenceNumber: monotonic incrementing identifier"]
    #[doc = "* @field status: Status Data for one of more signalized intersections."]
    #[doc = "* @field regional: optional region specific data."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SignalStatusMessage {
        #[rasn(identifier = "timeStamp")]
        pub time_stamp: Option<MinuteOfTheYear>,
        pub second: DSecond,
        #[rasn(identifier = "sequenceNumber")]
        pub sequence_number: Option<MsgCount>,
        pub status: SignalStatusList,
        pub regional: Option<SignalStatusMessageRegional>,
    }
    impl SignalStatusMessage {
        pub fn new(
            time_stamp: Option<MinuteOfTheYear>,
            second: DSecond,
            sequence_number: Option<MsgCount>,
            status: SignalStatusList,
            regional: Option<SignalStatusMessageRegional>,
        ) -> Self {
            Self {
                time_stamp,
                second,
                sequence_number,
                status,
                regional,
            }
        }
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousSignalStatusPackageRegional {
        #[rasn(identifier = "regionId")]
        pub region_id: RegionId,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousSignalStatusPackageRegional {
        pub fn new(region_id: RegionId, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    impl AnonymousSignalStatusPackageRegional {
        pub fn decode_reg_ext_value<D: Decoder>(
            &self,
            decoder: &mut D,
        ) -> Result<RegSignalStatusPackage_Type, D::Error> {
            RegSignalStatusPackage_Type::decode(decoder, Some(&self.reg_ext_value), &self.region_id)
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct SignalStatusPackageRegional(pub SequenceOf<AnonymousSignalStatusPackageRegional>);
    #[doc = "*"]
    #[doc = "* This DF contains all the data needed to describe the preemption or priority state"]
    #[doc = "* of the signal controller with respect to a given request and to uniquely identify the party who requested that state to occur."]
    #[doc = "* It should be noted that this data frame describes both active and anticipated states of the controller. A requested service"]
    #[doc = "* may not be active when the message is created and issued. A requested service may be rejected. This structure allows"]
    #[doc = "* the description of pending requests that have been granted (accepted rather than rejected) but are not yet active and"]
    #[doc = "* being serviced. It also provides for the description of rejected requests so that the initial message is acknowledged"]
    #[doc = "* (completing a dialog using the broadcast messages)."]
    #[doc = "*"]
    #[doc = "* @field requester:  The party that made the initial SREM request."]
    #[doc = "* @field inboundOn:  estimated lane / approach of vehicle."]
    #[doc = "* @field outboundOn: estimated lane / approach of vehicle."]
    #[doc = "* @field minute:     The Estimated Time of Arrival (ETA) when the service is requested. This data echos the data of the request."]
    #[doc = "* @field second:     seconds part of ETA."]
    #[doc = "* @field duration:   duration part of ETA."]
    #[doc = "* @field status:     Status of request, this may include rejection."]
    #[doc = "* @field regional:   optional region specific data."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SignalStatusPackage {
        pub requester: Option<SignalRequesterInfo>,
        #[rasn(identifier = "inboundOn")]
        pub inbound_on: IntersectionAccessPoint,
        #[rasn(identifier = "outboundOn")]
        pub outbound_on: Option<IntersectionAccessPoint>,
        pub minute: Option<MinuteOfTheYear>,
        pub second: Option<DSecond>,
        pub duration: Option<DSecond>,
        pub status: PrioritizationResponseStatus,
        pub regional: Option<SignalStatusPackageRegional>,
    }
    impl SignalStatusPackage {
        pub fn new(
            requester: Option<SignalRequesterInfo>,
            inbound_on: IntersectionAccessPoint,
            outbound_on: Option<IntersectionAccessPoint>,
            minute: Option<MinuteOfTheYear>,
            second: Option<DSecond>,
            duration: Option<DSecond>,
            status: PrioritizationResponseStatus,
            regional: Option<SignalStatusPackageRegional>,
        ) -> Self {
            Self {
                requester,
                inbound_on,
                outbound_on,
                minute,
                second,
                duration,
                status,
                regional,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* This DF consists of a list of @ref SignalStatusPackage entries."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"))]
    pub struct SignalStatusPackageList(pub SequenceOf<SignalStatusPackage>);
    #[doc = "*"]
    #[doc = "* This data element represents the recommended velocity of an object, typically a vehicle speed along a roadway,"]
    #[doc = "* expressed in unsigned units of 0.1 meters per second."]
    #[doc = "*"]
    #[doc = "* - LSB units are 0.1 m/s"]
    #[doc = "* - the value 499 shall be used for values at or greater than 49.9 m/s"]
    #[doc = "* - the value 500 shall be used to indicate that speed is unavailable"]
    #[doc = "*"]
    #[doc = "* @unit: 0.1 m/s"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=500"))]
    pub struct SpeedAdvice(pub u16);
    #[doc = "*"]
    #[doc = "* This DE is used to provide the 95% confidence level for the currently reported"]
    #[doc = "* value of @ref Speed, taking into account the current calibration and precision of the sensor(s) used to measure and/or"]
    #[doc = "* calculate the value. This data element is only to provide the listener with information on the limitations of the sensing"]
    #[doc = "* system, not to support any type of automatic error correction or to imply a guaranteed maximum error. This data element"]
    #[doc = "* should not be used for fault detection or diagnosis, but if a vehicle is able to detect a fault, the confidence interval should"]
    #[doc = "* be increased accordingly."]
    #[doc = "*"]
    #[doc = "* - 0 - `unavailable` : Not Equipped or unavailable"]
    #[doc = "* - 1 - `prec100ms`   : 100  meters / sec"]
    #[doc = "* - 2 - `prec10ms`    : 10   meters / sec"]
    #[doc = "* - 3 - `prec5ms`     : 5    meters / sec"]
    #[doc = "* - 4 - `prec1ms`     : 1    meters / sec"]
    #[doc = "* - 5 - `prec0-1ms`   : 0.1  meters / sec"]
    #[doc = "* - 6 - `prec0-05ms`  : 0.05 meters / sec"]
    #[doc = "* - 7 - `prec0-01ms`  : 0.01 meters / sec"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum SpeedConfidenceDSRC {
        unavailable = 0,
        prec100ms = 1,
        prec10ms = 2,
        prec5ms = 3,
        prec1ms = 4,
        #[rasn(identifier = "prec0-1ms")]
        prec0_1ms = 5,
        #[rasn(identifier = "prec0-05ms")]
        prec0_05ms = 6,
        #[rasn(identifier = "prec0-01ms")]
        prec0_01ms = 7,
    }
    #[doc = "*"]
    #[doc = "* This DF consists of a list of SpeedLimit entries."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=9"))]
    pub struct SpeedLimitList(pub SequenceOf<RegulatorySpeedLimit>);
    #[doc = "*"]
    #[doc = "* This DE relates the type of speed limit to which a given speed refers."]
    #[doc = "*"]
    #[doc = "* - unknown: Speed limit type not available"]
    #[doc = "* - maxSpeedInSchoolZone: Only sent when the limit is active"]
    #[doc = "* - maxSpeedInSchoolZoneWhenChildrenArePresent: Sent at any time"]
    #[doc = "* - maxSpeedInConstructionZone: Used for work zones, incident zones, etc. where a reduced speed is present"]
    #[doc = "* - vehicleMinSpeed: Regulatory speed limit for general traffic"]
    #[doc = "* - vehicleMaxSpeed: Regulatory speed limit for general traffic"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum SpeedLimitType {
        unknown = 0,
        maxSpeedInSchoolZone = 1,
        maxSpeedInSchoolZoneWhenChildrenArePresent = 2,
        maxSpeedInConstructionZone = 3,
        vehicleMinSpeed = 4,
        vehicleMaxSpeed = 5,
        vehicleNightMaxSpeed = 6,
        truckMinSpeed = 7,
        truckMaxSpeed = 8,
        truckNightMaxSpeed = 9,
        vehiclesWithTrailersMinSpeed = 10,
        vehiclesWithTrailersMaxSpeed = 11,
        vehiclesWithTrailersNightMaxSpeed = 12,
    }
    #[doc = "*"]
    #[doc = "* This DF is a single data frame combining multiple related bit fields into one concept."]
    #[doc = "*"]
    #[doc = "* @field heading: confidence for heading values"]
    #[doc = "* @field speed: confidence for speed values"]
    #[doc = "* @field throttle: confidence for throttle values "]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct SpeedandHeadingandThrottleConfidence {
        pub heading: HeadingConfidenceDSRC,
        pub speed: SpeedConfidenceDSRC,
        pub throttle: ThrottleConfidence,
    }
    impl SpeedandHeadingandThrottleConfidence {
        pub fn new(
            heading: HeadingConfidenceDSRC,
            speed: SpeedConfidenceDSRC,
            throttle: ThrottleConfidence,
        ) -> Self {
            Self {
                heading,
                speed,
                throttle,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* This is the 4 octet random device identifier, called the TemporaryID. When used for a mobile OBU device, this value"]
    #[doc = "* will change periodically to ensure the overall anonymity of the vehicle, unlike a typical wireless or wired 802 device ID."]
    #[doc = "* Because this value is used as a means to identify the local vehicles that are interacting during an encounter, it is used in"]
    #[doc = "* the message set. Other devices, such as infrastructure (RSUs), may have a fixed value for the temporary ID value. See"]
    #[doc = "* also @ref StationId which is used in other deployment regions."]
    #[doc = "*"]
    #[doc = "* @note: The circumstances and times at which various DSRC devices (notably OBUs) create and change their current"]
    #[doc = "*        Temporary ID is a complex application level topic. It should be noted that the Temporary ID is not the same as a device"]
    #[doc = "*        MAC value, although when used as a means to uniquely identify a device, both have many common properties. It should"]
    #[doc = "*        further be noted that the MAC value for a mobile OBU device (unlike a typical wireless or wired 802 device) will"]
    #[doc = "*        periodically change to a new random value to ensure the overall anonymity of the vehicle."]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct TemporaryID(pub FixedOctetString<4usize>);
    #[doc = "*"]
    #[doc = "* This DE is used to provide the 95% confidence level for the currently reported"]
    #[doc = "* value of DE @ref Throttle, taking into account the current calibration and precision of the sensor(s) used to measure and/or"]
    #[doc = "* calculate the value. This data element is only to provide information on the limitations of the sensing system, not to"]
    #[doc = "* support any type of automatic error correction or to imply a guaranteed maximum error. This data element should not be"]
    #[doc = "* used for fault detection or diagnosis, but if a vehicle is able to detect a fault, the confidence interval should be increased"]
    #[doc = "* accordingly. If a fault that triggers the MIL is of a nature to render throttle performance unreliable, then ThrottleConfidence"]
    #[doc = "* should be represented as \"notEquipped.\""]
    #[doc = "*"]
    #[doc = "* - 0 - `unavailable`:    B'00 Not Equipped or unavailable"]
    #[doc = "* - 1 - `prec10percent`:  B'01 10 percent Confidence level"]
    #[doc = "* - 2 - `prec1percent`:   B'10 1 percent Confidence level"]
    #[doc = "* - 3 - `prec0-5percent`: B'11 0.5 percent Confidence level"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum ThrottleConfidence {
        unavailable = 0,
        prec10percent = 1,
        prec1percent = 2,
        #[rasn(identifier = "prec0-5percent")]
        prec0_5percent = 3,
    }
    #[doc = "*"]
    #[doc = "* This DF conveys details about the timing of a phase within a movement. The core"]
    #[doc = "* data concept expressed is the time stamp (time mark) at which the related phase will change to the next state. This is"]
    #[doc = "* often found in the MinEndTime element, but the other elements may be needed to convey the full concept when adaptive"]
    #[doc = "* timing is employed."]
    #[doc = "*"]
    #[doc = "* @field startTime: is used to relate when the phase itself started or is expected to start. This in turn allows the"]
    #[doc = "*                   indication that a set of time change details refers to a future phase, rather than a currently active phase."]
    #[doc = "*                   By this method, timing information about \"pre\" phase events (which are the short transitional phase used to alert OBUs to"]
    #[doc = "*                   an impending green/go or yellow/caution phase) and the longer yellow-caution phase data is supported in the same form"]
    #[doc = "*                   as various green/go phases. In theory, the time change details could be sent for a large sequence of phases if the signal"]
    #[doc = "*                   timing was not adaptive and the operator wished to do so. In practice, it is expected only the \"next\" future phase will"]
    #[doc = "*                   commonly be sent. It should be noted that this also supports the sending of time periods regarding various red phases;"]
    #[doc = "*                   however, this is not expected to be done commonly."]
    #[doc = "* @field minEndTime: is used to convey the earliest time possible at which the phase could change, except when"]
    #[doc = "*                   unpredictable events relating to a preemption or priority call disrupt a currently active timing plan. In a phase where the"]
    #[doc = "*                   time is fixed (as in a fixed yellow or clearance time), this element shall be used alone. This value can be viewed as the"]
    #[doc = "*                   earliest possible time at which the phase could change, except when unpredictable events relating to a preemption or"]
    #[doc = "*                   priority call come into play and disrupt a currently active timing plan."]
    #[doc = "* @field maxEndTime: is used to convey the latest time possible which the phase could change,"]
    #[doc = "*                   except when unpredictable events relating to a preemption or priority"]
    #[doc = "*                   call come into play and disrupt a currently active timing plan. In a phase where the time is fixed (as in a fixed yellow or"]
    #[doc = "*                   clearance time), this element shall be used alone."]
    #[doc = "* @field likelyTime: is used to convey the most likely time the phase changes. This occurs between MinEndTime and"]
    #[doc = "*                   MaxEndTime and is only relevant for traffic-actuated control programs. This time might be calculated out of logged"]
    #[doc = "*                   historical values, detected events (e.g., from inductive loops), or from other sources."]
    #[doc = "* @field confidence: is used to convey basic confidence data about the likelyTime."]
    #[doc = "* @field nextTime:   is used to express a general (and presumably less precise) value regarding when this phase will"]
    #[doc = "*                   next occur. This is intended to be used to alert the OBU when the next green/go may occur so that various ECO driving"]
    #[doc = "*                   applications can better manage the vehicle during the intervening stopped time."]
    #[doc = "*"]
    #[doc = "* @note: Remarks: It should be noted that all times are expressed as absolute values and not as countdown timer values. When"]
    #[doc = "*          the stated time mark is reached, the state changes to the next state. Several technical reasons led to this choice; among"]
    #[doc = "*          these was that with a countdown embodiment, there is an inherent need to update the remaining time every time a SPAT"]
    #[doc = "*          message is issued. This would require re-formulating the message content as as well as cryptographically signing the"]
    #[doc = "*          message each time. With the use of absolute values (time marks) chosen here, the current count down time when the"]
    #[doc = "*          message is created is added to the then-current time to create an absolute value and can be used thereafter without"]
    #[doc = "*          change. The message content need only change when the signal controller makes a timing decision to be published. This"]
    #[doc = "*          allows a clean separation of the logical functions of message creation from the logical functions of message scheduling"]
    #[doc = "*          and sending, and fulfills the need to minimize further real time processing when possible. This Standard sets no limits on"]
    #[doc = "*          where each of these functions is performed in the overall roadside system."]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct TimeChangeDetails {
        #[rasn(identifier = "startTime")]
        pub start_time: Option<TimeMark>,
        #[rasn(identifier = "minEndTime")]
        pub min_end_time: TimeMark,
        #[rasn(identifier = "maxEndTime")]
        pub max_end_time: Option<TimeMark>,
        #[rasn(identifier = "likelyTime")]
        pub likely_time: Option<TimeMark>,
        pub confidence: Option<TimeIntervalConfidence>,
        #[rasn(identifier = "nextTime")]
        pub next_time: Option<TimeMark>,
    }
    impl TimeChangeDetails {
        pub fn new(
            start_time: Option<TimeMark>,
            min_end_time: TimeMark,
            max_end_time: Option<TimeMark>,
            likely_time: Option<TimeMark>,
            confidence: Option<TimeIntervalConfidence>,
            next_time: Option<TimeMark>,
        ) -> Self {
            Self {
                start_time,
                min_end_time,
                max_end_time,
                likely_time,
                confidence,
                next_time,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* This DE is used to provide the 95% confidence level for the currently reported value"]
    #[doc = "* of time, taking into account the current calibration and precision of the sensor(s) used to measure and/or calculate the"]
    #[doc = "* value. This data element is only to provide information on the limitations of the sensing system, not to support any type of"]
    #[doc = "* automatic error correction or to imply a guaranteed maximum error. This data element should not be used for fault"]
    #[doc = "* detection or diagnosis, but if a vehicle is able to detect a fault, the confidence interval should be increased accordingly."]
    #[doc = "*"]
    #[doc = "* - 0 - `unavailable`  : Not Equipped or unavailable"]
    #[doc = "* - 1 - `time-100-000` : Better than 100 Seconds"]
    #[doc = "* - 2 - `time-050-000` : Better than 50 Seconds"]
    #[doc = "* - 3 - `time-020-000` : Better than 20 Seconds"]
    #[doc = "* - 4 - `time-010-000` : Better than 10 Seconds"]
    #[doc = "* - 5 - `time-002-000` : Better than 2 Seconds"]
    #[doc = "* - 6 - `time-001-000` : Better than 1 Second"]
    #[doc = "* - 7 - `time-000-500` : Better than 0.5 Seconds"]
    #[doc = "* - 8 - `time-000-200` : Better than 0.2 Seconds"]
    #[doc = "* - 9 - `time-000-100` : Better than 0.1 Seconds"]
    #[doc = "* - 10 - `time-000-050` : Better than 0.05 Seconds"]
    #[doc = "* - 11 - `time-000-020` : Better than 0.02 Seconds"]
    #[doc = "* - 12 - `time-000-010` : Better than 0.01 Seconds"]
    #[doc = "* - 13 - `time-000-005` : Better than 0.005 Seconds"]
    #[doc = "* - 14 - `time-000-002` : Better than 0.001 Seconds"]
    #[doc = "* - 15 - `time-000-001` : Better than 0.001 Seconds"]
    #[doc = "* - 16 - `time-000-000-5` : Better than 0.000,5 Seconds"]
    #[doc = "* - 17 - `time-000-000-2` : Better than 0.000,2 Seconds"]
    #[doc = "* - 18 - `time-000-000-1` : Better than 0.000,1 Seconds"]
    #[doc = "* - 19 - `time-000-000-05` : Better than 0.000,05 Seconds"]
    #[doc = "* - 20 - `time-000-000-02` : Better than 0.000,02 Seconds"]
    #[doc = "* - 21 - `time-000-000-01` : Better than 0.000,01 Seconds"]
    #[doc = "* - 22 - `time-000-000-005` : Better than 0.000,005 Seconds"]
    #[doc = "* - 23 - `time-000-000-002` : Better than 0.000,002 Seconds"]
    #[doc = "* - 24 - `time-000-000-001` : Better than 0.000,001 Seconds"]
    #[doc = "* - 25 - `time-000-000-000-5` : Better than 0.000,000,5 Seconds"]
    #[doc = "* - 26 - `time-000-000-000-2` : Better than 0.000,000,2 Seconds"]
    #[doc = "* - 27 - `time-000-000-000-1` : Better than 0.000,000,1 Seconds"]
    #[doc = "* - 28 - `time-000-000-000-05` : Better than 0.000,000,05 Seconds"]
    #[doc = "* - 29 - `time-000-000-000-02` : Better than 0.000,000,02 Seconds"]
    #[doc = "* - 30 - `time-000-000-000-01` : Better than 0.000,000,01 Seconds"]
    #[doc = "* - 31 - `time-000-000-000-005` : Better than 0.000,000,005 Seconds"]
    #[doc = "* - 32 - `time-000-000-000-002` : Better than 0.000,000,002 Seconds"]
    #[doc = "* - 33 - `time-000-000-000-001` : Better than 0.000,000,001 Seconds"]
    #[doc = "* - 34 - `time-000-000-000-000-5` : Better than 0.000,000,000,5 Seconds"]
    #[doc = "* - 35 - `time-000-000-000-000-2` : Better than 0.000,000,000,2 Seconds"]
    #[doc = "* - 36 - `time-000-000-000-000-1` : Better than 0.000,000,000,1 Seconds"]
    #[doc = "* - 37 - `time-000-000-000-000-05` : Better than 0.000,000,000,05 Seconds"]
    #[doc = "* - 38 - `time-000-000-000-000-02` : Better than 0.000,000,000,02 Seconds"]
    #[doc = "* - 39 - `time-000-000-000-000-01` : Better than 0.000,000,000,01 Seconds"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum TimeConfidence {
        unavailable = 0,
        #[rasn(identifier = "time-100-000")]
        time_100_000 = 1,
        #[rasn(identifier = "time-050-000")]
        time_050_000 = 2,
        #[rasn(identifier = "time-020-000")]
        time_020_000 = 3,
        #[rasn(identifier = "time-010-000")]
        time_010_000 = 4,
        #[rasn(identifier = "time-002-000")]
        time_002_000 = 5,
        #[rasn(identifier = "time-001-000")]
        time_001_000 = 6,
        #[rasn(identifier = "time-000-500")]
        time_000_500 = 7,
        #[rasn(identifier = "time-000-200")]
        time_000_200 = 8,
        #[rasn(identifier = "time-000-100")]
        time_000_100 = 9,
        #[rasn(identifier = "time-000-050")]
        time_000_050 = 10,
        #[rasn(identifier = "time-000-020")]
        time_000_020 = 11,
        #[rasn(identifier = "time-000-010")]
        time_000_010 = 12,
        #[rasn(identifier = "time-000-005")]
        time_000_005 = 13,
        #[rasn(identifier = "time-000-002")]
        time_000_002 = 14,
        #[rasn(identifier = "time-000-001")]
        time_000_001 = 15,
        #[rasn(identifier = "time-000-000-5")]
        time_000_000_5 = 16,
        #[rasn(identifier = "time-000-000-2")]
        time_000_000_2 = 17,
        #[rasn(identifier = "time-000-000-1")]
        time_000_000_1 = 18,
        #[rasn(identifier = "time-000-000-05")]
        time_000_000_05 = 19,
        #[rasn(identifier = "time-000-000-02")]
        time_000_000_02 = 20,
        #[rasn(identifier = "time-000-000-01")]
        time_000_000_01 = 21,
        #[rasn(identifier = "time-000-000-005")]
        time_000_000_005 = 22,
        #[rasn(identifier = "time-000-000-002")]
        time_000_000_002 = 23,
        #[rasn(identifier = "time-000-000-001")]
        time_000_000_001 = 24,
        #[rasn(identifier = "time-000-000-000-5")]
        time_000_000_000_5 = 25,
        #[rasn(identifier = "time-000-000-000-2")]
        time_000_000_000_2 = 26,
        #[rasn(identifier = "time-000-000-000-1")]
        time_000_000_000_1 = 27,
        #[rasn(identifier = "time-000-000-000-05")]
        time_000_000_000_05 = 28,
        #[rasn(identifier = "time-000-000-000-02")]
        time_000_000_000_02 = 29,
        #[rasn(identifier = "time-000-000-000-01")]
        time_000_000_000_01 = 30,
        #[rasn(identifier = "time-000-000-000-005")]
        time_000_000_000_005 = 31,
        #[rasn(identifier = "time-000-000-000-002")]
        time_000_000_000_002 = 32,
        #[rasn(identifier = "time-000-000-000-001")]
        time_000_000_000_001 = 33,
        #[rasn(identifier = "time-000-000-000-000-5")]
        time_000_000_000_000_5 = 34,
        #[rasn(identifier = "time-000-000-000-000-2")]
        time_000_000_000_000_2 = 35,
        #[rasn(identifier = "time-000-000-000-000-1")]
        time_000_000_000_000_1 = 36,
        #[rasn(identifier = "time-000-000-000-000-05")]
        time_000_000_000_000_05 = 37,
        #[rasn(identifier = "time-000-000-000-000-02")]
        time_000_000_000_000_02 = 38,
        #[rasn(identifier = "time-000-000-000-000-01")]
        time_000_000_000_000_01 = 39,
    }
    #[doc = "*"]
    #[doc = "* This is the statistical confidence for the predicted time of signal group state change. For evaluation, the formula"]
    #[doc = "* 10^(x/a)-b with a=82.5 and b=1.3 was used. The values are encoded as probability classes with proposed values listed in"]
    #[doc = "* the below table in the ASN.1 specification."]
    #[doc = "*"]
    #[doc = "* Value: Probability"]
    #[doc = "* - 0 - 21%"]
    #[doc = "* - 1 - 36%"]
    #[doc = "* - 2 - 47%"]
    #[doc = "* - 3 - 56%"]
    #[doc = "* - 4 - 62%"]
    #[doc = "* - 5 - 68%"]
    #[doc = "* - 6 - 73%"]
    #[doc = "* - 7 - 77%"]
    #[doc = "* - 8 - 81%"]
    #[doc = "* - 9 - 85%"]
    #[doc = "* - 10 - 88%"]
    #[doc = "* - 11 - 91%"]
    #[doc = "* - 12 - 94%"]
    #[doc = "* - 13 - 96%"]
    #[doc = "* - 14 - 98%"]
    #[doc = "* - 15 - 100%"]
    #[doc = "*"]
    #[doc = "* @unit: percent"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=15"))]
    pub struct TimeIntervalConfidence(pub u8);
    #[doc = "*"]
    #[doc = "* This DE is used to relate a moment in UTC (Coordinated Universal Time)-based time when a"]
    #[doc = "* signal phase is predicted to change, with a precision of 1/10 of a second. A range of 60 full minutes is supported and it"]
    #[doc = "* can be presumed that the receiver shares a common sense of time with the sender which is kept aligned to within a"]
    #[doc = "* fraction of a second or better."]
    #[doc = "*"]
    #[doc = "* If there is a need to send a value greater than the range allowed by the data element (over one hour in the future), the"]
    #[doc = "* value 36000 shall be sent and shall be interpreted to indicate an indefinite future time value. When the value to be used is"]
    #[doc = "* undefined or unknown a value of 36001 shall be sent. Note that leap seconds are also supported."]
    #[doc = "*"]
    #[doc = "* The value is tenths of a second in the current or next hour in units of 1/10th second from UTC time"]
    #[doc = "* - A range of 0-36000 covers one hour"]
    #[doc = "* - The values 35991..35999 are used when a leap second occurs"]
    #[doc = "* - The value 36000 is used to indicate time >3600 seconds"]
    #[doc = "* - 36001 is to be used when value undefined or unknown"]
    #[doc = "*"]
    #[doc = "* @note: Note that this is NOT expressed in GPS time or in local time"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=36001"))]
    pub struct TimeMark(pub u16);
    #[doc = "*"]
    #[doc = "* This DE is used to provide the R09 tour information."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V2.2.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=4294967295"))]
    pub struct TourNumber(pub u32);
    #[doc = "*"]
    #[doc = "* This DE is used to provide the R09 train length."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V2.2.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=7"))]
    pub struct TrainLength(pub u8);
    #[doc = "*"]
    #[doc = "* This DE is used to provide the R09 direction information."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V2.2.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct TransitDirection(pub u8);
    #[doc = "*"]
    #[doc = "*  This DE is used to relate basic level of current ridership."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum TransitVehicleOccupancy {
        occupancyUnknown = 0,
        occupancyEmpty = 1,
        occupancyVeryLow = 2,
        occupancyLow = 3,
        occupancyMed = 4,
        occupancyHigh = 5,
        occupancyNearlyFull = 6,
        occupancyFull = 7,
    }
    #[doc = "*"]
    #[doc = "* This DE is used to relate basic information about the transit run in progress. This is"]
    #[doc = "* typically used in a priority request to a signalized system and becomes part of the input processing for how that system"]
    #[doc = "* will respond to the request."]
    #[doc = "*"]
    #[doc = "* - 0 - `loading`:     parking and unable to move at this time"]
    #[doc = "* - 1 - `anADAuse`:    an ADA access is in progress (wheelchairs, kneeling, etc.)"]
    #[doc = "* - 2 - `aBikeLoad`:   loading of a bicycle is in progress"]
    #[doc = "* - 3 - `doorOpen`:    a vehicle door is open for passenger access"]
    #[doc = "* - 4 - `charging`:    a vehicle is connected to charging point"]
    #[doc = "* - 5 - `atStopLine`:  a vehicle is at the stop line for the lane it is in"]
    #[doc = "*"]
    #[doc = "* @note: Most of these values are used to detect that the transit vehicle in not in a state where movement can occur"]
    #[doc = "* (and that therefore any priority signal should be ignored until the vehicle is again ready to depart)."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct TransitVehicleStatus(pub FixedBitString<8usize>);
    #[doc = "*"]
    #[doc = "* This DF expresses the speed of the vehicle and the state of the transmission."]
    #[doc = "* The transmission state of **reverse** can be used as a sign value for the speed element when needed."]
    #[doc = "*"]
    #[doc = "* @field transmisson: state of the transmission"]
    #[doc = "* @field speed: speed of the vehicle"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct TransmissionAndSpeed {
        pub transmisson: TransmissionState,
        pub speed: Velocity,
    }
    impl TransmissionAndSpeed {
        pub fn new(transmisson: TransmissionState, speed: Velocity) -> Self {
            Self { transmisson, speed }
        }
    }
    #[doc = "*"]
    #[doc = "* This DE is used to provide the current state of the vehicle transmission."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum TransmissionState {
        neutral = 0,
        park = 1,
        forwardGears = 2,
        reverseGears = 3,
        reserved1 = 4,
        reserved2 = 5,
        reserved3 = 6,
        unavailable = 7,
    }
    #[doc = "*"]
    #[doc = "* The height of the vehicle, measured from the ground to the highest surface, excluding any antenna(s), and"]
    #[doc = "* expressed in units of 5 cm. In cases of vehicles with adjustable ride heights, camper shells, and other devices which may"]
    #[doc = "* cause the overall height to vary, the largest possible height will be used."]
    #[doc = "*"]
    #[doc = "* Value is the height of the vehicle, LSB units of 5 cm, range to 6.35 meters"]
    #[doc = "*"]
    #[doc = "* @unit: 5cm"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=127"))]
    pub struct VehicleHeight(pub u8);
    #[doc = "*"]
    #[doc = "* This DF is used to contain either a (US) TemporaryID or an (EU) StationID in a simple frame."]
    #[doc = "* These two different value domains are used to uniquely identify a vehicle or other object in these two regional DSRC"]
    #[doc = "* value is unavailable but needed by another type of user (such as the roadside infrastructure sending data about an"]
    #[doc = "* environments. In normal use cases, this value changes over time to prevent tracking of the subject vehicle. When this"]
    #[doc = "* unequipped vehicle), the value zero shall be used. A typical restriction on the use of this value during a dialog or other"]
    #[doc = "* exchange is that the value remains constant for the duration of that exchange. Refer to the performance requirements for"]
    #[doc = "* a given application for details."]
    #[doc = "*"]
    #[doc = "* @field entityID: representation for US stations"]
    #[doc = "* @field stationID: representation for EU stations"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum VehicleID {
        entityID(TemporaryID),
        stationID(StationID),
    }
    #[doc = "*"]
    #[doc = "* This DE is a type list (i.e., a classification list) of the vehicle in terms of overall size. The"]
    #[doc = "* data element entries follow the definitions defined in the US DOT Highway Performance Monitoring System (HPMS)."]
    #[doc = "* Many infrastructure roadway operators collect and classify data according to this list for regulatory reporting needs."]
    #[doc = "* Within the ITS industry and within the DSRC message set standards work, there are many similar lists of types for"]
    #[doc = "* overlapping needs and uses."]
    #[doc = "*"]
    #[doc = "* - 0 - `none`:       Not Equipped, Not known or unavailable"]
    #[doc = "* - 1 - `unknown`:    Does not fit any other category"]
    #[doc = "* - 2 - `special`:    Special use"]
    #[doc = "* - 3 - `moto`:       Motorcycle"]
    #[doc = "* - 4 - `car`:        Passenger car"]
    #[doc = "* - 5 - `carOther`:   Four tire single units"]
    #[doc = "* - 6 - `bus`:        Buses"]
    #[doc = "* - 7 - `axleCnt2`:   Two axle, six tire single units"]
    #[doc = "* - 8 - `axleCnt3`:   Three axle, single units"]
    #[doc = "* - 9 - `axleCnt4`:   Four or more axle, single unit"]
    #[doc = "* - 10 - `axleCnt4Trailer`:        Four or less axle, single trailer"]
    #[doc = "* - 11 - `axleCnt5Trailer`:        Five or less axle, single trailer"]
    #[doc = "* - 12 - `axleCnt6Trailer`:        Six or more axle, single trailer"]
    #[doc = "* - 13 - `axleCnt5MultiTrailer`:   Five or less axle, multi-trailer"]
    #[doc = "* - 14 - `axleCnt6MultiTrailer`:   Six axle, multi-trailer"]
    #[doc = "* - 15 - `axleCnt7MultiTrailer`:   Seven or more axle, multi-trailer"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum VehicleType {
        none = 0,
        unknown = 1,
        special = 2,
        moto = 3,
        car = 4,
        carOther = 5,
        bus = 6,
        axleCnt2 = 7,
        axleCnt3 = 8,
        axleCnt4 = 9,
        axleCnt4Trailer = 10,
        axleCnt5Trailer = 11,
        axleCnt6Trailer = 12,
        axleCnt5MultiTrailer = 13,
        axleCnt6MultiTrailer = 14,
        axleCnt7MultiTrailer = 15,
    }
    #[doc = "*"]
    #[doc = "* This DE represents the velocity of an object, typically a vehicle speed or the recommended speed of"]
    #[doc = "* travel along a roadway, expressed in unsigned units of 0.02 meters per second. When used with motor vehicles it may be"]
    #[doc = "* combined with the transmission state to form a data frame for use. A value of 8191 shall be used when the speed is"]
    #[doc = "* unavailable. Note that Velocity as used here is intended to be a scalar value and not a vector."]
    #[doc = "*"]
    #[doc = "* The value 8191 indicates that velocity is unavailable"]
    #[doc = "*"]
    #[doc = "* @unit: 0.02 m/s"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=8191"))]
    pub struct Velocity(pub u16);
    #[doc = "*"]
    #[doc = "* This DE is used to provide the R09 version information."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V2.2.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=4294967295"))]
    pub struct VersionId(pub u32);
    #[doc = "*"]
    #[doc = "* This DE is used to indicate to the vehicle that it must stop at the stop line and not move past."]
    #[doc = "*"]
    #[doc = "* If \"true\", the vehicles on this specific connecting maneuver have to stop on the stop-line and not to enter the collision area"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(delegate)]
    pub struct WaitOnStopline(pub bool);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=10000"))]
    pub struct ZoneLength(pub u16);
    pub const ADD_GRP_A: RegionId = RegionId(1);
    pub const ADD_GRP_B: RegionId = RegionId(2);
    pub const ADD_GRP_C: RegionId = RegionId(3);
    pub const DIESEL: FuelType = FuelType(3);
    pub const ELECTRIC: FuelType = FuelType(4);
    pub const ETHANOL: FuelType = FuelType(2);
    pub const GASOLINE: FuelType = FuelType(1);
    pub const HYBRID: FuelType = FuelType(5);
    pub const HYDROGEN: FuelType = FuelType(6);
    pub const NAT_GAS_COMP: FuelType = FuelType(8);
    pub const NAT_GAS_LIQUID: FuelType = FuelType(7);
    pub const NO_REGION: RegionId = RegionId(0);
    pub const PROPANE: FuelType = FuelType(9);
    pub const UNKNOWN_FUEL: FuelType = FuelType(0);
}
#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod etsi_its_dsrc_add_grp_c {
    extern crate alloc;
    use super::etsi_its_cdd::{Altitude, DeltaAltitude, StationID, VehicleMass};
    use super::etsi_its_dsrc::{
        DeltaTime, FuelType, IntersectionID, LaneConnectionID, LaneID, NodeOffsetPointXY,
        NodeSetXY, PrioritizationResponseStatus, SignalGroupID, VehicleHeight,
    };
    use core::borrow::Borrow;
    use lazy_static::lazy_static;
    use rasn::prelude::*;
    #[doc = "*"]
    #[doc = "* This DE defines an enumerated list of battery states."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum BatteryStatus {
        unknown = 0,
        critical = 1,
        low = 2,
        good = 3,
    }
    #[doc = "*"]
    #[doc = "* This DF adds positioning support from the infrastructure to the vehicle."]
    #[doc = "*"]
    #[doc = "* @field itsStationPositions: defines a list of ITS stations (e.g. vehicles) and their corresponding position on"]
    #[doc = "*                             the driving lane as defined in the lane topology of the MapData message or the GNSS position"]
    #[doc = "*                             deviation of the ITS Station from the high precision reference position in X/Y coordinates. It"]
    #[doc = "*                             enables accurate, real-time positioning support to the moving ITS entities by the infrastructure.*"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "ConnectionManeuverAssist-addGrpC")]
    #[non_exhaustive]
    pub struct ConnectionManeuverAssistAddGrpC {
        #[rasn(identifier = "itsStationPosition")]
        pub its_station_position: Option<ItsStationPositionList>,
    }
    impl ConnectionManeuverAssistAddGrpC {
        pub fn new(its_station_position: Option<ItsStationPositionList>) -> Self {
            Self {
                its_station_position,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* This DF defines the trajectory for travelling through the conflict area of an intersection and connects "]
    #[doc = "* e.g an ingress with an egress lane. The trajectory is defined by two or more nodes. "]
    #[doc = "* The starting node overlaps e.g. with the node of the ingress lane towards the"]
    #[doc = "* conflict zone. The ending node overlaps e.g. with the first node of the connected egress lane. "]
    #[doc = "* See the example in clause [ISO TS 19091] G.8.2.5."]
    #[doc = "*"]
    #[doc = "* @field nodes: defines a list of nodes for the trajectory. It defines e.g. a geometric trajectory from an ingressing"]
    #[doc = "*               to a connected egressing lane and the X/Y position value of the first node of the trajectory is the same as"]
    #[doc = "*               the node of the ingress lane. The X/Y position of the last node is the same as the X/Y position of the first"]
    #[doc = "*               node of the egressing lane."]
    #[doc = "* @field connectionID: defines the identifier of an allowed `maneuver` (e.g. ingress / egress relation). "]
    #[doc = "*               A generic Lane offers one or more allowed `maneuvers`, therefore the trajectory is reference to the related `maneuver`."]
    #[doc = "*"]
    #[doc = "* @note: @ref Reg-GenericLane allows providing up to 4 connecting trajectories. In case a lane has more than 4 connecting trajectories,"]
    #[doc = "* priority should be given to connecting trajectories of motorized traffic and complex manoeuvres."]
    #[doc = "*"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "ConnectionTrajectory-addGrpC")]
    #[non_exhaustive]
    pub struct ConnectionTrajectoryAddGrpC {
        pub nodes: NodeSetXY,
        #[rasn(identifier = "connectionID")]
        pub connection_id: LaneConnectionID,
    }
    impl ConnectionTrajectoryAddGrpC {
        pub fn new(nodes: NodeSetXY, connection_id: LaneConnectionID) -> Self {
            Self {
                nodes,
                connection_id,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* This DE defines an enumerated list of toxic emission types for vehicles."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum EmissionType {
        euro1 = 0,
        euro2 = 1,
        euro3 = 2,
        euro4 = 3,
        euro5 = 4,
        euro6 = 5,
    }
    #[doc = "*"]
    #[doc = "* This DE defines a list of reasons for sudden changes in"]
    #[doc = "* eventState parameters, thereby offering a reason for extended waiting times."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum ExceptionalCondition {
        unknown = 0,
        publicTransportPriority = 1,
        emergencyVehiclePriority = 2,
        trainPriority = 3,
        bridgeOpen = 4,
        vehicleHeight = 5,
        weather = 6,
        trafficJam = 7,
        tunnelClosure = 8,
        meteringActive = 9,
        truckPriority = 10,
        bicyclePlatoonPriority = 11,
        vehiclePlatoonPriority = 12,
    }
    #[doc = "*"]
    #[doc = "* This DF defines a list of prioritization responses e.g. public transport acceleration."]
    #[doc = "* The signal prioritization (e.g. public transport) procedure in this profile follows two strategies."]
    #[doc = "* - For simple prioritization requests, the CAM/SPAT messages are used. "]
    #[doc = "*   This allows the migration of old legal systems towards C-ITS."]
    #[doc = "*   In this case, the CAM message is used to trigger the request towards the traffic light controller. "]
    #[doc = "*   The traffic light controller checks the request and broadcasts the status for the priority request with this DF (see [ISO TS 19091] G.5.1.9)."]
    #[doc = "* - For more complex signal requests, the SignalRequestMessage/SignalStatusMessage messages are to be used."]
    #[doc = "*"]
    #[doc = "* @field activePrioritizations: list of Prioritizations."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "IntersectionState-addGrpC")]
    #[non_exhaustive]
    pub struct IntersectionStateAddGrpC {
        #[rasn(identifier = "activePrioritizations")]
        pub active_prioritizations: Option<PrioritizationResponseList>,
    }
    impl IntersectionStateAddGrpC {
        pub fn new(active_prioritizations: Option<PrioritizationResponseList>) -> Self {
            Self {
                active_prioritizations,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* This DF is used to provide real-time positioning information feedback to a specific ITS station "]
    #[doc = "* (e.g. vehicle, pedestrian, bicycle) by infrastructure equipment."]
    #[doc = "*  The position information includes, for example, the driving, crossing lane and/or the X/Y coordinates in relation to"]
    #[doc = "* the reference position of the MapData. The `timeReference` indicates the time stamp of the the"]
    #[doc = "* message (received from an ITS station) for which the positioning feedback has been computed."]
    #[doc = "* "]
    #[doc = "* @field stationID: unique identifier."]
    #[doc = "* @field laneID: LaneID."]
    #[doc = "* @field nodeXY: NodeOffsetPointXY."]
    #[doc = "* @field timeReference: TimeReference."]
    #[doc = "*"]
    #[doc = "* @note: The computation of the positioning feedback is out of focus of this standard."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ItsStationPosition {
        #[rasn(identifier = "stationID")]
        pub station_id: StationID,
        #[rasn(identifier = "laneID")]
        pub lane_id: Option<LaneID>,
        #[rasn(identifier = "nodeXY")]
        pub node_xy: Option<NodeOffsetPointXY>,
        #[rasn(identifier = "timeReference")]
        pub time_reference: Option<TimeReference>,
    }
    impl ItsStationPosition {
        pub fn new(
            station_id: StationID,
            lane_id: Option<LaneID>,
            node_xy: Option<NodeOffsetPointXY>,
            time_reference: Option<TimeReference>,
        ) -> Self {
            Self {
                station_id,
                lane_id,
                node_xy,
                time_reference,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=5"))]
    pub struct ItsStationPositionList(pub SequenceOf<ItsStationPosition>);
    #[doc = "*"]
    #[doc = "* Lanes may have limitations regarding vehicle height (e.g. due to a tunnel) and vehicle weight (e.g. due to a bridge). "]
    #[doc = "*"]
    #[doc = "* @field maxVehicleHeight: maximum allowed vehicle height"]
    #[doc = "* @field maxVehicleWeight: maximum allowed vehicle mass"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "LaneAttributes-addGrpC")]
    #[non_exhaustive]
    pub struct LaneAttributesAddGrpC {
        #[rasn(identifier = "maxVehicleHeight")]
        pub max_vehicle_height: Option<VehicleHeight>,
        #[rasn(identifier = "maxVehicleWeight")]
        pub max_vehicle_weight: Option<VehicleMass>,
    }
    impl LaneAttributesAddGrpC {
        pub fn new(
            max_vehicle_height: Option<VehicleHeight>,
            max_vehicle_weight: Option<VehicleMass>,
        ) -> Self {
            Self {
                max_vehicle_height,
                max_vehicle_weight,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* This DF defines a list of three-dimensional positions of signal heads in an intersection. "]
    #[doc = "* It enables vehicles to identify the signal head location for optical evaluation of the traffic light. "]
    #[doc = "* Combined with the SPAT/MapData messages, it enables e.g. driving vehicles to enhance safety decision in critical situations."]
    #[doc = "*"]
    #[doc = "* @field signalHeadLocations: list of geo positions"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "MapData-addGrpC")]
    #[non_exhaustive]
    pub struct MapDataAddGrpC {
        #[rasn(identifier = "signalHeadLocations")]
        pub signal_head_locations: Option<SignalHeadLocationList>,
    }
    impl MapDataAddGrpC {
        pub fn new(signal_head_locations: Option<SignalHeadLocationList>) -> Self {
            Self {
                signal_head_locations,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* Priority and preemption have a considerable impact to the timing parameters in the SPAT message (eventState)."]
    #[doc = "* User acceptance is expected to increase if the reason for sudden changes in timing parameters is communicated to them."]
    #[doc = "*"]
    #[doc = "* @field stateChangeReason: reason code"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "MovementEvent-addGrpC")]
    #[non_exhaustive]
    pub struct MovementEventAddGrpC {
        #[rasn(identifier = "stateChangeReason")]
        pub state_change_reason: Option<ExceptionalCondition>,
    }
    impl MovementEventAddGrpC {
        pub fn new(state_change_reason: Option<ExceptionalCondition>) -> Self {
            Self {
                state_change_reason,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* This DF is used to to identify a node of a lane (waypoint) by its `lane` and node identifier `id`. "]
    #[doc = "*"]
    #[doc = "* The `intersectionID` is used if the referenced lane belongs to an adjacent intersection. If the node"]
    #[doc = "* belongs to a connection trajectory ([ISO TS 19091] G.5.1.2) the `connectionID` is used."]
    #[doc = "*"]
    #[doc = "* @field id: unique identifier."]
    #[doc = "* @field lane: identifier from lane."]
    #[doc = "* @field connectionID: identifier from connection."]
    #[doc = "* @field intersectionID: identifier from intersection."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct Node {
        pub id: Integer,
        pub lane: Option<LaneID>,
        #[rasn(identifier = "connectionID")]
        pub connection_id: Option<LaneConnectionID>,
        #[rasn(identifier = "intersectionID")]
        pub intersection_id: Option<IntersectionID>,
    }
    impl Node {
        pub fn new(
            id: Integer,
            lane: Option<LaneID>,
            connection_id: Option<LaneConnectionID>,
            intersection_id: Option<IntersectionID>,
        ) -> Self {
            Self {
                id,
                lane,
                connection_id,
                intersection_id,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* This DF defines additional attributes to support public transport and to enable a simple way of defining lane links."]
    #[doc = "*"]
    #[doc = "* @field ptvRequest: defines control types attached to a node on a lane used by public transport for triggering"]
    #[doc = "*                    the transmission of messages (e.g. prioritization request). It includes control points for public transport prioritization. "]
    #[doc = "*                    These control points are currently implemented by legacy systems using hardware sensors mounted on the roadside."]
    #[doc = "* @field nodeLink:   defines a link to one or to a set of another node/lane from this node. The nodeLink allows to set a link between specific nodes "]
    #[doc = "*                    of generic lanes or trajectories. This supports e.g. lane merging/diverging situations ([ISO TS 19091] G.8.2.7) and the linking of trajectories "]
    #[doc = "*                    in the conflict zone to lanes (see example [ISO TS 19091] G.8.2.5)."]
    #[doc = "* @field node:       defines an identifier of this node."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "NodeAttributeSet-addGrpC")]
    #[non_exhaustive]
    pub struct NodeAttributeSetAddGrpC {
        #[rasn(identifier = "ptvRequest")]
        pub ptv_request: Option<PtvRequestType>,
        #[rasn(identifier = "nodeLink")]
        pub node_link: Option<NodeLink>,
        pub node: Option<Node>,
    }
    impl NodeAttributeSetAddGrpC {
        pub fn new(
            ptv_request: Option<PtvRequestType>,
            node_link: Option<NodeLink>,
            node: Option<Node>,
        ) -> Self {
            Self {
                ptv_request,
                node_link,
                node,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=5"))]
    pub struct NodeLink(pub SequenceOf<Node>);
    #[doc = "*"]
    #[doc = "* This DF includes the altitude data element defined in the common data dictionary [ETSI CDD]."]
    #[doc = "*"]
    #[doc = "* @field elevation: the data element is replaced by the ETSI `altitude` data element using the regional extension. "]
    #[doc = "*                   The `altitude` data element is defined in Position3D-addGrpC of this profile."]
    #[doc = "*                   Position3D-addGrpC extends the @ref Position3D using the regional extension framework."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "Position3D-addGrpC")]
    #[non_exhaustive]
    pub struct Position3DAddGrpC {
        pub altitude: Altitude,
    }
    impl Position3DAddGrpC {
        pub fn new(altitude: Altitude) -> Self {
            Self { altitude }
        }
    }
    #[doc = "*"]
    #[doc = "* This DF is used to provide the prioritization status response and the"]
    #[doc = "* signal group identifier for a specific ITS station (e.g. vehicle)."]
    #[doc = "*"]
    #[doc = "* @field stationID: StationID."]
    #[doc = "* @field priorState: PrioritizationResponseStatus."]
    #[doc = "* @field signalGroup: SignalGroupID."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct PrioritizationResponse {
        #[rasn(identifier = "stationID")]
        pub station_id: StationID,
        #[rasn(identifier = "priorState")]
        pub prior_state: PrioritizationResponseStatus,
        #[rasn(identifier = "signalGroup")]
        pub signal_group: SignalGroupID,
    }
    impl PrioritizationResponse {
        pub fn new(
            station_id: StationID,
            prior_state: PrioritizationResponseStatus,
            signal_group: SignalGroupID,
        ) -> Self {
            Self {
                station_id,
                prior_state,
                signal_group,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=10"))]
    pub struct PrioritizationResponseList(pub SequenceOf<PrioritizationResponse>);
    #[doc = "*"]
    #[doc = "* This DE defines a list of activation requests used for C-ITS migration of legacy public "]
    #[doc = "* transport prioritization systems. "]
    #[doc = "* The activation points are used while approaching to an intersection."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum PtvRequestType {
        preRequest = 0,
        mainRequest = 1,
        doorCloseRequest = 2,
        cancelRequest = 3,
        emergencyRequest = 4,
    }
    #[doc = "*"]
    #[doc = "* This DE defines a list of reasons for rejected priority requests."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum RejectedReason {
        unknown = 0,
        exceptionalCondition = 1,
        maxWaitingTimeExceeded = 2,
        ptPriorityDisabled = 3,
        higherPTPriorityGranted = 4,
        vehicleTrackingUnknown = 5,
    }
    #[doc = "*"]
    #[doc = "* Some road authorities like to give priority to vehicles based on the type of fuel they use. In addition,"]
    #[doc = "* electric vehicles may receive priority based on their battery status."]
    #[doc = "*"]
    #[doc = "* @field fuel: fuel used by vehicle."]
    #[doc = "* @field batteryStatus: current batter status of vehicle."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "RequestorDescription-addGrpC")]
    #[non_exhaustive]
    pub struct RequestorDescriptionAddGrpC {
        pub fuel: Option<FuelType>,
        #[rasn(identifier = "batteryStatus")]
        pub battery_status: Option<BatteryStatus>,
    }
    impl RequestorDescriptionAddGrpC {
        pub fn new(fuel: Option<FuelType>, battery_status: Option<BatteryStatus>) -> Self {
            Self {
                fuel,
                battery_status,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* This DF defines the driving restriction based on toxic emission type. "]
    #[doc = "* The meaning of the word `restriction` is ambiguous as it may have a double interpretation, being:"]
    #[doc = "*  - only these vehicles are allowed OR "]
    #[doc = "*  - these vehicles are not allowed and all others are. "]
    #[doc = "* The former is what is intended by the base standard."]
    #[doc = "*"]
    #[doc = "* @field emission: restriction baesed on emission."]
    #[doc = "* @field fuel: restriction baesed on fuel."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "RestrictionUserType-addGrpC")]
    #[non_exhaustive]
    pub struct RestrictionUserTypeAddGrpC {
        pub emission: Option<EmissionType>,
        pub fuel: Option<FuelType>,
    }
    impl RestrictionUserTypeAddGrpC {
        pub fn new(emission: Option<EmissionType>, fuel: Option<FuelType>) -> Self {
            Self { emission, fuel }
        }
    }
    #[doc = "*"]
    #[doc = "* This DF defines the XYZ position of a signal head within an intersection"]
    #[doc = "* and indicates the related signal group identifier."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SignalHeadLocation {
        #[rasn(identifier = "nodeXY")]
        pub node_xy: NodeOffsetPointXY,
        #[rasn(identifier = "nodeZ")]
        pub node_z: DeltaAltitude,
        #[rasn(identifier = "signalGroupID")]
        pub signal_group_id: SignalGroupID,
    }
    impl SignalHeadLocation {
        pub fn new(
            node_xy: NodeOffsetPointXY,
            node_z: DeltaAltitude,
            signal_group_id: SignalGroupID,
        ) -> Self {
            Self {
                node_xy,
                node_z,
                signal_group_id,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=64"))]
    pub struct SignalHeadLocationList(pub SequenceOf<SignalHeadLocation>);
    #[doc = "*"]
    #[doc = "* The traffic control centre (TCC) may advice a public transport vehicle (e.g. bus) to synchronize his travel time. "]
    #[doc = "* This may happen when, for example, two busses, due to special traffic conditions, are out of schedule. "]
    #[doc = "* The first might be too late, the second too fast. The consequence is that the second is driving"]
    #[doc = "* just behind the first and is empty as all passengers are within the first one. To avoid this often-occurring"]
    #[doc = "* situation, the TCC transmits time synchronization advices to the public transport vehicles using the"]
    #[doc = "* signal status message. "]
    #[doc = "*"]
    #[doc = "* @field synchToSchedule: DeltaTime."]
    #[doc = "* @field rejectedReason: RejectedReason."]
    #[doc = "*"]
    #[doc = "* @Note: The @ref PrioritizationResponseStatus provides optionally the reason for prioritization response rejection."]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SignalStatusPackage-addGrpC")]
    #[non_exhaustive]
    pub struct SignalStatusPackageAddGrpC {
        #[rasn(identifier = "synchToSchedule")]
        pub synch_to_schedule: Option<DeltaTime>,
        #[rasn(identifier = "rejectedReason")]
        pub rejected_reason: Option<RejectedReason>,
    }
    impl SignalStatusPackageAddGrpC {
        pub fn new(
            synch_to_schedule: Option<DeltaTime>,
            rejected_reason: Option<RejectedReason>,
        ) -> Self {
            Self {
                synch_to_schedule,
                rejected_reason,
            }
        }
    }
    #[doc = "*"]
    #[doc = "* This DE defines a value in milliseconds in the current minute related to UTC time. "]
    #[doc = "* The range of 60 000 covers one minute (60 seconds * 1 000 milliseconds)"]
    #[doc = "*"]
    #[doc = "* @category: Infrastructure information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=60000"))]
    pub struct TimeReference(pub u16);
}
#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod etsi_its_dsrc_region {
    extern crate alloc;
    use super::etsi_its_dsrc::*;
    use super::etsi_its_dsrc_add_grp_c::{
        ConnectionManeuverAssistAddGrpC, ConnectionTrajectoryAddGrpC, IntersectionStateAddGrpC,
        LaneAttributesAddGrpC, MapDataAddGrpC, MovementEventAddGrpC, NodeAttributeSetAddGrpC,
        Position3DAddGrpC, RequestorDescriptionAddGrpC, RestrictionUserTypeAddGrpC,
        SignalStatusPackageAddGrpC,
    };
    use core::borrow::Borrow;
    use lazy_static::lazy_static;
    use rasn::prelude::*;
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegAdvisorySpeed_Type {}
    impl RegAdvisorySpeed_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegAdvisorySpeed_id {}
    impl RegAdvisorySpeed_id {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegComputedLane_Type {}
    impl RegComputedLane_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegComputedLane_id {}
    impl RegComputedLane_id {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegConnectionManeuverAssist_Type {
        AddGrpC(ConnectionManeuverAssistAddGrpC),
    }
    impl RegConnectionManeuverAssist_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                i if i == &ADD_GRP_C => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::AddGrpC)?),
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                (Self::AddGrpC(inner), i) if i == &ADD_GRP_C => inner.encode(encoder),
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegGenericLane_Type {
        AddGrpC(ConnectionTrajectoryAddGrpC),
    }
    impl RegGenericLane_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                i if i == &ADD_GRP_C => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::AddGrpC)?),
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                (Self::AddGrpC(inner), i) if i == &ADD_GRP_C => inner.encode(encoder),
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegIntersectionGeometry_Type {}
    impl RegIntersectionGeometry_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegIntersectionGeometry_id {}
    impl RegIntersectionGeometry_id {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegIntersectionState_Type {
        AddGrpC(IntersectionStateAddGrpC),
    }
    impl RegIntersectionState_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                i if i == &ADD_GRP_C => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::AddGrpC)?),
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                (Self::AddGrpC(inner), i) if i == &ADD_GRP_C => inner.encode(encoder),
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegLaneAttributes_Type {
        AddGrpC(LaneAttributesAddGrpC),
    }
    impl RegLaneAttributes_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                i if i == &ADD_GRP_C => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::AddGrpC)?),
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                (Self::AddGrpC(inner), i) if i == &ADD_GRP_C => inner.encode(encoder),
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegLaneDataAttribute_Type {}
    impl RegLaneDataAttribute_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegLaneDataAttribute_id {}
    impl RegLaneDataAttribute_id {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegMapData_Type {
        AddGrpC(MapDataAddGrpC),
    }
    impl RegMapData_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                i if i == &ADD_GRP_C => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::AddGrpC)?),
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                (Self::AddGrpC(inner), i) if i == &ADD_GRP_C => inner.encode(encoder),
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegMovementEvent_Type {
        AddGrpC(MovementEventAddGrpC),
    }
    impl RegMovementEvent_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                i if i == &ADD_GRP_C => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::AddGrpC)?),
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                (Self::AddGrpC(inner), i) if i == &ADD_GRP_C => inner.encode(encoder),
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegMovementState_Type {}
    impl RegMovementState_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegMovementState_id {}
    impl RegMovementState_id {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegNodeAttributeSetXY_Type {
        AddGrpC(NodeAttributeSetAddGrpC),
    }
    impl RegNodeAttributeSetXY_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                i if i == &ADD_GRP_C => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::AddGrpC)?),
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                (Self::AddGrpC(inner), i) if i == &ADD_GRP_C => inner.encode(encoder),
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegNodeOffsetPointXY_Type {}
    impl RegNodeOffsetPointXY_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegNodeOffsetPointXY_id {}
    impl RegNodeOffsetPointXY_id {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegPosition3D_Type {
        AddGrpC(Position3DAddGrpC),
    }
    impl RegPosition3D_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                i if i == &ADD_GRP_C => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::AddGrpC)?),
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                (Self::AddGrpC(inner), i) if i == &ADD_GRP_C => inner.encode(encoder),
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegRTCMcorrections_Type {}
    impl RegRTCMcorrections_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegRTCMcorrections_id {}
    impl RegRTCMcorrections_id {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegRequestorDescription_Type {
        AddGrpC(RequestorDescriptionAddGrpC),
    }
    impl RegRequestorDescription_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                i if i == &ADD_GRP_C => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::AddGrpC)?),
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                (Self::AddGrpC(inner), i) if i == &ADD_GRP_C => inner.encode(encoder),
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegRequestorType_Type {}
    impl RegRequestorType_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegRequestorType_id {}
    impl RegRequestorType_id {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegRestrictionUserType_Type {
        AddGrpC(RestrictionUserTypeAddGrpC),
    }
    impl RegRestrictionUserType_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                i if i == &ADD_GRP_C => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::AddGrpC)?),
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                (Self::AddGrpC(inner), i) if i == &ADD_GRP_C => inner.encode(encoder),
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegRoadSegment_Type {}
    impl RegRoadSegment_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegRoadSegment_id {}
    impl RegRoadSegment_id {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegSPAT_Type {}
    impl RegSPAT_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegSPAT_id {}
    impl RegSPAT_id {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegSignalControlZone_Type {}
    impl RegSignalControlZone_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegSignalControlZone_id {}
    impl RegSignalControlZone_id {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegSignalRequest_Type {}
    impl RegSignalRequest_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegSignalRequest_id {}
    impl RegSignalRequest_id {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegSignalRequestMessage_Type {}
    impl RegSignalRequestMessage_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegSignalRequestMessage_id {}
    impl RegSignalRequestMessage_id {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegSignalRequestPackage_Type {}
    impl RegSignalRequestPackage_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegSignalRequestPackage_id {}
    impl RegSignalRequestPackage_id {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegSignalStatus_Type {}
    impl RegSignalStatus_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegSignalStatus_id {}
    impl RegSignalStatus_id {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegSignalStatusMessage_Type {}
    impl RegSignalStatusMessage_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegSignalStatusMessage_id {}
    impl RegSignalStatusMessage_id {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegSignalStatusPackage_Type {
        AddGrpC(SignalStatusPackageAddGrpC),
    }
    impl RegSignalStatusPackage_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                i if i == &ADD_GRP_C => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::AddGrpC)?),
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                (Self::AddGrpC(inner), i) if i == &ADD_GRP_C => inner.encode(encoder),
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
}
#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod mapem_pdu_descriptions {
    extern crate alloc;
    use super::etsi_its_cdd::ItsPduHeader;
    use super::etsi_its_dsrc::MapData;
    use core::borrow::Borrow;
    use lazy_static::lazy_static;
    use rasn::prelude::*;
    #[doc = "*"]
    #[doc = "* Map (lane topology) extended Message"]
    #[doc = "* This DF includes DEs for the MAPEM: protocolVersion, the MAPEM message type identifier `messageID`, "]
    #[doc = "* the station identifier `stationID` of the originating ITS-S and the Map data from ETSI-ITS-DSRC."]
    #[doc = "* "]
    #[doc = "* @field header:  The DE `protocolVersion` is used to select the appropriate protocol decoder at the receiving ITS-S. "]
    #[doc = "*                 It shall be set to 2."]
    #[doc = "*                 The DE `messageID` shall be mapem(5)."]
    #[doc = "* @field map:     contains the MAP data as defined in ETSI-ITS-DSRC."]
    #[doc = "* "]
    #[doc = "* @category: Basic Information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct MAPEM {
        pub header: ItsPduHeader,
        pub map: MapData,
    }
    impl MAPEM {
        pub fn new(header: ItsPduHeader, map: MapData) -> Self {
            Self { header, map }
        }
    }
}
#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod rtcmem_pdu_descriptions {
    extern crate alloc;
    use super::etsi_its_cdd::ItsPduHeader;
    use super::etsi_its_dsrc::RTCMcorrections;
    use core::borrow::Borrow;
    use lazy_static::lazy_static;
    use rasn::prelude::*;
    #[doc = "*"]
    #[doc = "* RTCM corrections extended extended Message"]
    #[doc = "* This DF includes DEs for the RTCMEM: protocolVersion, the RTCMEM message type identifier `messageID`,"]
    #[doc = "* the station identifier `stationID` of the originating ITS-S and the RTCM corrections as of ETSI-ITS-DSRC."]
    #[doc = "*"]
    #[doc = "* @field header: The DE `protocolVersion` is used to select the appropriate protocol decoder at the receiving ITS-S. "]
    #[doc = "*                It shall be set to 1."]
    #[doc = "*                The DE `messageID` shall be rtcmem(13)."]
    #[doc = "* @field rtcmc:  contains the RTCM corrections data as defined in ETSI-ITS-DSRC."]
    #[doc = "* "]
    #[doc = "* @category: Basic Information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct RTCMEM {
        pub header: ItsPduHeader,
        pub rtcmc: RTCMcorrections,
    }
    impl RTCMEM {
        pub fn new(header: ItsPduHeader, rtcmc: RTCMcorrections) -> Self {
            Self { header, rtcmc }
        }
    }
}
#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod spatem_pdu_descriptions {
    extern crate alloc;
    use super::etsi_its_cdd::ItsPduHeader;
    use super::etsi_its_dsrc::*;
    use core::borrow::Borrow;
    use lazy_static::lazy_static;
    use rasn::prelude::*;
    #[doc = "*"]
    #[doc = "* Signal phase and timing extended Message"]
    #[doc = "*"]
    #[doc = "* Signal phase and timing extended Message Root"]
    #[doc = "* This DF includes DEs for the SPATEM: protocolVersion, the SPATEM message type identifier `messageID`,"]
    #[doc = "* the station identifier `stationID` of the originating ITS-S and the SPaT data from ETSI-ITS-DSRC module."]
    #[doc = "*"]
    #[doc = "* @field header:  The DE `protocolVersion` used to select the appropriate protocol decoder at the receiving ITS-S. "]
    #[doc = "*                 It shall be set to 2."]
    #[doc = "*                 The DE `messageID` shall be spatem(4)."]
    #[doc = "* @field spat:    contains the SPaT data as defined in ETSI-ITS-DSRC."]
    #[doc = "* "]
    #[doc = "* @category: Basic Information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct SPATEM {
        pub header: ItsPduHeader,
        pub spat: SPAT,
    }
    impl SPATEM {
        pub fn new(header: ItsPduHeader, spat: SPAT) -> Self {
            Self { header, spat }
        }
    }
}
#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod srem_pdu_descriptions {
    extern crate alloc;
    use super::etsi_its_cdd::ItsPduHeader;
    use super::etsi_its_dsrc::SignalRequestMessage;
    use core::borrow::Borrow;
    use lazy_static::lazy_static;
    use rasn::prelude::*;
    #[doc = "*"]
    #[doc = "* Signal request extended Message Message"]
    #[doc = "* This DF includes DEs for the SREM: protocolVersion, the SREM message type identifier `messageID`,"]
    #[doc = "* the station identifier `stationID` of the originating ITS-S and the signal request data from ETSI-ITS-DSRC."]
    #[doc = "*"]
    #[doc = "* @field header: The DE `protocolVersion` is used to select the appropriate protocol decoder at the receiving ITS-S. "]
    #[doc = "*                It shall be set to 2."]
    #[doc = "*                The DE `messageID` shall be srem(9)."]
    #[doc = "* @field srm:    contains the Signal request data as defined in ETSI-ITS-DSRC."]
    #[doc = "* "]
    #[doc = "* @category: Basic Information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct SREM {
        pub header: ItsPduHeader,
        pub srm: SignalRequestMessage,
    }
    impl SREM {
        pub fn new(header: ItsPduHeader, srm: SignalRequestMessage) -> Self {
            Self { header, srm }
        }
    }
}
#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod ssem_pdu_descriptions {
    extern crate alloc;
    use super::etsi_its_cdd::ItsPduHeader;
    use super::etsi_its_dsrc::SignalStatusMessage;
    use core::borrow::Borrow;
    use lazy_static::lazy_static;
    use rasn::prelude::*;
    #[doc = "*"]
    #[doc = "* Signal status extended Message"]
    #[doc = "* "]
    #[doc = "* This DF includes DEs for the SSEM: protocolVersion, the SSEM message type identifier `messageID` and"]
    #[doc = "* the station identifier `stationID` of the originating ITS-S and the signal status data from ETSI-ITS-DSRC."]
    #[doc = "*"]
    #[doc = "* @field header: The DE `protocolVersion` is used to select the appropriate protocol decoder at the receiving ITS-S. "]
    #[doc = "*                It shall be set to 2."]
    #[doc = "*                The DE `messageID` shall be ssem(10)."]
    #[doc = "* @field ssm:    contains the Signal status data as defined in ETSI-ITS-DSRC."]
    #[doc = "* "]
    #[doc = "* @category: Basic Information"]
    #[doc = "* @revision: V1.3.1"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct SSEM {
        pub header: ItsPduHeader,
        pub ssm: SignalStatusMessage,
    }
    impl SSEM {
        pub fn new(header: ItsPduHeader, ssm: SignalStatusMessage) -> Self {
            Self { header, ssm }
        }
    }
}
