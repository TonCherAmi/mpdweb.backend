package com.github.toncherami.mpd.web.adapter.services

interface MpdWriteOnlyService {

    fun play()
    fun stop()
    fun pause()
    fun next()
    fun previous()
    fun clear()
    fun update()
    fun add(uri: String)
    fun setvol(vol: Int)
    fun seekcur(time: String)
    fun playid(id: Int?)

}
