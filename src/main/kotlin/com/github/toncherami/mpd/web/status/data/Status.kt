package com.github.toncherami.mpd.web.status.data

import com.fasterxml.jackson.databind.annotation.JsonSerialize
import com.github.toncherami.mpd.web.common.serializers.DurationSerializer
import com.github.toncherami.mpd.web.status.data.enums.PlaybackState
import java.time.Duration

enum class SingleState {

    ON,
    OFF,
    ONESHOT,

}

data class CurrentSong(
    val id: Int,
    val position: Int,
    @JsonSerialize(using = DurationSerializer::class)
    val elapsed: Duration,
    @JsonSerialize(using = DurationSerializer::class)
    val duration: Duration,
)

data class CurrentPlaylist(
    val length: Int,
    @JsonSerialize(using = DurationSerializer::class)
    val elapsed: Duration,
    @JsonSerialize(using = DurationSerializer::class)
    val duration: Duration,
)

data class Status(
    val state: PlaybackState,
    val volume: Int,
    val song: CurrentSong?,
    val playlist: CurrentPlaylist,
    val single: SingleState,
    val random: Boolean,
    val repeat: Boolean,
    val consume: Boolean,
)

val Status.isStopped: Boolean
    get() = state == PlaybackState.STOPPED

val Status.isPlaying: Boolean
    get() = state == PlaybackState.PLAYING
