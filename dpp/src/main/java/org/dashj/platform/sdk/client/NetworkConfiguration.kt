package org.dashj.platform.sdk.client

import org.bitcoinj.core.NetworkParameters
import org.bitcoinj.core.Sha256Hash
import org.bitcoinj.params.DevNetParams
import org.dashj.platform.dapiclient.provider.DAPIAddress
import org.dashj.platform.dapiclient.provider.ListDAPIAddressProvider
import java.net.URI

/** Explicit network identity and HTTPS endpoints, shared by the Kotlin and native clients. */
class NetworkConfiguration(val network: String, endpoints: List<String>) {
    val endpoints: List<String> = java.util.Collections.unmodifiableList(endpoints.toList())

    init {
        require(network == "mainnet" || network == "testnet" || network.matches(Regex("devnet-[A-Za-z0-9-]+"))) {
            "Expected mainnet, testnet, or devnet-<name>"
        }
        require(this.endpoints.isNotEmpty()) { "At least one DAPI endpoint is required" }
        this.endpoints.forEach {
            val uri = URI(it)
            require(uri.scheme == "https" && !uri.host.isNullOrBlank() && uri.port in 1..65535 &&
                uri.userInfo == null && (uri.path.isNullOrEmpty() || uri.path == "/") &&
                uri.query == null && uri.fragment == null) {
                "DAPI endpoints must be HTTPS URLs with an explicit port and no credentials, path, query or fragment"
            }
        }
    }

    fun validate(params: NetworkParameters) {
        val expected = when {
            params is DevNetParams -> params.devNetName
            params.id == NetworkParameters.ID_MAINNET -> "mainnet"
            params.id == NetworkParameters.ID_TESTNET -> "testnet"
            else -> throw IllegalArgumentException("Unsupported Core network: ${params.id}")
        }
        require(network == expected) { "Platform network $network does not match Core network $expected" }
    }

    internal fun addressProvider(banBaseTime: Int) = ListDAPIAddressProvider(endpoints.map {
        val uri = URI(it)
        DAPIAddress(uri.host.removePrefix("[").removeSuffix("]"), uri.port, uri.port, Sha256Hash.ZERO_HASH)
    }, banBaseTime)

    companion object {
        /** Validator endpoints advertised by the Sakura dashboard, verified 2026-10-09. */
        @JvmStatic
        fun sakura() = NetworkConfiguration("devnet-sakura", listOf(
            "68.67.122.86", "68.67.122.87", "68.67.122.240", "68.67.122.241",
            "68.67.122.88", "68.67.122.89", "68.67.122.90", "68.67.122.91",
            "68.67.122.92", "68.67.122.93", "68.67.122.94", "68.67.122.95", "68.67.122.232"
        ).map { "https://$it:1443" })
    }
}
