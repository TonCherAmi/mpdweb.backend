package com.github.toncherami.mpd.web.dto

import com.github.toncherami.mpd.web.dto.enums.PlayerState

data class PlayerStatus(
    val volume: Int,
    val state: PlayerState,
    val elapsed: Double,
    val duration: Double
)

val PlayerStatus.isStopped: Boolean
  get() = state == PlayerState.STOPPED

val PlayerStatus.isPlaying: Boolean
  get() = state == PlayerState.PLAYING
