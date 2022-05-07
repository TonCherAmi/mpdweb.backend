package com.github.toncherami.mpd.web.volume.dto.api.request

import javax.validation.constraints.Max
import javax.validation.constraints.Min

data class VolumeSetBody(
    @get:Min(0)
    @get:Max(100)
    val volume: Int,
)
