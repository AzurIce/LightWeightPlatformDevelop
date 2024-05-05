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
    miniserve ./ --index "index.html" -p 8080
    ```
- 或者，直接使用这个 VSCode 插件：
    https://github.com/ritwickdey/vscode-live-server
    安装后编辑器的右下角状态栏会有一个 `Go Live` 按钮（启动了则显示端口）
    启动后会以整个编辑器打开的文件夹为根目录进行 serve。
- 或者，......

## 架构

既然是「游戏」，那么就按照经典的 渲染 + 逻辑 拆分的模式来做。
<s>既然是我写，那肯定要用点 Rust</s>

- `core`：游戏引擎本体，Rust 编写
    负责整个游戏的逻辑数据计算，通过编译为 wasm 在前端中使用
- `frontend`：前端目录

游戏的主循环写在 `js` 中，由于 `js` 是单线程的，所以只能通过“调度”来实现逻辑与渲染刻的步进：

```js
const mspt = 50
let lastTick = 0;

// time is the end time of the previous frame's rendering,
// decimal number, in milliseconds
function mainloop(time) {
  if (lastTime == 0) {
    lastTick = time;
  } else {
    if (time - lastTick > mspt) {
        lastTick = time;
        tick();
    }
  }
  render(time - lastTick);
  windows.requestAnimationFrame(mainloop);
}

windows.requestAnimationFrame(mainloop);
```



逻辑线程由 js 调用 `tick` 来更新 `GameState`。

渲染线程获取数据，并根据 `timeDelta` 插值绘制。

渲染无非就是两个信息：

- 画什么？
- 在哪画

而对于这个作业来说，图元种类只有一种 —— 图像，且资源数量并不多，因此直接在 Rust 中用一个枚举类型来实现。

## 功能

- [ ] 我方飞机使用鼠标或者键盘控制移动，<s>自动</s> 手动发射子弹，敌机小飞机随机出现，子弹击落敌机后得分，我方飞机遭遇敌方飞机撞击，视为失败，统计分数。（60分）
- [ ] 游戏可以暂停、继续、重新开始（5分）
- [x] 我方飞机和敌方飞机均有运行/中弹/损毁 动画（5分）
- [x] 三种飞机，不同宽度/速度/血量/得分（10分）
    小飞机1发子弹，速度3，得分100分；中飞机2发子弹，速度2，得分600分；大飞机8发子弹，速度1，得分10000分。（数值可自行更改，合理范围即可）
- [ ] 道具补给（10分）
    每隔30秒有一个随机的道具补给，分为两种道具，**全屏炸弹** 最多只能存放3枚，**双倍子弹** 可以维持18秒钟的效果; （数值可自行更改，合理范围即可）
- [x] 根据分数来逐步提高难度，难度的提高表现为飞机数量的增多以及速度的加快（5分）
- [x] 界面优美，动画流畅（5分）
- [ ] 排行榜功能（5分）
- [ ] 录像回放功能，将录制数据放在localstorage/websql上，以供变速回放（15分）