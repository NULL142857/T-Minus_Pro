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

## 从零构建（克隆后照这个顺序走）

```bat
git clone https://github.com/NULL142857/T-Minus_Pro.git
cd T-Minus_Pro

npm install
npm run build
```

第一次构建会下载 Rust 依赖和 NSIS 工具链，约 10–20 分钟（取决于网速）；
之后增量构建约 1–2 分钟。

**如果是全新环境，第一次构建完请再跑一次 `npm run build`。**
原因见下一节——NSIS 工具链是第一次构建时才下载的，而字体补丁在下载前跑不到，
第一次的安装包韩语界面可能是方块，第二次构建即修正。

产物：

```
src-tauri/target/release/bundle/nsis/T-Minus_x.y.z_x64-setup.exe
```

## 韩语字体与预构建补丁

NSIS 的韩语语言文件（`Contrib\Language files\Korean.nlf`）把界面字体写死成 **Gulim（굴림）**。
这个字体只有韩文版 Windows 才有，中文/英文系统上缺失，字体回退后的默认字体没有韩文字形，
于是韩语界面的正文和按钮全部变成方块（标题行反而正常，因为标题用的是另一个字体通道）。

修法：仓库里存了一份改好字体的 `src-tauri/nsis/Korean.nlf`（`굴림` → `맑은 고딕` / Malgun Gothic），
构建时由 `scripts/patch-nsis.mjs` 覆盖到本机工具链：

```jsonc
// src-tauri/tauri.conf.json
"build": {
  "frontendDist": "../src",
  "beforeBundleCommand": "node scripts/patch-nsis.mjs"
}
```

几点说明：

- 补丁改的是 **本机工具链**（`%LOCALAPPDATA%\tauri\NSIS`），不是仓库内容，所以每次构建都会重跑一遍，幂等。
- 脚本按自身位置解析路径，不依赖当前工作目录。
- 找不到 NSIS 目录时只打印「跳过」并正常退出，不会让构建失败——这就是上一节「再跑一次」的由来。
- 想把安装包发给别人，只需要发 exe；字体已经烤进安装包里了。

已实测的语言：简中、繁中、英、法、日、韩、俄、西、阿（RTL 自动镜像）、印地、印尼，均显示正常。

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
