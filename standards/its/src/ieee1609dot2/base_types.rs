extern crate alloc;
use crate::delegate;
use crate::ts103097::extension_module::ExtId;
use bon::Builder;
use rasn::prelude::*;

/// OID for IEEE 1609.2 Base Types module
pub const IEEE1609_DOT2_BASE_TYPES_OID: &Oid = Oid::const_new(&[
    1,    // iso
    3,    // identified-organization
    111,  // ieee
    2,    // standards-association-numbered-series-standards
    1609, // wave-stds
    2,    // dot2
    1,    // base
    2,    // base-types
    2,    // major-version-2
    4,    // minor-version-4
]);

// ***************************************************************************
// **                            Integer Types                              **
// ***************************************************************************
/// This atomic type is used in the definition of other data structures.
/// It is for non-negative integers up to 7, i.e., (hex)07.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate, value("0..=7"))]
pub struct Uint3(pub u8);
/// This atomic type is used in the definition of other data structures.
/// It is for non-negative integers up to 255, i.e., (hex)ff.
pub type Uint8 = u8;
/// This atomic type is used in the definition of other data structures.
/// It is for non-negative integers up to 65,535, i.e., (hex)ff ff.
pub type Uint16 = u16;
/// This atomic type is used in the definition of other data structures.
/// It is for non-negative integers up to 4,294,967,295, i.e.,
/// (hex)ff ff ff ff.
pub type Uint32 = u32;
/// This atomic type is used in the definition of other data structures.
/// It is for non-negative integers up to 18,446,744,073,709,551,615, i.e.,
/// (hex)ff ff ff ff ff ff ff ff.
pub type Uint64 = u64;

/// This type is used for clarity of definitions.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate)]
pub struct SequenceOfUint16(pub SequenceOf<Uint16>);

delegate!(SequenceOf<Uint16>, SequenceOfUint16);

/// This type is used for clarity of definitions.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate)]
pub struct SequenceOfUint8(pub SequenceOf<Uint8>);

delegate!(SequenceOf<Uint8>, SequenceOfUint8);

// ***************************************************************************
// **                          OCTET STRING Types                           **
// ***************************************************************************

/// This is a synonym for ASN.1 OCTET STRING, and is used in the
/// definition of other data structures.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate)]
pub struct Opaque(pub OctetString);

delegate!(OctetString, Opaque);

/// A type containing the truncated hash of another data structure.
///
/// # Hash Calculation
/// The `HashedId3` is calculated by:
/// 1. Computing the hash of the encoded data structure
/// 2. Taking the low-order three bytes of the hash output
/// 3. Using the last three bytes of the 32-byte hash when represented in network byte order
/// 4. Canonicalizing the data structure before hashing if required
///
/// # Hash Algorithm Selection
/// The hash algorithm used for calculating `HashedId3` is context-dependent:
/// - Each structure including a `HashedId3` field specifies how the hash algorithm
///   is determined
/// - See discussion in section 5.3.9 for more details
///
/// # Example
/// Using SHA-256 hash of an empty string:
/// ```text
/// SHA-256("") =
/// e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
///
/// Resulting HashedId3 = 52b855
/// ```
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate)]
pub struct HashedId3(pub FixedOctetString<3usize>);

delegate!(FixedOctetString<3usize>, HashedId3);

/// This type is used for clarity of definitions.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate)]
pub struct SequenceOfHashedId3(pub SequenceOf<HashedId3>);

delegate!(SequenceOf<HashedId3>, SequenceOfHashedId3);

/// A type containing the truncated hash of another data structure.
///
/// # Hash Calculation
/// The `HashedId8` is calculated by:
/// 1. Computing the hash of the encoded data structure
/// 2. Taking the low-order eight bytes of the hash output
/// 3. Using the last eight bytes of the hash when represented in network byte order
/// 4. Canonicalizing the data structure before hashing if required
///
/// # Hash Algorithm Selection
/// The hash algorithm used for calculating `HashedId8` is context-dependent:
/// - Each structure including a `HashedId8` field specifies how the hash algorithm
///   is determined
/// - See discussion in section 5.3.9 for more details
///
/// # Example
/// Using SHA-256 hash of an empty string:
/// ```text
/// SHA-256("") =
/// e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
///
/// Resulting HashedId8 = a495991b7852b855
/// ```
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate)]
pub struct HashedId8(pub FixedOctetString<8usize>);

delegate!(FixedOctetString<8usize>, HashedId8);

/// A type containing the truncated hash of another data structure.
///
/// # Hash Calculation
/// The `HashedId10` is calculated by:
/// 1. Computing the hash of the encoded data structure
/// 2. Taking the low-order ten bytes of the hash output
/// 3. Using the last ten bytes of the hash when represented in network byte order
/// 4. Canonicalizing the data structure before hashing if required
///
/// # Hash Algorithm Selection
/// The hash algorithm used for calculating `HashedId10` is context-dependent:
/// - Each structure including a `HashedId10` field specifies how the hash algorithm
///   is determined
/// - See discussion in section 5.3.9 for more details
///
/// # Example
/// Using SHA-256 hash of an empty string:
/// ```text
/// SHA-256("") =
/// e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
///
/// Resulting HashedId10 = 934ca495991b7852b855
/// ```

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate)]
pub struct HashedId10(pub FixedOctetString<10usize>);

delegate!(FixedOctetString<10usize>, HashedId10);

/// A type containing the truncated hash of another data structure.
///
/// # Hash Calculation
/// The `HashedId32` is calculated by:
/// 1. Computing the hash of the encoded data structure
/// 2. Taking the low-order 32 bytes of the hash output
/// 3. Using the last 32 bytes of the hash when represented in network byte order
/// 4. Canonicalizing the data structure before hashing if required
///
/// # Hash Algorithm Selection
/// The hash algorithm used for calculating `HashedId32` is context-dependent:
/// - Each structure including a `HashedId32` field specifies how the hash algorithm
///   is determined
/// - See discussion in section 5.3.9 for more details
///
/// # Example
/// Using SHA-256 hash of an empty string:
/// ```text
/// SHA-256("") =
/// e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
///
/// Resulting HashedId32 =
/// e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
/// ```

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate)]
pub struct HashedId32(pub FixedOctetString<32usize>);

delegate!(FixedOctetString<32usize>, HashedId32);

/// A type containing the truncated hash of another data structure.
///
/// # Hash Calculation
/// The `HashedId48` is calculated by:
/// 1. Computing the hash of the encoded data structure
/// 2. Taking the low-order 48 bytes of the hash output
/// 3. Using the last 48 bytes of the hash when represented in network byte order
/// 4. Canonicalizing the data structure before hashing if required
///
/// # Hash Algorithm Selection
/// The hash algorithm used for calculating `HashedId48` is context-dependent:
/// - Each structure including a `HashedId48` field specifies how the hash algorithm
///   is determined
/// - See discussion in section 5.3.9 for more details
///
/// # Example
/// Using SHA-384 hash of an empty string:
/// ```text
/// SHA-384("") =
/// 38b060a751ac96384cd9327eb1b1e36a21fdb71114be07434c0cc7bf63f6e1da274edebfe76f65fbd51ad2f14898b95b
///
/// Resulting HashedId48 =
/// 38b060a751ac96384cd9327eb1b1e36a21fdb71114be07434c0cc7bf63f6e1da274edebfe76f65fbd51ad2f14898b95b
/// ```
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate)]
pub struct HashedId48(pub FixedOctetString<48usize>);

