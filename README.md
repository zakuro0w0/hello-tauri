# hello-tauri
- tauriを動かしてみるリポジトリ

## 公式
- [Prerequisites | Tauri Apps](https://tauri.app/v1/guides/getting-started/prerequisites)
- [Install Rust](https://www.rust-lang.org/tools/install)
- [nvm](https://github.com/nvm-sh/nvm)

## 環境構築＠windows
### node.js
```ps
scoop install nvm
```

- 一度windowsを再起動

```ps
nvm install lts
```

```
nvm use {version}
```

### rustup
- [Install Rust](https://www.rust-lang.org/tools/install)
  - RUSTUP-INIT.exe(64-BIT)を実行
  - これで`rustup`, `rustc`, `cargo`などが使えるようになる

### webview
- windows11の場合はデフォルトでインストール済みなので作業は不要
- windows10やlinuxの場合は別途インストールが必要らしい

## 環境構築＠linux(WSL)
### node.js

```
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.3/install.sh | bash
```

- 一度WindowsTerminalまたはvscodeを閉じて開きなおす

```
nvm install --lts
```

```
nvm use {version}
```

### rustup

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### webview

```
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

## tauriプロジェクトを作る
- 次のコマンドの後、いくつか選択する必要がある

```
npm create tauri-app
```

### プロジェクト名の入力
- ここで指定した名前のディレクトリが直下に生成される
```
zakuro0w0@surface:~/win$ npm create tauri-app

? Project name (tauri-app) ›
```

### パッケージマネージャの選択
- 今回は`npm`を選択した
```
zakuro0w0@surface:~/win$ npm create tauri-app

✔ Project name · my-tauri-app
? Choose your package manager ›
  cargo
  pnpm
  yarn
❯ npm
```

### フロントエンドUIテンプレートの選択
- 今回は`react-ts`を選択した
```
zakuro0w0@surface:~/win$ npm create tauri-app

✔ Project name · my-tauri-app
✔ Choose your package manager · npm
? Choose your UI template ›
  vanilla
  vanilla-ts
  vue
  vue-ts
  svelte
  svelte-ts
  react
❯ react-ts
  solid
  solid-ts
  next
  next-ts
  preact
  preact-ts
  angular
  clojurescript
  svelte-kit
  svelte-kit-ts
```

### ここでデフォルトのファイル群が生成される

### npmパッケージを集める

```
npm install
```

### tauriアプリを起動する

```
npm run tauri dev
```
