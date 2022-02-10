package com.github.toncherami.mpd.web.adapter.exceptions

class MpdException(
    val code: Int,
    val command: String?,
    val commandIndex: Int,
    message: String,
) : RuntimeException(message)
