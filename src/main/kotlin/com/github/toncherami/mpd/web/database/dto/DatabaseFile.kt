package com.github.toncherami.mpd.web.database.dto

import com.fasterxml.jackson.databind.annotation.JsonSerialize
import com.github.toncherami.mpd.web.adapter.dto.MpdFile
import com.github.toncherami.mpd.web.common.serializers.DurationSerializer
import com.github.toncherami.mpd.web.common.utils.toDuration
import com.github.toncherami.mpd.web.database.dto.enums.DatabaseItemType
import java.time.Duration
import java.util.concurrent.TimeUnit

open class DatabaseFile(
    val title: String?,
    val artist: String?,
    @JsonSerialize(using = DurationSerializer::class)
    val duration: Duration,
    override val uri: String,
) : DatabaseItem {

    override val type: DatabaseItemType = DatabaseItemType.FILE

    companion object {
        fun of(mpdFile: MpdFile): DatabaseFile {
            return mpdFile.let {
                DatabaseFile(
                    uri = it.file,
                    artist = it.artist,
                    title = it.title,
                    duration = it.duration.toDuration(TimeUnit.SECONDS)
                )
            }
        }
    }

}
