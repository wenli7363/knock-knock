# knock-knock

[中文文档](README.zh-CN.md)

Terminal AI agent notification tool. Get notified when your background agents need attention.

## Problem

Running multiple AI agents (Claude Code, etc.) across terminal tabs? They often pause waiting for your input — and you forget they're waiting.

knock-knock pops a Windows notification so you never miss them. The notification title automatically shows the terminal window name, so you know exactly which tab to switch to.

## How it works

```
AI Agent pauses → hook triggers → knock-knock notify → Windows Toast pops up
                                                        (with terminal title)
```

No polling. No daemon. Click the notification to jump straight back to that terminal.

## Install

### From source

```bash
git clone https://github.com/user/knock-knock.git
cd knock-knock
cargo build --release
```

The binary is at `target/release/knock-knock.exe` (~768KB). Copy it somewhere in your PATH.

### Build requirements

- Rust toolchain (stable, MSVC target)
- Windows SDK
- Visual Studio Build Tools (MSVC linker)

## Usage

```bash
# Basic — title auto-detected from terminal window name
knock-knock notify "Waiting for your confirmation"

# Explicit title
knock-knock notify --title "my-project" "Allow Bash: npm install?"

# Urgent — persistent toast with sound
knock-knock notify --urgent "Permission confirmation needed"

# With source label
knock-knock notify --source "claude-code" "Task completed"
```

### Auto terminal title

When you don't pass `--title`, knock-knock reads the current terminal window title automatically. In tabby, each tab has its own title — so the notification tells you exactly which tab needs attention.

```
┌──────────────────────────────────────┐
│  claude-code: refactor-auth          │  ← terminal tab title as notification title
│                                      │
│  Allow Bash: npm install? (y/n)      │  ← message body
└──────────────────────────────────────┘
```

### Click to focus

When you click the notification, knock-knock brings the terminal window to the foreground automatically — matching the window by its title. No more hunting through tabs.

The process stays alive briefly (up to 60s) to handle the click, then exits.

### Focus command

You can also bring a terminal to the foreground directly:

```bash
knock-knock focus --title "claude: api-migration"
```

### CLI reference

```
knock-knock notify [OPTIONS] <MESSAGE>

Arguments:
  <MESSAGE>  Notification body message

Options:
  -t, --title <TITLE>    Notification title (defaults to terminal window title)
  -u, --urgent           Mark as urgent (persistent toast with sound)
  -s, --source <SOURCE>  Source label shown as attribution text
  -h, --help             Print help

knock-knock focus --title <TITLE>

Options:
  -t, --title <TITLE>    Window title to search for
  -h, --help             Print help
```

## Claude Code Integration

Claude Code supports [hooks](https://docs.anthropic.com/en/docs/claude-code/hooks) that run shell commands on lifecycle events. Add to your `~/.claude/settings.json`:

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

Now whenever Claude Code pauses for input or finishes a task, you get a desktop notification with your terminal tab title — no need to keep checking manually.

### What triggers notifications

- Agent waiting for permission approval (y/n)
- Task completed
- Agent encountered an error and stopped

## Multi-agent workflow

Typical setup with tabby terminal:

```
Tab 1: "claude: auth-refactor"     → runs Claude Code on auth module
Tab 2: "claude: api-migration"     → runs Claude Code on API layer
Tab 3: "claude: test-suite"        → runs Claude Code on tests
```

When Tab 2 pauses for input, you get:

```
┌──────────────────────────────────────┐
│  claude: api-migration               │
│                                      │
│  Allow edit: src/api/routes.rs?      │
└──────────────────────────────────────┘
```

Click the notification → Tab 2 pops to the foreground.

## Requirements

- Windows 10/11
- That's it. Single binary, no runtime dependencies.

## Roadmap

- [x] Windows Toast notifications
- [x] Auto-detect terminal window title
- [x] Urgent/normal notification levels
- [x] Click-to-focus (bring terminal window to foreground)
- [ ] Custom AUMID registration (branded notifications)
- [ ] Daemon mode (aggregation, deduplication, throttling)
- [ ] Cross-platform (macOS, Linux)

## License

MIT
