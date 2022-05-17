package com.github.toncherami.mpd.web.playlist.controllers

import com.github.toncherami.mpd.web.playlist.dto.PlaylistItem
import com.github.toncherami.mpd.web.playlist.dto.api.request.PlaylistAddBody
import com.github.toncherami.mpd.web.playlist.dto.api.request.PlaylistDeleteBody
import com.github.toncherami.mpd.web.playlist.dto.api.request.PlaylistReplaceBody
import com.github.toncherami.mpd.web.playlist.services.PlaylistService
import org.springframework.web.bind.annotation.DeleteMapping
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.PatchMapping
import org.springframework.web.bind.annotation.PutMapping
import org.springframework.web.bind.annotation.RequestBody
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.bind.annotation.RestController

@RestController
@RequestMapping("/playlist")
class PlaylistController(private val playlistService: PlaylistService) {

    @GetMapping
    fun get(): List<PlaylistItem> {
        return playlistService.get()
    }

    @PatchMapping
    fun add(@RequestBody body: PlaylistAddBody) {
        playlistService.add(body.uri)
    }

    @DeleteMapping
    fun delete(@RequestBody body: PlaylistDeleteBody) {
        if (body.id == null) {
            playlistService.clear()

            return
        }

        playlistService.delete(id = body.id)
    }

    @PutMapping
    fun replace(@RequestBody body: PlaylistReplaceBody) {
        playlistService.replace(body.uri)
    }

}
