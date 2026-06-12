# reqtui

A postman tui clone that is being built for me to detox from the AI hype.
Pure, handwritten code. Would an LLM write this faster and better, yeap most likely.
Do I care, absolutely not.

<img width="1830" height="627" alt="demo" src="https://github.com/user-attachments/assets/a1835c8d-0ca1-4c68-8063-17519840b252" />

## Features

- Send HTTP requests: **GET, POST, PUT, PATCH, DELETE**
- **Async non-blocking requests** with a 4-second timeout
- **Session history** — browse and re-load previous requests

## Build & Run

```bash
# Run (debug)
cargo run

# Build release binary
cargo build --release
./target/release/reqtui
```

No config files, no environment variables, no arguments needed.

## Keybindings

### Main Screen
| Key | Action |
|---|---|
| `e` | Open request editor |
| `h` | Open history |
| `Enter` | Send request |
| `Tab` | Cycle HTTP method |
| `p` | Toggle HTTP / HTTPS |
| `↑` / `k` | Scroll response up |
| `↓` / `j` | Scroll response down |
| `q` / `Esc` | Quit |

### Editing Overlay
| Key | Action |
|---|---|
| `Tab` | Switch focus between URL and Body |
| `Esc` / `Enter` | Return to main screen |

### History Overlay
| Key | Action |
|---|---|
| `↑` / `k` | Previous entry |
| `↓` / `j` | Next entry |
| `Enter` | Load selected request |
| `Esc` | Return to main screen |
