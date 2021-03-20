package com.github.toncherami.mpd.web.database.dto

import com.fasterxml.jackson.databind.annotation.JsonSerialize
import com.github.toncherami.mpd.web.adapter.dto.MpdCount
import com.github.toncherami.mpd.web.common.serializers.DurationSerializer
import com.github.toncherami.mpd.web.common.utils.toDuration
import java.time.Duration
import java.util.concurrent.TimeUnit

data class DatabaseCount(
    @JsonSerialize(using = DurationSerializer::class)
    val playtime: Duration,
    val songCount: Long
) {

    companion object {
        fun of(mpdCount: MpdCount): DatabaseCount {
            return DatabaseCount(
                playtime = mpdCount.playtime.toDuration(TimeUnit.SECONDS),
                songCount = mpdCount.songs
            )
        }
    }

}
