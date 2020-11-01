package com.github.toncherami.mpd.web.playlist.controllers

import com.github.toncherami.mpd.web.database.dto.File
import com.github.toncherami.mpd.web.playlist.services.PlaylistService
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.PostMapping
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.bind.annotation.RestController

@RestController
@RequestMapping("/playlist")
class PlaylistController(private val playlistService: PlaylistService) {

    @GetMapping
    fun playlist(): List<File> {
        return playlistService.get()
    }

    @PostMapping("/clear")
    fun clear() {
        return playlistService.clear()
    }


}
