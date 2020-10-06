package com.github.toncherami.mpd.web.adapter.gateways

import com.github.toncherami.mpd.web.adapter.config.MPD_TCP_NO_TIMEOUT_CHANNEL_ID
import com.github.toncherami.mpd.web.adapter.gateways.base.TcpGateway
import org.springframework.integration.annotation.MessagingGateway

@MessagingGateway(defaultRequestChannel = MPD_TCP_NO_TIMEOUT_CHANNEL_ID)
interface MpdNoTimeoutTcpGateway : TcpGateway
