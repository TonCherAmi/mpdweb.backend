package com.github.toncherami.mpd.web.status.services.impl

import com.github.toncherami.mpd.web.status.config.STATUS_POLLING_INTEGRATION_FLOW_ID
import com.github.toncherami.mpd.web.status.data.isPlaying
import com.github.toncherami.mpd.web.status.services.StatusPollingService
import com.github.toncherami.mpd.web.status.services.StatusService
import org.springframework.beans.factory.annotation.Qualifier
import org.springframework.integration.dsl.IntegrationFlow
import org.springframework.integration.dsl.StandardIntegrationFlow
import org.springframework.stereotype.Service

@Service
class StatusPollingServiceImpl(
    @Qualifier(STATUS_POLLING_INTEGRATION_FLOW_ID)
    statusIntegrationFlow: IntegrationFlow,
    private val statusService: StatusService
) : StatusPollingService {

    private val statusIntegrationFlow = statusIntegrationFlow as? StandardIntegrationFlow
        ?: throw RuntimeException("Something is very wrong with Spring Integration")

    override fun stop() {
        statusIntegrationFlow.stop()
    }

    override fun toggle() {
        if (statusService.get().isPlaying) {
            statusIntegrationFlow.start()
        } else {
            statusIntegrationFlow.stop()
        }
    }

}
