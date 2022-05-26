package com.github.toncherami.mpd.web.queue.services

import com.github.toncherami.mpd.web.queue.data.QueueItem
import com.github.toncherami.mpd.web.queue.data.QueueSource

interface QueueService {

    fun get(): List<QueueItem>
    fun add(source: QueueSource)
    fun clear()
    fun replace(source: QueueSource)
    fun delete(id: Int)

}
