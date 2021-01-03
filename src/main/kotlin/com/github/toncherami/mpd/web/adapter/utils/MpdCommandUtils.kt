package com.github.toncherami.mpd.web.adapter.utils

import java.lang.IllegalStateException

enum class MpdCommand(val value: String) {

    ADD("add"),
    IDLE("idle"),
    PING("ping"),
    PLAY("play"),
    STOP("stop"),
    PAUSE("pause"),
    NEXT("next"),
    PREVIOUS("previous"),
    STATUS("status"),
    PASSWORD("password"),
    PLAYLISTINFO("playlistinfo"),
    CLEAR("clear"),
    UPDATE("update"),
    LSINFO("lsinfo");

}

enum class MpdBoolean(val value: String) {

    TRUE("1"),
    FALSE("0")

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
        val commandCode = command?.value
            ?: throw IllegalStateException("MPD command must be specified before calling build")

        return "$commandCode " + arguments.joinToString(" ") {
            "\"$it\""
        }
    }

}
