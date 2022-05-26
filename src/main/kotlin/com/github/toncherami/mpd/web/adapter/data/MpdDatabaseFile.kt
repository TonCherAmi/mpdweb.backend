package com.github.toncherami.mpd.web.adapter.data

import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.annotation.JsonSubTypes

@JsonSubTypes(
    JsonSubTypes.Type(MpdPlaylistItem::class),
)
open class MpdDatabaseFile(
    val file: String,
    val duration: Double,
    @JsonProperty("Title")
    val title: String?,
    @JsonProperty("Artist")
    val artist: String?,
    @JsonProperty("Album")
    val album: String?,
    @JsonProperty("Format")
    val format: String
) : MpdDatabaseItem()
