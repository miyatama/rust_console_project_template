use util::AppResult;

//TODO Usecaseのテスト組み込み
/*
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
*/
pub trait DeleteTodoUseCase {
    fn run(&self, id: u32) -> AppResult<()>;
}
