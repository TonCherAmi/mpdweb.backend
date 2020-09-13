package com.github.toncherami.mpd.web.tcpclients

import com.github.toncherami.mpd.web.configs.TCP_CHANNEL_ID
import org.springframework.integration.annotation.MessagingGateway

@MessagingGateway(defaultRequestChannel = TCP_CHANNEL_ID)
interface MpdTcpClient {

    fun sendMessage(message: String): String

}
