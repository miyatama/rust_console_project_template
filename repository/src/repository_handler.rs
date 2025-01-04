use crate::TodoRepository;

// TODO RepositoryHandlerのautomockしたい
// #[cfg_attr(feature = "mock", mockall::automock)]
pub trait RepositoryHandler {
    type Todo: TodoRepository;
    fn todo_repository(&self) -> &Self::Todo;
}
