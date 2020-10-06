package com.github.toncherami.mpd.web.adapter.dto

import com.github.toncherami.mpd.web.adapter.dto.enums.MpdSubsystem

data class MpdChange(
    val changed: MpdSubsystem
)
