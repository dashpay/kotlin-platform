/**
 * Copyright (c) 2025-present, Dash Core Group
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
package dashj.org.platform

import org.bitcoinj.crypto.MnemonicCode
import org.bitcoinj.wallet.authentication.AuthenticationGroupExtension
import org.dashj.platform.dashpay.BlockchainIdentity
import org.dashj.platform.dpp.identity.Identity
import org.dashj.platform.dpp.toBase58
import org.dashj.platform.sdk.Client
import org.dashj.platform.sdk.client.ClientOptions
import org.dashj.platform.sdk.client.WalletOptions
import org.dashj.platform.sdk.platform.DomainDocument
import java.util.Scanner

class RestoreIdentities {
    companion object {
        lateinit var client: Client

        @JvmStatic
        fun main(args: Array<String>) {
            if (args.isEmpty()) {
                println("Usage: RestoreIdentities network [recoveryPhrase]")
                println("  network: testnet, bintang, etc.")
                println("  recoveryPhrase: optional recovery phrase (will prompt if not provided)")
                return
            }

            val recoveryPhrase = if (args.size >= 2) {
                args[1]
            } else {
                println("Enter recovery phrase: ")
                val scanner = Scanner(System.`in`)
                val phrase = scanner.nextLine()
                if (phrase == "default") {
                    DefaultIdentity(args[0]).seed
                } else {
                    phrase
                }
            }

            // Validate the recovery phrase
            try {
                MnemonicCode().check(recoveryPhrase.split(" ", "\n"))
            } catch (e: Exception) {
                println("Error: Invalid recovery phrase - ${e.message}")
                return
            }

            client = Client(ClientOptions(network = args[0], walletOptions = WalletOptions(recoveryPhrase)))
            client.platform.useValidNodes()
            restoreIdentities()
        }

        private fun restoreIdentities() {
            println("=".repeat(80))
            println("Restoring identities from wallet...")
            println("=".repeat(80))
            println()

            val authGroup = client.wallet!!.getKeyChainExtension(AuthenticationGroupExtension.EXTENSION_ID) as AuthenticationGroupExtension

            // Try to recover identities from indices 0 to 9
            var identitiesFound = 0
            val identities = arrayListOf<Identity>()
            for (index in 0 until 10) {
                try {
                    val identityKey = authGroup.identityKeyChain.getKey(index)
                    val pubKeyHash = identityKey.pubKeyHash

                    // Try to recover identity by public key hash
                    val identity = client.platform.identities.getByPublicKeyHash(pubKeyHash)

                    if (identity != null) {
                        if (identities.contains(identity)) {
                            return
                        }
                        identities.add(identity)
                        identitiesFound++
                        println("Identity #$index:")
                        println("-".repeat(80))
                        println("  Identity ID: ${identity.id.toBuffer().toBase58()}")
                        println("  Balance: ${identity.balance} duffs")

                        // Get all domain names for this identity
                        val domainDocuments = client.platform.names.getByOwnerId(identity.id)

                        if (domainDocuments.isNotEmpty()) {
                            println("  Domain names (${domainDocuments.size}):")
                            domainDocuments.forEach { doc ->
                                val domain = DomainDocument(doc)
                                println("    - ${domain.label} (normalized: ${domain.normalizedLabel})")
                            }
                        } else {
                            println("  Domain names: None")
                        }

                        // Create BlockchainIdentity to get username information
                        try {
                            val blockchainIdentity = BlockchainIdentity(client.platform, identity.id.toSha256Hash())
                            blockchainIdentity.identity = identity
                            blockchainIdentity.registrationStatus = org.dashj.platform.dashpay.IdentityStatus.REGISTERED

                            // Recover usernames to populate current, primary, and secondary
                            blockchainIdentity.recoverUsernames()

                            println("  Username Information:")
                            println("    Current Username: ${blockchainIdentity.currentUsername ?: "None"}")
                            println("    Primary Username: ${blockchainIdentity.primaryUsername ?: "None"}")
                            println("    Secondary Username: ${blockchainIdentity.secondaryUsername ?: "None"}")

                            val allUsernames = blockchainIdentity.getUsernames()
                            if (allUsernames.isNotEmpty()) {
                                println("    All Usernames: ${allUsernames.joinToString(", ")}")
                            }
                        } catch (e: Exception) {
                            println("  Error creating BlockchainIdentity: ${e.message}")
                        }

                        println()
                    }
                } catch (e: Exception) {
                    // Identity not found or error occurred, continue to next index
                    // Silently skip if no identity found
                }
            }

            println("=".repeat(80))
            println("Total identities found: $identitiesFound")
            println("=".repeat(80))
        }
    }
}