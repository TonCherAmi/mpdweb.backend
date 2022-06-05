package com.github.toncherami.mpd.web.adapter.utils

import com.fasterxml.jackson.databind.ObjectMapper
import com.github.toncherami.mpd.web.adapter.deserializers.data.MpdError
import com.github.toncherami.mpd.web.adapter.deserializers.data.MpdResponse
import com.github.toncherami.mpd.web.adapter.exceptions.MpdException
import com.github.toncherami.mpd.web.adapter.gateways.base.TcpGateway
import com.github.toncherami.mpd.web.common.data.Either

class MpdRequestHandler(val tcpGateway: TcpGateway, val objectMapper: ObjectMapper) {

    inline fun <reified T> retrieve(
        mpdCommand: MpdCommand,
        action: MpdCommandBuilder.() -> Unit = { },
    ): T {
        return MpdCommandBuilder.command(mpdCommand)
            .apply(action)
            .build()
            .let(this::retrieve)
    }

    inline fun <reified T> retrieve(message: String): T {
        val response = tcpGateway.send(message).getOrThrow()

        return objectMapper.convertValue(response.pairs.toMap(), T::class.java)
    }

    inline fun <reified T> retrieveList(
        mpdCommand: MpdCommand,
        action: MpdCommandBuilder.() -> Unit = { },
    ): List<T> {
        return MpdCommandBuilder.command(mpdCommand)
            .apply(action)
            .build()
            .let(::retrieveList)
    }

    inline fun <reified T> retrieveList(message: String): List<T> {
        val response = tcpGateway.send(message).getOrThrow()

        if (response.pairs.isEmpty()) {
            return emptyList()
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

        return items.mapNotNull {
            runCatching {
                objectMapper.convertValue(it, T::class.java)
            }.getOrNull()
        }
    }

    inline fun <reified T> retrieveBinary(
        mpdCommand: MpdCommand,
        action: MpdCommandBuilder.() -> Unit = { },
    ): Pair<T, ByteArray> {
        return MpdCommandBuilder.command(mpdCommand)
            .apply(action)
            .build()
            .let(::retrieveBinary)
    }

    inline fun <reified T> retrieveBinary(message: String): Pair<T, ByteArray> {
        val response = tcpGateway.send(message).getOrThrow()

        val binary = response.binary.firstOrNull()
            ?: byteArrayOf()

        return Pair(
            objectMapper.convertValue(response.pairs.toMap(), T::class.java),
            binary,
        )
    }

    fun perform(
        mpdCommand: MpdCommand,
        action: MpdCommandBuilder.() -> Unit = { },
    ) {
        return MpdCommandBuilder.command(mpdCommand)
            .apply(action)
            .build()
            .let(this::perform)
    }

    fun perform(message: String) {
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
