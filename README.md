# advent2020

## 開発環境

**※ Docker と docker-compose を使用します！**  
Rust 環境がなくても Docker のみで試せるようにしています。  
Rust はコンパイルに時間がかかるので気長にお待ち下さい。

### Docker +  Rust

ローカルに Rust 環境を作った方向けのコマンドです。  

```bash
docker-compose -f docker-compose.node.yml up --build
cd advent2020_api
cargo run
```

### Docker only

```bash
docker-compose -f docker-compose.dev.yml up --build
```

## 本番環境

```bash
docker-compoose up --build
```
