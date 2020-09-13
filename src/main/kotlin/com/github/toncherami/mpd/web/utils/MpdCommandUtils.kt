package com.github.toncherami.mpd.web.utils

import java.lang.IllegalStateException

enum class MpdCommand(val code: String) {

    STATUS("status"),
    PLAYLISTINFO("playlistinfo");

}

class MpdCommandBuilder {

    private var command: MpdCommand? = null
    private val arguments: MutableList<String> = mutableListOf()

    fun command(mpdCommand: MpdCommand): MpdCommandBuilder = this.apply {
        command = mpdCommand
    }

    fun argument(argument: String): MpdCommandBuilder = this.apply {
        arguments.add(argument)
    }

    fun build(): String {
        val commandCode = command?.code
            ?: throw IllegalStateException("MPD command must be specified before calling build")

        return commandCode + " " + arguments.joinToString(" ")
    }

}
