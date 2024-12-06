use snafu::Snafu;

/// Error types for inner subtype constraints
#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum InnerSubtypeConstraintError {
    /// Subtype constraint violation: invalid component combination
    #[snafu(display("Invalid component combination in {type_name}: {details}"))]
    InvalidCombination {
        /// The name of the type where the invalid component combination was found.
        type_name: &'static str,
        /// Detailed information about the invalid component combination.
        details: &'static str,
    },

    /// Subtype constraint violation: required component is missing
    #[snafu(display("Missing required components in {type_name}: all must be present:"))]
    MissingRequiredComponent {
        /// The name of the type where the required component is missing.
        type_name: &'static str,
        /// List of required components that are missing.
        components: &'static [&'static str],
    },
    /// Subtype constraint violation: at least one of the components must be present
    #[snafu(display(
        "At least one of the components must be present in {type_name}: {components:?}"
    ))]
    MissingAtLeastOneComponent {
        /// The name of the type where at least one of the components must be present.
        type_name: &'static str,
        /// List of components that must be present. At least one of these must be present.
        components: &'static [&'static str],
    },

    /// Subtype constraint violation: mutually exclusive components are present
    #[snafu(display("Mutually exclusive components present in {type_name}: {components:?}"))]
    MutuallyExclusiveViolation {
        /// The name of the type where the mutually exclusive components are present.
        type_name: &'static str,
        /// List of mutually exclusive components that are present.
        components: &'static [&'static str],
    },
    /// Invalid value for a component
    #[snafu(display("Invalid value for component {component_name} in {type_name}: {details}"))]
    InvalidComponentValue {
        /// The name of the type where the invalid component value was found.
        type_name: &'static str,
        /// The name of the component with the invalid value.
        component_name: &'static str,
        /// Detailed information about the invalid component value.
        details: alloc::string::String,
    },
}
