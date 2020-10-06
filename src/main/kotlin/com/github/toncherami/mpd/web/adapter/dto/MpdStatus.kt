package com.github.toncherami.mpd.web.adapter.dto

import com.github.toncherami.mpd.web.adapter.dto.enums.MpdState

data class MpdStatus(
    val volume: Int,
    val state: MpdState,
    val elapsed: Double,
    val duration: Double,
    val song: Int?
)
