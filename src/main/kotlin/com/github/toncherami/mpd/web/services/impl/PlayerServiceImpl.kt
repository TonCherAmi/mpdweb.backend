package com.github.toncherami.mpd.web.services.impl

import com.fasterxml.jackson.databind.ObjectMapper
import com.github.toncherami.mpd.web.dto.PlayerPlaylistItem
import com.github.toncherami.mpd.web.dto.PlayerStatus
import com.github.toncherami.mpd.web.services.PlayerService
import com.github.toncherami.mpd.web.tcpclients.MpdTcpClient
import com.github.toncherami.mpd.web.utils.MpdCommand
import com.github.toncherami.mpd.web.utils.MpdRequestHandler
import org.springframework.stereotype.Service

@Service
class PlayerServiceImpl(
    objectMapper: ObjectMapper,
    @Suppress("SpringJavaInjectionPointsAutowiringInspection")
    private val mpdTcpClient: MpdTcpClient
) : PlayerService {

    private val mpdRequestHandler = MpdRequestHandler(mpdTcpClient, objectMapper)

    override fun getStatus(): PlayerStatus {
        return mpdRequestHandler.performRequest(MpdCommand.STATUS)
    }

    override fun getPlaylistItems(): List<PlayerPlaylistItem> {
        return mpdRequestHandler.performListRequest(MpdCommand.PLAYLISTINFO)
    }

}
