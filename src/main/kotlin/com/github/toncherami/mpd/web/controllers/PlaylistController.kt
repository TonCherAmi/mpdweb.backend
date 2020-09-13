package com.github.toncherami.mpd.web.controllers

import com.github.toncherami.mpd.web.dto.PlayerPlaylistItem
import com.github.toncherami.mpd.web.services.PlayerService
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.bind.annotation.RestController

@RestController
@RequestMapping("/playlist")
class PlaylistController(private val playerService: PlayerService) {

    @GetMapping
    fun get(): List<PlayerPlaylistItem> {
        return playerService.getPlaylistItems()
    }

}
