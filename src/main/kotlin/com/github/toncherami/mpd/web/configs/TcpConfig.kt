package com.github.toncherami.mpd.web.configs

import com.github.toncherami.mpd.web.properties.MpdProperties
import org.springframework.context.annotation.Bean
import org.springframework.context.annotation.Configuration
import org.springframework.integration.annotation.MessageEndpoint
import org.springframework.integration.annotation.ServiceActivator
import org.springframework.integration.annotation.Transformer
import org.springframework.integration.ip.tcp.TcpOutboundGateway
import org.springframework.integration.ip.tcp.connection.AbstractClientConnectionFactory
import org.springframework.integration.ip.tcp.connection.TcpNetClientConnectionFactory
import org.springframework.integration.ip.tcp.serializer.ByteArrayElasticRawDeserializer
import org.springframework.messaging.MessageHandler

const val TCP_CHANNEL_ID = "toTcp"

private const val TCP_RESULT_CHANNEL_ID = "resultToString"

@Configuration
class TcpConfig(private val mpdProperties: MpdProperties) {

    @MessageEndpoint
    class DefaultMessageEndpoint {
        @Transformer(inputChannel = TCP_RESULT_CHANNEL_ID)
        fun convertResult(byteArray: ByteArray): String {
            return String(byteArray)
        }
    }

    @Bean
    @ServiceActivator(inputChannel = TCP_CHANNEL_ID)
    fun tcpOutGate(connectionFactory: AbstractClientConnectionFactory): MessageHandler {
        return TcpOutboundGateway().also {
            it.setCloseStreamAfterSend(true)
            it.setOutputChannelName(TCP_RESULT_CHANNEL_ID)
            it.setConnectionFactory(connectionFactory)
        }
    }

    @Bean
    fun clientFactory(): AbstractClientConnectionFactory {
        return TcpNetClientConnectionFactory(mpdProperties.host, mpdProperties.port).also {
            it.isSingleUse = true
            it.deserializer = ByteArrayElasticRawDeserializer()
        }
    }

}
