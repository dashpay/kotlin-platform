package org.dashj.platform.dapiclient

import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class SdkTimeoutTest {
    @Test fun defaultTimeoutsReachTheSdkInSeconds() {
        assertEquals(5, millisToSdkSeconds(DapiClient.DEFAULT_TIMEOUT))
        assertEquals(10, millisToSdkSeconds(DapiClient.DEFAULT_TIMEOUT * 2))
    }

    @Test fun partialSecondsRoundUp() {
        assertEquals(2, millisToSdkSeconds(1500))
        assertEquals(1, millisToSdkSeconds(1))
    }

    @Test fun zeroOrNegativeNeverDisablesTheTimeout() {
        assertEquals(1, millisToSdkSeconds(0))
        assertEquals(1, millisToSdkSeconds(-5000))
    }
}