delegate!(FixedOctetString<48usize>, HashedId48);

// ***************************************************************************
// **                           Time Structures                             **
// ***************************************************************************

/// This type gives the number of (TAI) seconds since 00:00:00 UTC, 1
/// January, 2004.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate)]
pub struct Time32(pub Uint32);

delegate!(Uint32, Time32);

/// This data structure is a 64-bit integer giving an estimate of the
/// number of (TAI) microseconds since 00:00:00 UTC, 1 January, 2004.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate)]
pub struct Time64(pub Uint64);

delegate!(Uint64, Time64);

/// This type gives the validity period of a certificate.
/// The start of the validity period is given by `start` and the end is given by `start + duration`.
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(automatic_tags)]
pub struct ValidityPeriod {
    pub start: Time32,
    pub duration: Duration,
}

/// This structure represents the duration of validity of a certificate.
/// The Uint16 value is the duration, given in the units denoted by the indicated choice.
/// A year is considered to be 31,556,952 seconds, which is the average number of seconds in a year.
///
/// # Note
/// Years can be mapped more closely to wall-clock days using the `hours` choice for up to 7 years
/// and the `sixtyHours` choice for up to 448 years.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(choice, automatic_tags)]
pub enum Duration {
    Microseconds(Uint16),
    Milliseconds(Uint16),
    Seconds(Uint16),
    Minutes(Uint16),
    Hours(Uint16),
    SixtyHours(Uint16),
    Years(Uint16),
}

// ***************************************************************************
// **                           Location Structures                         **
// ***************************************************************************

/// Geographic region representation with specified forms.
///
/// A certificate is not valid if any part of the region indicated in its scope field
/// lies outside the region indicated in the scope of its issuer.
///
/// # Variants
/// - `CircularRegion`: Contains a single instance of the `CircularRegion` structure
/// - `RectangularRegion`: Array of `RectangularRegion` structures containing at least
///   one entry. Interpreted as a series of rectangles, which may overlap or be
///   disjoint. The permitted region is any point within any of the rectangles.
/// - `PolygonalRegion`: Contains a single instance of the `PolygonalRegion` structure
/// - `IdentifiedRegion`: Array of `IdentifiedRegion` structures containing at least
///   one entry. The permitted region is any point within any of the identified regions.
///
/// # Critical Information Fields
/// This is a critical information field as defined in 5.2.6:
///
/// - An implementation that does not recognize the indicated CHOICE when verifying
///   a signed SPDU shall indicate that the signed SPDU is invalid (per 4.2.2.3.2)
///
/// - For `RectangularRegion`:
///   - Implementation must support at least eight entries
///   - If number of entries is not supported, SPDU must be marked invalid
///
/// - For `IdentifiedRegion`:
///   - Implementation must support at least eight entries
///   - If number of entries is not supported, SPDU must be marked invalid
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum GeographicRegion {
    CircularRegion(CircularRegion),
    RectangularRegion(SequenceOfRectangularRegion),
    PolygonalRegion(PolygonalRegion),
    IdentifiedRegion(SequenceOfIdentifiedRegion),
}
/// A structure specifying a circle with its center at `center`, radius in meters,
/// and located tangential to the reference ellipsoid.
///
/// The indicated region includes all points on the surface of the reference
/// ellipsoid whose distance to the center point over the reference ellipsoid
/// is less than or equal to the radius.
///
/// # Note
/// A point containing an elevation component is considered to be within the
/// circular region if its horizontal projection onto the reference ellipsoid
/// lies within the region.
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(automatic_tags)]
pub struct CircularRegion {
    pub center: TwoDLocation,
    pub radius: Uint16,
}
/// Specifies a "rectangle" on the surface of the WGS84 ellipsoid where the sides
/// are given by lines of constant latitude or longitude.
///
/// # Points with Elevation
/// A point which contains an elevation component is considered to be within the
/// rectangular region if its horizontal projection onto the reference ellipsoid
/// lies within the region.
///
/// # Validity Rules
/// A RectangularRegion is invalid if:
/// - The `north_west` value is south of the `south_east` value
/// - The latitude values in the two points are equal
/// - The longitude values in the two points are equal
///
/// A certificate containing an invalid RectangularRegion is considered invalid.
///
/// # Fields
/// - `north_west`: The north-west corner of the rectangle
/// - `south_east`: The south-east corner of the rectangle
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(automatic_tags)]
pub struct RectangularRegion {
    #[rasn(identifier = "northWest")]
    pub north_west: TwoDLocation,
    #[rasn(identifier = "southEast")]
    pub south_east: TwoDLocation,
}

/// This type is used for clarity of definitions.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate)]
pub struct SequenceOfRectangularRegion(pub SequenceOf<RectangularRegion>);

delegate!(SequenceOf<RectangularRegion>, SequenceOfRectangularRegion);

/// Defines a region using a series of distinct geographic points on the surface of
/// the reference ellipsoid.
///
/// The region is specified by:
/// - Connecting points in their order of appearance via geodesics on the reference ellipsoid
/// - Completing the polygon by connecting the final point to the first point
/// - The allowed region is the interior of the polygon and its boundary
///
/// # Points with Elevation
/// A point containing an elevation component is considered within the polygonal
/// region if its horizontal projection onto the reference ellipsoid lies within
/// the region.
///
/// # Validity Rules
/// A valid `PolygonalRegion`:
/// - Must contain at least three points
/// - Must not have intersecting sides (implied lines making up the polygon)
///
/// # Limitations
/// Does not support enclaves/exclaves (may be addressed in future versions of
/// the standard).
///
/// # Critical Information Fields
/// This is a critical information field as defined in 5.2.6:
/// - Implementation must support at least eight `TwoDLocation` entries
/// - If the number of `TwoDLocation` entries is not supported when verifying
///   a signed SPDU, the implementation must indicate that the SPDU is invalid
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate, size("3.."))]
pub struct PolygonalRegion(pub SequenceOf<TwoDLocation>);

delegate!(SequenceOf<TwoDLocation>, PolygonalRegion);

/// Defines a region using a series of distinct geographic points on the surface of
/// the reference ellipsoid.
///
/// The region is specified by:
/// - Connecting points in their order of appearance via geodesics on the reference ellipsoid
/// - Completing the polygon by connecting the final point to the first point
/// - The allowed region is the interior of the polygon and its boundary
///
/// # Points with Elevation
/// A point containing an elevation component is considered within the polygonal
/// region if its horizontal projection onto the reference ellipsoid lies within
/// the region.
///
/// # Validity Rules
/// A valid `PolygonalRegion`:
/// - Must contain at least three points
/// - Must not have intersecting sides (implied lines making up the polygon)
///
/// # Limitations
/// Does not support enclaves/exclaves (may be addressed in future versions of
/// the standard).
///
/// # Critical Information Fields
/// This is a critical information field as defined in 5.2.6:
/// - Implementation must support at least eight `TwoDLocation` entries
/// - If the number of `TwoDLocation` entries is not supported when verifying
///   a signed SPDU, the implementation must indicate that the SPDU is invalid
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(automatic_tags)]
pub struct TwoDLocation {
    pub latitude: Latitude,
    pub longitude: Longitude,
}
/// Indicates the region of validity of a certificate using region identifiers.
///
/// A conformant implementation must support at least one of the possible CHOICE values.
/// The Protocol Implementation Conformance Statement (PICS) in Annex A allows an
/// implementation to state which `CountryOnly` values it recognizes.
///
/// # Variants
/// - `CountryOnly`: Indicates only a country (or a geographic entity included in
///   a country list) is given
/// - `CountryAndRegions`: Indicates one or more top-level regions within a country
///   (as defined by the region listing associated with that country) is given
/// - `CountryAndSubregions`: Indicates one or more regions smaller than the
///   top-level regions within a country (as defined by the region listing
///   associated with that country) is given
///
/// # Critical Information Fields
/// This is a critical information field as defined in 5.2.6:
/// - An implementation that does not recognize the indicated CHOICE when verifying
///   a signed SPDU shall indicate that the SPDU is invalid (per 4.2.2.3.2)
/// - Invalid in this context means its validity cannot be established
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum IdentifiedRegion {
    CountryOnly(UnCountryId),
    CountryAndRegions(CountryAndRegions),
    CountryAndSubregions(CountryAndSubregions),
}

