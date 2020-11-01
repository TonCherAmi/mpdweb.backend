package com.github.toncherami.mpd.web.database.dto

import com.github.toncherami.mpd.web.adapter.dto.MpdPlaylist
import com.github.toncherami.mpd.web.database.dto.enums.DatabaseItemType

data class Playlist(
    val playlist: String
) : DatabaseItem(DatabaseItemType.PLAYLIST) {

    companion object {
        fun of(mpdPlaylist: MpdPlaylist): Playlist {
            return Playlist(mpdPlaylist.playlist)
        }
    }

}
