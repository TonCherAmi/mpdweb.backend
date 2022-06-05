package com.github.toncherami.mpd.web.adapter.exceptions

import com.github.toncherami.mpd.web.adapter.deserializers.data.enums.MpdErrorCode

class MpdException(
    val code: MpdErrorCode,
    val command: String?,
    val commandIndex: Int,
    message: String,
) : RuntimeException(message)
