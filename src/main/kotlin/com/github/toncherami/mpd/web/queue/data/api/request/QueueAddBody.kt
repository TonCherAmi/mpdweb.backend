package com.github.toncherami.mpd.web.queue.data.api.request

import com.github.toncherami.mpd.web.queue.data.QueueSource

data class QueueAddBody(
    val source: QueueSource,
)
