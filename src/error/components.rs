use snafu::Snafu;

/// Error types for inner subtype constraints
#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum InnerSubtypeConstraintError {
    /// General error for subtype constraint violation: invalid inner component combination in a newtype.
    /// Mostly useful when there are specific variants for a base type, e.g. implicit or explicit variants.
    /// Other use discouraged.
    #[snafu(display("Invalid component combination in {component_path}: {details}"))]
    SubtypeConstraintViolation {
        /// The path to the component where the invalid component combination was found.
        component_path: &'static str,
        /// Detailed information about the invalid component combination.
        details: &'static str,
    },

    /// A required component is missing. All components stated in `components` must be present.
    #[snafu(display(
        "Missing required components in {component_path}: the following must be present:"
    ))]
    MissingRequiredComponent {
        /// The path to the component where the required component is missing.
        component_path: &'static str,
        /// List of required components that are missing.
        components: &'static [&'static str],
    },

    /// Subtype constraint violation: mutually exclusive components are present
    #[snafu(display("Mutually exclusive components present in {component_path}: {components:?}"))]
    MutuallyExclusiveViolation {
        /// The path to the componenent where the mutually exclusive components are present.
        component_path: &'static str,
        /// List of mutually exclusive components that are present.
        components: &'static [&'static str],
    },
    /// Invalid value for a component
    #[snafu(display(
        "Invalid value for component {component_path} in {component_name}: {details}"
    ))]
    InvalidComponentValue {
        /// The path to the component where the invalid component value was found.
        component_path: &'static str,
        /// The name of the component type or field with the invalid value.
        component_name: &'static str,
        /// Detailed information about the invalid component value.
        details: alloc::string::String,
    },
    /// Invalid component variant (applies to enums and choice types)
    #[snafu(display(
        "Invalid variant for component {component_type} in {component_path}: {details}"
    ))]
    InvalidComponentVariant {
        /// The path to the component where the invalid component variant was found.
        component_path: &'static str,
        /// The name of the component type with the invalid variant.
        component_type: &'static str,
        /// Detailed information about the invalid component variant.
        details: alloc::string::String,
    },
    /// Invalid size constraint for a component
    #[snafu(display(
        "Invalid size constraint for component {component_name} in {component_path}: {details}"
    ))]
    InvalidComponentSize {
        /// The name of the type where the invalid component value was found.
        component_path: &'static str,
        /// The name of the component type or field with the invalid size.
        component_name: &'static str,
        /// Detailed information about the inner error.
        details: alloc::string::String,
    },
    /// A field that should be absent is present
    #[snafu(display(
        "Component that should be absent is present in {component_path}: {component_name}"
    ))]
    UnexpectedComponentPresent {
        /// The name of the type where the absent component is present.
        component_path: &'static str,
        /// The name of the absent component that is present.
        component_name: &'static str,
    },
    /// An error if inner `CONTAINING` constraint is not satisfied.
    #[snafu(display("CONTAINING does not contain the expected component {expected}: {err}"))]
    InvalidInnerContaining {
        /// The name of the expected value in CONTAINING constraint.
        expected: &'static str,
        /// Inner decode error
        err: crate::error::DecodeError,
    },
}
