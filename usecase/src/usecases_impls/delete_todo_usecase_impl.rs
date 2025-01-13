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

#[cfg(test)]
mod tests {
    use super::*;
    use repository::MockTodoRepository as TodoRepository;

    #[tokio::test]
    async fn delete_todo_usecase_success() {
        let mock_result = Ok(());
        let mut mock_todo_repository = TodoRepository::new();
        mock_todo_repository
            .expect_delete()
            .times(1)
            .return_once_st(move |_| mock_result);
        let usecase = DeleteTodoUseCaseImpl::new(&mock_todo_repository);
        let result = usecase.run(101u32);
        assert_eq!(result.is_ok(), true);
    }
}
