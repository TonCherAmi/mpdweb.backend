package com.github.toncherami.mpd.web.adapter.utils

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
    LSINFO("lsinfo"),
    COUNT("count"),
    SEARCH("search"),
    ALBUMART("albumart");
    COMMAND_LIST_BEGIN("command_list_begin"),
    COMMAND_LIST_END("command_list_end"),

}

enum class MpdBoolean(val value: String) {

    TRUE("1"),
    FALSE("0")

}

class MpdCommandBuilder private constructor(mpdCommand: MpdCommand) {

    private val stringBuilder = StringBuilder()

    init {
        stringBuilder.append(mpdCommand.value)
            .append(' ')
    }

    private val arguments: MutableList<String> = mutableListOf()

    fun command(mpdCommand: MpdCommand): MpdCommandBuilder = this.apply {
        stringBuilder
            .append(
                joinArguments()
            )
            .append('\n')
            .append(mpdCommand.value)
            .append(' ')

        arguments.clear()
    }

    fun argument(argument: String): MpdCommandBuilder = this.apply {
        arguments.add(argument)
    }

    fun build(): String {
        return "$stringBuilder " + joinArguments()
    }

    private fun joinArguments() = arguments.joinToString(" ") {
        "\"$it\""
    }

    companion object {

        fun command(mpdCommand: MpdCommand) = MpdCommandBuilder(mpdCommand)

    }

}
