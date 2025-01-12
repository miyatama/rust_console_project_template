use crate::usecases::get_todo_list_usecase::GetTodoListUseCase;
use repository::TodoRepository;
use util::AppResult;
use util::Todo;

pub struct GetTodoListUseCaseImpl<'r, R: TodoRepository> {
    todo_repository: &'r R,
}

impl<'r, R: TodoRepository> GetTodoListUseCaseImpl<'r, R> {
    pub fn new(todo_repository: &'r R) -> Self {
        Self {
            todo_repository: todo_repository,
        }
    }
}

impl<'r, R: TodoRepository> GetTodoListUseCase for GetTodoListUseCaseImpl<'r, R> {
    fn run(&self) -> AppResult<Vec<Todo>> {
        self.todo_repository.list()
    }
}
