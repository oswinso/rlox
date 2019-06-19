pub mod composed_error;
pub mod fatal_error;
pub mod incorrect_arguments_error;
pub mod runtime_error;
pub mod type_error;
pub mod undefined_variable_error;

pub use composed_error::ComposedError;
pub use fatal_error::FatalError;
pub use incorrect_arguments_error::IncorrectArgumentsError;
pub use runtime_error::RuntimeError;
pub use type_error::TypeError;
pub use undefined_variable_error::UndefinedVariableError;
