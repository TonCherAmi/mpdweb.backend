package com.github.toncherami.mpd.web.common.config

import org.springframework.context.annotation.Configuration
import org.springframework.messaging.simp.config.MessageBrokerRegistry
import org.springframework.web.socket.config.annotation.EnableWebSocketMessageBroker
import org.springframework.web.socket.config.annotation.StompEndpointRegistry
import org.springframework.web.socket.config.annotation.WebSocketMessageBrokerConfigurer

const val STOMP_CONNECT_ENDPOINT = "/connect"

const val STOMP_PLAYER_STATUS_DESTINATION = "/status"
const val STOMP_PLAYER_CHANGES_DESTINATION = "/changes"

@Configuration
@EnableWebSocketMessageBroker
class WebSocketConfig : WebSocketMessageBrokerConfigurer {

    override fun registerStompEndpoints(registry: StompEndpointRegistry) {
        registry
            .addEndpoint(STOMP_CONNECT_ENDPOINT)
            .setAllowedOrigins("*")
    }

    override fun configureMessageBroker(registry: MessageBrokerRegistry) {
        registry.enableSimpleBroker(
            STOMP_PLAYER_STATUS_DESTINATION,
            STOMP_PLAYER_CHANGES_DESTINATION
        )
    }

}
