package com.github.toncherami.mpd.web.status.services.impl

import com.github.toncherami.mpd.web.adapter.dto.MpdStatus
import com.github.toncherami.mpd.web.adapter.dto.enums.MpdState
import com.github.toncherami.mpd.web.adapter.services.MpdService
import com.github.toncherami.mpd.web.common.config.STOMP_PLAYER_STATUS_DESTINATION
import com.github.toncherami.mpd.web.common.utils.toDuration
import com.github.toncherami.mpd.web.status.dto.Status
import com.github.toncherami.mpd.web.status.dto.enums.State
import com.github.toncherami.mpd.web.status.services.StatusService
import org.springframework.messaging.simp.SimpMessagingTemplate
import org.springframework.stereotype.Service
import java.util.concurrent.TimeUnit

@Service
class StatusServiceImpl(
    private val mpdService: MpdService,
    private val simpMessagingTemplate: SimpMessagingTemplate
) : StatusService {

    override fun get(): Status {
        return mpdService.status().toDto()
    }

    override fun send(status: Status) {
        simpMessagingTemplate.convertAndSend(STOMP_PLAYER_STATUS_DESTINATION, status)
    }

}

private fun MpdStatus.toDto(): Status = Status(
    state = state.toDto(),
    volume = volume,
    elapsed = elapsed.toDuration(TimeUnit.SECONDS),
    duration = duration.toDuration(TimeUnit.SECONDS),
    currentSong = song
)

private fun MpdState.toDto(): State = State.valueOf(name)

