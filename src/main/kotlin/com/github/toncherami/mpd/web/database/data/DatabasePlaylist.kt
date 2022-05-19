package com.github.toncherami.mpd.web.database.data

import com.github.toncherami.mpd.web.adapter.data.MpdDatabasePlaylist
import com.github.toncherami.mpd.web.database.data.enums.DatabaseItemType

data class DatabasePlaylist(
    override val uri: String,
) : DatabaseItem {

    override val type: DatabaseItemType = DatabaseItemType.PLAYLIST

    companion object {
        fun of(mpdDatabasePlaylist: MpdDatabasePlaylist): DatabasePlaylist {
            return DatabasePlaylist(mpdDatabasePlaylist.playlist)
        }
    }

}