/// This type is used for clarity of definitions.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate)]
pub struct SequenceOfIdentifiedRegion(pub SequenceOf<IdentifiedRegion>);

delegate!(SequenceOf<IdentifiedRegion>, SequenceOfIdentifiedRegion);

/// Integer representation of country or area identifiers as defined by the United
/// Nations Statistics Division (October 2013, see normative references in Clause 0).
///
/// # Implementation Requirements
/// A conformant implementation of `IdentifiedRegion` must:
/// - Recognize at least one value of `UnCountryId` (ability to determine if a
///   2D location lies inside/outside the identified borders)
/// - May declare recognized `UnCountryId` values in the Protocol Implementation
///   Conformance Statement (PICS) in Annex A
///
/// # Historical Changes
/// Since 2013 and before this standard's publication, three changes to the country
/// code list:
/// - Added "sub-Saharan Africa" region
/// - Removed "developed regions"
/// - Removed "developing regions"
///
/// Conformant implementations may recognize these region identifiers.
///
/// # Verification Behavior
/// When verifying geographic information in a signed SPDU against a certificate:
/// - SDS may indicate SPDU validity even with unrecognized instances if recognized
///   instances completely contain the relevant geographic information
/// - Not considered a "critical information field" (ref: 5.2.6) as unrecognized
///   values are permitted if SPDU validity can be established with recognized values
///
/// # Important Note
/// An unrecognized value in a certificate may still prevent determining the
/// validity of both the certificate and SPDU.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate)]
pub struct UnCountryId(pub Uint16);

delegate!(Uint16, UnCountryId);

/// This type is defined only for backwards compatibility.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate)]
pub struct CountryOnly(pub UnCountryId);

delegate!(UnCountryId, CountryOnly);

/// A type representing country and region information with specific implementation requirements.
///
/// # Implementation Requirements
/// - Must support a `regions` field containing at least eight entries
/// - Must be able to determine whether a two-dimensional location lies inside or
///   outside the borders identified by at least:
///   - One value of `UnCountryId`
///   - One region within the country indicated by that recognized `UnCountryId` value
///
/// # Current Version Requirements
/// The only way to satisfy the implementation requirements in this version is to:
/// - Recognize the `UnCountryId` value indicating USA
/// - Recognize at least one of the FIPS state codes for US states
///
/// # Verification Behavior
/// When verifying geographic information in a signed SPDU against a certificate:
/// - The SDS may indicate validity even with unrecognized country/region values
/// - Validity can be determined if recognized values completely contain the relevant
///   geographic information
/// - This is not a "critical information field" (ref: 5.2.6) as unrecognized values
///   are permitted if validity can be established with recognized values
/// - Note: Unrecognized values in a certificate may still prevent determining
///   certificate validity and consequently SPDU validity
///
/// # Fields
/// - `countryOnly`: A `UnCountryId` value identifying the country
/// - `regions`: One or more regions within the country:
///   - For USA: Uses integer version of 2010 FIPS codes from U.S. Census Bureau
///   - For other countries: Region meaning is undefined in current version
///
/// # PICS Conformance
/// The Protocol Implementation Conformance Statement (PICS) in Annex A allows
/// implementations to declare:
/// - Recognized `UnCountryId` values
/// - Recognized region values within each country
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(automatic_tags)]
pub struct CountryAndRegions {
    #[rasn(identifier = "countryOnly")]
    pub country_only: UnCountryId,
    pub regions: SequenceOfUint8,
}

/// Implementation requirements and behavior specification for country and subregion handling.
///
/// # Implementation Requirements
/// A conformant implementation:
/// - Must support at least eight entries in the `region_and_subregions` field
/// - Must recognize at least one country value and one region within that country
/// - Currently must specifically recognize:
///   - USA as a `UnCountryId` value
///   - At least one FIPS state code for US states
///
/// The Protocol Implementation Conformance Statement (PICS) in Annex A allows
/// implementations to declare:
/// - Recognized `UnCountryId` values
/// - Recognized region values within each country
///
/// # Verification Behavior
/// When verifying geographic information in a signed SPDU against a certificate:
/// - SDS may indicate SPDU validity even with unrecognized country or
///   `region_and_subregions` values if recognized instances completely contain
///   the relevant geographic information
/// - Not considered a "critical information field" (ref: 5.2.6) as unrecognized
///   values are permitted if SPDU validity can be established with recognized values
///
/// # Important Note
/// An unrecognized value in a certificate may prevent determining the validity
/// of both the certificate and SPDU.
///
/// # Fields
/// - `country`: A `UnCountryId` value identifying the country
/// - `region_and_subregions`: Identifies one or more subregions within the country
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(automatic_tags)]
pub struct CountryAndSubregions {
    #[rasn(identifier = "countryOnly")]
    pub country_only: UnCountryId,
    #[rasn(identifier = "regionAndSubregions")]
    pub region_and_subregions: SequenceOfRegionAndSubregions,
}

/// Represents regions and subregions within an "enclosing country" context.
///
/// # Context
/// Fields are interpreted within the context of an enclosing country:
/// - In `CountryAndSubregions`, the enclosing country is specified by the `country` field
/// - Future uses will specify how the enclosing country is determined
///
/// # USA-Specific Implementation
/// When the enclosing country is the United States of America:
/// - `region`: Identifies state/equivalent entity using 2010 FIPS codes (U.S. Census Bureau)
/// - `subregions`: Identifies county/equivalent entity using 2010 FIPS codes
///
/// For other countries, the meaning is not defined in this version of the standard.
///
/// # Implementation Requirements
/// A conformant implementation must:
/// - Recognize at least one region within an enclosing country
/// - Recognize at least one subregion for the indicated region
/// - For USA specifically:
///   - Recognize at least one FIPS state code
///   - Recognize at least one county code in at least one recognized state
/// - Support at least eight entries in the `subregions` field
///
/// The PICS (Annex A) allows implementations to declare recognized:
/// - `UnCountryId` values
/// - Region values within countries
///
/// # Verification Behavior
/// When verifying geographic information in a signed SPDU against a certificate:
/// - SDS may indicate SPDU validity even with unrecognized subregion values if
///   recognized instances completely contain the relevant geographic information
/// - Not considered a "critical information field" (ref: 5.2.6) as unrecognized
///   values are permitted if SPDU validity can be established with recognized values
///
/// # Important Note
/// An unrecognized value in a certificate may prevent determining the validity
/// of both the certificate and SPDU.
///
/// # Fields
/// - `region`: Identifies a region within a country
/// - `subregions`: Identifies one or more subregions within the region
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(automatic_tags)]
pub struct RegionAndSubregions {
    pub region: Uint8,
    pub subregions: SequenceOfUint16,
}

