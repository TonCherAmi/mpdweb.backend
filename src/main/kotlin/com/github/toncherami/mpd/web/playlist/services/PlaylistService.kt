package com.github.toncherami.mpd.web.playlist.services

import com.github.toncherami.mpd.web.playlist.dto.PlaylistItem

interface PlaylistService {

    fun get(): List<PlaylistItem>

}
