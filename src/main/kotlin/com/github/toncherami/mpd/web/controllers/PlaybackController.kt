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
        playerService.togglePlayback()

        return true
    }

    @PostMapping("/stop")
    fun stop(): Boolean {
        playerService.stopPlayback()

        return true
    }

}
