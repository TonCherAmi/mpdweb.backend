package com.github.toncherami.mpd.web.playback.services

interface PlaybackService {

    fun stop()
    fun toggle()
    fun next()
    fun prev()

}
