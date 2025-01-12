use crate::repositories::TodoRepository;
use domain::TodoApiClient;
use util::AppResult;
use util::Todo;

pub struct TodoRepositoryImpl<'r, T: TodoApiClient> {
    todo_api_client: &'r T,
}

impl<'r, T: TodoApiClient> TodoRepositoryImpl<'r, T> {
    pub fn new(todo_api_client: &'r T) -> Self {
        Self {
            todo_api_client: todo_api_client,
        }
    }
}

impl<'r, T: TodoApiClient> TodoRepository for TodoRepositoryImpl<'r, T> {
    fn create(&self, text: String) -> AppResult<Todo> {
        self.todo_api_client.create(Todo { id: 0, text: text })
    }

    fn update(&self, todo: Todo) -> AppResult<Todo> {
        self.todo_api_client.update(todo)
    }

    fn list(&self) -> AppResult<Vec<Todo>> {
        self.todo_api_client.list()
    }

    fn delete(&self, id: u32) -> AppResult<()> {
        self.todo_api_client.delete(Todo {
            id: id,
            text: "".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::MockTodoApiClient as TodoApiClient;
    use util::Todo;

    #[test]
    fn test_create() {
        let expect_todo = Todo {
            id: 100,
            text: "test2".to_string(),
        };
        let mock_result = Ok(expect_todo.clone());
        let mut mock_todo_api_client = TodoApiClient::new();
        mock_todo_api_client
            .expect_create()
            .times(1)
            .return_once_st(move |_| mock_result);
        let repository = TodoRepositoryImpl::new(&mock_todo_api_client);
        let result = repository.create("test".to_string());
        assert_eq!(expect_todo, result.unwrap());
    }

    #[test]
    fn test_update() {
        let expect_todo = Todo {
            id: 100,
            text: "test2".to_string(),
        };
        let mock_result = Ok(expect_todo.clone());
        let mut mock_todo_api_client = TodoApiClient::new();
        mock_todo_api_client
            .expect_update()
            .times(1)
            .return_once_st(move |_| mock_result);
        let repository = TodoRepositoryImpl::new(&mock_todo_api_client);
        let result = repository.update(Todo {
            id: 1000,
            text: "test".to_string(),
        });
        assert_eq!(expect_todo, result.unwrap());
    }

    #[test]
    fn test_list() {
        let expect_todo = vec![
            Todo {
                id: 100,
                text: "test2".to_string(),
            },
            Todo {
                id: 101,
                text: "test3".to_string(),
            },
        ];
        let mock_result = Ok(expect_todo.clone());
        let mut mock_todo_api_client = TodoApiClient::new();
        mock_todo_api_client
            .expect_list()
            .times(1)
            .return_once_st(move || mock_result);
        let repository = TodoRepositoryImpl::new(&mock_todo_api_client);
        let result = repository.list();
        assert_eq!(expect_todo, result.unwrap());
    }

    #[test]
    fn test_delete() {
        let mock_result = Ok(());
        let mut mock_todo_api_client = TodoApiClient::new();
        mock_todo_api_client
            .expect_delete()
            .times(1)
            .return_once_st(move |_| mock_result);
        let repository = TodoRepositoryImpl::new(&mock_todo_api_client);
        let result = repository.delete(101);
        assert_eq!(true, result.is_ok());
    }
}
