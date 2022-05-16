package com.github.toncherami.mpd.web.changes.handlers

import com.github.toncherami.mpd.web.changes.config.CHANGES_POLLING_CHANNEL_ID
import com.github.toncherami.mpd.web.changes.enums.Change
import com.github.toncherami.mpd.web.changes.services.ChangesPushService
import com.github.toncherami.mpd.web.common.services.WebSocketConnectionCountService
import com.github.toncherami.mpd.web.status.services.StatusPollingService
import org.springframework.integration.annotation.ServiceActivator
import org.springframework.stereotype.Component

@Component
class ChangesPollingHandler(
    private val changesPushService: ChangesPushService,
    private val statusPollingService: StatusPollingService,
    private val webSocketConnectionCountService: WebSocketConnectionCountService
) {

    private val areConnectionsPresent: Boolean
        get() = webSocketConnectionCountService.get() != 0

    @ServiceActivator(inputChannel = CHANGES_POLLING_CHANNEL_ID)
    fun handleChanges(changes: List<Change>) {
        maybeToggleStatusPolling(changes)

        changesPushService.push(changes)
    }

    private fun maybeToggleStatusPolling(changes: List<Change>) {
        val shouldToggleStatusPolling = areConnectionsPresent && changes.hasPlayerChanged

        if (shouldToggleStatusPolling) {
            statusPollingService.toggle()
        }
    }

    private val Collection<Change>.hasPlayerChanged
        get() = contains(Change.PLAYER)

}
