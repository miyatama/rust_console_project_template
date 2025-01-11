use repository::TodoRepository;

pub trait RepositoryHandler {
    type Todo: TodoRepository;
    fn todo_repository(&self) -> &Self::Todo;
}
