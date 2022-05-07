package com.github.toncherami.mpd.web.status.services.impl

import com.github.toncherami.mpd.web.adapter.dto.MpdStatus
import com.github.toncherami.mpd.web.adapter.dto.enums.MpdState
import com.github.toncherami.mpd.web.adapter.services.MpdService
import com.github.toncherami.mpd.web.common.config.STOMP_PLAYER_STATUS_DESTINATION
import com.github.toncherami.mpd.web.common.utils.toDuration
import com.github.toncherami.mpd.web.playlist.dto.PlaylistItem
import com.github.toncherami.mpd.web.playlist.services.PlaylistService
import com.github.toncherami.mpd.web.status.dto.CurrentPlaylist
import com.github.toncherami.mpd.web.status.dto.CurrentSong
import com.github.toncherami.mpd.web.status.dto.Status
import com.github.toncherami.mpd.web.status.dto.enums.State
import com.github.toncherami.mpd.web.status.services.StatusService
import org.springframework.messaging.simp.SimpMessagingTemplate
import org.springframework.stereotype.Service
import java.time.Duration
import java.util.concurrent.TimeUnit

@Service
class StatusServiceImpl(
    private val mpdService: MpdService,
    private val playlistService: PlaylistService,
    private val simpMessagingTemplate: SimpMessagingTemplate,
) : StatusService {

    override fun get(): Status {
        val playlist = playlistService.get()

        return mpdService.status().toDto(playlist)
    }

    override fun send(status: Status) {
        simpMessagingTemplate.convertAndSend(STOMP_PLAYER_STATUS_DESTINATION, status)
    }

}

private fun MpdStatus.toDto(playlist: List<PlaylistItem>): Status {
    val currentSongElapsed = elapsed.toDuration(TimeUnit.SECONDS)

    return Status(
        state = state.toDto(),
        volume = volume,
        song = if (song == null || songid == null) {
            null
        } else {
            CurrentSong(
                id = songid,
                position = song,
                elapsed = currentSongElapsed,
                duration = duration.toDuration(TimeUnit.SECONDS),
            )
        },
        playlist = CurrentPlaylist(
            length = playlistlength,
            elapsed = if (song == null) {
                Duration.ZERO
            } else {
                currentSongElapsed + playlist
                    .takeWhile { it.position < song }
                    .fold(Duration.ZERO) { acc, playlistItem ->
                        acc + playlistItem.duration
                    }
            },
            duration = playlist
                .fold(Duration.ZERO) { acc, playlistItem ->
                    acc + playlistItem.duration
                }
        ),
    )
}

private fun MpdState.toDto(): State = State.valueOf(name)
