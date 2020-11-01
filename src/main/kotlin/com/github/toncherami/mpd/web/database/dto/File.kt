package com.github.toncherami.mpd.web.database.dto

import com.fasterxml.jackson.databind.annotation.JsonSerialize
import com.github.toncherami.mpd.web.adapter.dto.MpdFile
import com.github.toncherami.mpd.web.common.serializers.DurationSerializer
import com.github.toncherami.mpd.web.common.utils.toDuration
import com.github.toncherami.mpd.web.database.dto.enums.DatabaseItemType
import java.time.Duration
import java.util.concurrent.TimeUnit

data class File(
    val file: String,
    val id: String?,
    val position: Int,
    val title: String?,
    val artist: String?,
    @JsonSerialize(using = DurationSerializer::class)
    val duration: Duration,
) : DatabaseItem(DatabaseItemType.FILE) {

    companion object {
        fun of(mpdFile: MpdFile): File {
            return mpdFile.let {
                File(
                    id = it.id,
                    position = it.position,
                    file = it.file,
                    artist = it.artist,
                    title = it.title,
                    duration = it.duration.toDuration(TimeUnit.SECONDS)
                )
            }
        }
    }

}
