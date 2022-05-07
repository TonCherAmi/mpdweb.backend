package com.github.toncherami.mpd.web.database.dto

import com.github.toncherami.mpd.web.adapter.dto.MpdDirectory
import com.github.toncherami.mpd.web.database.dto.enums.DatabaseItemType

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
