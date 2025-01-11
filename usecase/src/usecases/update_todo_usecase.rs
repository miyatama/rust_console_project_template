use util::AppResult;
use util::Todo;

//TODO Usecaseのテスト組み込み
/*
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
*/
pub trait UpdateTodoUseCase {
    fn run(&self, id: u32, text: String) -> AppResult<Todo>;
}
