package com.github.toncherami.mpd.web.database.dto

import com.github.toncherami.mpd.web.adapter.dto.MpdPlaylist
import com.github.toncherami.mpd.web.database.dto.enums.DatabaseItemType

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
