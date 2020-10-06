package com.github.toncherami.mpd.web.adapter.config

import com.github.toncherami.mpd.web.adapter.deserializers.MpdDeserializer
import com.github.toncherami.mpd.web.adapter.interceptors.MpdHandshakeInterceptor
import com.github.toncherami.mpd.web.adapter.properties.MpdProperties
import org.springframework.beans.factory.annotation.Qualifier
import org.springframework.context.annotation.Bean
import org.springframework.context.annotation.Configuration
import org.springframework.integration.annotation.ServiceActivator
import org.springframework.integration.ip.tcp.TcpOutboundGateway
import org.springframework.integration.ip.tcp.connection.TcpConnectionInterceptorFactory
import org.springframework.integration.ip.tcp.connection.TcpConnectionInterceptorFactoryChain
import org.springframework.integration.ip.tcp.connection.TcpNetClientConnectionFactory
import org.springframework.integration.ip.tcp.serializer.ByteArrayLfSerializer

const val MPD_TCP_REGULAR_CHANNEL_ID = "toRegularMpdTcp"
const val MPD_TCP_NO_TIMEOUT_CHANNEL_ID = "toNoTimeoutMpdTcp"

@Configuration
class MpdTcpConfig(private val mpdProperties: MpdProperties) {

    @Bean
    @ServiceActivator(inputChannel = MPD_TCP_REGULAR_CHANNEL_ID)
    fun tcpRegularOutGate(
        @Qualifier("tcpNetClientConnectionFactory1")
        tcpNetClientConnectionFactory: TcpNetClientConnectionFactory
    ): TcpOutboundGateway {
        return TcpOutboundGateway().also {
            it.setConnectionFactory(tcpNetClientConnectionFactory)
        }
    }

    @Bean("noTimeout")
    @ServiceActivator(inputChannel = MPD_TCP_NO_TIMEOUT_CHANNEL_ID)
    fun tcpNoTimeoutOutGate(
        @Qualifier("tcpNetClientConnectionFactory2")
        tcpNetClientConnectionFactory: TcpNetClientConnectionFactory
    ): TcpOutboundGateway {
        return TcpOutboundGateway().also {
            it.setRemoteTimeout(Long.MAX_VALUE)
            it.setConnectionFactory(tcpNetClientConnectionFactory)
        }
    }

    @Bean("tcpNetClientConnectionFactory1")
    fun tcpNetClientConnectionFactory1(
        tcpConnectionInterceptorFactoryChain: TcpConnectionInterceptorFactoryChain
    ): TcpNetClientConnectionFactory {
        return getTcpNetClientConnectionFactory(tcpConnectionInterceptorFactoryChain)
    }

    @Bean("tcpNetClientConnectionFactory2")
    fun tcpNetClientConnectionFactory2(
        tcpConnectionInterceptorFactoryChain: TcpConnectionInterceptorFactoryChain
    ): TcpNetClientConnectionFactory {
        return getTcpNetClientConnectionFactory(tcpConnectionInterceptorFactoryChain)
    }

    @Bean
    fun tcpConnectionInterceptorFactoryChain(mpdProperties: MpdProperties): TcpConnectionInterceptorFactoryChain {
        val factories = arrayOf(
            TcpConnectionInterceptorFactory {
                MpdHandshakeInterceptor(mpdProperties)
            }
        )

        return TcpConnectionInterceptorFactoryChain().also {
            it.setInterceptors(factories)
        }
    }

    private fun getTcpNetClientConnectionFactory(
        tcpConnectionInterceptorFactoryChain: TcpConnectionInterceptorFactoryChain
    ): TcpNetClientConnectionFactory {
        return TcpNetClientConnectionFactory(mpdProperties.host, mpdProperties.port).also {
            it.serializer = ByteArrayLfSerializer()
            it.deserializer = MpdDeserializer()
            it.setInterceptorFactoryChain(tcpConnectionInterceptorFactoryChain)
        }
    }

}
