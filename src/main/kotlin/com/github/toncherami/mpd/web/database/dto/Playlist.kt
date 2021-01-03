package com.github.toncherami.mpd.web.database.dto

import com.github.toncherami.mpd.web.adapter.dto.MpdPlaylist
import com.github.toncherami.mpd.web.database.dto.enums.DatabaseItemType

class Playlist(
    uri: String
) : DatabaseItem(uri, DatabaseItemType.PLAYLIST) {

    companion object {
        fun of(mpdPlaylist: MpdPlaylist): Playlist {
            return Playlist(mpdPlaylist.playlist)
        }
    }

}
