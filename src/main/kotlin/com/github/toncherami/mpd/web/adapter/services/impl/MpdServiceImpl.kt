package com.github.toncherami.mpd.web.adapter.services.impl

import com.fasterxml.jackson.databind.ObjectMapper
import com.github.toncherami.mpd.web.adapter.dto.MpdChange
import com.github.toncherami.mpd.web.adapter.dto.MpdPlaylistItem
import com.github.toncherami.mpd.web.adapter.dto.MpdStatus
import com.github.toncherami.mpd.web.adapter.gateways.MpdNoTimeoutTcpGateway
import com.github.toncherami.mpd.web.adapter.gateways.MpdTcpGateway
import com.github.toncherami.mpd.web.adapter.services.MpdService
import com.github.toncherami.mpd.web.adapter.utils.MpdBoolean
import com.github.toncherami.mpd.web.adapter.utils.MpdCommand
import com.github.toncherami.mpd.web.adapter.utils.MpdRequestHandler
import org.springframework.stereotype.Service

@Service
class MpdServiceImpl(
    objectMapper: ObjectMapper,
    @Suppress("SpringJavaInjectionPointsAutowiringInspection")
    mpdTcpClient: MpdTcpGateway,
    @Suppress("SpringJavaInjectionPointsAutowiringInspection")
    mpdNoTimeoutTcpClient: MpdNoTimeoutTcpGateway
) : MpdService {

    private val mpdRequestHandler = MpdRequestHandler(mpdTcpClient, objectMapper)
    private val mpdNoTimeoutRequestHandler = MpdRequestHandler(mpdNoTimeoutTcpClient, objectMapper)

    override fun idle(): List<MpdChange> {
        return mpdNoTimeoutRequestHandler.performListRequest(MpdCommand.IDLE)
    }

    override fun play() {
        mpdRequestHandler.performRequest<Unit>(MpdCommand.PLAY)
    }

    override fun stop() {
        mpdRequestHandler.performRequest<Unit>(MpdCommand.STOP)
    }

    override fun pause() {
        mpdRequestHandler.performRequest<Unit>(MpdCommand.PAUSE) {
            argument(MpdBoolean.TRUE.value)
        }
    }

    override fun next() {
        mpdRequestHandler.performRequest<Unit>(MpdCommand.NEXT)
    }

    override fun previous() {
        mpdRequestHandler.performRequest<Unit>(MpdCommand.PREVIOUS)
    }

    override fun clear() {
        mpdRequestHandler.performRequest<Unit>(MpdCommand.CLEAR)
    }

    override fun status(): MpdStatus {
        return mpdRequestHandler.performRequest(MpdCommand.STATUS)
    }

    override fun playlistinfo(): List<MpdPlaylistItem> {
        return mpdRequestHandler.performListRequest(MpdCommand.PLAYLISTINFO)
    }

}
