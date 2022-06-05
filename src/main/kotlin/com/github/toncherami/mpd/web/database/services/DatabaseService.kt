package com.github.toncherami.mpd.web.database.services

import com.github.toncherami.mpd.web.database.data.DatabaseCount
import com.github.toncherami.mpd.web.database.data.DatabaseItem
import com.github.toncherami.mpd.web.database.data.enums.DatabaseCoverType

interface DatabaseService {

    fun update()
    fun get(uri: String): List<DatabaseItem>
    fun count(uri: String): DatabaseCount
    fun search(term: String): List<DatabaseItem>
    fun cover(uri: String, type: DatabaseCoverType): ByteArray

}
