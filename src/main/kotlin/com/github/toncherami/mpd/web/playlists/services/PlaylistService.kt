package com.github.toncherami.mpd.web.playlists.services

import com.github.toncherami.mpd.web.database.data.DatabaseFile
import com.github.toncherami.mpd.web.playlists.data.Playlist

interface PlaylistService {

    fun get(): List<Playlist>
    fun delete(name: String)
    fun rename(from: String, to: String)
    fun getFiles(name: String): List<DatabaseFile>
    fun addTracks(name: String, uri: String)
    fun deleteFiles(name: String, positions: List<Int>)

}