/// This type is used for clarity of definitions.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate)]
pub struct SequenceOfRegionAndSubregions(pub SequenceOf<RegionAndSubregions>);

delegate!(
    SequenceOf<RegionAndSubregions>,
    SequenceOfRegionAndSubregions
);

/// A structure containing an estimate of 3D location, with field-specific details
/// provided in individual field documentation.
///
/// # Compatibility Note
/// The units used in this structure are consistent with SAE J2735 B26 location
/// data structures, though the encoding is incompatible.
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(automatic_tags)]
pub struct ThreeDLocation {
    pub latitude: Latitude,
    pub longitude: Longitude,
    pub elevation: Elevation,
}

/// An INTEGER encoding of latitude estimate with 1/10th microdegree precision,
/// relative to the World Geodetic System (WGS-84) datum as defined in NIMA
/// Technical Report TR8350.2.
///
/// # Value Range
/// - Minimum: -900,000,000
/// - Maximum: 900,000,000
/// - Special Value: 900,000,001 indicates latitude was not available to sender
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate)]
pub struct Latitude(pub NinetyDegreeInt);

delegate!(NinetyDegreeInt, Latitude);

/// An INTEGER encoding of longitude estimate with 1/10th microdegree precision,
/// relative to the World Geodetic System (WGS-84) datum as defined in NIMA
/// Technical Report TR8350.2.
///
/// # Value Range
/// - Minimum: -1,799,999,999
/// - Maximum: 1,800,000,000
/// - Special Value: 1,800,000,001 indicates longitude was not available to sender
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate)]
pub struct Longitude(pub OneEightyDegreeInt);

delegate!(OneEightyDegreeInt, Longitude);

/// This structure contains an estimate of the geodetic altitude above
/// or below the WGS84 ellipsoid. The 16-bit value is interpreted as an
/// integer number of decimeters representing the height above a minimum
/// height of -409.5 m, with the maximum height being 6143.9 m.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate)]
pub struct Elevation(pub Uint16);

delegate!(Uint16, Elevation);

/// The integer in the latitude field is no more than 900,000,000 and
/// no less than -900,000,000, except that the value 900,000,001 is used to
/// indicate the latitude was not available to the sender.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate, value("-900000000..=900000001"))]
pub struct NinetyDegreeInt(i32);

impl NinetyDegreeInt {
    pub const MIN: i32 = -900_000_000;
    pub const MAX: i32 = 900_000_000;
    pub const UNKNOWN: i32 = 900_000_001;
    pub const fn new(value: i32) -> Option<Self> {
        if value >= Self::MIN && value <= Self::UNKNOWN {
            Some(Self(value))
        } else {
            None
        }
    }
}

delegate!(i32, NinetyDegreeInt);

/// The known latitudes are from -900,000,000 to +900,000,000 in 0.1
/// microdegree intervals.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate, value("-900000000..=900000000"))]
pub struct KnownLatitude(NinetyDegreeInt);

impl KnownLatitude {
    pub const MIN: i32 = -900_000_000;
    pub const MAX: i32 = 900_000_000;
    pub const fn new(value: i32) -> Option<Self> {
        if value >= Self::MIN && value <= Self::MAX {
            Some(Self(NinetyDegreeInt(value)))
        } else {
            None
        }
    }
}

delegate!(NinetyDegreeInt, KnownLatitude);

/// The value 900,000,001 indicates that the latitude was not
/// available to the sender.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate, value("900000001"))]
pub struct UnknownLatitude(NinetyDegreeInt);

impl UnknownLatitude {
    pub const UNKNOWN: NinetyDegreeInt = NinetyDegreeInt(900_000_001);
    pub const fn new() -> Self {
        Self(Self::UNKNOWN)
    }
}

impl Default for UnknownLatitude {
    fn default() -> Self {
        Self::new()
    }
}

delegate!(NinetyDegreeInt, UnknownLatitude);

/// An integer type representing longitude values with named boundary values
/// and constraints.
///
/// # Value Range
/// - Minimum: -1,799,999,999
/// - Maximum: 1,800,000,000
/// - Special Value: 1,800,000,001 (indicates longitude not available to sender)
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate, value("-1799999999..=1800000001"))]
pub struct OneEightyDegreeInt(pub i32);

impl OneEightyDegreeInt {
    pub const MIN: i32 = -1_799_999_999;
    pub const MAX: i32 = 1_800_000_000;
    pub const UNKNOWN: i32 = 1_800_000_001;
    /// Creates a new OneEightyDegreeInt if the value is within valid range
    pub const fn new(value: i32) -> Option<Self> {
        if Self::is_valid(value) {
            Some(Self(value))
        } else {
            None
        }
    }
    /// Creates a new OneEightyDegreeInt with an unknown value
    pub const fn unknown() -> Self {
        Self(Self::UNKNOWN)
    }
    /// Returns true if this is a valid longitude value (including unknown)
    pub const fn is_valid(value: i32) -> bool {
        (value >= Self::MIN && value <= Self::MAX) || value == Self::UNKNOWN
    }
    /// Returns true if this represents a known longitude value
    pub const fn is_known(&self) -> bool {
        self.0 >= OneEightyDegreeInt::MIN && self.0 <= OneEightyDegreeInt::MAX
    }
    /// Returns true if this represents an unknown longitude value
    pub const fn is_unknown(&self) -> bool {
        self.0 == Self::UNKNOWN
    }
}

delegate!(i32, OneEightyDegreeInt);

/// Represents a known longitude value in 0.1 microdegree intervals.
///
/// # Value Range
/// - Minimum: -1,799,999,999 (-180 degrees)
/// - Maximum: 1,800,000,000 (180 degrees)
/// - Precision: 0.1 microdegrees
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate, value("-1799999999..=1800000000"))]
pub struct KnownLongitude(OneEightyDegreeInt);

impl KnownLongitude {
    pub const MIN: i32 = -1_799_999_999;
    pub const MAX: i32 = 1_800_000_000;
    /// Creates a new KnownLongitude if the value is within valid range
    pub const fn new(value: i32) -> Option<Self> {
        if value >= Self::MIN && value <= Self::MAX {
            Some(Self(OneEightyDegreeInt(value)))
        } else {
            None
        }
    }
}

delegate!(OneEightyDegreeInt, KnownLongitude);

/// Represents a longitude value that is explicitly unknown/unavailable.
///
/// This type can only have the value 1,800,000,001, indicating that the longitude
/// was not available to the sender.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate, value("1800000001"))]
pub struct UnknownLongitude(OneEightyDegreeInt);

impl UnknownLongitude {
    pub const UNKNOWN: OneEightyDegreeInt = OneEightyDegreeInt(1_800_000_001);
    /// Creates a new UnknownLongitude instance
    pub const fn new() -> Self {
        Self(Self::UNKNOWN)
    }
}
impl Default for UnknownLongitude {
    fn default() -> Self {
        Self::new()
    }
}

