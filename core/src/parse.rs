use sp_runtime::StateVersion;

pub(crate) fn parse_hash(block_hash: &str) -> Result<String, String> {
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
        Ok(block_hash.to_string())
    }
}

pub(crate) fn parse_url(s: &str) -> Result<String, String> {
    if s.starts_with("ws://") || s.starts_with("wss://") {
        Ok(s.to_string())
    } else {
        Err("not a valid WS(S) url: must start with 'ws://' or 'wss://'".to_string())
    }
}

pub(crate) fn parse_state_version(s: &str) -> Result<StateVersion, String> {
    s.parse::<u8>()
        .map_err(|_| ())
        .and_then(StateVersion::try_from)
        .map_err(|_| "Invalid state version.".to_string())
}
