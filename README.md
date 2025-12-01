# @iracing-data/broadcast-client

Node-API bindings for [iracing-broadcast](https://crates.io/crates/iracing-broadcast), providing a convenient JavaScript interface for driving the iRacing spectator and camera broadcast API from Node.js. The package ships prebuilt Windows binaries for the supported CPU architectures so you can control an iRacing session without compiling Rust yourself.

## Installation

```bash
# npm
yarn add @iracing-data/broadcast-client
# or
npm install @iracing-data/broadcast-client
```

> **Note:** iRacing exposes its broadcast API on Windows. The prebuilt artifacts published with this package target `x86_64`, `ia32`, and `arm64` Windows systems.

## Usage

Instantiate a `BroadcastClient` and call the helper methods that wrap the iRacing broadcast protocol. All enums are exported from the package for type-safe usage in TypeScript.

```ts
import {
  BroadcastClient,
  ReplaySearchMode,
  PitCommandMode,
  TelemetryCommandMode,
  VideoCaptureMode,
} from '@iracing-data/broadcast-client'

const client = new BroadcastClient()

// Camera and replay control
client.sendCameraSwitchNumber('12', 0, 0)
client.setReplayPlaySpeed(1, false)
client.searchReplay(ReplaySearchMode.NextIncident)

// Pit and chat commands
client.sendPitCommand(PitCommandMode.Fuel, 60)
client.sendTelemetryCommand(TelemetryCommandMode.Restart)

// Capture a quick screenshot
client.controlVideoCapture(VideoCaptureMode.ScreenShot)
```

Available methods include:

- Camera control (`sendCameraSwitchPosition`, `sendCameraSwitchNumber`, `setCameraState`)
- Replay control (`setReplayPlaySpeed`, `setReplayPlayPosition`, `searchReplay`, `toggleReplayState`)
- Texture reloads (`reloadAllTextures`, `reloadTextures`)
- Chat commands (`sendChatCommand`, `sendChatMacro`)
- Pit and telemetry commands (`sendPitCommand`, `sendTelemetryCommand`, `sendForceFeedbackCommand`)
- Session search and video capture helpers (`searchReplaySessionTime`, `controlVideoCapture`)

See the [TypeScript definitions](./index.d.ts) for the full API surface.

## Local development

The repository uses Yarn 4 and ships a set of scripts to help with maintenance:

- `yarn build` – builds platform-specific native artifacts via `@napi-rs/cli`.
- `yarn test` – runs the AVA test suite against the built bindings.
- `yarn lint` – runs oxlint on the JavaScript/TypeScript sources.
- `yarn format` – formats JavaScript/TypeScript, Rust, and TOML files.
- `yarn bench` – optional micro-benchmarks with tinybench.

Rust (with `cargo`) and Node.js are required for local development. The workspace uses the `napi` toolchain to generate bindings; see [`package.json`](./package.json) for the full script list.

## Continuous integration

GitHub Actions is configured in [`.github/workflows/CI.yml`](./.github/workflows/CI.yml) to ensure the bindings stay healthy:

- **Lint job (Windows):** runs Yarn install, oxlint, `cargo fmt --check`, and `cargo clippy` to validate code quality.
- **Build matrix (Windows targets):** cross-compiles release artifacts for `x86_64`, `ia32`, and `arm64` Windows using the `@napi-rs/cli` build commands and uploads the resulting `.node`/`.wasm` files as artifacts.
- **Binding tests:** downloads the built artifacts, installs dependencies, and runs `yarn test` on both `node@20` and `node@22`.
- **Publish:** after successful lint and test jobs, the workflow collects artifacts and publishes to npm when the latest commit message resembles a version tag (with `NPM_TOKEN` provided in repository secrets). This job also enables npm provenance.

If you add new targets or change the package name, remember to update the `APP_NAME` environment variable and the build matrix in `CI.yml`.

## Releasing

To cut a release:

1. Bump the version with `npm version [major | minor | patch]` (or a specific semver).
2. Push the version commit to `main`.
3. Ensure `NPM_TOKEN` is defined in the repository secrets; the publish job will pick up version-like commit messages and publish the platform packages automatically.

Publishing directly with `npm publish` is intentionally disabled—let CI handle it to ensure binaries are bundled for each platform.
