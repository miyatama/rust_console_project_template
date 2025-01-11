use crate::RepositoryHandler;
use domain_handler::DomainHandler;
use repository::TodoRepositoryImpl;

pub struct RepositoryHandlerImpl<'d, D: DomainHandler> {
    todo_repository: TodoRepositoryImpl<'d, D::TodoApi>,
}

impl<'d, D: DomainHandler> RepositoryHandlerImpl<'d, D> {
    pub fn new(handler: &'d D) -> Self {
        let todo_repository = TodoRepositoryImpl::new(handler.todo_api_client());
        Self {
            todo_repository: todo_repository,
        }
    }
}

impl<'d, D: DomainHandler> RepositoryHandler for RepositoryHandlerImpl<'d, D> {
    type Todo = TodoRepositoryImpl<'d, D::TodoApi>;
    fn todo_repository(&self) -> &Self::Todo {
        &self.todo_repository
    }
}
