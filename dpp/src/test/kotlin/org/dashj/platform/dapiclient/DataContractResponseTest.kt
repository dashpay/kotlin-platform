package org.dashj.platform.dapiclient

import org.dashj.platform.sdk.DataContract
import org.dashj.platform.sdk.base.Result
import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test
import org.mockito.Mockito
import java.util.Optional

class DataContractResponseTest {
    @Test fun absentContractIsNull() {
        assertNull(decodeDataContractResult(Result.Ok(Optional.empty<DataContract>())))
    }

    @Test fun proofAndTransportFailuresAreNotAbsence() {
        val error = assertThrows(Exception::class.java) {
            decodeDataContractResult(Result.Err("proof verification failed"))
        }
        assertTrue(error.message!!.contains("proof verification failed"))
    }

    @Test fun conversionFailureIsNotAbsence() {
        val contract = Mockito.mock(DataContract::class.java)
        Mockito.`when`(contract.id).thenThrow(IllegalStateException("invalid contract"))
        assertThrows(IllegalStateException::class.java) {
            decodeDataContractResult(Result.Ok(Optional.of(contract)))
        }
    }
}
