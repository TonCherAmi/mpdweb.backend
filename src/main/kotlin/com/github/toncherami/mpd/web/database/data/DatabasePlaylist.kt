package com.github.toncherami.mpd.web.database.data

import com.github.toncherami.mpd.web.adapter.data.MpdPlaylist
import com.github.toncherami.mpd.web.database.data.enums.DatabaseItemType

data class DatabasePlaylist(
    override val uri: String,
) : DatabaseItem {

    override val type: DatabaseItemType = DatabaseItemType.PLAYLIST

    companion object {
        fun of(mpdPlaylist: MpdPlaylist): DatabasePlaylist {
            return DatabasePlaylist(mpdPlaylist.playlist)
        }
    }

}
