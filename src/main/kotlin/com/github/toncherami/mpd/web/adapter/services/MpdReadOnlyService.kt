package com.github.toncherami.mpd.web.adapter.services

import com.github.toncherami.mpd.web.adapter.dto.MpdChange
import com.github.toncherami.mpd.web.adapter.dto.MpdCount
import com.github.toncherami.mpd.web.adapter.dto.MpdDatabaseItem
import com.github.toncherami.mpd.web.adapter.dto.MpdPlaylistItem
import com.github.toncherami.mpd.web.adapter.dto.MpdRegexFileFilter
import com.github.toncherami.mpd.web.adapter.dto.MpdStatus

interface MpdReadOnlyService {

    fun idle(): List<MpdChange>
    fun status(): MpdStatus
    fun playlistinfo(): List<MpdPlaylistItem>
    fun lsinfo(uri: String): List<MpdDatabaseItem>
    fun count(vararg filter: String): MpdCount
    fun search(mpdRegexFileFilter: MpdRegexFileFilter): List<MpdDatabaseItem>
    fun albumart(uri: String): ByteArray

}
