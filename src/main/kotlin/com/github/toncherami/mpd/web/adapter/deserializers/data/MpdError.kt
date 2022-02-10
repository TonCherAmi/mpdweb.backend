package com.github.toncherami.mpd.web.adapter.deserializers.data

data class MpdError(
    val code: Int,
    val message: String,
    val command: String?,
    val commandIndex: Int,
)