delegate!(OneEightyDegreeInt, UnknownLongitude);

// ***************************************************************************
// **                           Crypto Structures                           **
// ***************************************************************************

/// Represents a signature for a supported public key algorithm, which may be
/// contained within `SignedData` or `Certificate`.
///
/// # Critical Information Field
/// This is a critical information field as defined in 5.2.5:
/// - An implementation that does not recognize the indicated CHOICE when verifying
///   a signed SPDU shall indicate that the SPDU is invalid (per 4.2.2.3.2)
/// - Invalid in this context means its validity cannot be established
///
/// # Canonicalization
/// This data structure is subject to canonicalization for operations specified
/// in 6.1.2. Canonicalization applies to:
/// - `EcdsaP256Signature` instances
/// - `EcdsaP384Signature` instances
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum Signature {
    EcdsaNistP256(EcdsaP256Signature),
    EcdsaBrainpoolP256r1(EcdsaP256Signature),
    #[rasn(extension_addition)]
    EcdsaBrainpoolP384r1(EcdsaP384Signature),
    #[rasn(extension_addition)]
    EcdsaNistP384(EcdsaP384Signature),
    #[rasn(extension_addition)]
    Sm2(EcsigP256Signature),
}

/// Represents an ECDSA signature, generated as specified in 5.3.1.
///
/// # Signature Process
/// - FIPS 186-4: If followed, integer r is represented as an `EccP256CurvePoint`
///   with `x-only` selection
/// - SEC 1: If followed, elliptic curve point R is represented as an `EccP256CurvePoint`
///   with sender's choice of:
///   - `compressed-y-0`
///   - `compressed-y-1`
///   - `uncompressed`
///
/// # Canonicalization
/// This data structure is subject to canonicalization for operations specified in 6.1.2:
/// - When canonicalized, the `EccP256CurvePoint` in `r_sig` must use `x-only` form
///
/// # Technical Details
/// For signatures with:
/// - `x-only` form: x-value in `r_sig` is an integer mod n (group order)
/// - `compressed-y-*` form: x-value in `r_sig` is an integer mod p (field prime)
///
/// Converting `compressed-y-*` to `x-only`: theoretically requires checking if x-value
/// is between n and p, reducing mod n if so. In practice, this check is unnecessary
/// due to Haase's Theorem (probability ≈ 2^(-128) for 256-bit curves).
///
/// # Curve Parameters (hexadecimal)
/// NIST p256:
/// - p = FFFFFFFF00000001000000000000000000000000FFFFFFFFFFFFFFFFFFFFFFFF
/// - n = FFFFFFFF00000000FFFFFFFFFFFFFFFFBCE6FAADA7179E84F3B9CAC2FC632551
///
/// Brainpool p256:
/// - p = A9FB57DBA1EEA9BC3E660A909D838D726E3BF623D52620282013481D1F6E5377
/// - n = A9FB57DBA1EEA9BC3E660A909D838D718C397AA3B561A6F7901E0E82974856A7
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(automatic_tags)]
pub struct EcdsaP256Signature {
    #[rasn(identifier = "rSig")]
    pub r_sig: EccP256CurvePoint,
    #[rasn(identifier = "sSig")]
    pub s_sig: FixedOctetString<32>,
}

/// Represents an ECDSA signature, generated as specified in 5.3.1.
///
/// # Signature Process
/// - FIPS 186-4: If followed, integer r is represented as an `EccP384CurvePoint`
///   with `x-only` selection
/// - SEC 1: If followed, elliptic curve point R is represented as an `EccP384CurvePoint`
///   with sender's choice of:
///   - `compressed-y-0`
///   - `compressed-y-1`
///   - `uncompressed`
///
/// # Canonicalization
/// This data structure is subject to canonicalization for operations specified in 6.1.2:
/// - When canonicalized, the `EccP384CurvePoint` in `r_sig` must use `x-only` form
///
/// # Technical Details
/// For signatures with:
/// - `x-only` form: x-value in `r_sig` is an integer mod n (group order)
/// - `compressed-y-*` form: x-value in `r_sig` is an integer mod p (field prime)
///
/// Converting `compressed-y-*` to `x-only`: theoretically requires checking if x-value
/// is between n and p, reducing mod n if so. In practice, this check is unnecessary
/// due to Haase's Theorem (probability ≈ 2^(-192) for 384-bit curves).
///
/// # Curve Parameters (hexadecimal)
/// ```text
/// p = 8CB91E82A3386D280F5D6F7E50E641DF152F7109ED5456B412B1DA197FB71123\
///     ACD3A729901D1A71874700133107EC53
/// n = 8CB91E82A3386D280F5D6F7E50E641DF152F7109ED5456B31F166E6CAC0425A7\
///     CF3AB6AF6B7FC3103B883202E9046565
/// ```
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(automatic_tags)]
pub struct EcdsaP384Signature {
    #[rasn(identifier = "rSig")]
    pub r_sig: EccP384CurvePoint,
    #[rasn(identifier = "sSig")]
    pub s_sig: FixedOctetString<48>,
}
/// Represents an elliptic curve signature where the component r is constrained
/// to be an integer. This structure supports SM2 signatures as specified in 5.3.1.3.
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(automatic_tags)]
pub struct EcsigP256Signature {
    #[rasn(identifier = "rSig")]
    pub r_sig: FixedOctetString<32>,
    #[rasn(identifier = "sSig")]
    pub s_sig: FixedOctetString<32>,
}
/// Specifies a point on an elliptic curve in Weierstrass form defined over a
/// 256-bit prime number.
///
/// # Supported Curves
/// - NIST p256 (FIPS 186-4)
/// - Brainpool p256r1 (RFC 5639)
/// - SM2 curve (GB/T 32918.5-2017)
///
/// # Encoding Details
/// Fields are OCTET STRINGS encoded according to IEEE Std 1363-2000, 5.5.6:
/// - x-coordinate: Always 32 octets, unsigned integer in network byte order
/// - y-coordinate: Encoding depends on point representation:
///   - `XOnly`: y is omitted
///   - `CompressedY0`: y's least significant bit is 0
///   - `CompressedY1`: y's least significant bit is 1
///   - `Uncompressed`: y is explicit 32 octets, unsigned integer in network byte order
///
/// # Canonicalization
/// Subject to canonicalization for operations specified in 6.1.2 when appearing in:
/// - `HeaderInfo`
/// - `ToBeSignedCertificate`
///
/// See respective type definitions for specific canonicalization operations.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(choice, automatic_tags)]
pub enum EccP256CurvePoint {
    #[rasn(identifier = "x-only")]
    XOnly(FixedOctetString<32>),
    Fill(()),
    #[rasn(identifier = "compressed-y-0")]
    CompressedY0(FixedOctetString<32>),
    #[rasn(identifier = "compressed-y-1")]
    CompressedY1(FixedOctetString<32>),
    #[rasn(identifier = "uncompressedP256")]
    Uncompressed(EccP256CurvePointUncompressedP256),
}
/// Inner type of `EccP256CurvePoint` representing an uncompressed point on the NIST P-256 curve.
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(automatic_tags)]
pub struct EccP256CurvePointUncompressedP256 {
    pub x: FixedOctetString<32>,
    pub y: FixedOctetString<32>,
}
/// Specifies a point on an elliptic curve in Weierstrass form defined over a
/// 384-bit prime number.
///
/// # Supported Curves
/// Only supports Brainpool p384r1 as defined in RFC 5639.
///
/// # Encoding Details
/// Fields are OCTET STRINGS encoded according to IEEE Std 1363-2000, 5.5.6:
/// - x-coordinate: Always 48 octets, unsigned integer in network byte order
/// - y-coordinate: Encoding depends on point representation:
///   - `XOnly`: y is omitted
///   - `CompressedY0`: y's least significant bit is 0
///   - `CompressedY1`: y's least significant bit is 1
///   - `Uncompressed`: y is explicit 48 octets, unsigned integer in network byte order
///
/// # Canonicalization
/// Subject to canonicalization for operations specified in 6.1.2 when appearing in:
/// - `HeaderInfo`
/// - `ToBeSignedCertificate`
///
/// See respective type definitions for specific canonicalization operations.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(choice, automatic_tags)]
pub enum EccP384CurvePoint {
    #[rasn(identifier = "x-only")]
    XOnly(FixedOctetString<48>),
    Fill(()),
    #[rasn(identifier = "compressed-y-0")]
    CompressedY0(FixedOctetString<48>),
    #[rasn(identifier = "compressed-y-1")]
    CompressedY1(FixedOctetString<48>),
    #[rasn(identifier = "uncompressedP384")]
    Uncompressed(EccP384CurvePointUncompressedP384),
}
/// Inner type of `EccP384CurvePoint` representing an uncompressed point on the Brainpool P-384 curve.
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(automatic_tags)]
pub struct EccP384CurvePointUncompressedP384 {
    pub x: FixedOctetString<48>,
    pub y: FixedOctetString<48>,
}
/// Indicates supported symmetric algorithms and their modes of operation.
///
/// # Supported Algorithms
/// - AES-128
/// - SM4
///
/// # Mode of Operation
/// Only supports Counter Mode Encryption With Cipher Block Chaining Message
/// Authentication Code (CCM).
///
/// Full implementation details are specified in section 5.3.8.
#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum SymmAlgorithm {
    Aes128Ccm = 0,
    #[rasn(extension_addition)]
    Sm4Ccm = 1,
}

