package com.github.toncherami.mpd.web.adapter.dto

import com.fasterxml.jackson.annotation.JsonProperty

class MpdPlaylistItem(
    @JsonProperty("Id")
    val id: Int,
    @JsonProperty("Pos")
    val position: Int,
    file: String,
    duration: Double,
    @JsonProperty("Title")
    title: String?,
    @JsonProperty("Artist")
    artist: String?
) : MpdFile(file = file, duration = duration, title = title, artist = artist)
