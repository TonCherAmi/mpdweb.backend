package com.github.toncherami.mpd.web.adapter.data

import com.fasterxml.jackson.annotation.JsonSubTypes
import com.fasterxml.jackson.annotation.JsonTypeInfo

@JsonSubTypes(
    JsonSubTypes.Type(MpdDatabaseFile::class),
    JsonSubTypes.Type(MpdDatabasePlaylist::class),
    JsonSubTypes.Type(MpdDatabaseDirectory::class)
)
@JsonTypeInfo(use = JsonTypeInfo.Id.DEDUCTION, defaultImpl = MpdDatabaseFile::class)
abstract class MpdDatabaseItem {

    companion object {

        const val PATH_SEPARATOR = "/"

    }

}
