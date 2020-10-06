package com.github.toncherami.mpd.web.playlist.dto

import com.fasterxml.jackson.databind.annotation.JsonSerialize
import com.github.toncherami.mpd.web.common.serializers.DurationSerializer
import java.time.Duration

data class PlaylistItem(
    val id: String,
    val position: Int,
    val file: String,
    val title: String?,
    val artist: String?,
    @JsonSerialize(using = DurationSerializer::class)
    val duration: Duration
)
