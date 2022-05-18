package com.github.toncherami.mpd.web.adapter.data

import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.annotation.JsonSubTypes

@JsonSubTypes(
    JsonSubTypes.Type(MpdPlaylistItem::class),
)
open class MpdFile(
    val file: String,
    val duration: Double,
    @JsonProperty("Title")
    val title: String?,
    @JsonProperty("Artist")
    val artist: String?
) : MpdDatabaseItem()
