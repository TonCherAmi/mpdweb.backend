package com.github.toncherami.mpd.web.queue.data

import com.github.toncherami.mpd.web.queue.data.enums.QueueSourceType

data class QueueSource(
    val id: String,
    val type: QueueSourceType,
)
