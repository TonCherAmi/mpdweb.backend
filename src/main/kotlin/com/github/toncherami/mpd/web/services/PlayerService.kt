package com.github.toncherami.mpd.web.services

import com.github.toncherami.mpd.web.dto.PlayerPlaylistItem
import com.github.toncherami.mpd.web.dto.PlayerStatus

interface PlayerService {

    fun stopPlayback()
    fun startPlayback()
    fun togglePlayback()
    fun getStatus(): PlayerStatus
    fun getPlaylistItems(): List<PlayerPlaylistItem>

}
