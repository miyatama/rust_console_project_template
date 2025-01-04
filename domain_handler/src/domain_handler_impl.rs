use crate::DomainHandler;
use domain::TodoApiClientImpl;

pub struct DomainHandlerImpl {
    todo_api_client: TodoApiClientImpl,
}

#[cfg_attr(feature = "mock", mockall::automock)]
impl DomainHandler for DomainHandlerImpl {
    type TodoApi = TodoApiClientImpl;
    fn todo_api_client(&self) -> &Self::TodoApi {
        &self.todo_api_client
    }
}

impl DomainHandlerImpl {
    pub fn new() -> Self {
        Self {
            todo_api_client: TodoApiClientImpl::new(),
        }
    }
}