/// Identifies supported hash algorithms. See section 5.3.3 for implementation details.
///
/// # Supported Algorithms
/// - SHA-256
/// - SHA-384
/// - SM3
///
/// # Critical Information Field
/// This is a critical information field as defined in 5.2.6:
/// - An implementation that does not recognize the enumerated value when verifying
///   a signed SPDU shall indicate that the SPDU is invalid (per 4.2.2.3.2)
/// - Invalid in this context means its validity cannot be established
#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum HashAlgorithm {
    Sha256 = 0,
    #[rasn(extension_addition)]
    Sha384 = 1,
    #[rasn(extension_addition)]
    Sm3 = 2,
}
/// Used to transfer a 16-byte symmetric key encrypted using ECIES as specified in
/// IEEE Std 1363a-2004.
///
/// The symmetric key is input to the key encryption process with no headers,
/// encapsulation, or length indication. Encryption and decryption are carried
/// out as specified in 5.3.5.1.
///
/// # Fields
/// - `v`: Sender's ephemeral public key (output V from encryption as specified
///   in 5.3.5.1)
/// - `c`: Encrypted symmetric key (output C from encryption as specified in 5.3.5.1).
///   The algorithm is identified by the CHOICE in the following `SymmetricCiphertext`.
///   For ECIES, this algorithm must be AES-128.
/// - `t`: Authentication tag (output tag from encryption as specified in 5.3.5.1)
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(automatic_tags)]
pub struct EciesP256EncryptedKey {
    pub v: EccP256CurvePoint,
    pub c: FixedOctetString<16>,
    pub t: FixedOctetString<16>,
}

/// Used to transfer a 16-byte symmetric key encrypted using SM2 encryption as
/// specified in 5.3.3.
///
/// The symmetric key is input to the key encryption process with no headers,
/// encapsulation, or length indication. Encryption and decryption are carried
/// out as specified in 5.3.5.2.
///
/// # Fields
/// - `v`: Sender's ephemeral public key (output V from encryption as specified
///   in 5.3.5.2)
/// - `c`: Encrypted symmetric key (output C from encryption as specified in 5.3.5.2).
///   The algorithm is identified by the CHOICE in the following `SymmetricCiphertext`.
///   For SM2, this algorithm must be SM4.
/// - `t`: Authentication tag (output tag from encryption as specified in 5.3.5.2)
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(automatic_tags)]
pub struct EcencP256EncryptedKey {
    pub v: EccP256CurvePoint,
    pub c: FixedOctetString<16>,
    pub t: FixedOctetString<32>,
}
/// Contains an encryption key, which may be either public or symmetric.
///
/// # Canonicalization
/// Subject to canonicalization for operations specified in 6.1.2 when appearing in:
/// - `HeaderInfo`
/// - `ToBeSignedCertificate`
///
/// Canonicalization applies to the `PublicEncryptionKey`. See respective type
/// definitions for specific canonicalization operations.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(choice, automatic_tags)]
pub enum EncryptionKey {
    Public(PublicEncryptionKey),
    Symmetric(SymmetricEncryptionKey),
}

/// Specifies a public encryption key and its associated symmetric algorithm used
/// for bulk data encryption when encrypting for that public key.
///
/// # Canonicalization
/// Subject to canonicalization for operations specified in 6.1.2 when appearing in:
/// - `HeaderInfo`
/// - `ToBeSignedCertificate`
///
/// Canonicalization applies to the `BasePublicEncryptionKey`. See respective type
/// definitions for specific canonicalization operations.
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(automatic_tags)]
pub struct PublicEncryptionKey {
    #[rasn(identifier = "supportedSymmAlg")]
    pub supported_symm_alg: SymmAlgorithm,
    #[rasn(identifier = "publicKey")]
    pub public_key: BasePublicEncryptionKey,
}

/// This structure specifies the bytes of a public encryption key for
/// a particular algorithm. Supported public key encryption algorithms are
/// defined in 5.3.5.
///
/// # Note
/// Canonicalization: This data structure is subject to canonicalization
/// for the relevant operations specified in 6.1.2 if it appears in a
/// HeaderInfo or in a ToBeSignedCertificate. See the definitions of HeaderInfo
/// and ToBeSignedCertificate for a specification of the canonicalization
/// operations.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum BasePublicEncryptionKey {
    EciesNistP256(EccP256CurvePoint),
    EciesBrainpoolP256r1(EccP256CurvePoint),
    #[rasn(extension_addition)]
    EcencSm2(EccP256CurvePoint),
}

/// Represents a public key and its associated verification algorithm.
/// Cryptographic mechanisms are defined in section 5.3.
///
/// # Validity Rules
/// An `EccP256CurvePoint` or `EccP384CurvePoint` within this structure is invalid
/// if it indicates the choice `x-only`.
///
/// # Critical Information Field
/// This is a critical information field as defined in 5.2.6:
/// - An implementation that does not recognize the indicated CHOICE when verifying
///   a signed SPDU shall indicate that the SPDU is invalid (per 4.2.2.3.2)
/// - Invalid in this context means its validity cannot be established
///
/// # Canonicalization
/// Subject to canonicalization for operations specified in 6.1.2:
/// - Applies to both `EccP256CurvePoint` and `EccP384CurvePoint`
/// - Points must be encoded in compressed form (`compressed-y-0` or `compressed-y-1`)
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum PublicVerificationKey {
    EcdsaNistP256(EccP256CurvePoint),
    EcdsaBrainpoolP256r1(EccP256CurvePoint),
    #[rasn(extension_addition)]
    EcdsaBrainpoolP384r1(EccP384CurvePoint),
    #[rasn(extension_addition)]
    EcdsaNistP384(EccP384CurvePoint),
    #[rasn(extension_addition)]
    EcsigSm2(EccP256CurvePoint),
}

