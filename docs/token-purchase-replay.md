# Token Purchase Replay Identity

The `dash-st` bridge accepts one unsigned token direct purchase. Preparation
returns `requestId`, a SHA-256 digest of a domain-separated canonical tuple:
owner, contract, token ID, token position, quantity, maximum credits and original
identity-contract nonce. Integer quantities and nonces retain unsigned 64-bit
precision. Token display metadata and ignored envelope signing-key IDs are not
part of the operation identity.

Submission rechecks the complete prepared summary, then uses DPP's existing
token-purchase transition constructor to sign the decoded operation with the
wallet's CRITICAL key and the **original nonce**. It does not sign incoming bytes
and does not substitute a fresh nonce. Normal Platform nonce validation provides
on-chain replay rejection. An identical purchase with a new valid nonce remains
a separate operation; stale requests need a new QR.

The previous bridge discarded the request nonce and rebuilt through an SDK
builder that allocated a fresh nonce. Hashing original envelope bytes in the
wallet therefore failed to identify equivalent requests. Hashing only displayed
terms would instead have forbidden legitimate repeated purchases. Canonical
identity and preserved submission semantics must be changed together.

After every broadcast result, including errors, the SDK invalidates the payer's
nonce cache because this operation did not allocate through that cache. Affected
state proofs remain affected-state proofs, not proof that this operation ran;
errors after native handoff still require a durable unknown-outcome receipt.

Wallet integrations must consume the native request ID, persist it before native
submission and retain it on uncertain outcomes. Older receipts lack the actual
nonce chosen by the previous SDK and need conservative legacy handling.

Native regressions cover ignored key-ID mutations, nonce and price changes,
unsigned 64-bit precision and signing/readback of all original operation fields.
This security fix predates the Platform 5.0.1 upgrade and must remain intact
through it.
