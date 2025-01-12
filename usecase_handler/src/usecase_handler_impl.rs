use crate::UsecaseHandler;
use repository_handler::RepositoryHandler;
use usecase::{
    AddTodoUseCaseImpl, DeleteTodoUseCaseImpl, GetTodoListUseCaseImpl, UpdateTodoUseCaseImpl,
};

pub struct UsecaseHandlerImpl<'r, R: RepositoryHandler> {
    get_todo_list_usecase: GetTodoListUseCaseImpl<'r, R::Todo>,
    add_todo_usecase: AddTodoUseCaseImpl<'r, R::Todo>,
    update_todo_usecase: UpdateTodoUseCaseImpl<'r, R::Todo>,
    delete_todo_usecase: DeleteTodoUseCaseImpl<'r, R::Todo>,
}

impl<'r, R: RepositoryHandler> UsecaseHandlerImpl<'r, R> {
    pub async fn new(handler: &'r R) -> Self {
        let get_todo_list_usecase = GetTodoListUseCaseImpl::new(handler.todo_repository());
        let add_todo_usecase = AddTodoUseCaseImpl::new(handler.todo_repository());
        let update_todo_usecase = UpdateTodoUseCaseImpl::new(handler.todo_repository());
        let delete_todo_usecase = DeleteTodoUseCaseImpl::new(handler.todo_repository());
        Self {
            get_todo_list_usecase: get_todo_list_usecase,
            add_todo_usecase: add_todo_usecase,
            update_todo_usecase: update_todo_usecase,
            delete_todo_usecase: delete_todo_usecase,
        }
    }
}

impl<'r, R: RepositoryHandler> UsecaseHandler for UsecaseHandlerImpl<'r, R> {
    type GetTodoList = GetTodoListUseCaseImpl<'r, R::Todo>;
    type AddTodo = AddTodoUseCaseImpl<'r, R::Todo>;
    type UpdateTodo = UpdateTodoUseCaseImpl<'r, R::Todo>;
    type DeleteTodo = DeleteTodoUseCaseImpl<'r, R::Todo>;

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
