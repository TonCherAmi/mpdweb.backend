package com.github.toncherami.mpd.web.dto

import com.fasterxml.jackson.annotation.JsonProperty

data class PlayerPlaylistItem(
    @JsonProperty("Id")
    val id: String,
    val file: String,
    @JsonProperty("Pos")
    val position: Int,
    val duration: Double,
    @JsonProperty("Title")
    val title: String?,
    @JsonProperty("Artist")
    val artist: String?
)
