# Explicit network configuration

Use `NetworkConfiguration` to send the same HTTPS endpoints to the Kotlin DAPI
client and the native Platform SDK. The legacy boolean constructors remain for
mainnet/testnet; they cannot identify a named devnet.

```kotlin
val configuration = NetworkConfiguration.sakura()
val options = ClientOptions.builder()
    .network(configuration, sakuraCoreParams)
    .build()
val client = Client(options)
```

`sakuraCoreParams` must be an operator-configured DashJ `DevNetParams` whose
`devNetName` is `devnet-sakura`. Supply the correct Core P2P port, spork address,
quorum settings and bootstrap peers. These are not inferred from Platform IPs.
Configure the client's masternode-list manager/context provider as usual before
making proof-verified requests. The native explicit-network constructor requires
a quorum-public-key callback; it does not silently use mainnet/testnet Core RPC.

For direct Platform construction:

```kotlin
val platform = Platform(sakuraCoreParams, NetworkConfiguration.sakura())
```

For another devnet, supply its name and complete endpoints:

```kotlin
val configuration = NetworkConfiguration(
    "devnet-example",
    listOf("https://node.example.org:1443")
)
```

Endpoints require HTTPS, an explicit port, and valid certificates. Credentials,
paths other than `/`, queries, fragments, and empty lists are rejected. Explicit
configuration does not disable certificate verification or read Rust `.env`
settings. Do not also supply `dapiAddresses` or a separate address provider in
`ClientOptions`. This is an initial node list, not automatic node discovery.

The Sakura preset contains the 13 validator IPs advertised by
https://status.testnet.networks.dash.org/n/devnet-sakura/connect on 2026-10-09,
using port 1443. One endpoint was checked with normal TLS verification. Devnet
deployments can change; applications may supply an updated list without an SDK
release. The dashboard advertised protocol v14 at inspection time; the SDK uses
its devnet bootstrap version and normal protocol discovery, not a Sakura-specific
hard-coded protocol version.

This SDK change does not add a wallet build flavor, provision Core parameters,
publish a Maven artifact, or prove that a funded registration succeeds on Sakura.
