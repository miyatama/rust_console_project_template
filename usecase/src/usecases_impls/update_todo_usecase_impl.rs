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

#[cfg(test)]
mod tests {
    use super::*;
    use repository::MockTodoRepository as TodoRepository;

    #[tokio::test]
    async fn update_todo_usecase_success() {
        let expect = Todo {
            id: 1,
            text: "test01".to_string(),
        };
        let mock_result = Ok(expect.clone());
        let mut mock_todo_repository = TodoRepository::new();
        mock_todo_repository
            .expect_update()
            .times(1)
            .return_once_st(move |_| mock_result);
        let usecase = UpdateTodoUseCaseImpl::new(&mock_todo_repository);
        let result = usecase.run(100u32, "test_param".to_string());
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expect);
    }
}
