package com.github.toncherami.mpd.web.queue.services.impl

import com.github.toncherami.mpd.web.adapter.services.MpdService
import com.github.toncherami.mpd.web.adapter.services.MpdWriteOnlyService
import com.github.toncherami.mpd.web.queue.data.QueueItem
import com.github.toncherami.mpd.web.queue.data.QueueSource
import com.github.toncherami.mpd.web.queue.data.enums.QueueSourceType
import com.github.toncherami.mpd.web.queue.services.QueueService
import org.springframework.stereotype.Service

@Service
class QueueServiceImpl(private val mpdService: MpdService) : QueueService {

    override fun get(): List<QueueItem> {
        return mpdService.playlistinfo().map(QueueItem::of)
    }

    override fun add(source: QueueSource) {
        mpdService.add(source.id, source.type)
    }

    override fun clear() {
        mpdService.clear()
    }

    override fun replace(source: QueueSource) {
        mpdService.commandList {
            clear()

            add(source.id, source.type)

            play()
        }
    }

    override fun delete(id: Int) {
        mpdService.deleteid(id)
    }

    private fun MpdWriteOnlyService.add(id: String, type: QueueSourceType) {
        when (type) {
            QueueSourceType.FILE -> add(id)
            QueueSourceType.PLAYLIST -> load(id)
        }
    }

}
