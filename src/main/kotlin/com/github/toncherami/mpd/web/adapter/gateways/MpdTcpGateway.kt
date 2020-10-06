package com.github.toncherami.mpd.web.adapter.gateways

import com.github.toncherami.mpd.web.adapter.config.MPD_TCP_REGULAR_CHANNEL_ID
import com.github.toncherami.mpd.web.adapter.gateways.base.TcpGateway
import org.springframework.integration.annotation.MessagingGateway

@MessagingGateway(defaultRequestChannel = MPD_TCP_REGULAR_CHANNEL_ID)
interface MpdTcpGateway : TcpGateway
