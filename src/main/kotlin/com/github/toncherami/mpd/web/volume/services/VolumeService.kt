package com.github.toncherami.mpd.web.volume.services

interface VolumeService {

    fun set(volume: Int)
    fun inc(volume: Int)
    fun dec(volume: Int)

}
