<!-- DO NOT REMOVE - contributor_list:data:start:["gleich", "Liberatys"]:end -->

# resin

⚗️ Superfast CLI for the conventional commits commit format

## ❓ What is resin?

resin is a CLI (command-line interface) tool that makes it easy to create commit messages that follow the [conventional commit format](https://www.conventionalcommits.org/). Here is a little demo:

![demo](demo.gif)

This demo will create the following commit message:

```txt
Chore: version bump
```

## ✨ Features

### 🚩 Flags

resin has three flags:

1. --help (-h) -> display a help message to the terminal
2. --all (-a) -> run `git add .` before committing the changes
3. --push (-p) -> run `git push` after committing the changes

Super simple and easy to use!

### ⚙️ Configuration

#### 📖 Scopes

You can configure resin to have your custom scopes. Below is an example config:

```toml
scopes = ['docker', 'github actions']
```

## 🚀 Install

You can install resin by downloading the latest version from the [release page](https://github.com/MM-Learning-Solutions-AG/resin/releases)
and running the provided shell script.
