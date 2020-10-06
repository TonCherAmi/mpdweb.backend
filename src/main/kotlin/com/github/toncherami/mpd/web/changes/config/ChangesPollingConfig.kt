package com.github.toncherami.mpd.web.changes.config

import com.github.toncherami.mpd.web.changes.services.ChangesService
import org.springframework.context.annotation.Bean
import org.springframework.context.annotation.Configuration
import org.springframework.integration.core.MessageSource
import org.springframework.integration.dsl.IntegrationFlow
import org.springframework.integration.dsl.IntegrationFlows
import org.springframework.integration.endpoint.MethodInvokingMessageSource
import kotlin.reflect.jvm.javaMethod

const val CHANGES_POLLING_CHANNEL_ID = "changesPollingChannel"
const val CHANGES_POLLING_INTEGRATION_FLOW_ID = "changesIntegrationFlow"

@Configuration
class ChangesPollingConfig {

    @Bean(CHANGES_POLLING_INTEGRATION_FLOW_ID)
    fun changesIntegrationFlow(changesService: ChangesService): IntegrationFlow {
        return IntegrationFlows
            .from(getChangesSource(changesService)) { spec ->
                spec.autoStartup(true)
                spec.poller {
                    it.fixedDelay(0L)
                }
            }
            .channel(CHANGES_POLLING_CHANNEL_ID)
            .get()
    }

    private fun getChangesSource(changesService: ChangesService): MessageSource<*> {
        return MethodInvokingMessageSource().also {
            it.setObject(changesService)
            it.setMethod(changesService::get.javaMethod)
        }
    }

}
