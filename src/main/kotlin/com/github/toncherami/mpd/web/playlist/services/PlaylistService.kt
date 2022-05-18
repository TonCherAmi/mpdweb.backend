package com.github.toncherami.mpd.web.playlist.services

import com.github.toncherami.mpd.web.playlist.data.PlaylistItem

interface PlaylistService {

    fun get(): List<PlaylistItem>
    fun add(uri: String)
    fun clear()
    fun replace(uri: String)
    fun delete(id: Int)

}
