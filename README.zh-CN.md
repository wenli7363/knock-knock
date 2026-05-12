# knock-knock

终端 AI Agent 通知工具。后台 agent 需要你关注时，自动弹出桌面通知。

## 解决什么问题

用 tabby 等终端同时跑多个 AI agent（Claude Code、Aider 等），它们经常暂停等你确认——但你忘了它们在等你。

knock-knock 在 agent 暂停/完成/出错时弹出 Windows 通知，并且自动显示终端标题，让你一眼知道该切哪个 tab。

## 工作原理

```
AI Agent 暂停 → hook 触发 → knock-knock notify → 桌面右下角弹出通知
                                                   （标题 = 终端窗口名）
```

不轮询、不常驻。点击通知直接跳转回对应终端窗口。

## 安装

### 从源码构建

```bash
git clone https://github.com/wenli7363/knock-knock.git
cd knock-knock
cargo build --release
```

产物在 `target/release/knock-knock.exe`（~768KB），复制到 PATH 目录即可。

### 构建依赖

- Rust 工具链（stable，MSVC target）
- Windows SDK
- Visual Studio Build Tools（MSVC linker）

## 使用方法

```bash
# 基本用法 — 标题自动取终端窗口名
knock-knock notify "等待你的确认"

# 手动指定标题
knock-knock notify --title "我的项目" "Allow Bash: npm install?"

# 紧急通知 — 持久弹窗 + 声音
knock-knock notify --urgent "需要权限确认"

# 带来源标签
knock-knock notify --source "claude-code" "任务已完成"
```

### 自动终端标题

不传 `--title` 时，knock-knock 自动读取当前终端窗口标题。在 tabby 中每个 tab 有独立标题，通知会直接告诉你是哪个 tab 的 agent 在等你：

```
┌──────────────────────────────────────┐
│  claude-code: refactor-auth          │  ← 终端 tab 标题，自动作为通知标题
│                                      │
│  Allow Bash: npm install? (y/n)      │  ← 消息正文
└──────────────────────────────────────┘
```

### 点击跳转

点击通知时，knock-knock 会自动通过窗口标题匹配，把对应的终端窗口拉到前台。不用再在一堆 tab 里翻找了。

进程会短暂存活（最多 60 秒）等待点击，之后自动退出。

### 直接聚焦窗口

也可以直接把某个终端拉到前台：

```bash
knock-knock focus --title "claude: api-migration"
```

### 命令参考

```
knock-knock notify [OPTIONS] <MESSAGE>

参数:
  <MESSAGE>  通知正文

选项:
  -t, --title <TITLE>    通知标题（默认自动读取终端窗口标题）
  -u, --urgent           紧急通知（持久弹窗 + 声音提醒）
  -s, --source <SOURCE>  来源标签（显示为归属文本）
  -h, --help             显示帮助

knock-knock focus --title <TITLE>

选项:
  -t, --title <TITLE>    要查找的窗口标题
  -h, --help             显示帮助
```

## 集成 Claude Code

Claude Code 支持 [hooks](https://docs.anthropic.com/en/docs/claude-code/hooks) 机制，在生命周期事件时执行 shell 命令。在 `~/.claude/settings.json` 中添加：

```json
{
  "hooks": {
    "notification": [
      {
        "matcher": "",
        "hooks": [
          {
            "type": "command",
            "command": "knock-knock notify \"$CLAUDE_NOTIFICATION\""
          }
        ]
      }
    ]
  }
}
```

配好之后，Claude Code 每次暂停/完成/出错都会自动弹出通知，不用再手动轮询各终端了。

### 什么时候会触发通知

- Agent 等待权限确认（y/n）
- 任务完成
- Agent 遇到错误并停止

## 多 Agent 工作流

典型的 tabby 多 tab 场景：

```
Tab 1: "claude: auth-refactor"     → 跑 Claude Code 重构认证模块
Tab 2: "claude: api-migration"     → 跑 Claude Code 迁移 API 层
Tab 3: "claude: test-suite"        → 跑 Claude Code 写测试
```

当 Tab 2 的 agent 暂停等待输入时，桌面弹出：

```
┌──────────────────────────────────────┐
│  claude: api-migration               │
│                                      │
│  Allow edit: src/api/routes.rs?      │
└──────────────────────────────────────┘
```

点击通知 → Tab 2 的终端窗口直接弹到前台。

## 系统要求

- Windows 10/11
- 没了。单二进制文件，无运行时依赖。

## 路线图

- [x] Windows Toast 桌面通知
- [x] 自动识别终端窗口标题
- [x] 通知分级（紧急/普通）
- [x] 点击通知跳转到对应终端窗口
- [ ] 自定义应用标识（品牌化通知图标）
- [ ] Daemon 模式（聚合、去重、节流）
- [ ] 跨平台支持（macOS、Linux）

## 许可证

MIT
