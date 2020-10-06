package com.github.toncherami.mpd.web.adapter.gateways.base

interface TcpGateway {

    fun send(message: String): String

}
