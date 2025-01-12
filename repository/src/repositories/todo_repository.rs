use util::AppResult;
use util::Todo;

#[mockall::automock]
pub trait TodoRepository {
    fn create(&self, text: String) -> AppResult<Todo>;
    fn update(&self, todo: Todo) -> AppResult<Todo>;
    fn delete(&self, id: u32) -> AppResult<()>;
    fn list(&self) -> AppResult<Vec<Todo>>;
}
