use crate::usecases::update_todo_usecase::UpdateTodoUseCase;
use repository::TodoRepository;
use util::AppResult;
use util::Todo;

pub struct UpdateTodoUseCaseImpl<'r, R: TodoRepository> {
    todo_repository: &'r R,
}

impl<'r, R: TodoRepository> UpdateTodoUseCaseImpl<'r, R> {
    pub fn new(todo_repository: &'r R) -> Self {
        Self {
            todo_repository: todo_repository,
        }
    }
}

impl<'r, R: TodoRepository> UpdateTodoUseCase for UpdateTodoUseCaseImpl<'r, R> {
    fn run(&self, id: u32, text: String) -> AppResult<Todo> {
        self.todo_repository.update(Todo { id: id, text: text })
    }
}
