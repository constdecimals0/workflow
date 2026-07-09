# Ratatui idioms for a small tick-driven game

> Resolves wayfinder ticket #2 (constdecimals0/workflow) — researched 2026-07-09 against primary sources only.

## 1. Crate versions

- **ratatui 0.30.2** is the current stable release (published 2026-06-19; 0.30.1 on 2026-06-05, 0.30.0 on 2025-12-26). It is `max_stable_version` = `max_version` on [crates.io/crates/ratatui](https://crates.io/crates/ratatui) — there is no 0.31/1.0 pre-release line beyond it as of 2026-07-09. The [v0.30 highlights](https://ratatui.rs/highlights/v030/) say new apps should use the standard `ratatui` crate (the 0.30 workspace split into `ratatui-core`/`ratatui-widgets`/`ratatui-crossterm` only matters to widget-library authors).
- **MSRV**: ratatui 0.30.2 declares `rust-version = 1.88.0`, edition 2024 ([crates.io version metadata](https://crates.io/api/v1/crates/ratatui/0.30.2); 0.30.0 was 1.86 per the [highlights](https://ratatui.rs/highlights/v030/)).
- **crossterm 0.29.0** is current stable (published 2025-04-05, MSRV 1.63) per [crates.io/crates/crossterm](https://crates.io/crates/crossterm), and is what ratatui uses by default: ratatui's default `crossterm` feature pulls in [`ratatui-crossterm`](https://crates.io/crates/ratatui-crossterm), which offers `crossterm_0_28`/`crossterm_0_29` feature flags with "the latest version enabled by default" ([0.30.0 release notes, PR #1841](https://github.com/ratatui/ratatui/releases/tag/ratatui-v0.30.0)).
- **Re-export**: yes — `ratatui::crossterm` exists (`pub use ratatui_crossterm::crossterm` on [docs.rs/ratatui 0.30.2](https://docs.rs/ratatui/0.30.2/ratatui/)). The [`ratatui-crossterm` docs](https://docs.rs/ratatui-crossterm/latest/ratatui_crossterm/) say using the re-export is how you "avoid version conflicts and ensure that all parts of the application use a consistent set of Crossterm types". The [official templates](https://github.com/ratatui/templates) instead pin a matching direct dep (`ratatui = "0.30.2"` + `crossterm = "0.29.0"`, e.g. [event-driven Cargo.toml](https://github.com/ratatui/templates/blob/main/event-driven-generated/Cargo.toml)) — both are current practice; the re-export can't drift.

## 2. Event-loop shape

Ratatui owns rendering only; "the user handles the event loop … and re-draws the entire UI on each iteration" ([FAQ](https://ratatui.rs/faq/)). Three loop shapes appear in first-party material:

1. **Blocking `event::read()`** — redraw only on input. Used by the [simple template](https://github.com/ratatui/templates/blob/main/simple-generated/src/main.rs) and the [counter-app tutorial](https://ratatui.rs/tutorials/counter-app/). No good for animation; the simple template's own doc comment says to switch to `event::poll` with a timeout "if your application needs to perform work in between handling events".
2. **Tick rate + `poll` with remaining timeout** (the classic, single-threaded): each iteration draw, then `event::poll(tick_rate.saturating_sub(last_tick.elapsed()))`; if it times out, fire a tick. Documented in the official [demo example `run_app`](https://github.com/ratatui/ratatui/blob/main/examples/apps/demo/src/crossterm.rs) and, in a threaded variant (event thread emitting `Event::Tick` / `Event::Crossterm` over `std::sync::mpsc` at `TICK_FPS = 30.0`), in the [event-driven template `event.rs`](https://github.com/ratatui/templates/blob/main/event-driven-generated/src/event.rs). The [colors-rgb example](https://github.com/ratatui/ratatui/blob/main/examples/apps/colors-rgb/src/main.rs) is the degenerate form: `poll(1/60 s)` every frame so it renders at ~60 FPS whether or not input arrives.
3. **Async event stream** — `tokio::select!` over `crossterm::event::EventStream` plus a `tokio::time::interval` tick, in the [event-driven-async template](https://github.com/ratatui/templates/blob/main/event-driven-async-generated/src/event.rs). Only worth the tokio dependency if the app already needs async I/O.

For architecture (not timing), [ratatui.rs concepts/event-handling](https://ratatui.rs/concepts/event-handling/) recommends centralized event handling with pattern matching for small apps. For terminal setup, 0.30's [`ratatui::run(|terminal| …)`](https://docs.rs/ratatui/latest/ratatui/fn.run.html) wraps `init()`/`restore()` and is positioned for "simple applications that need a terminal with reasonable defaults for the entire lifetime of the application". Match only `KeyEventKind::Press` (release/repeat events also arrive on some platforms — noted in the [simple template](https://github.com/ratatui/templates/blob/main/simple-generated/src/main.rs)).

## 3. Non-blocking animation timing

The first-party idiom is **never sleep; keep time in the model and let the tick/frame drive it**:

- The templates' `Event::Tick` doc comment states its purpose verbatim: "use this event to run any code which has to run outside of being a direct response to a user event, e.g. … updating animations, or rendering the UI based on a fixed frame rate" ([event-driven template](https://github.com/ratatui/templates/blob/main/event-driven-generated/src/event.rs)).
- The [demo example](https://github.com/ratatui/ratatui/blob/main/examples/apps/demo/src/crossterm.rs) advances all animation state in `app.on_tick()`, called only when `poll` times out — input handling is never blocked because `poll`'s timeout is the time remaining until the next tick.
- The [colors-rgb example](https://github.com/ratatui/ratatui/blob/main/examples/apps/colors-rgb/src/main.rs) keeps `Instant`/`frame_count` fields in its widgets and derives the animation from them each render.

For a short flash (Simon pad lit ~400 ms), the pattern is deadline state, not duration sleeps: store e.g. `lit_until: Option<Instant>` (or `flash_started: Instant` + const duration) in the app model when the flash starts; on every tick compare against `Instant::now()`, clear the flash / advance to the next sequence step when the deadline passes; render reads only the model. A 20–30 Hz tick (the templates default to 30 FPS) gives ±33–50 ms resolution on a 400 ms flash, which is imperceptible.

## 4. Examples worth mirroring

Official (ratatui org):

- [demo example](https://github.com/ratatui/ratatui/tree/main/examples/apps/demo) — the canonical single-threaded `tick_rate` + `poll(remaining)` loop with `on_tick()` animation; smallest correct shape for a timed game.
- [colors-rgb example](https://github.com/ratatui/ratatui/blob/main/examples/apps/colors-rgb/src/main.rs) — `ratatui::run` entry point, app-as-widget rendering, fixed-frame animation; the most modern (0.30-era) example style.
- [event-driven template](https://github.com/ratatui/templates) (`cargo generate ratatui/templates`) — first-party scaffold with an `Event::{Tick, Crossterm, App}` enum over an mpsc channel; the structure to copy if we want named app events (e.g. `AppEvent::PadPressed`). The async twin shows the tokio variant.
- [counter-app tutorial](https://ratatui.rs/tutorials/counter-app/) — the official minimal app-struct/`run`/`handle_events` skeleton (blocking read; upgrade its input loop to poll+tick).

Community games (from the first-party [awesome-ratatui](https://github.com/ratatui/awesome-ratatui) list, maintained under the ratatui org; loop styles verified in source):

- [ratatui-snake](https://github.com/kriskw1999/ratatui-snake) — tiny snake game whose whole loop is draw + `event::poll(16 ms)` + time-based movement; closest existing thing to the Simon game's scale.
- [sxtetris](https://github.com/shixinhuang99/sxtetris) — polished terminal Tetris on tokio `EventStream` with `Tick`/`Gravity`/`Blink` events ([handler.rs](https://github.com/shixinhuang99/sxtetris/blob/main/src/handler.rs)) — its `Blink` event is directly analogous to a Simon pad flash, but it shows the async shape is heavier than we need.

## Recommendation for the Simon game

- **Pin**: `ratatui = "0.30.2"` (default features — that already selects crossterm 0.29 via `ratatui-crossterm`), MSRV/toolchain ≥ 1.88, edition 2024. Don't add a direct `crossterm` dependency; import event types through the [`ratatui::crossterm` re-export](https://docs.rs/ratatui/0.30.2/ratatui/) so the version can never mismatch.
- **Loop**: single-threaded classic loop, as in the [demo example](https://github.com/ratatui/ratatui/blob/main/examples/apps/demo/src/crossterm.rs) — `ratatui::run(|terminal| app.run(terminal))`; each iteration `terminal.draw(...)`, then `event::poll(tick_rate.saturating_sub(last_tick.elapsed()))`; on timeout call `app.on_tick()`. Tick rate 30 Hz (~33 ms). No threads, no tokio.
- **Flash timing**: model sequence playback as a state machine in the app model — e.g. `Phase::Playback { step: usize, lit_until: Instant }` — set `lit_until = Instant::now() + FLASH_MS` when a pad lights; `on_tick()` checks `Instant::now() >= lit_until` to unlight/advance (plus a short gap deadline between steps). Input stays live throughout because nothing ever sleeps.
- **Mirror**: the demo example for the loop, colors-rgb for `ratatui::run` + app-as-widget structure, ratatui-snake as proof of the whole shape at mini-game scale.
