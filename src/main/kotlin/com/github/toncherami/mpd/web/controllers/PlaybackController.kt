package com.github.toncherami.mpd.web.controllers

import com.github.toncherami.mpd.web.services.PlayerService
import org.springframework.web.bind.annotation.PostMapping
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.bind.annotation.RestController

@RestController
@RequestMapping("/playback")
class PlaybackController(private val playerService: PlayerService) {

    @PostMapping("/toggle")
    fun toggle(): Boolean {
        return false
    }

    @PostMapping("/play")
    fun play(): Boolean {
        return false
    }

    @PostMapping("/stop")
    fun stop(): Boolean {
        return false
    }

}
