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
import com.github.toncherami.mpd.web.adapter.services.MpdWriteOnlyService
import com.github.toncherami.mpd.web.adapter.utils.MpdBoolean
import com.github.toncherami.mpd.web.adapter.utils.MpdCommand
import com.github.toncherami.mpd.web.adapter.utils.MpdCommandBuilder
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
        return mpdNoTimeoutRequestHandler.retrieveList(MpdCommand.IDLE)
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
        return mpdRequestHandler.retrieve(MpdCommand.STATUS)
    }

    override fun playlistinfo(): List<MpdPlaylistItem> {
        return mpdRequestHandler.retrieveList(MpdCommand.PLAYLISTINFO)
    }

    override fun update() {
        mpdRequestHandler.perform(MpdCommand.UPDATE)
    }

    override fun lsinfo(uri: String): List<MpdDatabaseItem> {
        return mpdRequestHandler.retrieveList(MpdCommand.LSINFO) {
            argument(uri)
        }
    }

    override fun add(uri: String) {
        mpdRequestHandler.perform(MpdCommand.ADD) {
            argument(uri)
        }
    }

    override fun count(vararg filter: String): MpdCount {
        return mpdRequestHandler.retrieve(MpdCommand.COUNT) {
            filter.forEach(::argument)
        }
    }

    override fun search(mpdRegexFileFilter: MpdRegexFileFilter): List<MpdDatabaseItem> {
        val escapedRegex = escapeArgument(mpdRegexFileFilter.regex)

        return mpdRequestHandler.retrieveList(MpdCommand.SEARCH) {
            argument(
                "(file =~ '$escapedRegex')"
            )
        }
    }

    override fun albumart(uri: String): ByteArray {
        val (data, binary) = mpdRequestHandler.retrieveBinary<MpdBinarySize>(MpdCommand.ALBUMART) {
            argument(uri)
            argument("0")
        }

        val outputStream = ByteArrayOutputStream(data.size)

        outputStream.writeBytes(binary)

        var current = data.binary

        while (current < data.size) {
            val (moreData, moreBinary) = mpdRequestHandler.retrieveBinary<MpdBinarySize>(MpdCommand.ALBUMART) {
                argument(uri)
                argument(current.toString())
            }

            current += moreData.binary

            outputStream.writeBytes(moreBinary)
        }

        return outputStream.toByteArray()
    }

    override fun setvol(vol: Int) {
        mpdRequestHandler.perform(MpdCommand.SETVOL) {
            argument(vol.toString())
        }
    }

    override fun seekcur(time: String) {
        mpdRequestHandler.perform(MpdCommand.SEEKCUR) {
            argument(time)
        }
    }

    override fun commandList(fn: MpdWriteOnlyService.() -> Unit) {
        val mpdCommandBuilder = MpdCommandBuilder.command(MpdCommand.COMMAND_LIST_BEGIN)

        object : MpdWriteOnlyService {

            override fun stop() {
                mpdCommandBuilder.command(MpdCommand.STOP)
            }

            override fun pause() {
                mpdCommandBuilder.command(MpdCommand.PAUSE)
            }

            override fun next() {
                mpdCommandBuilder.command(MpdCommand.NEXT)
            }

            override fun previous() {
                mpdCommandBuilder.command(MpdCommand.PREVIOUS)
            }

            override fun clear() {
                mpdCommandBuilder.command(MpdCommand.CLEAR)
            }

            override fun update() {
                mpdCommandBuilder.command(MpdCommand.UPDATE)
            }

            override fun add(uri: String) {
                mpdCommandBuilder.command(MpdCommand.ADD)
                    .argument(uri)
            }

            override fun setvol(vol: Int) {
                mpdCommandBuilder.command(MpdCommand.SETVOL)
                    .argument(vol.toString())
            }

            override fun seekcur(time: String) {
                mpdCommandBuilder.command(MpdCommand.SEEKCUR)
                    .argument(time)
            }
        }.fn()

        val command = mpdCommandBuilder.command(MpdCommand.COMMAND_LIST_END)
            .build()

        mpdRequestHandler.perform(command)
    }

    // TODO: this is not really sufficient
    private fun escapeArgument(argument: String): String {
        return argument
            .replace("'", """\\'""")
            .replace(""""""", """\\\"""")
    }

}
