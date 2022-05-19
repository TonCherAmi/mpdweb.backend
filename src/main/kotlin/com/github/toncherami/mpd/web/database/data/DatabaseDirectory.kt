package com.github.toncherami.mpd.web.database.data

import com.github.toncherami.mpd.web.adapter.data.MpdDatabaseDirectory
import com.github.toncherami.mpd.web.database.data.enums.DatabaseItemType

data class DatabaseDirectory(
    override val uri: String,
) : DatabaseItem {

    override val type: DatabaseItemType = DatabaseItemType.DIRECTORY

    companion object {
        fun of(mpdDatabaseDirectory: MpdDatabaseDirectory): DatabaseDirectory {
            return DatabaseDirectory(mpdDatabaseDirectory.directory)
        }
    }

}
