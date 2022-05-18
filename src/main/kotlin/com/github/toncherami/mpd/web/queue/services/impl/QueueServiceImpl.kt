package com.github.toncherami.mpd.web.queue.services.impl

import com.github.toncherami.mpd.web.adapter.services.MpdService
import com.github.toncherami.mpd.web.queue.data.QueueItem
import com.github.toncherami.mpd.web.queue.services.QueueService
import org.springframework.stereotype.Service

@Service
class QueueServiceImpl(private val mpdService: MpdService) : QueueService {

    override fun get(): List<QueueItem> {
        return mpdService.playlistinfo().map(QueueItem::of)
    }

    override fun add(uri: String) {
        mpdService.add(uri)
    }

    override fun clear() {
        mpdService.clear()
    }

    override fun replace(uri: String) {
        mpdService.commandList {
            clear()
            add(uri)
            play()
        }
    }

    override fun delete(id: Int) {
        mpdService.deleteid(id)
    }

}
