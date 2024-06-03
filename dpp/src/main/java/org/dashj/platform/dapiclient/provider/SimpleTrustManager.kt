package org.dashj.platform.dapiclient.provider

import java.security.cert.X509Certificate
import javax.net.ssl.X509TrustManager

class SimpleTrustManager : X509TrustManager {
    override fun checkClientTrusted(chain: Array<X509Certificate?>?, authType: String?) {
        // Do nothing, thereby trusting any client.
        println("checkClientTrusted: $authType")
        chain?.forEach { println(it) }
    }

    override fun checkServerTrusted(chain: Array<X509Certificate?>?, authType: String?) {
        // Do nothing, thereby trusting any server.
        println("checkClientTrusted: $authType")
        chain?.forEach { println(it) }
    }

    override fun getAcceptedIssuers(): Array<X509Certificate> {
        return arrayOf()
    }
}