/// Provides key bytes for use with an identified symmetric algorithm.
///
/// # Supported Algorithms
/// - AES-128 in CCM mode
/// - SM4 in CCM mode
///
/// See section 5.3.8 for implementation details.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum SymmetricEncryptionKey {
    Aes128Ccm(FixedOctetString<16>),
    #[rasn(extension_addition)]
    Sm4Ccm(FixedOctetString<16>),
}

// ***************************************************************************
// **                               PSID / ITS-AID                          **
// ***************************************************************************

/// Represents permissions for a certificate holder regarding activities in a single
/// application area, identified by a `Psid`.
///
/// # Permission Determination
/// - The SDEE (not SDS) determines if activities are consistent with PSID and
///   ServiceSpecificPermissions
/// - SDS provides PSID and SSP information to SDEE for determination
/// - See section 5.2.4.3.3 for details
///
/// # SDEE Specification Requirements
/// The SDEE specification must:
/// - Specify permitted activities for particular `ServiceSpecificPermissions` values
/// - Either:
///   - Specify permitted activities when `ServiceSpecificPermissions` is omitted, or
///   - State that `ServiceSpecificPermissions` must always be present
///
/// # Consistency Rules
/// ## With Signed SPDU
/// Consistency between SSP and signed SPDU is defined by PSID-specific rules
/// (out of scope for this standard, see 5.1.1)
///
/// ## With Issuing Certificate
/// When `ssp` field is omitted, entry A is consistent with issuing certificate if
/// the certificate contains a `PsidSspRange` P where:
/// - P's `psid` field equals A's `psid` field and either:
///   - P's `sspRange` field indicates "all"
///   - P's `sspRange` field indicates "opaque" and contains an empty OCTET STRING
///
/// See following subclauses for consistency rules with other `ssp` field forms.
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(automatic_tags)]
pub struct PsidSsp {
    pub psid: Psid,
    pub ssp: Option<ServiceSpecificPermissions>,
}

/// This type is used for clarity of definitions.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate)]
pub struct SequenceOfPsidSsp(pub SequenceOf<PsidSsp>);

delegate!(SequenceOf<PsidSsp>, SequenceOfPsidSsp);

/// This type represents the PSID defined in IEEE Std 1609.2.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate, value("0.."))]
pub struct Psid(pub Integer);

delegate!(Integer, Psid);

/// This type is used for clarity of definitions.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate)]
pub struct SequenceOfPsid(pub SequenceOf<Psid>);

delegate!(SequenceOf<Psid>, SequenceOfPsid);

/// Represents Service Specific Permissions (SSP) relevant to a given entry in
/// a `PsidSsp`. The meaning of the SSP is specific to the associated `Psid`.
///
/// SSPs may be either:
/// - PSID-specific octet strings
/// - Bitmap-based
///
/// See Annex C for guidance on choosing SSP forms for application specifiers.
///
/// # Consistency with Issuing Certificate
/// For an `appPermissions` entry A with `opaque` SSP field, A is consistent with
/// the issuing certificate if it contains one of:
///
/// ## Option 1
/// A `SubjectPermissions` field indicating "all" and no `PsidSspRange` field
/// containing A's `psid` field
///
/// ## Option 2
/// A `PsidSspRange` P where:
/// - P's `psid` field equals A's `psid` field and either:
///   - P's `sspRange` field indicates "all"
///   - P's `sspRange` field indicates "opaque" and contains an OCTET STRING
///     identical to A's opaque field
///
/// See following subclauses for consistency rules with other
/// `ServiceSpecificPermissions` types.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum ServiceSpecificPermissions {
    Opaque(OctetString),
    #[rasn(extension_addition)]
    BitmapSsp(BitmapSsp),
}

/// This structure represents a bitmap representation of a SSP. The
/// mapping of the bits of the bitmap to constraints on the signed SPDU is
/// PSID-specific.
///
/// Consistency with issuing certificate: If a certificate has an
/// appPermissions entry A for which the ssp field is bitmapSsp, A is
/// consistent with the issuing certificate if the certificate contains one
/// of the following:
///   - (OPTION 1) A SubjectPermissions field indicating the choice all and no PsidSspRange field containing the psid field in A;
///   - (OPTION 2) A PsidSspRange P for which the following holds:
///     - The psid field in P is equal to the psid field in A and one of the following is true:
///       - EITHER The sspRange field in P indicates all
///       - OR The sspRange field in P indicates bitmapSspRange and for every bit set to 1 in the sspBitmask in P, the bit in the identical position in the sspValue in A is set equal to the bit in that position in the sspValue in P.
///
/// # Note
/// A BitmapSsp B is consistent with a BitmapSspRange R if for every
/// bit set to 1 in the sspBitmask in R, the bit in the identical position in
/// B is set equal to the bit in that position in the sspValue in R. For each
/// bit set to 0 in the sspBitmask in R, the corresponding bit in the
/// identical position in B may be freely set to 0 or 1, i.e., if a bit is
/// set to 0 in the sspBitmask in R, the value of corresponding bit in the
/// identical position in B has no bearing on whether B and R are consistent.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate, size("0..=31"))]
pub struct BitmapSsp(pub OctetString);

delegate!(OctetString, BitmapSsp);

/// Represents the certificate issuing or requesting permissions of the certificate
/// holder for a particular set of application permissions.
///
/// # Fields
/// - `psid`: Identifies the application area
/// - `ssp_range`: Identifies the SSPs associated with the PSID for which the holder
///   may issue or request certificates. If omitted, the holder may issue or request
///   certificates for any SSP for that PSID.
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(automatic_tags)]
pub struct PsidSspRange {
    pub psid: Psid,
    #[rasn(identifier = "sspRange")]
    pub ssp_range: Option<SspRange>,
}

/// This type is used for clarity of definitions.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate)]
pub struct SequenceOfPsidSspRange(pub SequenceOf<PsidSspRange>);

delegate!(SequenceOf<PsidSspRange>, SequenceOfPsidSspRange);

/// Identifies the SSPs associated with a PSID for which the holder may issue or
/// request certificates.
///
/// # Consistency with Issuing Certificate
/// ## For Opaque SSP Field
/// A `PsidSspRange` A is consistent with the issuing certificate if it contains
/// one of:
///
/// ### Option 1
/// A `SubjectPermissions` field indicating "all" and no `PsidSspRange` field
/// containing A's `psid` field
///
/// ### Option 2
/// A `PsidSspRange` P where:
/// - P's `psid` field equals A's `psid` field and either:
///   - P's `sspRange` field indicates "all"
///   - Both P and A indicate "opaque", and every OCTET STRING in A's opaque
///     matches one in P's opaque
///
/// ## For All SSP Field
/// A `PsidSspRange` A is consistent if the issuing certificate contains either:
///
/// ### Option 1
/// A `SubjectPermissions` field indicating "all" and no `PsidSspRange` field
/// containing A's `psid` field
///
/// ### Option 2
/// A `PsidSspRange` P where:
/// - P's `psid` field equals A's `psid` field
/// - P's `sspRange` field indicates "all"
///
/// See following subclauses for consistency rules with other `SspRange` types.
///
/// # Note
/// While "all" can be indicated either by omitting `SspRange` in the enclosing
/// `PsidSspRange` or explicitly, omission is preferred.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum SspRange {
    Opaque(SequenceOf<OctetString>),
    All(()),
    #[rasn(extension_addition)]
    BitmapSspRange(BitmapSspRange),
}

