package com.github.toncherami.mpd.web.configs

import com.github.toncherami.mpd.web.services.PlayerService
import org.springframework.context.annotation.Bean
import org.springframework.context.annotation.Configuration
import org.springframework.integration.channel.DirectChannel
import org.springframework.integration.core.MessageSource
import org.springframework.integration.dsl.IntegrationFlow
import org.springframework.integration.dsl.IntegrationFlows
import org.springframework.integration.endpoint.MethodInvokingMessageSource
import kotlin.reflect.jvm.javaMethod

const val PLAYER_POLL_CHANNEL_ID = "playerPollChannel"

@Configuration
class PlayerPollConfig {

    @Bean(PLAYER_POLL_CHANNEL_ID)
    fun messageChannel() = DirectChannel()

    @Bean
    fun integrationFlow(playerService: PlayerService): IntegrationFlow {
        return IntegrationFlows
            .from(playerStatusSource(playerService)) { spec ->
                spec.autoStartup(false)
                spec.poller {
                    it.fixedDelay(600)
                }
            }
            .channel(PLAYER_POLL_CHANNEL_ID)
            .get()
    }

    private fun playerStatusSource(playerService: PlayerService): MessageSource<*> {
        return MethodInvokingMessageSource().also {
            it.setObject(playerService)
            it.setMethod(PlayerService::getStatus.javaMethod)
        }
    }

}
