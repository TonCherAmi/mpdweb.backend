package com.github.toncherami.mpd.web.playback.dto.api.request

import com.github.toncherami.mpd.web.playback.dto.api.request.enums.PlaybackSeekMode
import javax.validation.constraints.Min

data class PlaybackSeekBody(
    @Min(0)
    val time: Double,
    val mode: PlaybackSeekMode = PlaybackSeekMode.ABSOLUTE,
)
