package com.github.toncherami.mpd.web.adapter.gateways.base

import com.github.toncherami.mpd.web.adapter.deserializers.data.MpdError
import com.github.toncherami.mpd.web.adapter.deserializers.data.MpdResponse
import com.github.toncherami.mpd.web.common.data.Either

interface TcpGateway {

    fun send(message: String): Either<MpdError, MpdResponse>

}
