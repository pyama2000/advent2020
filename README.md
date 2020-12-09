# advent2020

**※ Docker と docker-compose を使用します！**

Rust 環境がなくても Docker のみで試せるようにしています。  
Rust はコンパイルに時間がかかるので気長にお待ち下さい。

## 動作確認

```bash
docker-compoose up --build
```

`http://localhost:3000` にアクセスすると Nuxt.js で作られたアプリケーションが表示されるので、開発者ツールを見ながら動作の確認を行ってください。  
Rust のWebサーバは `http://localhost:8000` でリッスンしているので、`curl` コマンドでも確認可能です。

## 開発時

### Docker +  Rust

ローカルに Rust 環境を作った方向けのコマンドです。  

```bash
docker-compose -f docker-compose.node.yml up --build
cd advent2020_api
cargo run
```

### Docker only

ローカルに Rust 環境を作りたくない人は Docker のみでも開発可能です。
`advent2020_api/src/main.rs` を編集すると自動的にファイルを読み込んで Docker 上で実行します（ホットリロード）。

```bash
docker-compose -f docker-compose.dev.yml up --build
```
