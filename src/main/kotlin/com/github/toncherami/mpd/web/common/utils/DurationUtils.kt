package com.github.toncherami.mpd.web.common.utils

import java.time.Duration
import java.util.concurrent.TimeUnit

fun Double.toDuration(timeUnit: TimeUnit): Duration {
    return times(timeUnit.toMillis(1))
        .toLong()
        .let(Duration::ofMillis)
}
