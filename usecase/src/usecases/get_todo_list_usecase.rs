use util::AppResult;
use util::Todo;

//TODO Usecaseのテスト組み込み
/*
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
*/
pub trait GetTodoListUseCase {
    fn run(&self) -> AppResult<Vec<Todo>>;
}
