
# Test Runtimes

Some runtimes to use in tests.

## Generation Instructions

### No Migrations

Just pass an empty Migrations tuple to executive.

### Bad Spec Name

Set the `spec_name` to something other than what's in the on-chain runtime.

### Non-Incrementing Spec Version

Set the `spec_version` to less than or equal to the current on-chain runtime version.

### Not Idempotent Execution

Add a migration that is non-idempotent. E.g.

```rust
pub struct NonIdempotentExceptionMigration;

impl frame_support::traits::OnRuntimeUpgrade for NonIdempotentExceptionMigration {
	fn on_runtime_upgrade() -> Weight {
		let key = sp_core::blake2_128(b"some_random_seed");
		if frame_support::storage::unhashed::get(&key[..]).unwrap_or(false) {
			panic!("exception");
		};
		frame_support::storage::unhashed::put::<bool>(&key[..], &true);

		<Runtime as frame_system::Config>::DbWeight::get().writes(1)
	}
}
```

### Not Idempotent State Root

Add a migration that is non-idempotnent w.r.t the state root. E.g.

```rust
pub struct NonIdempotentStateRootMigration;

impl frame_support::traits::OnRuntimeUpgrade for NonIdempotentStateRootMigration {
	fn on_runtime_upgrade() -> Weight {
		let key = sp_core::blake2_128(b"some_random_seed");
		let cur = frame_support::storage::unhashed::get(&key[..]).unwrap_or(0);
		frame_support::storage::unhashed::put::<u32>(&key[..], &(cur + 1u32));
		<Runtime as frame_system::Config>::DbWeight::get().writes(1)
	}
}
```

### Weight Issue

Add a migration that is overweight. E.g.

```rust
pub struct OverweightMigration;

impl frame_support::traits::OnRuntimeUpgrade for OverweightMigration {
	fn on_runtime_upgrade() -> Weight {
		<Runtime as frame_system::Config>::BlockWeights::get().max_block
	}
}
```
