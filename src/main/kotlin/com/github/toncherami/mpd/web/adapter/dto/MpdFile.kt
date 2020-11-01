package com.github.toncherami.mpd.web.adapter.dto

import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.databind.annotation.JsonDeserialize

data class MpdFile(
    val file: String,
    @JsonProperty("Id")
    val id: String?,
    @JsonProperty("Pos")
    val position: Int,
    val duration: Double,
    @JsonProperty("Title")
    val title: String?,
    @JsonProperty("Artist")
    val artist: String?
) : MpdDatabaseItem()
