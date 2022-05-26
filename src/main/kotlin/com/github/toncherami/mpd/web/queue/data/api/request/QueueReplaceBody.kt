package com.github.toncherami.mpd.web.queue.data.api.request

import com.github.toncherami.mpd.web.queue.data.QueueSource

data class QueueReplaceBody(
    val source: QueueSource,
)
