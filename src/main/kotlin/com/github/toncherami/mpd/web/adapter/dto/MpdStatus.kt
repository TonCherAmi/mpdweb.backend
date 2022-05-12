package com.github.toncherami.mpd.web.adapter.dto

import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import com.github.toncherami.mpd.web.adapter.deserializers.MpdBooleanDeserializer
import com.github.toncherami.mpd.web.adapter.dto.enums.MpdSingleState
import com.github.toncherami.mpd.web.adapter.dto.enums.MpdPlaybackState

data class MpdStatus(
    val volume: Int,
    val state: MpdPlaybackState,
    val elapsed: Double,
    val duration: Double,
    val song: Int?,
    val songid: Int?,
    val playlistlength: Int,
    val single: MpdSingleState,
    @JsonDeserialize(using = MpdBooleanDeserializer::class)
    val repeat: Boolean,
    @JsonDeserialize(using = MpdBooleanDeserializer::class)
    val random: Boolean,
    @JsonDeserialize(using = MpdBooleanDeserializer::class)
    val consume: Boolean,
)
