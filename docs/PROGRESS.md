# knock-knock 项目进度

## 项目状态：MVP 已完成

## 里程碑

### Phase 1 - MVP

目标：最短路径跑通 hook → 通知全链路。

| 任务 | 状态 | 说明 |
|------|------|------|
| 项目需求讨论 | ✅ 完成 | 确定核心方案：Rust + Hook + Fire-and-forget + Toast |
| 需求文档 | ✅ 完成 | docs/PRD.md |
| 初始化 Rust 项目 | ✅ 完成 | cargo init + clap + winrt-toast-reborn |
| 实现 Windows Toast 通知 | ✅ 完成 | 通过 winrt-toast-reborn 调用 WinRT API |
| CLI 参数解析 | ✅ 完成 | 支持 --title、--urgent、--source 参数 |
| 编写使用文档 | ✅ 完成 | README.md + examples/claude-code-hook.json |
| Claude Code Hook 集成验证 | ⬜ 待验证 | 需要在实际 Claude Code 中配置 hook 测试 |
| 端到端测试 | ⬜ 待验证 | 需从 Claude Code 暂停触发到弹出通知的完整链路 |

### Phase 2 - 体验优化（按需）

| 任务 | 状态 | 说明 |
|------|------|------|
| 通知分级（紧急/普通） | ✅ 完成 | --urgent 标志已实现 |
| 声音提示 | ✅ 完成 | urgent 场景自动带声音 |
| 点击跳转 | ⬜ 待开始 | 点击通知定位到对应终端 |
| 通知内容模板 | ⬜ 待开始 | 预设常见 agent 的通知格式 |

### Phase 3 - 扩展（远期）

| 任务 | 状态 | 说明 |
|------|------|------|
| Daemon 模式 | ⬜ 待开始 | 聚合、去重、节流 |
| 托盘图标 | ⬜ 待开始 | 显示等待数量 |
| 多通道通知 | ⬜ 待开始 | Telegram / Webhook |
| 跨平台支持 | ⬜ 待开始 | macOS / Linux |

## 技术细节

- 语言：Rust (edition 2024)
- 依赖：clap 4 + winrt-toast-reborn 0.3
- 使用 PowerShell AUMID 作为零配置方案
- Release 二进制大小：688KB
- 构建注意：需要 MSVC linker + Windows SDK (`.cargo/config.toml` 已 gitignore)

## 决策记录

| 日期 | 决策 | 理由 |
|------|------|------|
| 2026-05-12 | 仅支持 Hook-driven，不做 PTY 监听 | 精准可靠；不支持 hook 的 agent 不值得兼容 |
| 2026-05-12 | 技术栈选用 Rust | 单二进制、低占用、Windows API 亲和 |
| 2026-05-12 | Fire-and-forget，不做 Daemon | MVP 极简，后续按需升级，接口不变 |
| 2026-05-12 | Windows Toast 弹窗，不做托盘 | 满足核心需求，避免过度设计 |
| 2026-05-12 | 使用 winrt-toast-reborn + clap | 原生 WinRT 调用，无 PowerShell 开销 |
| 2026-05-12 | PowerShell AUMID 零配置 | 免注册即可弹通知，后续可加自定义注册 |

## 技术风险

| 风险 | 影响 | 状态 |
|------|------|------|
| Toast 通知需注册 AppID | 初次使用可能需额外配置 | ✅ 已解决：使用 PowerShell AUMID 免注册 |
| Rust WinRT 绑定成熟度 | 可能遇到 API 缺失 | ✅ 已验证：winrt-toast-reborn 工作正常 |
| MSVC linker PATH 冲突 | Git link.exe 优先级高于 MSVC | ✅ 已解决：.cargo/config.toml 指定路径 |
| Claude Code hook 能力边界不清 | 可能无法获取足够上下文 | ⬜ 待验证 |
