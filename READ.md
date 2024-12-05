# Python-Rust Dev Container Project

このプロジェクトは、PythonからRustコードを呼び出すためのDev Container環境を提供します。

## セットアップ手順

1. **VS Code** を開き、プロジェクトフォルダを開きます。
2. 左下の緑色のアイコンをクリックし、`Reopen in Container` を選択します。
3. コンテナのビルドと初期化が完了したら、Pythonスクリプトを実行できます。

## 使用方法

Pythonスクリプトを実行して、Rust関数を呼び出します。

```bash
cd python
python main.py
```

## セットアップとビルドの詳細

Dev Containerの設定により、コンテナ起動時に以下のコマンドが実行されます。

1. `pip install --upgrade pip`
2. `pip install -r python/requirements.txt`
3. `cd rust_lib && maturin develop`

これにより、Rustライブラリがビルドされ、Python環境にインストールされます。

## 依存関係

- **Python**: 3.x
- **Rust**: 最新安定版
- **PyO3**: Pythonとの連携を容易にするRustクレート
- **maturin**: RustをPythonモジュールとしてビルド・管理するツール
- **Dev Container**: 開発環境をコンテナ化するための設定
