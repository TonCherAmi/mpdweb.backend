package com.github.toncherami.mpd.web.status.services.impl

import com.github.toncherami.mpd.web.common.config.STOMP_PLAYER_STATUS_DESTINATION
import com.github.toncherami.mpd.web.status.data.Status
import com.github.toncherami.mpd.web.status.services.StatusPushService
import org.springframework.messaging.simp.SimpMessagingTemplate
import org.springframework.stereotype.Service

@Service
class StatusPushServiceImpl(
    private val simpMessagingTemplate: SimpMessagingTemplate,
) : StatusPushService {

    override fun push(status: Status) {
        simpMessagingTemplate.convertAndSend(STOMP_PLAYER_STATUS_DESTINATION, status)
    }

}
