package com.github.toncherami.mpd.web.database.dto

import com.github.toncherami.mpd.web.adapter.dto.MpdDirectory
import com.github.toncherami.mpd.web.database.dto.enums.DatabaseItemType

class DatabaseDirectory(
    uri: String
) : DatabaseItem(uri, DatabaseItemType.DIRECTORY) {

    companion object {
        fun of(mpdDirectory: MpdDirectory): DatabaseDirectory {
            return DatabaseDirectory(mpdDirectory.directory)
        }
    }

}
