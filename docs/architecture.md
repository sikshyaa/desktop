# Architecture

Sikshyaa is an offline-first desktop application built with Tauri, SvelteKit, Rust, and SurrealDB.

The current architecture is intentionally simple:

```text
SvelteKit frontend
        |
        v
Tauri commands and startup integration (`src-tauri/src`)
        |
        v
SikshyaaApp (`src-tauri/core/src/app.rs`)
        |
        v
SurrealDB
```

## Responsibilities

### `src-tauri/src` — presentation and framework integration

This is the Tauri-facing layer. It owns:

- Tauri commands
- Application startup
- Managed Tauri state
- Filesystem and application paths
- Plugins and window configuration
- Logging initialization
- Converting application results into frontend responses

Tauri handlers should remain thin. They should receive frontend input, call `SikshyaaApp`, and return a frontend-safe result.

### `src-tauri/core` — application and domain code

The core crate contains Sikshyaa-specific behavior and types. It should not depend on Tauri or frontend code.

It currently contains:

- `SikshyaaApp`, the application façade
- Domain models such as `Video` and `Source`
- Domain/application errors
- Use-case methods such as `create_video`
- Core tests using the in-memory SurrealDB engine

At the moment, `SikshyaaApp` owns the SurrealDB connection directly. This is the architecture currently implemented—not a repository abstraction yet.

### `core/src/models` — domain models

Models describe Sikshyaa concepts and their data:

```rust
pub struct Video {
    pub grade: String,
    pub subject: String,
    pub topic: String,
    pub sub_topic: String,
    pub teacher_name: Option<String>,
    pub source: Option<String>,
}
```

Models should primarily contain data and model-level invariants. Operations that coordinate persistence or other services belong on `SikshyaaApp` or a future application service, rather than directly inside `models::video`.

## Startup and database lifecycle

The Tauri `run` function initializes logging and creates the application state during the `setup` hook:

1. Load local `.env` values.
2. Initialize the `tracing` subscriber.
3. Resolve Tauri's platform-specific application data directory.
4. Create the directory if necessary.
5. Open the file-backed SurrealDB database at `sikshyaa.db`.
6. Register `SikshyaaApp` with `app.manage(...)`.
7. Start the Tauri event loop.

```rust
.setup(|app| {
    let app_data_dir = app.path().app_data_dir()?;
    std::fs::create_dir_all(&app_data_dir)?;

    let database_path = app_data_dir.join("sikshyaa.db");
    let sikshyaa_app = tauri::async_runtime::block_on(
        SikshyaaApp::with_file_surreal(&database_path),
    )?;

    app.manage(sikshyaa_app);
    Ok(())
})
```

Initialization happens before the application accepts commands, so handlers receive a ready `SikshyaaApp`.

## Accessing application state from a handler

Once a command is added, Tauri injects the managed application state using `tauri::State`:

```rust
#[tauri::command]
async fn some_command(
    app: tauri::State<'_, SikshyaaApp>,
) -> Result<(), String> {
    tracing::debug!("some_command invoked");
    // Call an application method through `app`.
    Ok(())
}
```

The command should not create another database connection. It should use the already initialized state.

## Logging

Logging is initialized once before the Tauri builder using `tracing` and `tracing-subscriber`. Because the subscriber is process-global, handlers and core methods can log directly:

```rust
tracing::debug!(subject = %video.subject, "creating video");
tracing::info!("video created");
```

For local development:

```bash
cp .env.example .env
pnpm tauri dev
```

The example environment contains:

```env
RUST_LOG=debug
```

Logs are printed in the terminal that runs `pnpm tauri dev`. `RUST_LOG` can be changed to reduce noise, for example:

```env
RUST_LOG=desktop_lib=debug,sikshyaa_core=debug,surrealdb=info
```

## Error handling

Fallible operations return `Result` rather than panicking. For example, database creation returns an `Option<Video>` because the database may not return a record. The application converts the missing value into a domain error:

```rust
let created: Option<Video> = self.db.create("video").content(video).await?;
let created = created.ok_or(SikshyaaError::VideoNotCreated)?;
Ok(created)
```

This keeps failure visible to the caller and avoids `unwrap()` in application code.

## Testing

Core tests can initialize an isolated in-memory database:

```rust
let app = SikshyaaApp::with_memory_surreal().await?;
let created = app.create_video(video).await?;
```

The existing `create_video` test verifies that a video can be created and returned without touching the user's persistent database.

## Where should new code go?

| Code | Location |
|---|---|
| Tauri command | `src-tauri/src` |
| Tauri state/startup/plugin setup | `src-tauri/src` |
| Domain/application error | `core/src/error.rs` |
| Domain model | `core/src/models` |
| Application use case | `core/src/app.rs` |
| SurrealDB initialization | `core/src/app.rs`, called by Tauri startup |
| Frontend-specific DTO or display formatting | frontend `src` |
| Database-specific query logic | currently `core/src/app.rs` |

## FAQ

### Where do models go?

Put Sikshyaa concepts such as `Video` and `Source` in `core/src/models`. Keep frontend-only display types in the frontend and Tauri transport types in `src-tauri/src`.

### Where does application-specific knowledge go?

Rules about Sikshyaa behavior belong in `core`. Tauri-specific knowledge—commands, paths, windows, plugins, and lifecycle hooks—belongs in `src-tauri/src`.

### Should I put `create_video` in `models::video`?

Not currently. `Video` is a domain value, while creating a video is an application operation involving persistence. `SikshyaaApp::create_video` is the current appropriate location.

### Is there a repository pattern yet?

No. The current implementation uses SurrealDB directly inside `SikshyaaApp`. A repository abstraction may be introduced later if the project needs alternate storage, more isolated unit tests, or a larger persistence layer. It is deliberately not part of the current architecture.

### Why is the core a separate crate?

It prevents domain/application code from becoming coupled to Tauri and makes the core testable independently. It also leaves room for another interface, such as a CLI, in the future.
