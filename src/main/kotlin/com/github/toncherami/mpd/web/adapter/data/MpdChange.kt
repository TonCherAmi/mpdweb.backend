package com.github.toncherami.mpd.web.adapter.data

import com.github.toncherami.mpd.web.adapter.data.enums.MpdSubsystem

data class MpdChange(
    val changed: MpdSubsystem
)
