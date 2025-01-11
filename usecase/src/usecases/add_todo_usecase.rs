use util::AppResult;
use util::Todo;

//TODO Usecaseのテスト組み込み
/*
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
*/
pub trait AddTodoUseCase {
    fn run(&self, text: String) -> AppResult<Todo>;
}
