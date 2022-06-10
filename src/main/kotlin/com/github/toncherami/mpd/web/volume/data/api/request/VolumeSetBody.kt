package com.github.toncherami.mpd.web.volume.data.api.request

import com.github.toncherami.mpd.web.volume.data.api.request.enums.VolumeSetMode
import javax.validation.constraints.Max
import javax.validation.constraints.Min

data class VolumeSetBody(
    @get:Min(0)
    @get:Max(100)
    val volume: Int,
    val mode: VolumeSetMode,
)
