# reqtui

A postman tui clone that is being built for me to detox from the AI hype.
Pure, handwritten code. Would an LLM write this faster and better, yeap most likely.
Do I care, absolutely not. 

<img width="1830" height="784" alt="demo" src="https://github.com/user-attachments/assets/55465cac-272e-4ddb-969b-0ca44ed8e420" />

## Features

- Send HTTP requests: **GET, POST, PUT, PATCH, DELETE**
- **Async non-blocking requests** with a 5-second default timeout
- **Session history** — browse and re-load previous requests
- **Collections** - save your requests between sessions
- **Load testing** TODO
    - **Basic** One request with result graphs
    - **Intermediate** Multiple requests concurrently with graphs
    - **Advanced** Dependecy support
- Import **Postman** collection TODO
- **Telescope** like search for requests TODO

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
| `c` | Open collections |
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
| `q` / `Esc` | Quit |

### Collection Overlay
| Key | Action |
|---|---|
| `↑` / `k` | Previous entry |
| `↓` / `j` | Next entry |
| `d` | Delete collection |
| `Enter` | Load selected collection |
| `Enter` | Add new collection |
| `Esc` | Return to main screen |
| `q` / `Esc` | Quit |

### Testing Overlay
| Key | Action |
|---|---|
| `Esc` | Return to main screen |
| `q` / `Esc` | Quit |

### Error Overlay
| Key | Action |
|---|---|
| `Esc` | Return to main screen |
| `q` / `Esc` | Quit |

