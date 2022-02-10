package com.github.toncherami.mpd.web.adapter.utils

import com.fasterxml.jackson.core.type.TypeReference
import com.fasterxml.jackson.databind.ObjectMapper
import com.github.toncherami.mpd.web.adapter.deserializers.data.MpdError
import com.github.toncherami.mpd.web.adapter.deserializers.data.MpdResponse
import com.github.toncherami.mpd.web.adapter.exceptions.MpdException
import com.github.toncherami.mpd.web.adapter.gateways.base.TcpGateway
import com.github.toncherami.mpd.web.common.data.Either
import java.lang.IllegalArgumentException

class MpdRequestHandler(val tcpGateway: TcpGateway, val objectMapper: ObjectMapper) {

    inline fun <reified T> get(
        mpdCommand: MpdCommand,
        action: MpdCommandBuilder.() -> Unit = { },
    ): T {
        return MpdCommandBuilder()
            .command(mpdCommand)
            .apply(action)
            .build()
            .let(this::get)
    }

    inline fun <reified T> get(message: String): T {
        val response = tcpGateway.send(message).getOrThrow()

        return objectMapper.convertValue(response.pairs.toMap(), T::class.java)
    }

    inline fun <reified T : List<*>> getList(
        mpdCommand: MpdCommand,
        action: MpdCommandBuilder.() -> Unit = { },
    ): T {
        return MpdCommandBuilder()
            .command(mpdCommand)
            .apply(action)
            .build()
            .let(::getList)
    }

    inline fun <reified T : List<*>> getList(message: String): T {
        val response = tcpGateway.send(message).getOrThrow()

        if (response.pairs.isEmpty()) {
            return objectMapper.convertValue(response.pairs, T::class.java)
        }

        val items = mutableListOf<Map<String, String>>()

        val (firstKey) = response.pairs.first()

        val accumulator = mutableListOf(
            response.pairs.first()
        )

        for (pair in response.pairs.drop(1)) {
            if (pair.first == firstKey) {
                items.add(accumulator.toMap())

                accumulator.clear()
            }

            accumulator.add(pair)
        }

        items.add(accumulator.toMap())

        return objectMapper.convertValue(items, object : TypeReference<T>() {})
    }

    inline fun <reified T> getBinary(
        mpdCommand: MpdCommand,
        action: MpdCommandBuilder.() -> Unit = { },
    ): Pair<T, ByteArray> {
        return MpdCommandBuilder()
            .command(mpdCommand)
            .apply(action)
            .build()
            .let(::getBinary)
    }

    inline fun <reified T> getBinary(message: String): Pair<T, ByteArray> {
        val response = tcpGateway.send(message).getOrThrow()

        val binary = response.binary.firstOrNull()
            ?: throw IllegalArgumentException("No binary data received in response to message '$message'")

        return Pair(
            objectMapper.convertValue(response.pairs.toMap(), T::class.java),
            binary,
        )
    }

    fun perform(
        mpdCommand: MpdCommand,
        action: MpdCommandBuilder.() -> Unit = { },
    ) {
        return MpdCommandBuilder()
            .command(mpdCommand)
            .apply(action)
            .build()
            .let(this::perform)
    }

    private fun perform(message: String) {
        tcpGateway.send(message).validate()
    }

}

fun Either<MpdError, MpdResponse>.validate() {
    if (this is Either.Left) {
        throw value.toMpdException()
    }
}

fun Either<MpdError, MpdResponse>.getOrThrow(): MpdResponse {
    return when (this) {
        is Either.Right -> {
            value
        }

        is Either.Left -> {
            throw value.toMpdException()
        }
    }
}

fun MpdError.toMpdException(): MpdException {
    return MpdException(
        code = code,
        message = message,
        command = command,
        commandIndex = commandIndex,
    )
}
