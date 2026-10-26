# Verified Contract Group Scope

## Why This Changed

The Android bridge previously returned only a contract group's owner. Owner
equality alone does not describe the authority granted by a group-bound key:
groups can contain whole contracts, document types and token positions, and
authorized administrators can add future members.

The wallet consequently rejected all group grants as a temporary safety measure.
That restriction broke Yappr's legitimate v2 login flow, whose primary CRITICAL
authentication key is group-bound. Single-contract substitution is not an
equivalent fix: the supported CRITICAL grant profile requires group bounds.

Platform already supplies proof-verifying group information and member queries.
The missing piece was the Android bridge, not a new consensus rule. This change
exposes the full verified scope so the wallet can restore group approvals with
accurate consent instead of removing verification or requiring a different QR.

## Guarantees

- `fetch_contract_group_snapshot_with_sdk` uses the SDK's proof-verifying `Fetch`
  path for owner/admin information and all three member kinds.
- Every member page must match the information query's block height, chain ID
  and block ID. A block mismatch retries the complete snapshot up to three times;
  no mixed-block or partial result is returned.
- Pages contain at most 100 entries. Full pages require continuation, including
  the final empty page when necessary. More than 1,000 aggregate members produces
  an explicit review-size error rather than truncating the scope.
- Proved absence is JSON null. Transport, proof, missing-page and cursor errors
  remain errors. The Kotlin decoder rejects incomplete or duplicate scope data.
- Kotlin member/admin sets ignore transport order. Block identity is internal
  to snapshot construction, so a newer block alone does not change consent.
- The existing owner-only API remains available for revocation audits; revoking
  recorded keys does not depend on fetching a complete membership snapshot.

The Android CMake tasks now depend on SWIG generation. Verification caught an
ARM64 library built before the new Java/JNI name was generated. Explicit ordering
prevents packaging an old C++ wrapper with a new Java SDK. Both packaged ABIs were
checked for the new JNI symbol, and the wallet APK libraries matched those bytes.

## Verification

Three offline Rust tests cover block consistency, pagination/caps and all member
kinds. Three Kotlin model tests cover decoding, semantic equality and malformed
scope. The targeted SDK regression set passed 17 tests in total.

On 2026-10-25 the Pixel 9 API 36 AOSP emulator completed the Yappr testnet login
with wallet 11.9.2-connect. Its log records a successful identity-update response
at 21:38:07 followed by system-contract response submission and lookup. This is
evidence for that tested happy path, not exhaustive live coverage of every group
size, configuration-change race or interrupted approval.
