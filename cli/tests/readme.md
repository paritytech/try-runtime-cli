
# tests

## ./runtimes and ./snaps

A state snapshot is included in ./snaps, and some runtimes in ./runtimes for use in tests.

- `bridge_hub_rococo_runtime_OK.compact.compressed.wasm` a runtime with correctly configured migrations
- `bridge_hub_rococo_runtime_WEIGHT_ISSUE.compact.compressed.wasm` a runtime with migrations that would exceed sensible values for a parachain
- `bridge_hub_rococo_runtime_NOT_IDEMPOTENT_EXECUTION.compact.compressed.wasm` a runtime where `try_on_runtime_upgrade` if migrations are executed for a second time
- `bridge_hub_rococo_runtime_NOT_IDEMPOTENT_STATE_ROOT.compact.compressed.wasm` a runtime which will succeed when migrations are executed for a second time, but the state changes are not idempotent
