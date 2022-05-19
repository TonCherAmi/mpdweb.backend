package com.github.toncherami.mpd.web.queue.data

import com.fasterxml.jackson.databind.annotation.JsonSerialize
import com.github.toncherami.mpd.web.adapter.data.MpdPlaylistItem
import com.github.toncherami.mpd.web.common.serializers.DurationSerializer
import com.github.toncherami.mpd.web.common.utils.toDuration
import com.github.toncherami.mpd.web.database.data.DatabaseAudioFormat
import com.github.toncherami.mpd.web.database.data.DatabaseFile
import java.time.Duration
import java.util.concurrent.TimeUnit

class QueueItem(
    val id: Int,
    val position: Int,
    title: String?,
    artist: String?,
    @JsonSerialize(using = DurationSerializer::class)
    duration: Duration,
    uri: String,
    format: DatabaseAudioFormat,
) : DatabaseFile(title = title, artist = artist, duration = duration, uri = uri, format = format) {

    companion object {
        fun of(mpdPlaylistItem: MpdPlaylistItem): QueueItem {
            return mpdPlaylistItem.let {
                QueueItem(
                    id = it.id,
                    position = it.position,
                    artist = it.artist,
                    title = it.title,
                    duration = it.duration.toDuration(TimeUnit.SECONDS),
                    uri = it.file,
                    format = DatabaseAudioFormat.of(it.format)
                )
            }
        }
    }

}
