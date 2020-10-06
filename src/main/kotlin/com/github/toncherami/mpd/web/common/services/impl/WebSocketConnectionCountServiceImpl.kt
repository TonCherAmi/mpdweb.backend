package com.github.toncherami.mpd.web.common.services.impl

import com.github.toncherami.mpd.web.common.services.WebSocketConnectionCountService
import org.springframework.stereotype.Service
import java.util.concurrent.atomic.AtomicInteger

@Service
class WebSocketConnectionCountServiceImpl : WebSocketConnectionCountService {

    var connectionCount = AtomicInteger(0)

    override fun get(): Int {
        return connectionCount.get()
    }

    override fun increment() {
        connectionCount.incrementAndGet()
    }

    override fun decrement() {
        connectionCount.decrementAndGet()
    }

}
