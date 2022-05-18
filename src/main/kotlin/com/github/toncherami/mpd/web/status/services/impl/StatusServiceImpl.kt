package com.github.toncherami.mpd.web.status.services.impl

import com.github.toncherami.mpd.web.adapter.data.MpdStatus
import com.github.toncherami.mpd.web.adapter.data.enums.MpdPlaybackState
import com.github.toncherami.mpd.web.adapter.data.enums.MpdSingleState
import com.github.toncherami.mpd.web.adapter.services.MpdService
import com.github.toncherami.mpd.web.common.utils.toDuration
import com.github.toncherami.mpd.web.queue.data.QueueItem
import com.github.toncherami.mpd.web.queue.services.QueueService
import com.github.toncherami.mpd.web.status.data.Queue
import com.github.toncherami.mpd.web.status.data.CurrentSong
import com.github.toncherami.mpd.web.status.data.SingleState
import com.github.toncherami.mpd.web.status.data.Status
import com.github.toncherami.mpd.web.status.data.enums.PlaybackState
import com.github.toncherami.mpd.web.status.services.StatusService
import org.springframework.stereotype.Service
import java.time.Duration
import java.util.concurrent.TimeUnit

@Service
class StatusServiceImpl(
    private val mpdService: MpdService,
    private val queueService: QueueService,
) : StatusService {

    override fun get(): Status {
        val queue = queueService.get()

        return mpdService.status().toStatus(queue)
    }

}

private fun MpdStatus.toStatus(queue: List<QueueItem>): Status {
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
        queue = Queue(
            length = playlistlength,
            elapsed = if (song == null) {
                Duration.ZERO
            } else {
                currentSongElapsed + queue
                    .takeWhile { it.position < song }
                    .fold(Duration.ZERO) { acc, playlistItem ->
                        acc + playlistItem.duration
                    }
            },
            duration = queue
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
