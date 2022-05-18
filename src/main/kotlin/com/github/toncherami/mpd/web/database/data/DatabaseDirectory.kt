package com.github.toncherami.mpd.web.database.data

import com.github.toncherami.mpd.web.adapter.data.MpdDirectory
import com.github.toncherami.mpd.web.database.data.enums.DatabaseItemType

data class DatabaseDirectory(
    override val uri: String,
) : DatabaseItem {

    override val type: DatabaseItemType = DatabaseItemType.DIRECTORY

    companion object {
        fun of(mpdDirectory: MpdDirectory): DatabaseDirectory {
            return DatabaseDirectory(mpdDirectory.directory)
        }
    }

}
