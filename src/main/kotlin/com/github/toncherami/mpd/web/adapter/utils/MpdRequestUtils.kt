package com.github.toncherami.mpd.web.adapter.utils

import com.fasterxml.jackson.databind.ObjectMapper
import com.github.toncherami.mpd.web.adapter.gateways.base.TcpGateway

class MpdRequestHandler(val tcpGateway: TcpGateway, val objectMapper: ObjectMapper) {

    inline fun <reified T> performRequest(
        mpdCommand: MpdCommand,
        action: MpdCommandBuilder.() -> Unit = { }
    ): T {
        return MpdCommandBuilder()
            .command(mpdCommand)
            .apply(action)
            .build()
            .let(::performRequest)
    }

    inline fun <reified T> performRequest(message: String): T {
        return tcpGateway
            .send(message)
            .let(::parseResponse)
            .also(this::validateResponseStatus)
            .let { objectMapper.convertValue(it.data, T::class.java) }
    }

    inline fun <reified T> performListRequest(
        mpdCommand: MpdCommand,
        action: MpdCommandBuilder.() -> Unit = { }
    ): List<T> {
        return MpdCommandBuilder()
            .command(mpdCommand)
            .apply(action)
            .build()
            .let(::performListRequest)
    }

    inline fun <reified T> performListRequest(message: String): List<T> {
        return tcpGateway
            .send(message)
            .let(::parseListResponse)
            .also(this::validateResponseStatus)
            .let(MpdResponse<List<Map<String, String>>>::data)
            .map { objectMapper.convertValue(it, T::class.java) }
    }

    fun <T> validateResponseStatus(mpdResponse: MpdResponse<T>) = apply {
        if (mpdResponse.status != "OK") {
            throw IllegalStateException("Invalid response status")
        }
    }

}
