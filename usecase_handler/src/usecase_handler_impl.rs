use crate::UsecaseHandler;
use repository_handler::RepositoryHandler;
use usecase::{
    AddTodoUseCaseImpl, DeleteTodoUseCaseImpl, GetTodoListUseCaseImpl, UpdateTodoUseCaseImpl,
};

pub struct UsecaseHandlerImpl<'r, R: RepositoryHandler> {
    get_todo_list_usecase: GetTodoListUseCaseImpl<'r, R>,
    add_todo_usecase: AddTodoUseCaseImpl<'r, R>,
    update_todo_usecase: UpdateTodoUseCaseImpl<'r, R>,
    delete_todo_usecase: DeleteTodoUseCaseImpl<'r, R>,
}

impl<'r, R: RepositoryHandler> UsecaseHandlerImpl<'r, R> {
    pub async fn new(handler: &'r R) -> Self {
        let get_todo_list_usecase = GetTodoListUseCaseImpl::new(handler);
        let add_todo_usecase = AddTodoUseCaseImpl::new(handler);
        let update_todo_usecase = UpdateTodoUseCaseImpl::new(handler);
        let delete_todo_usecase = DeleteTodoUseCaseImpl::new(handler);
        Self {
            get_todo_list_usecase: get_todo_list_usecase,
            add_todo_usecase: add_todo_usecase,
            update_todo_usecase: update_todo_usecase,
            delete_todo_usecase: delete_todo_usecase,
        }
    }
}

impl<'r, R: RepositoryHandler> UsecaseHandler for UsecaseHandlerImpl<'r, R> {
    type GetTodoList = GetTodoListUseCaseImpl<'r, R>;
    type AddTodo = AddTodoUseCaseImpl<'r, R>;
    type UpdateTodo = UpdateTodoUseCaseImpl<'r, R>;
    type DeleteTodo = DeleteTodoUseCaseImpl<'r, R>;

    fn get_todo_list(&self) -> &Self::GetTodoList {
        &self.get_todo_list_usecase
    }
    fn add_todo(&self) -> &Self::AddTodo {
        &self.add_todo_usecase
    }
    fn update_todo(&self) -> &Self::UpdateTodo {
        &self.update_todo_usecase
    }
    fn delete_todo(&self) -> &Self::DeleteTodo {
        &self.delete_todo_usecase
    }
}
