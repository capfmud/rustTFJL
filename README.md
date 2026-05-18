塔防精灵AI (TFJLai)
基于 Rust + egui 的塔防游戏辅助工具，从 C# WinForms 版本完整移植而来。
界面预览
```
┌──────────────────────────────────────────────────────────┐
│  [中文] [白天]                          user  用户       │  ← 工具栏
├────────────────────────────┬─────────────────────────────┤
│  ┌─ 信息面板 ────────────┐ │  AI模型: [豆包        ▼]  │
│  │ 关卡：[000]           │ │  ┌─────────────────────┐  │
│  │ ┌─ 火炮按钮 ────────┐│ │  │ URL: [___________] │  │
│  │ │火炮1 火炮2 ... 火炮7││ │  │                     │  │
│  │ │火炮8 火炮9 ... 火炮14││ │  │  WebView2 浏览器    │  │
│  │ │       VS            ││ │  │  (待集成 wry)       │  │
│  │ └────────────────────┘│ │  │                     │  │
│  │ 🎯 窗口句柄：[____]   │ │  └─────────────────────┘  │
│  │    窗口名称：[____]   │ │                             │
│  │    窗口类名：[____]   │ │                             │
│  └────────────────────────┘ │                             │
│  ┌─ 截图预览 ────────────┐ │                             │
│  │                       │ │                             │
│  │    (630 x 302)        │ │                             │
│  │                       │ │                             │
│  └────────────────────────┘ │                             │
│  [动态截图] [实时状态]  [提取图标]                          │
├────────────────────────────┴─────────────────────────────┤
│  就绪                                                    │  ← 状态栏
└──────────────────────────────────────────────────────────┘
```
功能模块
模块	说明
工具栏	语言切换（中文/ENGLISH）、主题切换（白天/夜晚）、用户信息
关卡信息	当前关卡编号显示与编辑
火炮按钮	2×7 网格布局，14个火炮选择按钮 + VS 标签
窗口查找	图标拖拽区域 + 窗口句柄/名称/类名输入
截图预览	游戏画面实时截图区域
操作按钮	动态截图、实时状态、提取图标
AI 模型选择	支持豆包、DeepSeek、千问、文心一言、智谱清言、讯飞星火、Kimi、腾讯混元
WebView 浏览器	内嵌浏览器区域（待集成 wry 库）
状态栏	底部操作状态提示
技术栈
语言: Rust (Edition 2021)
GUI 框架: egui 0.31 + eframe 0.31
原始版本: C# WinForms (.NET)
环境要求
Rust 1.70+
Windows: 需要 Visual Studio Build Tools（勾选"使用 C++ 的桌面开发"）
Linux: 需要 `libxkbcommon-x11`、`libxcb` 等系统库
快速开始
```bash
# 克隆项目
git clone https://github.com/capfmud/rustTFJL.git
cd rustTFJL

# 编译运行
cargo run --release
```
项目结构
```
rustTFJL/
├── Cargo.toml          # 项目配置与依赖
├── src/
│   └── main.rs         # 主程序（界面定义 + 应用逻辑）
└── README.md
```
C# → Rust 组件对照
C# WinForms	Rust / egui
`toolStrip1`	`TopBottomPanel::top` + `ui.horizontal`
`splitContainer1`	`SidePanel::left` + `CentralPanel`
`groupBox1` / `groupBox2`	`Frame::group`
`button1~button14`	`ui.button` 循环网格
`pictureBox1`	`allocate_exact_size` + 自定义绘制
`pictureBox2`	`Frame::canvas`
`textBox1/2/3`	`TextEdit::singleline`
`comboBox1`	`ComboBox::from_label`
`webView21`	占位区域（可集成 wry）
`statusStrip1`	`TopBottomPanel::bottom`
待完成功能
[ ] WebView2 浏览器集成（wry 库）
[ ] 动态截图功能
[ ] 实时状态监控
[ ] 图标提取功能
[ ] 窗口拖拽查找
[ ] 多语言国际化（i18n）
[ ] AI 模型 API 对接
License
MIT
