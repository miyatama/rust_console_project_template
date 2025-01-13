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

#[cfg(test)]
mod tests {
    use super::*;
    use repository::MockTodoRepository as TodoRepository;

    #[tokio::test]
    async fn get_todo_list_usecase_success() {
        let expect_list = vec![
            Todo {
                id: 1,
                text: "test01".to_string(),
            },
            Todo {
                id: 2,
                text: "test02".to_string(),
            },
        ];
        let mock_result = Ok(expect_list.clone());
        let mut mock_todo_repository = TodoRepository::new();
        mock_todo_repository
            .expect_list()
            .times(1)
            .return_once_st(move || mock_result);
        let usecase = GetTodoListUseCaseImpl::new(&mock_todo_repository);
        let result = usecase.run();
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expect_list);
    }
}
