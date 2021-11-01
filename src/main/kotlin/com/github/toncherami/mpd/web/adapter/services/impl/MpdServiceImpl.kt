package com.github.toncherami.mpd.web.adapter.services.impl

import com.fasterxml.jackson.databind.ObjectMapper
import com.github.toncherami.mpd.web.adapter.dto.MpdChange
import com.github.toncherami.mpd.web.adapter.dto.MpdCount
import com.github.toncherami.mpd.web.adapter.dto.MpdDatabaseItem
import com.github.toncherami.mpd.web.adapter.dto.MpdFile
import com.github.toncherami.mpd.web.adapter.dto.MpdRegexFileFilter
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
        return mpdNoTimeoutRequestHandler.performListRequest(MpdCommand.IDLE, listOf("changed"))
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

    override fun playlistinfo(): List<MpdFile> {
        return mpdRequestHandler.performListRequest(MpdCommand.PLAYLISTINFO, listOf("file"))
    }

    override fun update() {
        mpdRequestHandler.performRequest<Unit>(MpdCommand.UPDATE)
    }

    override fun lsinfo(uri: String): List<MpdDatabaseItem> {
        return mpdRequestHandler.performListRequest(
            MpdCommand.LSINFO,
            listOf("file", "playlist", "directory")
        ) { argument(uri) }
    }

    override fun add(uri: String) {
        mpdRequestHandler.performRequest<Unit>(MpdCommand.ADD) {
            argument(uri)
        }
    }

    override fun count(vararg filter: String): MpdCount {
        return mpdRequestHandler.performRequest(MpdCommand.COUNT) {
            filter.forEach(::argument)
        }
    }

    override fun search(mpdRegexFileFilter: MpdRegexFileFilter): List<MpdDatabaseItem> {
        val escapedRegex = escapeArgument(mpdRegexFileFilter.regex)

        return mpdRequestHandler.performListRequest(MpdCommand.SEARCH, listOf("file")) {
            argument(
                "(file =~ '$escapedRegex')"
            )
        }
    }

    // TODO: this is not really sufficient
    private fun escapeArgument(argument: String): String {
        return argument
            .replace("'", """\\'""")
            .replace(""""""", """\\\"""")
    }

}
