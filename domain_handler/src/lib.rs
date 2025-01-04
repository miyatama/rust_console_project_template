mod domain_handler;
mod domain_handler_impl;

#[cfg(feature = "mock")]
use mockall_double::double;

pub use domain_handler::DomainHandler;
#[cfg_attr(feature = "mock", double)]
pub use domain_handler_impl::DomainHandlerImpl;
