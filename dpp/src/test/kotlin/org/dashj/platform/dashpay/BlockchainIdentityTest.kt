package org.dashj.platform.dashpay

import org.dashj.platform.dpp.identifier.Identifier
import org.dashj.platform.dpp.identity.Identity
import kotlin.random.Random
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNotEquals
import org.junit.jupiter.api.Test

class BlockchainIdentityTest : PlatformNetwork() {

    val blockchainIdentity = BlockchainIdentity(platform, 0, wallet, authenticationGroupExtension)

    @Test
    fun checkWalletSeedTest() {
        assertEquals(seed.split(' '), blockchainIdentity.wallet!!.keyChainSeed.mnemonicCode)
    }

    @Test
    fun updateProfileTest() {
        val displayName = "Bob Lee " + Random.nextInt()

        blockchainIdentity.recoverIdentity(authenticationGroupExtension.identityKeyChain.getKey(0, true).pubKeyHash)

        val currentProfile = blockchainIdentity.getProfileFromPlatform()!!

        val updatedProfile = blockchainIdentity.updateProfile(displayName, null, null, null, null, null)

        val retrievedProfile = blockchainIdentity.getProfileFromPlatform()!!

        assertNotEquals(currentProfile, retrievedProfile)
        assertEquals(updatedProfile, Profile(retrievedProfile))
    }

    @Test
    fun sendContactRequestTest() {

        blockchainIdentity.recoverIdentity(authenticationGroupExtension.identityKeyChain.getKey(0, true).pubKeyHash)

        val cr = ContactRequests(platform)

        // my-test-1
        val toUserIdentity = platform.identities.get("6ffk46v6m79RPbiomNT1kLXX7unqgw5FkWhsTuPi4Aj7")

        val document = cr.create(blockchainIdentity, toUserIdentity!!, null)

        println(document)
    }
}
