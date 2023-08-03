use sp_version::StateVersion;

pub(crate) fn hash(block_hash: &str) -> Result<String, String> {
    let (block_hash, offset) = if let Some(block_hash) = block_hash.strip_prefix("0x") {
        (block_hash, 2)
    } else {
        (block_hash, 0)
    };

    if let Some(pos) = block_hash.chars().position(|c| !c.is_ascii_hexdigit()) {
        Err(format!(
            "Expected block hash, found illegal hex character at position: {}",
            offset + pos,
        ))
    } else {
        Ok(block_hash.into())
    }
}

pub(crate) fn url(s: &str) -> Result<String, &'static str> {
    if s.starts_with("ws://") || s.starts_with("wss://") {
        // could use Url crate as well, but lets keep it simple for now.
        Ok(s.to_string())
    } else {
        Err("not a valid WS(S) url: must start with 'ws://' or 'wss://'")
    }
}

pub(crate) fn state_version(s: &str) -> Result<StateVersion, &'static str> {
    s.parse::<u8>()
        .map_err(|_| ())
        .and_then(StateVersion::try_from)
        .map_err(|_| "Invalid state version.")
}
