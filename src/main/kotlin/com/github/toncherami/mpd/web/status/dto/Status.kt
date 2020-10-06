package com.github.toncherami.mpd.web.status.dto

import com.fasterxml.jackson.databind.annotation.JsonSerialize
import com.github.toncherami.mpd.web.status.dto.enums.State
import com.github.toncherami.mpd.web.common.serializers.DurationSerializer
import java.time.Duration

data class Status(
    val state: State,
    val volume: Int,
    @JsonSerialize(using = DurationSerializer::class)
    val elapsed: Duration,
    @JsonSerialize(using = DurationSerializer::class)
    val duration: Duration,
    val currentSong: Int?
)

val Status.isStopped: Boolean
    get() = state == State.STOPPED

val Status.isPlaying: Boolean
    get() = state == State.PLAYING
