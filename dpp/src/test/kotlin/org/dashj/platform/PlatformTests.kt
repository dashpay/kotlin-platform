package org.dashj.platform

/**
 * Copyright (c) 2020-present, Dash Core Group
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
import com.google.common.io.BaseEncoding
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test

class PlatformTests {

    private fun bouncy64(ba: ByteArray): String {
        return org.bouncycastle.util.encoders.Base64.toBase64String(ba)
    }

    private fun google64(ba: ByteArray): String {
        return BaseEncoding.base64().encode(ba)
    }

    private fun google64op(ba: ByteArray): String {
        return BaseEncoding.base64().omitPadding().encode(ba)
    }

    @Test
    fun Base64Test() {
        val sig = "QR8sFMxpk4N+hjQHvNu1QnLhxDqTglVlGc18MaSxt4uIFFp7TArHA0j+JE7Yev0XNQpm1wIii1oV4XrXiW/bmgDl"

        val bouncy = org.bouncycastle.util.encoders.Base64.decode(sig)
        val google = BaseEncoding.base64().decode(sig)
        val googleop = BaseEncoding.base64().omitPadding().decode(sig)

        val b = bouncy
        // val bouncy64 =
        assertEquals(bouncy64(b), google64(b))
        assertEquals(bouncy64(b), google64op(b))
    }

    @Test fun regexTest() {
        val errorString = "Protocol error: Generic Error: Document C2z6f1BtabvWyDVHbeCqzKRn5jn9uHgmQdUe5mpEhZbU has duplicate unique properties [\"normalizedParentDomainName\", \"normalizedLabel\"] with other documents"
        val errorString2 = "Attempted to unwrap a Failure: Protocol error: Generic Error: Document obQ535W6AGqn5DhoCJfM8XYzK9QMddGLXS9ytmFMGNb has duplicate unique properties [\"normalizedParentDomainName\", \"normalizedLabel\"] with other documents"
        val errorString3 = "Document 6pHYWaENTWKSKHT5iTFRTDidb9qj3Pte8WKXQ1hUmZZZ has duplicate unique properties [\"normalizedParentDomainName\", \"normalizedLabel\"] with other documents\n"
        assertTrue(errorString.contains(
            Regex(
               "Document [a-zA-Z0-9]{32,44} has duplicate unique properties \\[\"normalizedParentDomainName\", \"normalizedLabel\"\\] with other documents"
            ))
        )

        assertTrue(errorString2.contains(
            Regex(
                "Document [a-zA-Z0-9]{32,44} has duplicate unique properties \\[\"normalizedParentDomainName\", \"normalizedLabel\"\\] with other documents"
            ))
        )

        assertTrue(errorString3.contains(
            Regex(
                "Document [a-zA-Z0-9]{32,44} has duplicate unique properties \\[\"normalizedParentDomainName\", \"normalizedLabel\"\\] with other documents"
            ))
        )
    }
}
