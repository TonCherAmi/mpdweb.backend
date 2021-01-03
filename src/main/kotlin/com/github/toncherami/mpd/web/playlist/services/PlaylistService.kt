package com.github.toncherami.mpd.web.playlist.services

import com.github.toncherami.mpd.web.database.dto.File

interface PlaylistService {

    fun get(): List<File>
    fun add(uri: String)
    fun clear()

}
