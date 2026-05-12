# knock-knock 产品需求文档

## 概述

knock-knock 是一个轻量级的终端 AI Agent 通知提醒工具。当后台运行的 AI agent（如 Claude Code CLI）暂停等待用户输入、任务完成或出错时，通过 Windows Toast 通知提醒用户。

## 背景与动机

### 问题场景

- 用户使用 tabby-terminal 等终端工具，同时开多个终端跑多个 AI agent
- AI agent 运行过程中常需要用户确认（如权限授权 y/n）才能继续
- 用户将任务放后台后容易遗忘，导致 agent 空等、时间浪费
- 目前只能手动轮询各终端查看状态

### 目标用户

使用终端工具运行多个 AI coding agent 的开发者。

### 竞品参考

Cursor：任务暂停/完成时触发 Windows 通知。但 Cursor 绑定自家 IDE，无法用于通用终端场景。

## 核心需求

### 功能需求

| 优先级 | 功能 | 描述 |
|--------|------|------|
| P0 | 接收通知消息 | CLI 接收标题、正文等参数 |
| P0 | 弹出 Windows Toast | 调用系统通知 API，在桌面右下角弹出消息卡片 |
| P0 | Claude Code Hook 集成 | 提供 hook 配置，使 Claude Code 在等待/完成时自动触发通知 |
| P1 | 通知分级 | 区分等待输入（紧急）和任务完成（普通）的通知样式 |
| P1 | 自定义通知内容 | 支持自定义标题、正文、图标 |
| P2 | 声音提示 | 紧急通知可带提示音 |
| P2 | 点击跳转 | 点击通知可定位到对应终端 |

### 非功能需求

- 单二进制文件，无外部依赖
- 启动到弹出通知 < 500ms
- 内存占用 < 5MB
- 支持 Windows 10/11

## 技术方案

### 架构

```
AI Agent (Claude Code等)
    │
    │ hook 触发
    ▼
knock-knock notify [options] "message"
    │
    │ 调用 Windows API
    ▼
Windows Toast Notification
    │
    │ 自动消失 / 用户交互
    ▼
通知中心（可事后查看）
```

### 技术选型

| 维度 | 决策 | 理由 |
|------|------|------|
| 语言 | Rust | 单二进制、低资源占用、Windows API 亲和 |
| 检测机制 | Hook-driven | 精准、零轮询、官方支持 |
| 进程模型 | Fire-and-forget | 极简、无需后台服务 |
| 通知 API | Windows Toast (WinRT) | 系统原生、功能完整 |

### CLI 接口设计（草案）

```bash
# 基本用法
knock-knock notify "Claude Code 等待你确认"

# 带标题
knock-knock notify --title "终端3" "Allow Bash: npm install?"

# 带优先级
knock-knock notify --urgent "需要你确认权限"

# 带来源标识
knock-knock notify --source "claude-code-1" --title "任务完成" "代码重构已完成"
```

### Claude Code Hook 配置示例

```json
{
  "hooks": {
    "notification": {
      "command": "knock-knock notify --title \"Claude Code\" \"$MESSAGE\""
    }
  }
}
```

## 明确不做的事

- 不做 PTY 通用监听（不支持 hook 的 agent 不兼容）
- 不做自动应答（只通知，不代替用户操作）
- 不做跨设备同步
- 不做终端内嵌 UI
- 不做 daemon 常驻服务（MVP 阶段）
- 不做托盘图标

## 后续扩展方向（非当前范围）

- Daemon 模式：通知聚合、去重、节流
- 托盘图标：显示当前等待数量
- 多通道：Telegram、Webhook、邮件
- 跨平台：macOS、Linux
- 更多 Agent 支持：Aider、Copilot CLI 等（需要它们支持 hook）
