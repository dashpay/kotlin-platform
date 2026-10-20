package org.dashj.platform.sdk.client

import org.bitcoinj.params.MainNetParams
import org.bitcoinj.params.TestNet3Params
import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test

class NetworkConfigurationTest {
    @Test
    fun endpointsAreAnImmutableSnapshot() {
        val endpoints = mutableListOf("https://127.0.0.1:1443")
        val config = NetworkConfiguration("devnet-sakura", endpoints)
        endpoints.clear()
        assertEquals(1, config.endpoints.size)
        assertThrows(UnsupportedOperationException::class.java) {
            (config.endpoints as MutableList<String>).clear()
        }
    }

    @Test
    fun rejectsConflictingNodeSourcesBeforeConstructingClient() {
        val config = NetworkConfiguration("testnet", listOf("https://127.0.0.1:1443"))
        assertThrows(IllegalArgumentException::class.java) {
            ClientOptions.builder().network(config, TestNet3Params.get())
                .dapiAddress("127.0.0.2").build()
        }
    }

    @Test
    fun legacyNetworkSelectionClearsExplicitConfiguration() {
        val config = NetworkConfiguration("testnet", listOf("https://127.0.0.1:1443"))
        val options = ClientOptions.builder().network(config, TestNet3Params.get()).network("mainnet").build()
        assertEquals("mainnet", options.network)
        assertNull(options.networkConfiguration)
        assertNull(options.networkParameters)
    }

    @Test
    fun sakuraUsesOnlyItsAdvertisedValidators() {
        val config = NetworkConfiguration.sakura()
        assertEquals("devnet-sakura", config.network)
        assertEquals(13, config.endpoints.size)
        val addresses = config.addressProvider(10).toList()
        assertTrue(addresses.all { it.grpcPort == 1443 && it.httpPort == 1443 })
        assertEquals("68.67.122.86", addresses.first().host)
        assertFalse(addresses.any { it.host == "68.67.122.1" })
    }

    @Test
    fun explicitPortsArePreserved() {
        val config = NetworkConfiguration("mainnet", listOf("https://127.0.0.1:443"))
        config.validate(MainNetParams.get())
        assertEquals(443, config.addressProvider(10).toList().single().grpcPort)
    }

    @Test
    fun rejectsCoreNetworkMismatch() {
        assertThrows(IllegalArgumentException::class.java) {
            NetworkConfiguration.sakura().validate(TestNet3Params.get())
        }
    }

    @Test
    fun rejectsInvalidEndpoints() {
        for (endpoint in listOf("127.0.0.1", "http://127.0.0.1:1443", "https://127.0.0.1",
            "https://127.0.0.1:0", "https://user@127.0.0.1:1443", "https://127.0.0.1:1443/path",
            "https://127.0.0.1:1443?x=y", "https://127.0.0.1:1443#fragment")) {
            try {
                NetworkConfiguration("devnet-sakura", listOf(endpoint))
                fail("Accepted $endpoint")
            } catch (_: IllegalArgumentException) { }
        }
    }
}
