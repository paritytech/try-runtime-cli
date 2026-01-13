// This file is part of try-runtime-cli.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
    if s.starts_with("ws://")
        || s.starts_with("wss://")
        || s.starts_with("http://")
        || s.starts_with("https://")
    {
        // could use Url crate as well, but lets keep it simple for now.
        Ok(s.to_string())
    } else {
        Err("not a valid URL: must start with 'ws://', 'wss://', 'http://' or 'https://'")
    }
}

/// Convert an input URI into a WS URI suitable for `ws_client`.
///
/// If the input is HTTP(S), it will be converted to WS(S). Otherwise returned unchanged.
pub(crate) fn to_ws_uri(uri: &str) -> String {
    if let Some(rest) = uri.strip_prefix("http://") {
        return format!("ws://{rest}");
    }
    if let Some(rest) = uri.strip_prefix("https://") {
        return format!("wss://{rest}");
    }
    uri.to_string()
}

/// Convert an input URI into an HTTP URI suitable for `frame_remote_externalities` online
/// transport.
///
/// If the input is WS(S), it will be converted to HTTP(S). Otherwise returned unchanged.
pub(crate) fn to_http_uri(uri: &str) -> String {
    if let Some(rest) = uri.strip_prefix("ws://") {
        return format!("http://{rest}");
    }
    if let Some(rest) = uri.strip_prefix("wss://") {
        return format!("https://{rest}");
    }
    uri.to_string()
}

pub(crate) fn state_version(s: &str) -> Result<StateVersion, &'static str> {
    s.parse::<u8>()
        .map_err(|_| ())
        .and_then(StateVersion::try_from)
        .map_err(|_| "Invalid state version.")
}
