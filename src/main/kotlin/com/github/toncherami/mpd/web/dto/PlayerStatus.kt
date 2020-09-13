package com.github.toncherami.mpd.web.dto

import com.github.toncherami.mpd.web.dto.enums.MpdState

data class PlayerStatus(
    val volume: Int,
    val state: MpdState,
    val elapsed: Double,
    val duration: Double
)
