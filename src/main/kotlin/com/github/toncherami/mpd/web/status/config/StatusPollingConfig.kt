package com.github.toncherami.mpd.web.status.config

import com.github.toncherami.mpd.web.status.properties.StatusPollingProperties
import com.github.toncherami.mpd.web.status.services.StatusService
import org.springframework.context.annotation.Bean
import org.springframework.context.annotation.Configuration
import org.springframework.integration.dsl.IntegrationFlow
import org.springframework.integration.dsl.IntegrationFlows
import java.util.function.Supplier

const val STATUS_POLLING_CHANNEL_ID = "statusPollingChannel"
const val STATUS_POLLING_INTEGRATION_FLOW_ID = "statusIntegrationFlow"

@Configuration
class StatusPollingConfig(private val statusPollingProperties: StatusPollingProperties) {

    @Bean(STATUS_POLLING_INTEGRATION_FLOW_ID)
    fun statusIntegrationFlow(statusService: StatusService): IntegrationFlow {
        val supplier = Supplier(statusService::get)

        return IntegrationFlows.fromSupplier(supplier) { spec ->
                spec.autoStartup(false)
                spec.poller {
                    it.fixedDelay(statusPollingProperties.interval)
                }
            }
            .channel(STATUS_POLLING_CHANNEL_ID)
            .get()
    }

}
