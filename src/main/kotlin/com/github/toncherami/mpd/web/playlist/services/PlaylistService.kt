package com.github.toncherami.mpd.web.playlist.services

import com.github.toncherami.mpd.web.database.dto.DatabaseFile

interface PlaylistService {

    fun get(): List<DatabaseFile>
    fun add(uri: String)
    fun clear()
    fun replace(uri: String)

}
