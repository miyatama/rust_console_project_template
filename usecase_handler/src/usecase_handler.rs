use usecase::{AddTodoUseCase, DeleteTodoUseCase, GetTodoListUseCase, UpdateTodoUseCase};
pub trait UsecaseHandler {
    type GetTodoList: GetTodoListUseCase;
    type AddTodo: AddTodoUseCase;
    type UpdateTodo: UpdateTodoUseCase;
    type DeleteTodo: DeleteTodoUseCase;
    fn get_todo_list(&self) -> &Self::GetTodoList;
    fn add_todo(&self) -> &Self::AddTodo;
    fn update_todo(&self) -> &Self::UpdateTodo;
    fn delete_todo(&self) -> &Self::DeleteTodo;
}
