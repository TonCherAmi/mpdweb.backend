package com.github.toncherami.mpd.web.status.dto

import com.fasterxml.jackson.databind.annotation.JsonSerialize
import com.github.toncherami.mpd.web.common.serializers.DurationSerializer
import com.github.toncherami.mpd.web.status.dto.enums.State
import java.time.Duration

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
    val state: State,
    val volume: Int,
    val song: CurrentSong?,
    val playlist: CurrentPlaylist,
)

val Status.isStopped: Boolean
    get() = state == State.STOPPED

val Status.isPlaying: Boolean
    get() = state == State.PLAYING
