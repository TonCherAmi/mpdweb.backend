package com.github.toncherami.mpd.web.status.services

import com.github.toncherami.mpd.web.status.dto.Status

interface StatusService {

    fun get(): Status
    fun send(status: Status)

}
