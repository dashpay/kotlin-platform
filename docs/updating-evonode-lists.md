# Updating the Evo node lists

`platform-mobile` ships two Evo node lists per network. They have different jobs:

| Network | Candidate list (every valid Evo node) | Shipped list (nodes that passed the check) |
|---|---|---|
| mainnet | `MAINNET_ADDRESS_LIST` in `platform-mobile/src/fetch_document.rs` | `MAINNET_ADDRESS_LIST` in `platform-mobile/src/config.rs` |
| testnet | `ADDRESS_LIST` in `platform-mobile/src/fetch_document.rs` | `TESTNET_ADDRESS_LIST` in `platform-mobile/src/config.rs` |

The candidate list feeds the `check_all_nodes_mainnet_test` / `check_all_testnet_nodes_test`
tests in `fetch_document.rs`. Each test sends a proof-verified DPNS query to every node, one at a
time. The nodes that answer correctly become the shipped list in `config.rs`, which is what the
SDK and the apps use.

Refresh both lists before a release, or whenever many nodes fail in the field.

## Prerequisites

- A local Dash Core (Dash-Qt or dashd) for the network, **fully synced**, with the RPC server on
  (`server=1`). The tests fetch quorum public keys from it to verify proofs, so with Core missing
  or behind every node fails. `scripts/evonodes.sh` reads Core's `.cookie` or `dash.conf` from the
  OS default data directory; set `DASHCORE_DIR` if yours is elsewhere.
- RPC credentials in `platform-mobile/mainnet.env` (or `testnet.env`): `RS_SDK_CORE_USER`,
  `RS_SDK_CORE_PASSWORD`, `RS_SDK_CORE_PORT` (9998 mainnet, 19998 testnet). They must match
  `rpcuser`/`rpcpassword` in Core's `dash.conf`.
- `bash`, `jq`, `curl`, `python3`, and about 80 GB free disk if `target/` is cold. A full disk can crash
  Core and force a multi-hour `-reindex`.

## Steps (mainnet; for testnet add `testnet` to the `status`/`list` calls and use the testnet names)

1. **Check Core is synced.**

   ```bash
   scripts/evonodes.sh status
   ```

   Wait until `blocks == headers`, `initialblockdownload` is `false`, and the masternode sync
   reports `IsSynced: true`.

2. **Pull every valid, non-banned Evo node from Core.**

   ```bash
   scripts/evonodes.sh list > /tmp/evonodes.txt
   scripts/evonodes.sh non443   # should print nothing; nodes on other ports are left out
   ```

   `list` reads `protx list valid true`, keeps `type == "Evo"` with `PoSeBanHeight == -1`, and
   takes the IP of each `platform_https` address on port 443.

3. **Write the candidate list.** Do not pre-filter it: unreachable nodes are what the test is
   for.

   ```bash
   scripts/evonodes.sh write /tmp/evonodes.txt platform-mobile/src/fetch_document.rs MAINNET_ADDRESS_LIST
   ```

4. **Run the check** (a few minutes per hundred nodes; failing nodes wait for the 10 s timeout).

   ```bash
   cd platform-mobile
   cargo test --lib fetch_document::check_all_nodes_mainnet_test -- --nocapture --exact > /tmp/nodecheck.log 2>&1
   cd ..
   ```

   Each node prints `<ip>: success` or `<ip>: error/fail: <reason>`. The test fails only when more
   nodes fail than pass.

5. **Write the shipped list from the passing nodes.**

   ```bash
   scripts/evonodes.sh passing /tmp/nodecheck.log > /tmp/passing.txt
   wc -l < /tmp/passing.txt
   scripts/evonodes.sh write /tmp/passing.txt platform-mobile/src/config.rs MAINNET_ADDRESS_LIST
   ```

   If a large share failed with the same error (for example quorum or proof errors), suspect the
   local Core rather than the nodes. Fix it and rerun step 4 before writing `config.rs`.

6. **Review and commit.** Update the `// Refreshed …` comment above each list with the date and
   counts. `git diff --stat` should show only the two list files. In the PR, give
   the counts: candidates, passing, and how many were added and removed versus the old
   `config.rs` list.
