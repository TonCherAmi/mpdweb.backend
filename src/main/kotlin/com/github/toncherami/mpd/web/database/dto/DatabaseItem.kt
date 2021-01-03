package com.github.toncherami.mpd.web.database.dto

import com.github.toncherami.mpd.web.adapter.dto.MpdDatabaseItem
import com.github.toncherami.mpd.web.adapter.dto.MpdDirectory
import com.github.toncherami.mpd.web.adapter.dto.MpdFile
import com.github.toncherami.mpd.web.adapter.dto.MpdPlaylist
import com.github.toncherami.mpd.web.database.dto.enums.DatabaseItemType
import java.lang.IllegalArgumentException

abstract class DatabaseItem(val uri: String, val type: DatabaseItemType) {

    companion object {
        fun of(mpdDatabaseItem: MpdDatabaseItem): DatabaseItem {
            return when(mpdDatabaseItem) {
                is MpdFile -> File.of(mpdDatabaseItem)
                is MpdPlaylist -> Playlist.of(mpdDatabaseItem)
                is MpdDirectory -> Directory.of(mpdDatabaseItem)
                else -> throw IllegalArgumentException("Unsupported database item subtype")
            }
        }
    }

}
