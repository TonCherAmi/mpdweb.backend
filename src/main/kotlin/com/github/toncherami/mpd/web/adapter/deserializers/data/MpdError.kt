package com.github.toncherami.mpd.web.adapter.deserializers.data

import com.github.toncherami.mpd.web.adapter.deserializers.data.enums.MpdErrorCode

data class MpdError(
    val code: MpdErrorCode,
    val message: String,
    val command: String?,
    val commandIndex: Int,
)
