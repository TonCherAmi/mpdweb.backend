package com.github.toncherami.mpd.web.status.listeners

import com.github.toncherami.mpd.web.common.services.WebSocketConnectionCountService
import com.github.toncherami.mpd.web.status.services.StatusPollingService
import org.springframework.context.event.EventListener
import org.springframework.stereotype.Component
import org.springframework.web.socket.messaging.SessionConnectedEvent
import org.springframework.web.socket.messaging.SessionDisconnectEvent

@Component
class StatusPollingListener(
    private val statusPollingService: StatusPollingService,
    private val webSocketConnectionCountService: WebSocketConnectionCountService
) {

    private var connectionCount = 0

    @EventListener(SessionConnectedEvent::class)
    fun handleConnectedEvent() {
        if (webSocketConnectionCountService.get() == 1) {
            statusPollingService.toggle()
        }
    }

    @EventListener(SessionDisconnectEvent::class)
    fun handleDisconnectEvent(event: SessionDisconnectEvent) {
        if (webSocketConnectionCountService.get() == 0) {
            statusPollingService.stop()
        }
    }

}
