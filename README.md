# my-todo

Rust練習用です。

#### サーバー起動

```shell
cargo run
```

```text
http://localhost:3000
```

#### テスト

```shell
cargo test
```

## エンドポイント

List
```text
GET /todos
```

Create
```text
POST /todos
```

Update
```text
PATCH /todos/:id
```

Delete
```text
DELETE /todos:id
```