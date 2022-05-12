package com.github.toncherami.mpd.web.adapter.dto.enums

import com.fasterxml.jackson.annotation.JsonValue

enum class MpdPlaybackState(private val value: String) {

    PLAYING("play"),
    PAUSED("pause"),
    STOPPED("stop");

    @JsonValue
    fun toValue() = value

}
