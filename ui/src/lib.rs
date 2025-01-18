use clap::{Parser, Subcommand};
use domain_handler::DomainHandlerImpl;
use repository_handler::RepositoryHandlerImpl;
use std::cmp::min;
use usecase::{AddTodoUseCase, DeleteTodoUseCase, GetTodoListUseCase, UpdateTodoUseCase};
use usecase_handler::{UsecaseHandler, UsecaseHandlerImpl};
use util::AppResult;
use tracing::{event, Level};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[clap(subcommand)]
    subcommand: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    List {
        #[clap(short = 'n', long = "number", required = true, ignore_case = true)]
        number: u32,
    },
    Add {
        #[clap(short = 't', long = "text", required = true, ignore_case = true)]
        text: String,
    },
    Update {
        #[clap(long = "id", required = true, ignore_case = true)]
        id: u32,

        #[clap(short = 't', long = "text", required = true, ignore_case = true)]
        text: String,
    },
    Delete {
        #[clap(long = "id", required = true, ignore_case = true)]
        id: u32,
    },
}

#[tracing::instrument]
pub async fn run(config: &Config) -> AppResult<()> {
    let domain_hanler = DomainHandlerImpl::new();
    match domain_hanler {
        Err(e) => {
            return Err(e);
        }
        _ => {}
    }
    let domain_hanler = domain_hanler.unwrap();
    let repository_handler = RepositoryHandlerImpl::new(&domain_hanler);
    let usecases = UsecaseHandlerImpl::new(&repository_handler).await;
    match &config.subcommand {
        SubCommands::List { number } => {
            let usecase = usecases.get_todo_list();
            match usecase.run() {
                Ok(todos) => {
                    let number = *number as usize;
                    let max_index = min(number, todos.len());
                    for i in 0..max_index {
                        let todo = &todos[i];
                        event!(Level::INFO, id = &todo.id, text = &todo.text);
                    }
                    return Ok(());
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        SubCommands::Add { text } => {
            let usecase = usecases.add_todo();
            match usecase.run(text.clone()) {
                Ok(todo) => {
                    event!(Level::INFO, id = &todo.id, text = &todo.text);
                    return Ok(());
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        SubCommands::Update { id, text } => {
            let usecase = usecases.update_todo();
            match usecase.run(*id, text.clone()) {
                Ok(todo) => {
                    event!(Level::INFO, id = &todo.id, text = &todo.text);
                    return Ok(());
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        SubCommands::Delete { id } => {
            let usecase = usecases.delete_todo();
            return usecase.run(*id);
        }
    }
}

pub fn get_args() -> AppResult<Config> {
    let args = Config::parse();
    Ok(args)
}
