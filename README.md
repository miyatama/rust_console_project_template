# outline

コマンドラインアプリのテンプレート。テストの組み込み方法などの実装方法調査で使用。

テストコマンド

```shell
cargo test -p domain --features mock
cargo test -p repository --features mock
cargo test -p usecase --features mock
```

## Layer

| name | description |
| :----- | :----- |
| ui | ユーザーへのインタラクション等を提供 |
| usecase | 各種ビジネスロジック |
| usecase_handleer | 各種ビジネスロジックの処理群を上位層へ提供 |
| repository | usecaseへdomainを提供 |
| repository_handler | repositoryの処理群を上位層へ提供 |
| domain | アトム的なデータアクセッサ |
| domain_hanlder | domainの処理群を上位層へ提供 |
| util | データ定義やログなど |

utilモジュール

| name | description |
| :----- | :----- |
| log | ログ等の共通ロジック |
| data | User等のアトムデータ定義 |

```mermaid
---
参照関係
---
classDiagram

  usecase_handler <.. ui 
  domain_handler <.. ui
  repository_handler <.. ui
  usecase <.. usecase_handler
  repository_handler <.. usecase
  repository <.. repository_handler
  domain_handler <.. repository
  domain <.. domain_handler
  util <.. ui

  class ui {
  }
  class usecase {
  }
  class repository_handler {
  }
  class repository {
  }
  class domain_handler {
  }
  class domain {
  }
```

## Settings

TODOのAPI提供サーバーの情報とか持たせる。yamlにしとく。

## build

local

```shell
cargo build --release
```

docker

```shell
version="1.0.0"
docker build -t rust_console:${version} .
docker tag rust_console:${version} rust_console:latest
```

## reference

