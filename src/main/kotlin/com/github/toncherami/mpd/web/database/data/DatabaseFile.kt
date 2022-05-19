package com.github.toncherami.mpd.web.database.data

import com.fasterxml.jackson.databind.annotation.JsonSerialize
import com.github.toncherami.mpd.web.adapter.data.MpdDatabaseFile
import com.github.toncherami.mpd.web.common.serializers.DurationSerializer
import com.github.toncherami.mpd.web.common.utils.toDuration
import com.github.toncherami.mpd.web.database.data.enums.DatabaseItemType
import java.time.Duration
import java.util.concurrent.TimeUnit

open class DatabaseFile(
    val title: String?,
    val artist: String?,
    val format: DatabaseAudioFormat,
    @JsonSerialize(using = DurationSerializer::class)
    val duration: Duration,
    override val uri: String,
) : DatabaseItem {

    override val type: DatabaseItemType = DatabaseItemType.FILE

    companion object {
        fun of(mpdDatabaseFile: MpdDatabaseFile): DatabaseFile {
            return mpdDatabaseFile.let {
                DatabaseFile(
                    uri = it.file,
                    artist = it.artist,
                    title = it.title,
                    format = DatabaseAudioFormat.of(it.format),
                    duration = it.duration.toDuration(TimeUnit.SECONDS)
                )
            }
        }
    }

}
