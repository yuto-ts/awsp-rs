# awsp-rs

AWS Profile Switcher — A Rust rewrite of [awsp](https://github.com/johnnyopao/awsp).

## Features

- 🔍 **Fuzzy Search** — Filter profiles as you type
- ⚡ **Fast** — Native Rust binary, no Node.js required
- 📋 **Current Profile Display** — Shows your current profile before switching
- 🛡️ **Safe** — Pressing Ctrl+C cancels without making changes

## Prerequisites

Configure your AWS profiles using the AWS CLI:

```sh
aws configure --profile PROFILE_NAME
```

## Installation

### Via Homebrew (Recommended)

```sh
brew tap yuto-ts/awsp
brew install awsp-rs
```

### From Source

```sh
cargo install --git https://github.com/yuto-ts/awsp-rs
```

Or clone and install locally:

```sh
git clone https://github.com/yuto-ts/awsp-rs.git
cd awsp-rs
cargo install --path .
```

## Setup

Add the following to your `.bashrc` or `.zshrc`:

```sh
alias awsp='eval "$(awsp-rs)"'
```

## Usage

```sh
awsp
```

When you run this:
1. Displays your current AWS profile
2. Shows a fuzzy-searchable list of profiles from `~/.aws/config`
3. Sets the `AWS_PROFILE` environment variable to your selection

## How It Works

`awsp-rs` outputs shell commands (`export AWS_PROFILE=...` or `unset AWS_PROFILE`) to stdout. The `eval` in the alias executes these commands in your current shell, setting the environment variable. All UI output goes to stderr, so it doesn't interfere with `eval`.

## Display AWS Profile in Shell Prompt

```sh
# For zsh (oh-my-zsh)
function aws_prof {
  local profile="${AWS_PROFILE:=default}"
  echo "%{$fg_bold[blue]%}aws:(%{$fg[yellow]%}${profile}%{$fg_bold[blue]%})%{$reset_color%} "
}

PROMPT='$(aws_prof) '$PROMPT
```
