package com.github.toncherami.mpd.web.playback.services

interface PlaybackService {

    fun stop()
    fun toggle()
    fun next()
    fun prev()
    fun seek(time: Double)
    fun seekBack(time: Double)
    fun seekForward(time: Double)
    fun play(id: Int?)

}