+ 引数
  + [RustのClapクレートがメチャクチャ良かった話](https://zenn.dev/shinobuy/articles/53aed032fe5977)
  + [Clap の Derive API で列挙型のコマンドラインオプションを実装する](https://zenn.dev/takanori_is/articles/rust-clap-derive-api-arg-enum)
+ Web
  + [Rust の新しい HTTP サーバーのクレート Axum をフルに活用してサーバーサイドアプリケーション開発をしてみる](https://blog-dry.com/entry/2021/12/26/002649#Dependency-Injection)
+ テスト
  + [[Rust] mockallで単体テスト](https://qiita.com/deepgreenAN/items/1b9887db759bbb96c9b6)
  + [RustのテストのTIPS](https://qiita.com/aoyagikouhei/items/dadafc8d6295db76ef0e#asyncawait%E3%81%AE%E3%83%86%E3%82%B9%E3%83%88)
  + [RustによるResult型のエラーハンドリング入門](https://zenn.dev/hakoten/articles/8ae9dd0d3a2080)
+ [【Rust】モックライブラリMockitoで非同期のテストコードを書く](https://zenn.dev/collabostyle/articles/c44cc1d20ad4fb)
+ WorkSpace
  + [Rust Crate Warning About Workspace Resolver - What is it, how to fix it?](https://forum.dfinity.org/t/rust-crate-warning-about-workspace-resolver-what-is-it-how-to-fix-it/23883)
  + [[Rust] フィーチャーフラグの使い方](https://qiita.com/osanshouo/items/43271813b5d62e89d598)
+ ログ
  + [Rust での tracing](https://blog.ojisan.io/rust-tracing/) 
  + [tracing/tracing-subscriberでログが出力される仕組みを理解する](https://blog.ymgyt.io/entry/how-tracing-and-tracing-subscriber-write-events/)
  + [Tracingを用いたRustのロギング理解の図とメモ](https://zenn.dev/scirexs/articles/c467a911218593)
  + [AWS Distro for Open Telemetryの超詳細解説](https://qiita.com/tech4anyone/items/1e34b0fcbed2712a3f79)
+ Docker
  + [[1.59.0対応] Rustを軽量イメージ化するためのDockerfile](https://zenn.dev/kyoheiu/articles/dcefe0c75f0e17)
  + [Docker hub - Rust](https://hub.docker.com/_/rust)
+ その他
  + [Rust の Future について](https://blog.tiqwab.com/2022/03/26/rust-future.html)
  + [Rustのマクロ展開後のコードを確認する](https://scrapbox.io/emanon001/Rust%E3%81%AE%E3%83%9E%E3%82%AF%E3%83%AD%E5%B1%95%E9%96%8B%E5%BE%8C%E3%81%AE%E3%82%B3%E3%83%BC%E3%83%89%E3%82%92%E7%A2%BA%E8%AA%8D%E3%81%99%E3%82%8B)

6831:6831/udp 
6832:6832/udp 
16686:16686 
14268:14268
## will

+ [x] DI構成をModules形式に変更
+ [x] Domainsのインジェクション
+ [x] Repositories内の全部参照に切り替えたい
+ [x] UseCases内の全部参照に切り替えたい
+ [x] Domainsの実装
+ [x] async -> Futureとしたが、Futureを内部に閉じ込めてIFはシンプルにAppResultとする
+ [x] 各層のimplはtraitを参照させる。直接implを参照させない。
  + traitを参照させようとしたら実態が見れないと怒られたので実装参照で実施
+ [x] テスト組み込み
+ [x] setting機能追加
+ [x] log -> tracing
+ [ ] ファイル管理とかのサンプル追加
+ [ ] 制御ファイル(yaml)の追加
+ [ ] 起動パラメタでのdry-runしたい
+ [ ] domain層のテスト追加(mockito利用)
+ [ ] Qiitaに記事を出す
  + base
    + struct only injection using async
  + test
    + divide trait
    + async to future -> closure future
    + write 1st test -> error mock is trait
  + for test 
    + trait injection

## problem

### warning: virtual workspace defaulting to `resolver = "1"` despite one or more workspace members being on edition 2021 which implies `resolver = "2"

```texgt
warning: virtual workspace defaulting to `resolver = "1"` despite one or more workspace members being on edition 2021 which implies `resolver = "2"`
note: to keep the current resolver, specify `workspace.resolver = "1"` in the workspace root's manifest
note: to use the edition 2021 resolver, specify `workspace.resolver = "2"` in the workspace root's manifest
note: for more details see https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions
```

ルートのcargo.tomlにresolverを指定

```toml
[workspace]
resolver = "2"
```

### warning: use of `async fn` in public traits is discouraged as auto trait bounds cannot be specified

```text
warning: use of `async fn` in public traits is discouraged as auto trait bounds cannot be specified
 --> domain\src\domains\todo_api_client.rs:6:5
  |
6 |     async fn list(&self) -> AppResult<Vec<Todo>>;
  |     ^^^^^
  |
  = note: you can suppress this lint if you plan to use the trait only in your own code, or do not care about auto traits like `Send` on the `Future`
  = note: `#[warn(async_fn_in_trait)]` on by default
help: you can alternatively desugar to a normal `fn` that returns `impl Future` and add any desired bounds such as `Send`, but these cannot be relaxed without a breaking API change
  |
6 -     async fn list(&self) -> AppResult<Vec<Todo>>;
6 +     fn list(&self) -> impl std::future::Future<Output = AppResult<Vec<Todo>>> + Send;
  |

```

トレイトに`async fun`を定義するのは良くない。`Future <Output = > + Sync`で定義する。Domain内部は`async move {}`でFutureを作って返す。

### the trait bound `Pin<Box<(dyn std::future::Future<Output = Result<util::Todo, Box<(dyn std::error::Error + 'static)>>> + Send + 'static)>>: From<Result<util::Todo, _>>` is not satisfied

```text
error[E0277]: the trait bound `Pin<Box<(dyn std::future::Future<Output = Result<util::Todo, Box<(dyn std::error::Error + 'static)>>> + Send + 'static)>>: From<Result<util::Todo, _>>` is not satisfied
  --> usecase\src\usecases_impls\add_todo_usecase_impl.rs:45:27
   |
45 |             .return_const(Ok(expect.clone()));
   |              ------------ ^^^^^^^^^^^^^^^^^^ the trait `From<Result<util::Todo, _>>` is not implemented for `Pin<Box<(dyn std::future::Future<Output = Result<util::Todo, Box<(dyn std::error::Error + 'static)>>> + Send + 'static)>>`, which is required by `Result<util::Todo, _>: Into<Pin<Box<(dyn std::future::Future<Output = Result<util::Todo, Box<(dyn std::error::Error + 'static)>>> + Send + 'static)>>>`
   |              |
   |              required by a bound introduced by this call
   |
```

Futureを引きまわすのではなく、一番下のレイヤーでFutureを待つ。`futures::executor`を利用。

### mismatched types

```text
error[E0308]: mismatched types
  --> usecase\src\usecases_impls\add_todo_usecase_impl.rs:46:47
   |
46 |         let usecase = AddTodoUseCaseImpl::new(mock_todo_repository);
   |                       ----------------------- ^^^^^^^^^^^^^^^^^^^^ expected `TodoRepositoryImpl`, found `MockTodoRepository`      
   |                       |
   |                       arguments to this function are incorrect
   |
```

mockall用にinjectionをtraitへ変更する

### custom attribute panicked

domainで定義したDomainHandler traitで発生。

```text
error: custom attribute panicked
  --> domain\src\domain_handler.rs:16:30
   |
16 | #[cfg_attr(feature = "mock", automock)]
   |                              ^^^^^^^^
   |
   = help: message: Default value not given for associated type.  More information may be available when mockall is built with the "nightly" feature.
```

automockはfeatureを指定してuseしてる

```rust
#[cfg(not(feature = "mock"))]
use crate::TodoApiClient;

#[cfg(feature = "mock")]
use mockall::automock;

#[cfg_attr(feature = "mock", automock)]
pub trait DomainHandler {
    type TodoApi: TodoApiClient;
    fn todo_api_client(&self) -> &Self::TodoApi;
}
```

どうもautomockなtraitを保持するautomockなtraitを使ってるのがまずいっぽい。 -> domainをdomain + domain_handlerに分割

###  no method named `json` found for struct `tracing_subscriber::fmt::Layer` in the current scope

```rust
error[E0599]: no method named `json` found for struct `tracing_subscriber::fmt::Layer` in the current scope
  --> ui\src/main.rs:17:48
   |
17 |         .with(tracing_subscriber::fmt::layer().json())
   |                                                ^^^^ method not found in `Layer<_>`

For more information about this error, try `rustc --explain E0599`.
error: could not compile `ui` (bin "ui") due to 1 previous error
```

tracingで下記を試そうとしたら発生

```rust
.with(tracing_subscriber::fmt::layer().json())
```

[tracing-subscriberの公式ドキュメント](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/)にもある通り、jsonフラグを立てる必要がある。

```toml
tracing-subscriber = {version = "0.3.19", features = ["std", "env-filter", "registry", "fmt", "json"]}
```