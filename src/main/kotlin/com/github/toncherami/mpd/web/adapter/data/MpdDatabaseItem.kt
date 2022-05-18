package com.github.toncherami.mpd.web.adapter.data

import com.fasterxml.jackson.annotation.JsonSubTypes
import com.fasterxml.jackson.annotation.JsonTypeInfo

@JsonSubTypes(
    JsonSubTypes.Type(MpdFile::class),
    JsonSubTypes.Type(MpdPlaylist::class),
    JsonSubTypes.Type(MpdDirectory::class)
)
@JsonTypeInfo(use = JsonTypeInfo.Id.DEDUCTION, defaultImpl = MpdFile::class)
abstract class MpdDatabaseItem {

    companion object {

        const val PATH_SEPARATOR = "/"

    }

}
