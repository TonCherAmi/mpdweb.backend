package com.github.toncherami.mpd.web.playlist.controllers

import com.github.toncherami.mpd.web.database.dto.DatabaseFile
import com.github.toncherami.mpd.web.playlist.dto.api.request.PlaylistAddBody
import com.github.toncherami.mpd.web.playlist.services.PlaylistService
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.PostMapping
import org.springframework.web.bind.annotation.RequestBody
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.bind.annotation.RestController

@RestController
@RequestMapping("/playlist")
class PlaylistController(private val playlistService: PlaylistService) {

    @GetMapping
    fun playlist(): List<DatabaseFile> {
        return playlistService.get()
    }

    @PostMapping("/add")
    fun add(@RequestBody body: PlaylistAddBody) {
        playlistService.add(body.uri)
    }

    @PostMapping("/clear")
    fun clear() {
        return playlistService.clear()
    }

}
