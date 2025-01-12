use crate::usecases::delete_todo_usecase::DeleteTodoUseCase;
use repository::TodoRepository;
use util::AppResult;

pub struct DeleteTodoUseCaseImpl<'r, R: TodoRepository> {
    todo_repository: &'r R,
}

impl<'r, R: TodoRepository> DeleteTodoUseCaseImpl<'r, R> {
    pub fn new(todo_repository: &'r R) -> Self {
        Self {
            todo_repository: todo_repository,
        }
    }
}

impl<'r, R: TodoRepository> DeleteTodoUseCase for DeleteTodoUseCaseImpl<'r, R> {
    fn run(&self, id: u32) -> AppResult<()> {
        self.todo_repository.delete(id)
    }
}
