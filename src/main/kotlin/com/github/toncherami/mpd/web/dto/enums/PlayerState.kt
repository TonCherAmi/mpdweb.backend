package com.github.toncherami.mpd.web.dto.enums

import com.fasterxml.jackson.annotation.JsonValue

enum class PlayerState(private val value: String) {

    PLAYING("play"),
    PAUSED("pause"),
    STOPPED("stop");

    @JsonValue
    fun toValue() = value

}
