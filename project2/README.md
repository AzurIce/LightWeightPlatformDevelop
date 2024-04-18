# Project2 - “飞机大战”

第二次作业要求实现一个“飞机大战”。

所用技术：
- Rust + wasm-bindgen -> wasm + ES6 module
    通过 wasm-bindgen 由 Rust 生成 wasm 模块，并用 `wasm-pack` 来直接导出浏览器可以加载的 ES6 模块。
- HTML、CSS、js

由于浏览器的跨域安全策略，不允许从 `file://` 源加载资源，因此本项目需要任意一个基本的 http_server 来展示：
- 如果你有 python：
    ```
    python3 -m http.server 8080
    ```
- 如果你有 cargo：
    ```
    cargo install miniserve
    miniserve . --index "index.html" -p 8080
    ```
- 或者，直接使用这个 VSCode 插件：
    https://github.com/ritwickdey/vscode-live-server
    安装后编辑器的右下角状态栏会有一个 `Go Live` 按钮（启动了则显示端口）
    启动后会以整个编辑器打开的文件夹为根目录进行 serve。

## 架构

既然是「游戏」，那么就按照经典的 渲染 + 逻辑 拆分的模式来做。
<s>既然是我写，那肯定要用点 Rust</s>

- `core`：游戏逻辑引擎本体，Rust 编写
    负责整个游戏的逻辑数据计算，通过编译为 wasm 在前端中使用
- `frontend`：前端目录

游戏核心按照固定的 tps 运行主循环，前端通过直接访问 wasm 内存获取数据进行渲染。