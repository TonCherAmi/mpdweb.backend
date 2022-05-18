package com.github.toncherami.mpd.web.queue.services

import com.github.toncherami.mpd.web.queue.data.QueueItem

interface QueueService {

    fun get(): List<QueueItem>
    fun add(uri: String)
    fun clear()
    fun replace(uri: String)
    fun delete(id: Int)

}
