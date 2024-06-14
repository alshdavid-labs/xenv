# `xenv` 📤

## Utility to `source` .env files 

This is a cross-platform utility that allows you to `source` idiomatic `.env` files in multiple shell environments on Window, Linux, MacOS, Bash, Zsh, PowerShell.

## Example

### Bash / Zsh

```bash
# bash
eval $(xenv .env)
echo $FOO
```

### PowerShell

```powershell
# powershell
xenv .env | Invoke-Expression
echo "$FOO"
```

# Development 🧩

## Prerequisite Tools

- [Rust](https://rustup.rs/)
- [Just](https://github.com/casey/just)

## Building

```
just build
target=release just build
```

## Running Development Build

```
just run
just run -f node_modules ~/Development
```