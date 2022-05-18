package com.github.toncherami.mpd.web.database.data

import com.github.toncherami.mpd.web.adapter.data.MpdDatabaseItem
import com.github.toncherami.mpd.web.adapter.data.MpdDirectory
import com.github.toncherami.mpd.web.adapter.data.MpdFile
import com.github.toncherami.mpd.web.adapter.data.MpdPlaylist
import com.github.toncherami.mpd.web.database.data.enums.DatabaseItemType
import java.lang.IllegalArgumentException

interface DatabaseItem {

    val uri: String

    val type: DatabaseItemType

    companion object {
        fun of(mpdDatabaseItem: MpdDatabaseItem): DatabaseItem {
            return when(mpdDatabaseItem) {
                is MpdFile -> DatabaseFile.of(mpdDatabaseItem)
                is MpdPlaylist -> DatabasePlaylist.of(mpdDatabaseItem)
                is MpdDirectory -> DatabaseDirectory.of(mpdDatabaseItem)
                else -> throw IllegalArgumentException("Unsupported database item subtype")
            }
        }
    }

}
