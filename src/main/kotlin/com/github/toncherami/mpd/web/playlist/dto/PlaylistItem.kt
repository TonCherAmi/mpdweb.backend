package com.github.toncherami.mpd.web.playlist.dto

import com.fasterxml.jackson.databind.annotation.JsonSerialize
import com.github.toncherami.mpd.web.adapter.dto.MpdPlaylistItem
import com.github.toncherami.mpd.web.common.serializers.DurationSerializer
import com.github.toncherami.mpd.web.common.utils.toDuration
import com.github.toncherami.mpd.web.database.dto.DatabaseFile
import java.time.Duration
import java.util.concurrent.TimeUnit

class PlaylistItem(
    val id: Int,
    val position: Int,
    title: String?,
    artist: String?,
    @JsonSerialize(using = DurationSerializer::class)
    duration: Duration,
    uri: String,
) : DatabaseFile(title = title, artist = artist, duration = duration, uri = uri) {

    companion object {
        fun of(mpdPlaylistItem: MpdPlaylistItem): PlaylistItem {
            return mpdPlaylistItem.let {
                PlaylistItem(
                    id = it.id,
                    position = it.position,
                    artist = it.artist,
                    title = it.title,
                    duration = it.duration.toDuration(TimeUnit.SECONDS),
                    uri = it.file,
                )
            }
        }
    }

}