/// A bitmap representation of a SSP. The `sspValue` indicates permissions and the
/// `sspBitmask` contains an octet string used to permit or constrain `sspValue`
/// fields in issued certificates. The `sspValue` and `sspBitmask` fields shall be
/// of the same length.
///
/// # Certificate Consistency
/// If a certificate has a `PsidSspRange` value P for which the sspRange field is
/// bitmapSspRange, P is consistent with the issuing certificate if the issuing
/// certificate contains one of:
///
/// ## Option 1
/// A SubjectPermissions field indicating the choice "all" and no PsidSspRange field
/// containing the psid field in P
///
/// ## Option 2
/// A PsidSspRange R where:
/// - The psid field in R equals the psid field in P and either:
///   - The sspRange field in R indicates "all", or
///   - The sspRange field in R indicates bitmapSspRange and for every bit set to 1
///     in the sspBitmask in R:
///     - The corresponding bit in sspBitmask in P is set to 1
///     - The corresponding bit in sspValue in P equals the bit in sspValue in R
///
/// Reference: ETSI TS 103 097

#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(automatic_tags)]
pub struct BitmapSspRange {
    #[rasn(size("1..=32"), identifier = "sspValue")]
    pub ssp_value: OctetString,
    #[rasn(size("1..=32"), identifier = "sspBitmask")]
    pub ssp_bitmask: OctetString,
}

// ***************************************************************************
// **                       Certificate Components                          **
// ***************************************************************************

/// Contains the certificate holder's assurance level, indicating the security of
/// both the platform and storage of secret keys, as well as the confidence in
/// this assessment.
///
/// # Bit Field Encoding
/// ```text
/// Bit number     |  7  |  6  |  5  |  4  |  3  |  2  |  1  |  0  |
/// -------------- | --- | --- | --- | --- | --- | --- | --- | --- |
/// Interpretation |  A  |  A  |  A  |  R  |  R  |  R  |  C  |  C  |
///
/// Where:
/// - A: Assurance level (bits 7-5)
/// - R: Reserved for future use (bits 4-2)
/// - C: Confidence level (bits 1-0)
/// - Bit 0 is least significant
/// ```
///
/// # Interpretation
/// - Higher assurance values indicate more trusted holders (when comparing
///   certificates with the same confidence value)
/// - Specific assurance level definitions and confidence level encoding are
///   outside this standard's scope
///
/// # Historical Note
/// Originally specified in ETSI TS 103 097. Future uses are expected to maintain
/// consistency with future versions of that standard.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate)]
pub struct SubjectAssurance(pub FixedOctetString<1usize>);

delegate!(FixedOctetString<1usize>, SubjectAssurance);

/// This integer identifies a series of CRLs issued under the authority of a particular CRACA.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct CrlSeries(pub Uint16);

delegate!(Uint16, CrlSeries);

// *****************************************************************************
// **                           Pseudonym Linkage                             **
// *****************************************************************************

/// This atomic type is used in the definition of other data structures.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate)]
pub struct IValue(pub Uint16);

delegate!(Uint16, IValue);

/// This is a UTF-8 string as defined in IETF RFC 3629. The contents are determined by policy.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate, size("0..=255"))]
pub struct Hostname(pub Utf8String);

delegate!(Utf8String, Hostname);

/// This is the individual linkage value. See 5.1.3 and 7.3 for details of use.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate)]
pub struct LinkageValue(pub FixedOctetString<9usize>);

delegate!(FixedOctetString<9usize>, LinkageValue);

/// This is the group linkage value. See 5.1.3 and 7.3 for details of use.
#[derive(Builder, AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(automatic_tags)]
pub struct GroupLinkageValue {
    #[rasn(identifier = "jValue")]
    pub j_value: FixedOctetString<4>,
    pub value: FixedOctetString<9>,
}

/// This structure contains a LA Identifier for use in the algorithms specified in 5.1.3.4.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate)]
pub struct LaId(pub FixedOctetString<2usize>);

delegate!(FixedOctetString<2usize>, LaId);

/// This type is used for clarity of definitions.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate)]
pub struct SequenceOfLinkageSeed(pub SequenceOf<LinkageSeed>);

delegate!(SequenceOf<LinkageSeed>, SequenceOfLinkageSeed);

/// This structure contains a linkage seed value for use in the algorithms specified in 5.1.3.4.
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rasn(delegate)]
pub struct LinkageSeed(pub FixedOctetString<16usize>);

delegate!(FixedOctetString<16usize>, LinkageSeed);

// ***************************************************************************
// **                 Information Object Classes and Sets                   **
// ***************************************************************************

// Excluding CERT-EXT-TYPE - other types here are defined in ETSI TS 103 097 extension module

// /**
//  * @brief This structure is the Information Object Class used to contain
//  * information about a set of certificate extensions that are associated with
//  * each other: an AppExtension, a CertIssueExtension, and a
//  * CertRequestExtension.
//  */
// CERT-EXT-TYPE ::= CLASS {
//   &id        ExtId,
//   &App,
//   &Issue,
//   &Req
// } WITH SYNTAX {ID &id APP &App ISSUE &Issue REQUEST &Req}

// /**
//  * @brief This parameterized type represents a (id, content) pair drawn from
//  * the set ExtensionTypes, which is constrained to contain objects defined by
//  * the class EXT-TYPE.
//  */
// Extension {EXT-TYPE : ExtensionTypes} ::= SEQUENCE {
//   id      EXT-TYPE.&extId({ExtensionTypes}),
//   content EXT-TYPE.&ExtContent({ExtensionTypes}{@.id})
// }

// /**
//  * @brief This class defines objects in a form suitable for import into the
//  * definition of HeaderInfo.
//  */
// EXT-TYPE ::= CLASS {
//   &extId      ExtId,
//   &ExtContent
// } WITH SYNTAX {&ExtContent IDENTIFIED BY &extId}

// /**
//  * @brief This type is used as an identifier for instances of ExtContent
//  * within an EXT-TYPE.
//  */
// ExtId ::= INTEGER(0..255)
//

/// This structure is the Information Object Class used to contain information about a set of certificate extensions that are associated with each other: an AppExtension, a CertIssueExtension, and a CertRequestExtension.
pub trait CertExtType {
    const ID: ExtId;
    type App: AsnType + Encode + Decode;
    type Issue: AsnType
        + Encode
        + Decode
        + core::fmt::Debug
        + Clone
        + PartialEq
        + PartialOrd
        + Eq
        + Ord
        + core::hash::Hash;
    type Req: AsnType
        + Encode
        + Decode
        + core::fmt::Debug
        + Clone
        + PartialEq
        + PartialOrd
        + Eq
        + Ord
        + core::hash::Hash;
}
