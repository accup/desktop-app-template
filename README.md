# winker

egui でデスクトップアプリケーションを構築するためのテンプレート（白紙のアプリ）

## 技術スタック

### 開発環境

| 要素           | 詳細    |
| -------------- | ------- |
| 言語           | Rust    |
| パッケージ管理 | Cargo   |
| ビルド         | Cargo   |
| フォーマット   | rustfmt |
| 静的解析       | clippy  |
| 自動テスト     | Cargo   |

### デスクトップ

| 要素             | 詳細                                                                                                    |
| ---------------- | ------------------------------------------------------------------------------------------------------- |
| GUI              | [egui](https://github.com/emilk/egui) / [eframe](https://github.com/emilk/egui/tree/main/crates/eframe) |
| プラットフォーム | Windows / macOS / Linux                                                                                 |
| 起動方式         | .exe (Windows) / .app (macOS) / .AppImage (Linux)                                                       |

## 使い方

```sh
cargo run --release --package app-desktop
```

起動すると白紙のウィンドウが表示される。

## 配布

### バンドルの生成

対象のプラットフォーム上で `just bundle` を実行すると、そのプラットフォーム向けの配布用バンドルが `target/bundle/` に生成される（クロスコンパイルには対応していない）。

いずれのプラットフォームでも、Rust ツールチェーンと [just](https://github.com/casey/just) が必要となる。加えてプラットフォームごとに以下のツールを使用する。

| プラットフォーム | 成果物                                     | 追加で必要なツール                                                                                                                                                     |
| ---------------- | ------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Windows          | `winker-<arch>-pc-windows-msvc.zip`        | bash（Git for Windows などで導入）、[7-Zip](https://www.7-zip.org)                                                                                                     |
| macOS            | `winker-<arch>-apple-darwin.zip`           | なし（OS 標準の `zip` を使用する）                                                                                                                                     |
| Linux            | `winker-<arch>-unknown-linux-gnu.AppImage` | `desktop-file-utils`・`file`（開発コンテナーには導入済み）。[appimagetool](https://github.com/AppImage/appimagetool) は `target/tools/` へ自動ダウンロードして使用する |

成果物の内訳は、Windows が `winker.exe` を含む zip、macOS が `winker.app` を含む zip、Linux が単一の AppImage となる。

### 対応していないプラットフォーム

- Linux は x86_64 のみ対応する（appimagetool の取得先を x86_64 版に固定しているため）。ARM 系 Linux 向けのバンドルは生成できない。
- 自動リリース（下記）が生成するのは Windows (x86_64)・macOS (Apple Silicon)・Linux (x86_64) の 3 種のみ。Intel Mac 向けは Intel Mac 上で `just bundle` を実行すれば生成できるが、自動リリースには含まれない。

### リリース

main へ push すると、GitHub Actions が既存の `v0.<y>.0` タグの `y` を 1 増やしたタグ（タグが 1 つもなければ `v0.1.0`）で GitHub Release を作成し、Windows・macOS・Linux のバンドルを添付する。
