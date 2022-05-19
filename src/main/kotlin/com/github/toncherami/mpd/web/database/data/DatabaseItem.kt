package com.github.toncherami.mpd.web.database.data

import com.github.toncherami.mpd.web.adapter.data.MpdDatabaseItem
import com.github.toncherami.mpd.web.adapter.data.MpdDatabaseDirectory
import com.github.toncherami.mpd.web.adapter.data.MpdDatabaseFile
import com.github.toncherami.mpd.web.adapter.data.MpdDatabasePlaylist
import com.github.toncherami.mpd.web.database.data.enums.DatabaseItemType
import java.lang.IllegalArgumentException

interface DatabaseItem {

    val uri: String

    val type: DatabaseItemType

    companion object {
        fun of(mpdDatabaseItem: MpdDatabaseItem): DatabaseItem {
            return when(mpdDatabaseItem) {
                is MpdDatabaseFile -> DatabaseFile.of(mpdDatabaseItem)
                is MpdDatabasePlaylist -> DatabasePlaylist.of(mpdDatabaseItem)
                is MpdDatabaseDirectory -> DatabaseDirectory.of(mpdDatabaseItem)
                else -> throw IllegalArgumentException("Unsupported database item subtype")
            }
        }
    }

}
