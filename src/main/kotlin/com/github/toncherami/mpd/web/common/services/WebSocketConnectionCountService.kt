package com.github.toncherami.mpd.web.common.services

interface WebSocketConnectionCountService {

    fun get(): Int
    fun increment()
    fun decrement()

}
