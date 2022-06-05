package com.github.toncherami.mpd.web.adapter.services.impl

import com.fasterxml.jackson.databind.ObjectMapper
import com.github.toncherami.mpd.web.adapter.data.MpdBinarySize
import com.github.toncherami.mpd.web.adapter.data.MpdChange
import com.github.toncherami.mpd.web.adapter.data.MpdCount
import com.github.toncherami.mpd.web.adapter.data.MpdDatabaseFile
import com.github.toncherami.mpd.web.adapter.data.MpdDatabaseItem
import com.github.toncherami.mpd.web.adapter.data.MpdPlaylist
import com.github.toncherami.mpd.web.adapter.data.MpdPlaylistItem
import com.github.toncherami.mpd.web.adapter.data.MpdRegexFileFilter
import com.github.toncherami.mpd.web.adapter.data.MpdStatus
import com.github.toncherami.mpd.web.adapter.deserializers.data.enums.MpdErrorCode
import com.github.toncherami.mpd.web.adapter.exceptions.MpdException
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

    override fun playid(id: Int?) {
        mpdRequestHandler.perform(MpdCommand.PLAYID) {
            if (id != null) {
                argument(id.toString())
            }
        }
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

    override fun single(state: String) {
        mpdRequestHandler.perform(MpdCommand.SINGLE) {
            argument(state)
        }
    }

    override fun random(state: String) {
        mpdRequestHandler.perform(MpdCommand.RANDOM) {
            argument(state)
        }
    }

    override fun repeat(state: String) {
        mpdRequestHandler.perform(MpdCommand.REPEAT) {
            argument(state)
        }
    }

    override fun consume(state: String) {
        mpdRequestHandler.perform(MpdCommand.CONSUME) {
            argument(state)
        }
    }

    override fun listplaylists(): List<MpdPlaylist> {
        return mpdRequestHandler.retrieveList(MpdCommand.LISTPLAYLISTS)
    }

    override fun listplaylistinfo(name: String): List<MpdDatabaseFile> {
        return mpdRequestHandler.retrieveList(MpdCommand.LISTPLAYLISTINFO) {
            argument(name)
        }
    }

    override fun albumart(uri: String): ByteArray {
        return retrieveAllBinary(MpdCommand.ALBUMART, listOf(uri))
    }

    override fun readpicture(uri: String): ByteArray {
        return retrieveAllBinary(MpdCommand.READPICTURE, listOf(uri))
    }

    private fun retrieveAllBinary(mpdCommand: MpdCommand, arguments: List<String>): ByteArray {
        val (data, binary) = mpdRequestHandler.retrieveBinary<MpdBinarySize>(mpdCommand) {
            arguments.forEach(this::argument)

            argument("0")
        }

        if (data.size == 0) {
            throw MpdException(MpdErrorCode.NO_EXIST, mpdCommand.value, 0, "empty binary response")
        }

        val outputStream = ByteArrayOutputStream(data.size)

        outputStream.writeBytes(binary)

        var current = data.binary

        while (current < data.size) {
            val (moreData, moreBinary) = mpdRequestHandler.retrieveBinary<MpdBinarySize>(mpdCommand) {
                arguments.forEach(this::argument)

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

    override fun deleteid(id: Int) {
        mpdRequestHandler.perform(MpdCommand.DELETEID) {
            argument(id.toString())
        }
    }

    override fun rm(name: String) {
        mpdRequestHandler.perform(MpdCommand.RM) {
            argument(name)
        }
    }

    override fun load(name: String) {
        mpdRequestHandler.perform(MpdCommand.LOAD) {
            argument(name)
        }
    }

    override fun rename(name: String, newName: String) {
        mpdRequestHandler.perform(MpdCommand.RENAME) {
            argument(name)
            argument(newName)
        }
    }

    override fun playlistadd(name: String, uri: String) {
        mpdRequestHandler.perform(MpdCommand.PLAYLISTADD) {
            argument(name)
            argument(uri)
        }
    }

    override fun playlistdelete(name: String, pos: Int) {
        mpdRequestHandler.perform(MpdCommand.PLAYLISTDELETE) {
            argument(name)
            argument(pos.toString())
        }
    }

    override fun commandList(fn: MpdWriteOnlyService.() -> Unit) {
        val mpdCommandBuilder = MpdCommandBuilder.command(MpdCommand.COMMAND_LIST_BEGIN)

        object : MpdWriteOnlyService {
            override fun play() {
                mpdCommandBuilder.command(MpdCommand.PLAY)
            }

            override fun playid(id: Int?) {
                mpdCommandBuilder.command(MpdCommand.PLAYID).apply {
                    if (id != null) {
                        argument(id.toString())
                    }
                }
            }

            override fun single(state: String) {
                mpdCommandBuilder.command(MpdCommand.SINGLE)
                    .argument(state)
            }

            override fun random(state: String) {
                mpdCommandBuilder.command(MpdCommand.RANDOM)
                    .argument(state)
            }

            override fun repeat(state: String) {
                mpdCommandBuilder.command(MpdCommand.REPEAT)
                    .argument(state)
            }

            override fun consume(state: String) {
                mpdCommandBuilder.command(MpdCommand.CONSUME)
                    .argument(state)
            }

            override fun deleteid(id: Int) {
                mpdCommandBuilder.command(MpdCommand.DELETEID)
                    .argument(id.toString())
            }

            override fun rm(name: String) {
                mpdCommandBuilder.command(MpdCommand.RM)
                    .argument(name)
            }

            override fun load(name: String) {
                mpdCommandBuilder.command(MpdCommand.LOAD)
                    .argument(name)
            }

            override fun rename(name: String, newName: String) {
                mpdCommandBuilder.command(MpdCommand.RENAME)
                    .argument(name)
                    .argument(newName)
            }

            override fun playlistadd(name: String, uri: String) {
                mpdCommandBuilder.command(MpdCommand.PLAYLISTADD)
                    .argument(name)
                    .argument(uri)
            }

            override fun playlistdelete(name: String, pos: Int) {
                mpdCommandBuilder.command(MpdCommand.PLAYLISTDELETE)
                    .argument(name)
                    .argument(pos.toString())
            }

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
