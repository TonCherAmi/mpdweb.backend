package com.github.toncherami.mpd.web.adapter.services.impl

import com.fasterxml.jackson.databind.ObjectMapper
import com.github.toncherami.mpd.web.adapter.dto.MpdBinarySize
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
import java.io.ByteArrayOutputStream

@Service
class MpdServiceImpl(
    objectMapper: ObjectMapper,
    @Suppress("SpringJavaInjectionPointsAutowiringInspection")
    mpdTcpClient: MpdTcpGateway,
    @Suppress("SpringJavaInjectionPointsAutowiringInspection")
    mpdNoTimeoutTcpClient: MpdNoTimeoutTcpGateway,
) : MpdService {

    private val mpdRequestHandler = MpdRequestHandler(mpdTcpClient, objectMapper)
    private val mpdNoTimeoutRequestHandler = MpdRequestHandler(mpdNoTimeoutTcpClient, objectMapper)

    override fun idle(): List<MpdChange> {
        return mpdNoTimeoutRequestHandler.getList(MpdCommand.IDLE)
    }

    override fun play() {
        mpdRequestHandler.perform(MpdCommand.PLAY)
    }

    override fun stop() {
        mpdRequestHandler.perform(MpdCommand.STOP)
    }

    override fun pause() {
        mpdRequestHandler.perform(MpdCommand.PAUSE) {
            argument(MpdBoolean.TRUE.value)
        }
    }

    override fun next() {
        mpdRequestHandler.perform(MpdCommand.NEXT)
    }

    override fun previous() {
        mpdRequestHandler.perform(MpdCommand.PREVIOUS)
    }

    override fun clear() {
        mpdRequestHandler.perform(MpdCommand.CLEAR)
    }

    override fun status(): MpdStatus {
        return mpdRequestHandler.get(MpdCommand.STATUS)
    }

    override fun playlistinfo(): List<MpdFile> {
        return mpdRequestHandler.getList(MpdCommand.PLAYLISTINFO)
    }

    override fun update() {
        mpdRequestHandler.perform(MpdCommand.UPDATE)
    }

    override fun lsinfo(uri: String): List<MpdDatabaseItem> {
        return mpdRequestHandler.getList(MpdCommand.LSINFO) {
            argument(uri)
        }
    }

    override fun add(uri: String) {
        mpdRequestHandler.perform(MpdCommand.ADD) {
            argument(uri)
        }
    }

    override fun count(vararg filter: String): MpdCount {
        return mpdRequestHandler.get(MpdCommand.COUNT) {
            filter.forEach(::argument)
        }
    }

    override fun search(mpdRegexFileFilter: MpdRegexFileFilter): List<MpdDatabaseItem> {
        val escapedRegex = escapeArgument(mpdRegexFileFilter.regex)

        return mpdRequestHandler.getList(MpdCommand.SEARCH) {
            argument(
                "(file =~ '$escapedRegex')"
            )
        }
    }

    override fun albumart(uri: String): ByteArray {
        val (data, binary) = mpdRequestHandler.getBinary<MpdBinarySize>(MpdCommand.ALBUMART) {
            argument(uri)
            argument("0")
        }

        val outputStream = ByteArrayOutputStream(data.size)

        outputStream.writeBytes(binary)

        var current = data.binary

        while (current < data.size) {
            val (moreData, moreBinary) = mpdRequestHandler.getBinary<MpdBinarySize>(MpdCommand.ALBUMART) {
                argument(uri)
                argument(current.toString())
            }

            current += moreData.binary

            outputStream.writeBytes(moreBinary)
        }

        return outputStream.toByteArray()
    }

    // TODO: this is not really sufficient
    private fun escapeArgument(argument: String): String {
        return argument
            .replace("'", """\\'""")
            .replace(""""""", """\\\"""")
    }

}
