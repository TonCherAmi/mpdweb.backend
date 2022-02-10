package com.github.toncherami.mpd.web.adapter.deserializers.data

data class MpdResponse(
    val binary: List<ByteArray>,
    val pairs: List<Pair<String, String>>,
)
