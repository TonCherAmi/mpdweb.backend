package com.github.toncherami.mpd.web.pollers

import com.github.toncherami.mpd.web.configs.PLAYER_POLL_CHANNEL_ID
import com.github.toncherami.mpd.web.configs.STOMP_PLAYER_STATUS_DESTINATION
import com.github.toncherami.mpd.web.dto.PlayerStatus
import org.springframework.context.event.EventListener
import org.springframework.integration.annotation.ServiceActivator
import org.springframework.integration.dsl.IntegrationFlow
import org.springframework.integration.dsl.StandardIntegrationFlow
import org.springframework.messaging.simp.SimpMessagingTemplate
import org.springframework.stereotype.Component
import org.springframework.web.socket.messaging.SessionDisconnectEvent
import org.springframework.web.socket.messaging.SessionSubscribeEvent

@Component
class PlayerPoller(
    integrationFlow: IntegrationFlow,
    private val simpMessagingTemplate: SimpMessagingTemplate
) {

    private var connectionCount = 0

    private val playerPollIntegrationFlow = integrationFlow
        as? StandardIntegrationFlow
        ?: throw RuntimeException("Something is very wrong with Spring Integration")

    @ServiceActivator(inputChannel = PLAYER_POLL_CHANNEL_ID)
    fun handlePlayerStatus(status: PlayerStatus) {
        simpMessagingTemplate.convertAndSend(STOMP_PLAYER_STATUS_DESTINATION, status)
    }

    @EventListener(SessionSubscribeEvent::class)
    fun handleSubscribeEvent() {
        if (connectionCount++ == 0) {
            playerPollIntegrationFlow.start()
        }
    }

    @EventListener(SessionDisconnectEvent::class)
    fun handleUnsubscribeEvent() {
        if (--connectionCount == 0) {
            playerPollIntegrationFlow.stop()
        }
    }

}
