package com.github.toncherami.mpd.web.playback.controllers

import com.github.toncherami.mpd.web.playback.dto.api.request.PlaybackPlayBody
import com.github.toncherami.mpd.web.playback.dto.api.request.PlaybackSeekBody
import com.github.toncherami.mpd.web.playback.dto.api.request.enums.PlaybackSeekMode
import com.github.toncherami.mpd.web.playback.services.PlaybackService
import org.springframework.web.bind.annotation.PostMapping
import org.springframework.web.bind.annotation.RequestBody
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.bind.annotation.RestController
import javax.validation.Valid

@RestController
@RequestMapping("/playback")
class PlaybackController(private val playbackService: PlaybackService) {

    @PostMapping("/stop")
    fun stop() {
        playbackService.stop()
    }

    @PostMapping("/play")
    fun play(@RequestBody body: PlaybackPlayBody?) {
        playbackService.play(id = body?.id)
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

    @PostMapping("/seek")
    fun seek(@Valid @RequestBody body: PlaybackSeekBody) {
        when (body.mode) {
            PlaybackSeekMode.BACK -> playbackService.seekBack(body.time)
            PlaybackSeekMode.FORWARD -> playbackService.seekForward(body.time)
            PlaybackSeekMode.ABSOLUTE -> playbackService.seek(body.time)
        }
    }

    @PostMapping("/single/cycle")
    fun single() {
        playbackService.single()
    }

    @PostMapping("/repeat/toggle")
    fun repeat() {
        playbackService.repeat()
    }

    @PostMapping("/consume/toggle")
    fun consume() {
        playbackService.consume()
    }

    @PostMapping("/random/toggle")
    fun random() {
        playbackService.random()
    }

}
