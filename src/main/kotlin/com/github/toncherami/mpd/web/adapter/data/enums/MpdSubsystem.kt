package com.github.toncherami.mpd.web.adapter.data.enums

import com.fasterxml.jackson.annotation.JsonValue

enum class MpdSubsystem(private val value: String) {

    MIXER("mixer"),
    PLAYER("player"),
    UPDATE("update"),
    OPTIONS("options"),
    DATABASE("database"),
    PLAYLIST("playlist");

    @JsonValue
    fun toValue() = value

}
