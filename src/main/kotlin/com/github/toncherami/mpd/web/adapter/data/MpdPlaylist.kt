package com.github.toncherami.mpd.web.adapter.data

import com.fasterxml.jackson.annotation.JsonProperty
import java.time.ZonedDateTime

data class MpdPlaylist(
    val playlist: String,
    @JsonProperty("Last-Modified")
    val lastModified: ZonedDateTime,
)
