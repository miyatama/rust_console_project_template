use domain::Settings;
use domain::TodoApiClient;

pub trait DomainHandler {
    type TodoApi: TodoApiClient;
    type Setting: Settings;

    fn todo_api_client(&self) -> &Self::TodoApi;
    fn settings(&self) -> &Self::Setting;
}
