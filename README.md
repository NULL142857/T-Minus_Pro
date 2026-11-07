# T-Minus

极简倒计时。一个窗口 = 一份任务 = `tasks` 目录里的一个 json。

许可：GPL-3.0（见 `LICENSE.txt`，安装包第 2 页显示的就是它）。

## 目录结构

```
T-Minus/
├─ package.json          构建脚本
├─ LICENSE.txt           许可协议（安装包第 2 页显示它）
├─ src/index.html        界面（从网页版搬过来，暂时未接文件读写）
└─ src-tauri/
   ├─ Cargo.toml         Rust 工程
   ├─ build.rs
   ├─ tauri.conf.json    窗口、图标、安装包配置
   ├─ capabilities/      窗口权限
   ├─ icons/             32/128/256 与 ico
   └─ src/main.rs        tasks 目录的读写命令
```

## 一次性准备

1. **Rust**：<https://rustup.rs> → 下载 `rustup-init.exe` → 一路回车（默认安装 stable-msvc）
2. **Visual Studio Build Tools**：<https://visualstudio.microsoft.com/visual-cpp-build-tools/>
   安装时勾选 **「使用 C++ 的桌面开发」**，其余默认
3. **Node.js**：已具备

装完在终端里验证：

```
rustc -V
cargo -V
node -v
```

三个都能出版本号即可。

## 构建

```bat
cd T-Minus
npm install
npm run build
```

第一次构建会下载 Rust 依赖、NSIS 工具链，大约 10–20 分钟（取决于网速）。之后增量构建大约 1–2 分钟。

产物：

```
src-tauri/target/release/bundle/nsis/T-Minus_0.1.0_x64-setup.exe
```

## 开发模式（改界面时用）

```bat
npm run dev
```

改 `src/index.html` 保存后窗口会自动刷新。

## 当前进度

- [x] 项目骨架、图标、安装包配置、许可协议页
- [x] `tasks` 目录自动创建与读写命令
- [x] 界面接上文件读写（保存覆盖、原子写入、标题改名）
- [x] 任务列表 / 新建 / 右键重命名 / 复制 / 删除 / 打开配置文件夹
- [x] 导入 = 复制进 tasks
- [x] 背景随时间变红 开关（默认关）
- [x] 进度条改进度、空心化
- [x] 环心预备期两行
- [x] 界面缩放 4 档（全局）
- [x] 设置抽屉动效 + 主题切换淡入上浮
- [x] 全屏开关 + F11

## 自定义 NSIS 模板

`src-tauri/nsis/installer.nsi` 是从 `tauri-cli-2.12.4` 的官方模板 fork 来的，改了三处（都带 `T-Minus:` 注释）：

1. 去掉「重新安装 / 卸载」维护页，已安装时直接覆盖
2. 安装前 `taskkill` 结束占用中的实例，失败则明确提示
3. 不把安装语言写进注册表，每次安装都显示语言选择

升级 Tauri 后需要重新取一份官方模板，把这三处改动再打一遍。

## 注意

安装包目前只配了「简体中文 + English」两种安装界面语言。但是软件内部支持 11 种语言。
