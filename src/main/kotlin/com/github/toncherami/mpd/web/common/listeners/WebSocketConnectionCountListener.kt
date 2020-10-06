package com.github.toncherami.mpd.web.common.listeners

import com.github.toncherami.mpd.web.common.services.WebSocketConnectionCountService
import org.springframework.context.event.EventListener
import org.springframework.core.Ordered
import org.springframework.core.annotation.Order
import org.springframework.stereotype.Component
import org.springframework.web.socket.messaging.SessionConnectedEvent
import org.springframework.web.socket.messaging.SessionDisconnectEvent

@Component
class WebSocketConnectionCountListener(private val webSocketConnectionCountService: WebSocketConnectionCountService) {

    @Order(Ordered.HIGHEST_PRECEDENCE)
    @EventListener(SessionConnectedEvent::class)
    fun handleConnectedEvent() {
        webSocketConnectionCountService.increment()
    }

    @Order(Ordered.HIGHEST_PRECEDENCE)
    @EventListener(SessionDisconnectEvent::class)
    fun handleDisconnectEvent(event: SessionDisconnectEvent) {
        webSocketConnectionCountService.decrement()
    }

}
