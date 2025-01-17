use crate::usecases::add_todo_usecase::AddTodoUseCase;

use repository::TodoRepository;
use util::AppResult;
use util::Todo;

pub struct AddTodoUseCaseImpl<'r, R: TodoRepository> {
    todo_repository: &'r R,
}

impl<'r, R: TodoRepository> AddTodoUseCaseImpl<'r, R> {
    pub fn new(todo_repository: &'r R) -> Self {
        Self {
            todo_repository: todo_repository,
        }
    }
}

impl<'r, R: TodoRepository> AddTodoUseCase for AddTodoUseCaseImpl<'r, R> {
    #[tracing::instrument(skip(self))]
    fn run(&self, text: String) -> AppResult<Todo> {
        self.todo_repository.create(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use repository::MockTodoRepository as TodoRepository;

    #[tokio::test]
    async fn add_todo_usecase_success() {
        let text = "test_parameter".to_string();
        let expect = Todo {
            id: 100,
            text: "test message".to_string(),
        };
        let mock_result = Ok(expect.clone());
        let mut mock_todo_repository = TodoRepository::new();
        mock_todo_repository
            .expect_create()
            .times(1)
            .return_once_st(move |_| mock_result);
        let usecase = AddTodoUseCaseImpl::new(&mock_todo_repository);
        let result = usecase.run(text);
        assert_eq!(result.is_ok(), true);
        let result = result.unwrap();
        assert_eq!(expect.id, result.id);
        assert_eq!(expect.text, result.text);
    }
}
