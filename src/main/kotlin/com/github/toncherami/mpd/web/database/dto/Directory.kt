package com.github.toncherami.mpd.web.database.dto

import com.github.toncherami.mpd.web.adapter.dto.MpdDirectory
import com.github.toncherami.mpd.web.database.dto.enums.DatabaseItemType

data class Directory(
    val directory: String
) : DatabaseItem(DatabaseItemType.DIRECTORY) {

    companion object {
        fun of(mpdDirectory: MpdDirectory): Directory {
            return Directory(mpdDirectory.directory)
        }
    }

}
