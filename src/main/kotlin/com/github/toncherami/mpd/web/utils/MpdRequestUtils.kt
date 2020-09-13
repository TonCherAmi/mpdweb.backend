package com.github.toncherami.mpd.web.utils

import com.fasterxml.jackson.databind.ObjectMapper
import com.github.toncherami.mpd.web.tcpclients.MpdTcpClient

class MpdRequestHandler(val mpdTcpClient: MpdTcpClient, val objectMapper: ObjectMapper) {

    inline fun <reified T> performRequest(mpdCommand: MpdCommand): T {
        return performRequest {
            command(mpdCommand)
        }
    }

    inline fun <reified T> performRequest(action: MpdCommandBuilder.() -> Unit): T {
        return MpdCommandBuilder()
            .apply(action)
            .build()
            .let(::performRequest)
    }

    inline fun <reified T> performRequest(message: String): T {
        return mpdTcpClient
            .sendMessage(message)
            .let(::parseResponse)
            .also(this::validateResponseStatus)
            .let { objectMapper.convertValue(it.data, T::class.java) }
    }

    inline fun <reified T> performListRequest(mpdCommand: MpdCommand): List<T> {
        return performListRequest {
            command(mpdCommand)
        }
    }

    inline fun <reified T> performListRequest(action: MpdCommandBuilder.() -> Unit): List<T> {
        return MpdCommandBuilder()
            .apply(action)
            .build()
            .let(::performListRequest)
    }

    inline fun <reified T> performListRequest(message: String): List<T> {
        return mpdTcpClient
            .sendMessage(message)
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
