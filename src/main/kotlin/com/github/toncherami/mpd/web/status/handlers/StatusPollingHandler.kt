package com.github.toncherami.mpd.web.status.handlers

import com.github.toncherami.mpd.web.status.config.STATUS_POLLING_CHANNEL_ID
import com.github.toncherami.mpd.web.status.dto.Status
import com.github.toncherami.mpd.web.status.services.StatusService
import org.springframework.integration.annotation.ServiceActivator
import org.springframework.stereotype.Component

@Component
class StatusPollingHandler(private val statusService: StatusService) {

    @ServiceActivator(inputChannel = STATUS_POLLING_CHANNEL_ID)
    fun handleStatus(status: Status) {
        statusService.send(status)
    }

}
