package com.github.toncherami.mpd.web.adapter.data.enums

import com.fasterxml.jackson.annotation.JsonValue

enum class MpdSingleState(private val value: String) {

    ON("1"),
    OFF("0"),
    ONESHOT("oneshot");

    @JsonValue
    fun toValue() = value

}
