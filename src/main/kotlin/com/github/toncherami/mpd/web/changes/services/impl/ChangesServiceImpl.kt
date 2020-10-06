package com.github.toncherami.mpd.web.changes.services.impl

import com.github.toncherami.mpd.web.adapter.dto.MpdChange
import com.github.toncherami.mpd.web.adapter.services.MpdService
import com.github.toncherami.mpd.web.changes.enums.Change
import com.github.toncherami.mpd.web.changes.services.ChangesService
import com.github.toncherami.mpd.web.common.config.STOMP_PLAYER_CHANGES_DESTINATION
import org.springframework.messaging.simp.SimpMessagingTemplate
import org.springframework.stereotype.Service

@Service
class ChangesServiceImpl(
    private val mpdService: MpdService,
    private val simpMessagingTemplate: SimpMessagingTemplate
) : ChangesService {

    override fun get(): List<Change> {
        return mpdService.idle().map(MpdChange::changed)
    }

    override fun send(changes: List<Change>) {
        simpMessagingTemplate.convertAndSend(STOMP_PLAYER_CHANGES_DESTINATION, changes)
    }

}
