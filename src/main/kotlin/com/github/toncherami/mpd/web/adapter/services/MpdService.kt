package com.github.toncherami.mpd.web.adapter.services

import com.github.toncherami.mpd.web.adapter.dto.MpdChange
import com.github.toncherami.mpd.web.adapter.dto.MpdPlaylistItem
import com.github.toncherami.mpd.web.adapter.dto.MpdStatus

interface MpdService {

    fun idle(): List<MpdChange>
    fun play()
    fun stop()
    fun pause()
    fun next()
    fun previous()
    fun status(): MpdStatus
    fun playlistinfo(): List<MpdPlaylistItem>
    fun clear()
    fun update()

}
