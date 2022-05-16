package com.github.toncherami.mpd.web.changes.services.impl

import com.github.toncherami.mpd.web.changes.enums.Change
import com.github.toncherami.mpd.web.changes.services.ChangesPushService
import com.github.toncherami.mpd.web.common.config.STOMP_PLAYER_CHANGES_DESTINATION
import org.springframework.messaging.simp.SimpMessagingTemplate
import org.springframework.stereotype.Service

@Service
class ChangesPushServiceImpl(
    private val simpMessagingTemplate: SimpMessagingTemplate,
) : ChangesPushService {

    override fun push(changes: List<Change>) {
        simpMessagingTemplate.convertAndSend(STOMP_PLAYER_CHANGES_DESTINATION, changes)
    }

}
