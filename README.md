# awsp-rs

AWS プロファイル切り替えツール — [awsp](https://github.com/johnnyopao/awsp) の Rust 製リライトです。

## 特徴

- 🔍 **ファジー検索** — 入力するだけでプロファイルを絞り込み
- ⚡ **高速** — Rust ネイティブバイナリ、Node.js 不要
- 📋 **現在のプロファイル表示** — 切り替え前に今のプロファイルを表示
- 🛡️ **安全** — Ctrl+C でキャンセルしても何も変更されない

## 前提条件

AWS CLI でプロファイルを設定しておいてください：

```sh
aws configure --profile PROFILE_NAME
```

## インストール

```sh
cargo install --path .
```

## セットアップ

`.bashrc` または `.zshrc` に以下を追加してください：

```sh
alias awsp='eval "$(awsp-rs)"'
```

## 使い方

```sh
awsp
```

実行すると：
1. 現在の AWS プロファイルを表示
2. `~/.aws/config` のプロファイル一覧をファジー検索付きで表示
3. 選択したプロファイルを `AWS_PROFILE` 環境変数に設定

## 仕組み

`awsp-rs` はシェルコマンド（`export AWS_PROFILE=...` または `unset AWS_PROFILE`）を stdout に出力します。alias の `eval` がこれを現在のシェルで実行し、環境変数を設定します。UI 出力はすべて stderr に出るため、eval と干渉しません。

## シェルプロンプトに AWS プロファイルを表示する

```sh
# zsh (oh-my-zsh) の場合
function aws_prof {
  local profile="${AWS_PROFILE:=default}"
  echo "%{$fg_bold[blue]%}aws:(%{$fg[yellow]%}${profile}%{$fg_bold[blue]%})%{$reset_color%} "
}

PROMPT='$(aws_prof) '$PROMPT
```
