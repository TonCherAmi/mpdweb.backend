package com.github.toncherami.mpd.web.playlists.controllers

import com.github.toncherami.mpd.web.database.data.DatabaseFile
import com.github.toncherami.mpd.web.playlists.data.Playlist
import com.github.toncherami.mpd.web.playlists.data.api.request.PlaylistAddBody
import com.github.toncherami.mpd.web.playlists.data.api.request.PlaylistDeleteBody
import com.github.toncherami.mpd.web.playlists.data.api.request.PlaylistRenameBody
import com.github.toncherami.mpd.web.playlists.services.PlaylistService
import org.springframework.web.bind.annotation.DeleteMapping
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.PathVariable
import org.springframework.web.bind.annotation.PostMapping
import org.springframework.web.bind.annotation.RequestBody
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.bind.annotation.RestController
import javax.validation.Valid

@RestController
@RequestMapping("/playlists")
class PlaylistController(private val playlistService: PlaylistService) {

    @GetMapping
    fun get(): List<Playlist> {
        return playlistService.get()
    }

    @GetMapping("/{name}/files")
    fun get(@PathVariable name: String): List<DatabaseFile> {
        return playlistService.getFiles(name)
    }

    @DeleteMapping("/{name}")
    fun delete(@PathVariable name: String) {
        playlistService.delete(name)
    }

    @PostMapping("/{from}")
    fun rename(@PathVariable from: String, @Valid @RequestBody body: PlaylistRenameBody) {
        playlistService.rename(from, body.to)
    }

    @PostMapping("/{name}/files")
    fun update(@PathVariable name: String, @RequestBody body: PlaylistAddBody) {
        playlistService.addTracks(name, body.uri)
    }

    @DeleteMapping("/{name}/files")
    fun delete(@PathVariable name: String, @RequestBody body: PlaylistDeleteBody) {
        playlistService.deleteFiles(name, body.positions)
    }

}
