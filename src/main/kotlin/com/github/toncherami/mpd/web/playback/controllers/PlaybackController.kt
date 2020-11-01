package com.github.toncherami.mpd.web.playback.controllers

import com.github.toncherami.mpd.web.playback.services.PlaybackService
import org.springframework.web.bind.annotation.PostMapping
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.bind.annotation.RestController

@RestController
@RequestMapping("/playback")
class PlaybackController(private val playbackService: PlaybackService) {

    @PostMapping("/stop")
    fun stop() {
        playbackService.stop()
    }

    @PostMapping("/toggle")
    fun toggle() {
        playbackService.toggle()
    }

    @PostMapping("/next")
    fun next() {
        playbackService.next()
    }

    @PostMapping("/prev")
    fun prev() {
        playbackService.prev()
    }

}
