package com.github.toncherami.mpd.web.adapter.services

import com.github.toncherami.mpd.web.adapter.data.MpdChange
import com.github.toncherami.mpd.web.adapter.data.MpdCount
import com.github.toncherami.mpd.web.adapter.data.MpdDatabaseFile
import com.github.toncherami.mpd.web.adapter.data.MpdDatabaseItem
import com.github.toncherami.mpd.web.adapter.data.MpdPlaylist
import com.github.toncherami.mpd.web.adapter.data.MpdPlaylistItem
import com.github.toncherami.mpd.web.adapter.data.MpdRegexFileFilter
import com.github.toncherami.mpd.web.adapter.data.MpdStatus

interface MpdReadOnlyService {

    fun idle(): List<MpdChange>
    fun status(): MpdStatus
    fun playlistinfo(): List<MpdPlaylistItem>
    fun lsinfo(uri: String): List<MpdDatabaseItem>
    fun count(vararg filter: String): MpdCount
    fun search(mpdRegexFileFilter: MpdRegexFileFilter): List<MpdDatabaseItem>
    fun albumart(uri: String): ByteArray
    fun readpicture(uri: String): ByteArray
    fun listplaylists(): List<MpdPlaylist>
    fun listplaylistinfo(name: String): List<MpdDatabaseFile>

}
