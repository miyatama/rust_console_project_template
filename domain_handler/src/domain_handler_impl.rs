use crate::DomainHandler;
use domain::SettingsImpl;
use domain::TodoApiClientImpl;
use util::AppResult;

pub struct DomainHandlerImpl {
    todo_api_client: TodoApiClientImpl,
    settings: SettingsImpl,
}

impl DomainHandler for DomainHandlerImpl {
    type TodoApi = TodoApiClientImpl;
    type Setting = SettingsImpl;
    fn todo_api_client(&self) -> &Self::TodoApi {
        &self.todo_api_client
    }
    fn settings(&self) -> &Self::Setting {
        &self.settings
    }
}

impl DomainHandlerImpl {
    pub fn new() -> AppResult<Self> {
        let settings = SettingsImpl::new().unwrap();
        Ok(Self {
            settings: settings.clone(),
            todo_api_client: TodoApiClientImpl::new(settings),
        })
    }
}
