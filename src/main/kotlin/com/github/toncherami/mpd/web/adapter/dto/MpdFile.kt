package com.github.toncherami.mpd.web.adapter.dto

import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.annotation.JsonSubTypes
import com.fasterxml.jackson.annotation.JsonTypeInfo
import com.fasterxml.jackson.databind.annotation.JsonDeserialize

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
