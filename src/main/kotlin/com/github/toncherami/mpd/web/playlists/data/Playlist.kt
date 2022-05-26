package com.github.toncherami.mpd.web.playlists.data

import com.github.toncherami.mpd.web.adapter.data.MpdPlaylist
import java.time.ZonedDateTime

data class Playlist(val name: String, val updatedAt: ZonedDateTime) {

    companion object {

        fun of(mpdPlaylist: MpdPlaylist): Playlist {
            return Playlist(
                name = mpdPlaylist.playlist,
                updatedAt = mpdPlaylist.lastModified,
            )
        }

    }

}
