package com.github.toncherami.mpd.web.adapter.deserializers.exceptions

class MpdDeserializationException(
    val what: String,
    val from: String,
) : RuntimeException("Unable to deserialize MPD $what from '$from'")
