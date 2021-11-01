package com.github.toncherami.mpd.web.database.services

import com.github.toncherami.mpd.web.database.dto.DatabaseCount
import com.github.toncherami.mpd.web.database.dto.DatabaseItem

interface DatabaseService {

    fun update()
    fun get(uri: String): List<DatabaseItem>
    fun count(uri: String): DatabaseCount
    fun search(term: String): List<DatabaseItem>

}
