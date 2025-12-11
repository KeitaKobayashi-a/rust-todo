<div id="top"></div>

## 使用技術一覧

<!-- シールド一覧 -->
<!-- 該当するプロジェクトの中から任意のものを選ぶ-->
<p style="display: inline">
  <!-- フロントエンドのフレームワーク一覧 -->
  <img src="https://img.shields.io/badge/-React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB">
  <!-- バックエンドのフレームワーク一覧 -->
  <img src="https://img.shields.io/badge/-axum-000000.svg?logo=&style=for-the-badge">
  <!-- バックエンドの言語一覧 -->
  <img src="https://img.shields.io/badge/-rust-000000.svg?logo=rust&style=for-the-badge">
  <!-- ミドルウェア一覧 -->
  <img src="https://img.shields.io/badge/-postgres-4169E1.svg?logo=&style=for-the-badge">
  <img src="https://img.shields.io/badge/-render-000000?style=for-the-badge&logo=render&logoColor=844EBA">
</p>

## 目次

1. [プロジェクトについて](#プロジェクトについて)
2. [環境](#環境)
3. [ディレクトリ構成](#ディレクトリ構成)
4. [開発環境構築](#開発環境構築)

<!-- プロジェクト名を記載 -->

# Rust Todo
<p align="left">(<a href="https://rust-todo-mdom.onrender.com">本番環境はこちら</a>)</p>
https://rust-todo-mdom.onrender.com

### プロジェクトについて

TodoリストのWebアプリです。


<p align="left">(<a href="#top">トップへ</a>)</p>

## 主な環境

<!-- 言語、フレームワーク、ミドルウェア、インフラの一覧とバージョンを記載 -->

| 言語・フレームワーク | バージョン   |
|------------|---------|
| axum       | 0.8.7   |
| serde      | 1.0.228 |
| tokio      | 1.48.0  |
| tower-http | 0.6.2   |
| Postgres   | 14      |
| React      | 19.2.0  |
| cargo      | 1.91.1  |
| Rust       | 1.91.1  |

その他のパッケージのバージョンは Cargo.toml と package.json を参照してください

<p align="left">(<a href="#top">トップへ</a>)</p>

## ディレクトリ構成

```text
.
├── .env
├── .gitignore
├── backend
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── migrations
│   ├── src
└── client
    ├── .gitignore
    ├── eslint.config.js
    ├── index.html
    ├── package-lock.json
    ├── package.json
    ├── src
    └── vite.config.js
 ```

<p align="left">(<a href="#top">トップへ</a>)</p>

## 開発環境構築

### コンテナの作成と起動

.env ファイルを以下の環境変数例を元に作成

.env
RUST_LOG=debug
POSTGRES_USER=postgres
POSTGRES_PASSWORD=todo
POSTGRES_DB=todo_db
DATABASE_URL=postgres://postgres:todo@localhost:5432/todo_db


.env ファイルを作成後、以下のコマンドで開発環境を構築

cd ../client && npm run dev

cd ../backend && cargo run

### 動作確認

http://127.0.0.1:5173 にアクセスできるか確認
アクセスできたら成功

<p align="left">(<a href="#top">トップへ</a>)</p>
