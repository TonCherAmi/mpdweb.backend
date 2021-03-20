package com.github.toncherami.mpd.web.adapter.services

import com.github.toncherami.mpd.web.adapter.dto.MpdChange
import com.github.toncherami.mpd.web.adapter.dto.MpdCount
import com.github.toncherami.mpd.web.adapter.dto.MpdDatabaseItem
import com.github.toncherami.mpd.web.adapter.dto.MpdFile
import com.github.toncherami.mpd.web.adapter.dto.MpdStatus

interface MpdService {

    fun idle(): List<MpdChange>
    fun play()
    fun stop()
    fun pause()
    fun next()
    fun previous()
    fun status(): MpdStatus
    fun playlistinfo(): List<MpdFile>
    fun clear()
    fun update()
    fun lsinfo(uri: String): List<MpdDatabaseItem>
    fun add(uri: String)
    fun count(vararg filter: String): MpdCount

}
