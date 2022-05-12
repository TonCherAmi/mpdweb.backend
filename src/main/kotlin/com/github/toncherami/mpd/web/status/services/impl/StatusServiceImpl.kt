package com.github.toncherami.mpd.web.status.services.impl

import com.github.toncherami.mpd.web.adapter.dto.MpdStatus
import com.github.toncherami.mpd.web.adapter.dto.enums.MpdSingleState
import com.github.toncherami.mpd.web.adapter.dto.enums.MpdPlaybackState
import com.github.toncherami.mpd.web.adapter.services.MpdService
import com.github.toncherami.mpd.web.common.config.STOMP_PLAYER_STATUS_DESTINATION
import com.github.toncherami.mpd.web.common.utils.toDuration
import com.github.toncherami.mpd.web.playlist.dto.PlaylistItem
import com.github.toncherami.mpd.web.playlist.services.PlaylistService
import com.github.toncherami.mpd.web.status.dto.CurrentPlaylist
import com.github.toncherami.mpd.web.status.dto.CurrentSong
import com.github.toncherami.mpd.web.status.dto.SingleState
import com.github.toncherami.mpd.web.status.dto.Status
import com.github.toncherami.mpd.web.status.dto.enums.PlaybackState
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

        return mpdService.status().toStatus(playlist)
    }

    override fun send(status: Status) {
        simpMessagingTemplate.convertAndSend(STOMP_PLAYER_STATUS_DESTINATION, status)
    }

}

private fun MpdStatus.toStatus(playlist: List<PlaylistItem>): Status {
    val currentSongElapsed = elapsed.toDuration(TimeUnit.SECONDS)

    return Status(
        state = state.toPlaybackState(),
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
        single = single.toSingleState(),
        random = random,
        repeat = repeat,
        consume = consume,
    )
}

private fun MpdSingleState.toSingleState(): SingleState {
    return when (this) {
        MpdSingleState.ON -> SingleState.ON
        MpdSingleState.OFF -> SingleState.OFF
        MpdSingleState.ONESHOT -> SingleState.ONESHOT
    }
}

private fun MpdPlaybackState.toPlaybackState(): PlaybackState = PlaybackState.valueOf(name)
