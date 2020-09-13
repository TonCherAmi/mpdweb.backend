package com.github.toncherami.mpd.web.services.impl

import com.fasterxml.jackson.databind.ObjectMapper
import com.github.toncherami.mpd.web.dto.PlayerPlaylistItem
import com.github.toncherami.mpd.web.dto.PlayerStatus
import com.github.toncherami.mpd.web.dto.isPlaying
import com.github.toncherami.mpd.web.dto.isStopped
import com.github.toncherami.mpd.web.services.PlayerService
import com.github.toncherami.mpd.web.tcpclients.MpdTcpClient
import com.github.toncherami.mpd.web.utils.MpdBoolean
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

    override fun stopPlayback() {
        mpdRequestHandler.performRequest<Unit>(MpdCommand.STOP)
    }

    override fun startPlayback() {
        mpdRequestHandler.performRequest<Unit>(MpdCommand.PLAY)
    }

    override fun togglePlayback() {
        val status = getStatus()

        if (status.isStopped) {
            return startPlayback()
        }

        val shouldPause = if (!status.isPlaying) MpdBoolean.FALSE else MpdBoolean.TRUE

        mpdRequestHandler.performRequest<Unit> {
            command(MpdCommand.PAUSE)
            argument(shouldPause.value)
        }
    }

    override fun getStatus(): PlayerStatus {
        return mpdRequestHandler.performRequest(MpdCommand.STATUS)
    }

    override fun getPlaylistItems(): List<PlayerPlaylistItem> {
        return mpdRequestHandler.performListRequest(MpdCommand.PLAYLISTINFO)
    }

}
