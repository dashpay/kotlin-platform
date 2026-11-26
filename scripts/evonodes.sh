#!/usr/bin/env bash
# Helpers for refreshing the Evo node lists in platform-mobile/src.
# See docs/updating-evonode-lists.md for the full procedure.
# Reads Dash Core's data dir from DASHCORE_DIR, else the OS default (macOS or ~/.dashcore).
#
#   scripts/evonodes.sh status [mainnet|testnet]       sync state of the local Dash Core
#   scripts/evonodes.sh list   [mainnet|testnet]       valid, non-banned Evo node IPs (Platform HTTPS on :443)
#   scripts/evonodes.sh non443 [mainnet|testnet]       Evo nodes whose Platform HTTPS port is not 443 (excluded by `list`)
#   scripts/evonodes.sh passing <test-log>             IPs that printed "<ip>: success" in a check_all_nodes_* run
#   scripts/evonodes.sh write <ip-file> <rust-file> <CONST_NAME>
#                                                      replace a `[&str; N]` IP-list constant with the IPs in <ip-file>
set -eo pipefail

NET="${2:-mainnet}"
if [[ -n "$DASHCORE_DIR" ]]; then D="$DASHCORE_DIR"
elif [[ "$(uname)" == Darwin ]]; then D="$HOME/Library/Application Support/DashCore"
else D="$HOME/.dashcore"; fi
if [[ "$NET" == testnet ]]; then DATA="$D/testnet3"; PORT=19998; else DATA="$D"; PORT=9998; fi

rpc() {
  local auth
  if [[ -r "$DATA/.cookie" ]]; then
    auth=$(cat "$DATA/.cookie")
  elif [[ -r "$D/dash.conf" ]]; then
    # rpcuser/rpcpassword from dash.conf; never echoed.
    auth="$(sed -n 's/^rpcuser=//p' "$D/dash.conf"):$(sed -n 's/^rpcpassword=//p' "$D/dash.conf")"
  else
    echo "evonodes.sh: no $DATA/.cookie or $D/dash.conf; set DASHCORE_DIR to Core's data directory" >&2
    return 1
  fi
  local out
  # Fail loudly: an unreachable Core or bad credentials must not look like an empty node list.
  if ! out=$(curl -s --user "$auth" -H 'content-type: text/plain' \
      --data-binary "{\"jsonrpc\":\"1.0\",\"id\":\"x\",\"method\":\"$1\",\"params\":$2}" "http://127.0.0.1:$PORT/"); then
    echo "evonodes.sh: cannot reach Dash Core RPC on 127.0.0.1:$PORT" >&2; return 1
  fi
  if ! jq -e '.error == null and .result != null' >/dev/null 2>&1 <<<"$out"; then
    local why="no response (check RPC credentials)"
    [[ -n "$out" ]] && why=$(jq -c '.error' <<<"$out" 2>/dev/null || printf '%s' "$out")
    echo "evonodes.sh: RPC $1 failed: $why" >&2
    return 1
  fi
  printf '%s\n' "$out"
}

# Fails if stdin is empty, so a broken step never yields an empty list.
nonempty() { local all; all=$(cat); [[ -n "$all" ]] || { echo "evonodes.sh: $1 produced no IPs" >&2; return 1; }; printf '%s\n' "$all"; }

evo_https() {
  rpc protx '["list","valid",true]' | jq -r '.result[]
    | select(.type == "Evo" and .state.PoSeBanHeight == -1)
    | .state.addresses.platform_https[]?'
}

sort_ips() { sort -t. -k1,1n -k2,2n -k3,3n -k4,4n -u; }

case "$1" in
  status)
    rpc getblockchaininfo '[]' | jq -c '.result | {chain, blocks, headers, verificationprogress, initialblockdownload}'
    rpc mnsync '["status"]' | jq -c '.result | {AssetName, IsSynced}'
    ;;
  list)
    evo_https | sed -n 's/:443$//p' | sort_ips | nonempty list
    ;;
  non443)
    evo_https | grep -v ':443$' || true
    ;;
  passing)
    sed -nE 's/^([0-9]+\.[0-9]+\.[0-9]+\.[0-9]+): success$/\1/p' "$2" | sort_ips | nonempty passing
    ;;
  write)
    python3 - "$2" "$3" "$4" <<'EOF'
import ipaddress, re, sys
ips_file, rust_file, name = sys.argv[1:4]
ips = [l.strip() for l in open(ips_file) if l.strip()]
if not ips:
    sys.exit(f"{ips_file} is empty; refusing to write an empty {name}")
for ip in ips:
    try:
        ipaddress.IPv4Address(ip)
    except ValueError:
        sys.exit(f"{ips_file}: not an IPv4 address: {ip!r}")
src = open(rust_file).read()
body = ",\n".join(f'    "{ip}"' for ip in ips)
pattern = rf'((?:pub )?const {re.escape(name)}: \[&str; )\d+(\] = \[\n).*?(\n\];)'
new, n = re.subn(pattern, lambda m: f"{m.group(1)}{len(ips)}{m.group(2)}{body}{m.group(3)}", src, count=1, flags=re.S)
if n != 1:
    sys.exit(f"{name} not found in {rust_file}")
open(rust_file, "w").write(new)
print(f"{rust_file}: {name} now has {len(ips)} entries")
EOF
    ;;
  *)
    sed -n '2,11p' "$0" >&2; exit 2 ;;
esac
